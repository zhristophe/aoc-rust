use std::{collections::HashMap, env, fs, path::PathBuf, time::Duration};

use crossterm::{
    cursor,
    event::{Event, KeyCode, KeyEvent, read},
    execute,
    terminal::{self, ClearType},
};
use reqwest::{
    Client, StatusCode,
    header::{COOKIE, HeaderMap, HeaderValue, USER_AGENT},
};
use tokio;

pub mod graph;
pub mod grid;
pub mod parse;
pub mod prelude;

pub use prelude::v1::*;

fn find_data_dir() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let mut current = env::current_dir()?;
    for _ in 0..5 {
        let data_path = current.join("data");
        if data_path.exists() && data_path.is_dir() {
            return Ok(data_path);
        }
        if !current.pop() {
            break;
        }
    }
    Err("找不到'data'目录. 请确保你在 workspace 中运行代码.".into())
}

pub fn read_input(module_path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let parts = module_path.split("::").collect::<Vec<_>>();

    // 找到以 "aoc" 开头的部分作为年份，以 "day" 开头的部分作为日期
    let year_part = parts
        .iter()
        .find(|s| s.starts_with("aoc"))
        .ok_or("Cannot parse year")?;
    let day_part = parts
        .iter()
        .find(|s| s.starts_with("day"))
        .ok_or("Cannot parse day")?;

    let year = &year_part[3..]; // 去掉 "aoc"，拿到 "2023"
    let day = &day_part[3..]; // 去掉 "day"，拿到 "01"

    // 保存路径形如: data/2023/01.txt
    let data_dir = find_data_dir()?;
    let file_path = data_dir.join(year).join(format!("{}.txt", day));

    if !file_path.exists() {
        let input = tokio::runtime::Runtime::new()?.block_on(download_input(year, day))?;

        if let Some(parent) = file_path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(&file_path, &input)?;

        return Ok(input);
    }

    Ok(fs::read_to_string(file_path)?)
}

const PROJECT_REPO: &str = "github.com/zhristophe/aoc-rust";

pub async fn download_input(year: &str, day: &str) -> Result<String, Box<dyn std::error::Error>> {
    // 读取 Cookie
    let data_dir = find_data_dir()?;
    let session_path = data_dir.join("cookie");

    if !session_path.exists() {
        return Err(format!("找不到 Cookie 文件，请检查路径: {:?}", session_path).into());
    }

    let session = fs::read_to_string(&session_path)?.trim().to_string();

    // 构造 User-Agent
    let user_agent_str = format!("{}", PROJECT_REPO);
    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, HeaderValue::from_str(&user_agent_str)?);

    // 创建客户端
    let client = Client::builder()
        .default_headers(headers)
        .timeout(Duration::from_secs(10))
        .http1_only() // 某些情况下有助于绕过指纹检测，可保留
        .build()?;

    // 去除 day 的前导 0 (API 要求 day/1 而不是 day/01)
    let day_num = day.trim_start_matches('0');
    let url = format!("https://adventofcode.com/{}/day/{}/input", year, day_num);

    println!("正在下载 {} 年第 {} 天的题目输入...", year, day_num);

    // 发送请求
    let response = client
        .get(&url)
        .header(COOKIE, format!("session={}", session))
        .send()
        .await?;

    let status = response.status();

    if status == StatusCode::ACCEPTED {
        // 202 代表被 AWS WAF 拦截并要求进行人机验证
        return Err("下载失败: 请求被 WAF 拦截 (状态码 202)。\n原因: 你的 IP 可能被标记为云服务器/非住宅 IP。\n建议: 请在本地 Windows/Mac 电脑运行，或手动复制 input 文件。".into());
    } else if status == StatusCode::FORBIDDEN || status == StatusCode::BAD_REQUEST {
        return Err(
            "下载失败: Cookie 无效或已过期，请在浏览器重新登录并更新 data/cookie 文件。".into(),
        );
    } else if status.is_success() {
        let text = response.text().await?;

        println!("下载成功！");
        return Ok(text);
    }

    // 其他未知错误
    Err(format!("请求失败，未知状态码: {}", status).into())
}

struct Guard<F>(F)
where
    F: FnOnce() + Copy;

impl<F> Drop for Guard<F>
where
    F: FnOnce() + Copy,
{
    fn drop(&mut self) {
        self.0()
    }
}

pub fn wait_key() -> Option<KeyCode> {
    let _guard = Guard(|| terminal::disable_raw_mode().unwrap());

    terminal::enable_raw_mode().unwrap();
    if let Event::Key(KeyEvent { code, .. }) = read().unwrap() {
        Some(code)
    } else {
        None
    }
}

pub fn clear_screen() {
    execute!(std::io::stdout(), terminal::Clear(ClearType::All)).unwrap();
    execute!(std::io::stdout(), cursor::MoveTo(0, 0)).unwrap();
}

#[derive(Debug)]
pub struct NamePool {
    map: HashMap<String, usize>,
    pool: Vec<String>,
}

impl NamePool {
    pub fn new() -> Self {
        NamePool {
            map: HashMap::new(),
            pool: Vec::new(),
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        NamePool {
            map: HashMap::with_capacity(capacity),
            pool: Vec::with_capacity(capacity),
        }
    }

    pub fn id(&mut self, name: impl AsRef<str>) -> usize {
        let name = name.as_ref().to_string();
        self.map
            .entry(name.clone())
            .or_insert_with(|| {
                let id = self.pool.len();
                self.pool.push(name);
                id
            })
            .clone()
    }

    pub fn get_id(&self, name: impl AsRef<str>) -> Option<usize> {
        self.map.get(name.as_ref()).cloned()
    }

    pub fn name(&self, id: usize) -> Option<&str> {
        self.pool.get(id).map(|s| s.as_str())
    }

    pub fn names(&self) -> impl Iterator<Item = &str> {
        self.pool.iter().map(|s| s.as_str())
    }

    pub fn len(&self) -> usize {
        self.pool.len()
    }

    pub fn contains(&self, name: impl AsRef<str>) -> bool {
        self.map.contains_key(name.as_ref())
    }

    pub fn reserve(&mut self, n: usize) {
        self.map.reserve(n);
        self.pool.reserve(n);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_download_input() {
        let text = download_input("2025", "01").await.unwrap();
        println!("{}", text);
    }

    #[test]
    fn test_point() {
        let p = Point::from((1, 2));
        assert_eq!(p.move_to(Direction::Up), Point::from((0, 2)));
        assert_eq!(p.move_to(Direction::Down), Point::from((2, 2)));
        assert_eq!(p.move_to(Direction::Left), Point::from((1, 1)));
        assert_eq!(p.move_to(Direction::Right), Point::from((1, 3)));
    }

    #[test]
    fn test_bfs() {
        let map = r"
S#E
.#.
...
"
        .trim();
        let map: Vec<Vec<char>> = map.lines().map(|s| s.chars().collect()).collect();
        let map = Grid::from(map);
        let stt = map.find_point('S').unwrap();
        let end = map.find_point('E').unwrap();
        let mut steps = Grid::new(map.size(), usize::MAX);
        steps.get_mut(stt).map(|v| *v = 0);
        map.bfs_iter(stt)
            .skip_tiles(&'#')
            .on_discover(|old, new| {
                let &old_val = steps.get(old).unwrap();
                steps.get_mut(new).map(|v| *v = (*v).min(old_val + 1));
            })
            .run_with_target(end);
        assert_eq!(
            steps,
            Grid::from(vec![
                vec![0, usize::MAX, 6],
                vec![1, usize::MAX, 5],
                vec![2, 3, 4]
            ])
        );
    }
}
