use crate::utils::loader::is_valid_mod;
use std::fs;
use std::io::Result;
use std::path::Path;

/// 判断模组备份/缓存文件夹是否存在
/// 不存在则创建
pub fn create_backup_folder() {
    if !Path::new("backup").exists() {
        fs::create_dir("backup").unwrap();
    }
    if !Path::new("cache").exists() {
        fs::create_dir("cache").unwrap();
    }
}

/// 获取当前目录中所有Jar文件对象
/// 返回Jar文件对象Vec
pub fn get_jar_files() -> Vec<fs::DirEntry> {
    let mut jar_files = Vec::new();
    for entry in fs::read_dir(".").unwrap() {
        let entry = entry.unwrap();
        if entry.path().extension().unwrap_or_default() == "jar" {
            jar_files.push(entry);
        }
    }
    jar_files
}

/// 从获取到的Vec中筛选出有效的Mod文件
/// 返回有效Mod文件Vec
pub fn filter_valid_mods(jar_files: Vec<fs::DirEntry>) -> Vec<fs::DirEntry> {
    let mut valid_mods = Vec::new();
    for jar_file in jar_files {
        if is_valid_mod(&jar_file) {
            valid_mods.push(jar_file);
        }
    }
    valid_mods
}

/// 将有效的Mod文件移动到当前目录
pub fn move_files_from_cache_to_current_dir(cache_dir: &Path) -> Result<()> {
    // 获取当前目录路径
    let current_dir = std::env::current_dir()?;

    // 遍历 cache 目录中的所有文件
    for entry in fs::read_dir(cache_dir)? {
        let entry = entry?;
        let entry_path = entry.path();

        // 确保是文件而非目录
        if entry_path.is_file() {
            let file_name = entry_path.file_name().unwrap();
            let destination = current_dir.join(file_name);

            // 移动文件到当前目录
            fs::rename(&entry_path, destination)?;
        }
    }

    // 删除 cache 目录及其内容
    fs::remove_dir_all(cache_dir)?;

    Ok(())
}
