use super::Interface;
use crate::CURSEFORGE_API_KEY;
use crate::LOGO;
use crate::VERSION;
use crate::api::curseforge::download_curseforge_mod;
use crate::api::modrinth::download_modrinth_mod;
use crate::scan::scan;
use crate::utils::loader::{self, detect_mod, get_mod_id, get_mod_version};
use crate::utils::version::validate_version;
use std::time::Instant;
pub struct ZhCnInterface;

impl ZhCnInterface {
    pub fn new() -> Self {
        ZhCnInterface
    }
}

impl Interface for ZhCnInterface {
    fn init(&self) {
        // 初始化
        println!("正在初始化 MiraMigrator 中...");
        scan::create_backup_folder();
        println!("MiraMigrator 初始化完毕.");
    }
    fn start(&self) {
        println!("{}", LOGO);
        println!("欢迎使用 MiraMigrator v{}", VERSION);
        println!("制作: 安禾辞 | QQ: 2301385546 | 交流群: 000000000");
        println!("请输入目标 Minecraft 版本 (格式如 1.20.6):");
        let mut version = String::new();
        loop {
            version.clear();
            std::io::stdin()
                .read_line(&mut version)
                .expect("获取用户输入版本失败.");
            if validate_version(version.trim()) {
                break;
            } else {
                println!("版本不合法, 请重新输入:");
            }
        }
        print!("目标 Minecraft 版本为: {}", version);
        // 扫描Mods目录
        println!("正在扫描 Mods 目录...");
        let jar_files = scan::get_jar_files();
        if jar_files.is_empty() {
            println!("未发现任何 Jar 文件, 请检查 Mods 目录.");
            return;
        }
        let jar_files = scan::filter_valid_mods(jar_files);
        if jar_files.is_empty() {
            println!("未发现任何有效 Mod, 请检查 Mods 目录.");
            return;
        }
        println!("发现 {} 个 Mod 文件, 列表如下:", jar_files.len(),);
        for jar_file in &jar_files {
            println!(" - {}", jar_file.file_name().to_string_lossy());
        }
        println!("是否确认执行版本迁移? 原 Mod 将会备份至当前目录下 backup 文件夹中 (y/n):");
        let mut confirm = String::new();
        loop {
            confirm.clear();
            std::io::stdin()
                .read_line(&mut confirm)
                .expect("获取用户输入确认失败.");
            if confirm.trim().eq_ignore_ascii_case("y") {
                break;
            } else if confirm.trim().eq_ignore_ascii_case("n") {
                println!("已取消版本迁移.");
                return;
            } else {
                println!("输入不合法, 请重新输入 (y/n):");
            }
        }
        // 备份 Mod 文件
        println!("开始备份 Mod 文件...");
        let start_time = Instant::now();
        for jar_file in &jar_files {
            let file_name_osstr = jar_file.file_name();
            let file_name = file_name_osstr.to_string_lossy();
            let backup_path = format!("backup/{}", file_name);
            std::fs::copy(jar_file.path(), backup_path).expect("备份 Mod 文件失败.");
        }
        let elapsed_time = start_time.elapsed();
        println!("文件备份完成, 共耗时: {:.2?}s", elapsed_time.as_secs_f64());

        // 执行版本迁移
        println!("开始执行版本迁移...");
        // 失败的 Mod列表
        let mut failed_mods = Vec::new();
        // 成功的 Mod列表
        let mut success_mods = Vec::new();
        // 缓存目录
        let cache_path = std::path::Path::new("cache");
        let start_time = Instant::now();
        for jar_file in &jar_files {
            let jar_file_path = jar_file.path();
            let mod_id = get_mod_id(&jar_file_path).ok().flatten();
            let mod_version = get_mod_version(&jar_file_path).ok().flatten();
            let mod_loader = detect_mod(&jar_file_path).ok().flatten();
            if mod_id.is_none() | mod_version.is_none() {
                println!("id {}", mod_id.is_none());
                println!("version {}", mod_version.is_none());
                println!("无法识别 Mod {}, 请手动迁移.", jar_file.file_name().to_string_lossy());
                std::fs::remove_file(jar_file.path()).expect("删除原 Jar 文件失败.");
                failed_mods.push(jar_file);
            } else  {
                std::fs::remove_file(jar_file.path()).expect("删除原 Jar 文件失败.");
                let old_mod_id = mod_id.unwrap();
                let old_mod_version = mod_version.unwrap();
                let loader = mod_loader.unwrap();
                let cache_path = std::path::Path::new("cache");
                let time = Instant::now();
                let new_mod_version;
                // 下载Mod
                // CourseForge API Key
                let api_key = CURSEFORGE_API_KEY;
                match loader {
                    loader::ModLoader::Fabric => {
                        println!("正在下载 Fabric Mod {}...", old_mod_id);
                        let result =
                            download_modrinth_mod(&old_mod_id, &version, "fabric", cache_path);
                        match result {
                            Ok(path) => {
                                new_mod_version = get_mod_version(&path).unwrap();
                            }
                            Err(error) => {
                                println!("从 Modrinth 拉取失败: {:?}", error);
                                println!("开始尝试从 CurseForge 拉取...");
                                let result = download_curseforge_mod(
                                    &old_mod_id,
                                    &version,
                                    "fabric",
                                    cache_path,
                                    api_key,
                                );
                                match result {
                                    Ok(path) => {
                                        new_mod_version = get_mod_version(&path).unwrap();
                                    }
                                    Err(error) => {
                                        println!("从 CurseForge 拉取失败: {:?}", error);
                                        println!("Fabric Mod {} 下载失败, 请手动迁移.", old_mod_id);
                                        failed_mods.push(jar_file);
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                    loader::ModLoader::Forge => {
                        println!("正在下载 Forge Mod {}...", old_mod_id);
                        let result =
                            download_modrinth_mod(&old_mod_id, &version, "forge", cache_path);
                        match result {
                            Ok(path) => {
                                new_mod_version = get_mod_version(&path).unwrap();
                            }
                            Err(error) => {
                                println!("从 Modrinth 拉取失败: {:?}", error);
                                println!("开始尝试从 CurseForge 拉取...");
                                let result = download_curseforge_mod(
                                    &old_mod_id,
                                    &version,
                                    "forge",
                                    cache_path,
                                    api_key,
                                );
                                match result {
                                    Ok(path) => {
                                        new_mod_version = get_mod_version(&path).unwrap();
                                    }
                                    Err(error) => {
                                        println!("从 CurseForge 拉取失败: {:?}", error);
                                        println!("Fabric Mod {} 下载失败, 请手动迁移.", old_mod_id);
                                        failed_mods.push(jar_file);
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                    loader::ModLoader::Quilt => {
                        println!("正在下载 Quilt Mod {}...", old_mod_id);
                        let result =
                            download_modrinth_mod(&old_mod_id, &version, "quilt", cache_path);
                        match result {
                            Ok(path) => {
                                new_mod_version = get_mod_version(&path).unwrap();
                            }
                            Err(error) => {
                                println!("从 Modrinth 拉取失败: {:?}", error);
                                println!("开始尝试从 CurseForge 拉取...");
                                let result = download_curseforge_mod(
                                    &old_mod_id,
                                    &version,
                                    "quilt",
                                    cache_path,
                                    api_key,
                                );
                                match result {
                                    Ok(path) => {
                                        new_mod_version = get_mod_version(&path).unwrap();
                                    }
                                    Err(error) => {
                                        println!("从 CurseForge 拉取失败: {:?}", error);
                                        println!("Fabric Mod {} 下载失败, 请手动迁移.", old_mod_id);
                                        failed_mods.push(jar_file);
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                    loader::ModLoader::None => {
                        println!("未知的模组加载器类型: {:?}", loader);
                        failed_mods.push(jar_file);
                        continue;
                    }
                }

                let end_time = time.elapsed();
                println!(
                    "{}-{} => {}-{} 耗时 {:.2?}s",
                    old_mod_id,
                    old_mod_version,
                    old_mod_id,
                    new_mod_version.unwrap(),
                    end_time.as_secs_f64()
                );
                success_mods.push(jar_file);
            }
        }
        if let Err(e) = scan::move_files_from_cache_to_current_dir(cache_path) {
            eprintln!("版本迁移失败: {}", e);
        } else {
            let elapsed_time = start_time.elapsed();
            println!(
                "\n版本迁移完成, 共耗时: {:.2?}s",
                elapsed_time.as_secs_f64()
            );
            println!(
                "成功迁移 {} 个 Mod, 失败 {} 个 Mod.",
                success_mods.len(),
                failed_mods.len()
            );
            if !failed_mods.is_empty() {
                println!("失败的 Mod 列表如下 (请手动迁移):");
                for jar_file in &failed_mods {
                    println!(" - {}", jar_file.file_name().to_string_lossy());
                }
            }
        }
    }
    fn exit(&self) {
        println!("按任意键退出MiraMigrator...");
        std::io::stdin()
            .read_line(&mut String::new())
            .expect("Failed to read line");
    }
}
