use crate::utils::loader::is_valid_mod;
use std::fs;
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
