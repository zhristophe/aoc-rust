use std::collections::HashMap;

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
    fn test_name_pool() {
        let mut pool = NamePool::new();
        assert_eq!(pool.id("foo"), 0);
        assert_eq!(pool.id("bar"), 1);
        assert_eq!(pool.id("foo"), 0);
        assert_eq!(pool.name(0), Some("foo"));
        assert_eq!(pool.name(1), Some("bar"));
        assert_eq!(pool.get_id("foo"), Some(0));
        assert_eq!(pool.get_id("baz"), None);
        assert_eq!(pool.len(), 2);
        assert!(pool.contains("foo"));
        assert!(!pool.contains("baz"));
    }
}
