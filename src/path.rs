use std::fs;
use std::path::PathBuf;

/// 规范化路径，处理 Windows UNC 路径前缀。
///
/// 对于存在的目录路径，通过 `fs::canonicalize` 获取绝对路径，
/// 并移除 Windows 上的 `\\?\` UNC 前缀。
pub fn normalize(path: &str) -> String {
    let path_buf = PathBuf::from(path);

    if let Ok(canonical) = fs::canonicalize(&path_buf) {
        let path_str = canonical.to_string_lossy();
        // 移除 Windows UNC 前缀 \\?\
        if let Some(stripped) = path_str.strip_prefix(r"\\?\") {
            return stripped.to_string();
        }
        return path_str.into_owned();
    }

    path.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn normalize_existing_dir() {
        let temp = env::temp_dir();
        let temp_str = temp.to_string_lossy();
        let result = normalize(&temp_str);
        // 结果不应以 \\?\ 开头
        assert!(!result.starts_with(r"\\?\"));
        // 结果应为非空字符串
        assert!(!result.is_empty());
    }

    #[test]
    fn normalize_nonexistent_path_returns_original() {
        let fake = r"Z:\nonexistent\path\12345";
        assert_eq!(normalize(fake), fake);
    }
}
