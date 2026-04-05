use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

/// 路径存储结构体，用于序列化和持久化路径别名数据。
#[derive(Serialize, Deserialize, Debug)]
pub struct PathStore {
    paths: HashMap<String, String>,
}

impl PathStore {
    /// 创建空的 PathStore。
    pub fn new() -> Self {
        Self {
            paths: HashMap::new(),
        }
    }

    /// 从配置文件加载数据。文件不存在或格式错误时返回空实例。
    pub fn load() -> Self {
        let Some(config_path) = Self::get_config_path() else {
            return Self::new();
        };
        fs::read_to_string(&config_path)
            .ok()
            .and_then(|content| serde_json::from_str(&content).ok())
            .unwrap_or_else(Self::new)
    }

    /// 持久化数据到配置文件。
    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let config_path =
            Self::get_config_path().ok_or("无法确定配置目录")?;
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)?;
        }
        let content = serde_json::to_string_pretty(self)?;
        fs::write(config_path, content)?;
        Ok(())
    }

    /// 获取配置文件路径。
    ///
    /// 平台示例：
    /// - Linux: `~/.config/m/paths.json`
    /// - Windows: `%APPDATA%\m\config\paths.json`
    /// - macOS: `~/Library/Application Support/com.m.m/paths.json`
    fn get_config_path() -> Option<PathBuf> {
        directories::ProjectDirs::from("com", "m", "m")
            .map(|dirs| dirs.config_dir().join("paths.json"))
    }

    /// 添加或更新路径别名。已存在的别名会被覆盖。
    pub fn add(&mut self, alias: String, path: String) {
        self.paths.insert(alias, path);
    }

    /// 移除别名。返回是否实际移除了条目。
    pub fn remove(&mut self, alias: &str) -> bool {
        self.paths.remove(alias).is_some()
    }

    /// 根据别名获取路径。
    pub fn get(&self, alias: &str) -> Option<&str> {
        self.paths.get(alias).map(String::as_str)
    }

    /// 列出所有别名，返回是否有数据。
    pub fn list(&self) -> bool {
        if self.paths.is_empty() {
            println!("没有保存的路径。");
            return false;
        }
        println!("已保存的路径：");
        for (alias, path) in &self.paths {
            println!("  {} -> {}", alias, path);
        }
        true
    }
}

impl Default for PathStore {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_and_get() {
        let mut store = PathStore::new();
        store.add("proj".into(), "/home/user/project".into());
        assert_eq!(store.get("proj"), Some("/home/user/project"));
    }

    #[test]
    fn add_overwrites_existing() {
        let mut store = PathStore::new();
        store.add("proj".into(), "/old/path".into());
        store.add("proj".into(), "/new/path".into());
        assert_eq!(store.get("proj"), Some("/new/path"));
    }

    #[test]
    fn remove_existing() {
        let mut store = PathStore::new();
        store.add("proj".into(), "/some/path".into());
        assert!(store.remove("proj"));
        assert_eq!(store.get("proj"), None);
    }

    #[test]
    fn remove_nonexistent() {
        let mut store = PathStore::new();
        assert!(!store.remove("nope"));
    }

    #[test]
    fn get_nonexistent() {
        let store = PathStore::new();
        assert_eq!(store.get("nope"), None);
    }

    #[test]
    fn serialize_roundtrip() {
        let mut store = PathStore::new();
        store.add("a".into(), "/path/a".into());
        store.add("b".into(), "/path/b".into());

        let json = serde_json::to_string(&store).unwrap();
        let loaded: PathStore = serde_json::from_str(&json).unwrap();

        assert_eq!(loaded.get("a"), Some("/path/a"));
        assert_eq!(loaded.get("b"), Some("/path/b"));
    }
}
