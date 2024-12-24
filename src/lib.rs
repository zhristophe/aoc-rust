use std::{collections::HashMap, fs, path::Path};

use crossterm::{
    cursor,
    event::{read, Event, KeyCode, KeyEvent},
    execute,
    terminal::{self, ClearType},
};
use tokio;

pub mod graph;
pub mod grid;
pub mod prelude;

pub use prelude::v1::*;

pub fn read_input(src_path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let name = src_path.split("::").last().unwrap();
    let file = format!("data/{}/input", name);
    let file = Path::new(&file);

    if !file.exists() {
        let input = tokio::runtime::Runtime::new()?.block_on(download_input(name))?;
        fs::create_dir_all(file.parent().unwrap())?;
        fs::write(file, input)?;
    }

    Ok(fs::read_to_string(file)?)
}

async fn download_input(name: &str) -> Result<String, Box<dyn std::error::Error>> {
    let session = "data/cookie";
    if !Path::new(session).exists() {
        return Err("cookie not found".into());
    }
    let session = fs::read_to_string(session)?;
    let client = reqwest::Client::new();
    let response = client
        .get(format!(
            "https://adventofcode.com/20{}/day/{}/input",
            &name[..2],
            &name[2..].trim_start_matches('0')
        ))
        .header("COOKIE", format!("session={}", session))
        .send()
        .await?;

    Ok(response.text().await?)
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
