use std::time::Instant;
use crate::scan::scan;
use crate::utils::version::validate_version;
use crate::utils::loader::{get_mod_version, get_mod_id};
use crate::VERSION;
use crate::LOGO;
pub struct ZhCnInterface;

impl ZhCnInterface {
    pub fn new() -> Self {
        Self {}
    }

    pub fn init(&self) {        
        // 初始化
        println!("正在初始化 MiraMigrator 中...");
        scan::create_backup_folder();
        println!("MiraMigrator 初始化完毕.");
    }
    pub fn start(&self) {
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
        println!("发现 {} 个 Mod 文件, 列表如下:", jar_files.len(), );
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
        let start_time = Instant::now();
        for jar_file in &jar_files {
            let jar_file_path = jar_file.path();
            let mod_id = get_mod_id(&jar_file_path).ok().flatten();
            let mod_version = get_mod_version(&jar_file_path).ok().flatten();
            if mod_id.is_none() && mod_version.is_none() {
                failed_mods.push(jar_file);
            } else {
                std::fs::remove_file(jar_file.path()).expect("删除原 Jar 文件失败.");   
                let old_mod_id = mod_id.unwrap();
                let old_mod_version = mod_version.unwrap();
                let time = Instant::now();
                


                // 下载Mod




                let end_time = time.elapsed();
                println!("{}-{} => {}-{} 耗时 {:.2?}s", old_mod_id, old_mod_version, old_mod_id, old_mod_version, end_time.as_secs_f64());
                success_mods.push(jar_file);
            }
        }
        let elapsed_time = start_time.elapsed();
        println!("版本迁移完成, 共耗时: {:.2?}s", elapsed_time.as_secs_f64());
        println!("成功迁移 {} 个 Mod, 失败 {} 个 Mod.", success_mods.len(), failed_mods.len());
        if !failed_mods.is_empty() {
            println!("失败的 Mod 列表如下 (请手动迁移):");
            for jar_file in &failed_mods {
                println!(" - {}", jar_file.file_name().to_string_lossy());
            }
        }
    }
    pub fn exit(&self) {
        println!("按任意键退出MiraMigrator...");
        std::io::stdin()
            .read_line(&mut String::new())
            .expect("Failed to read line");
    }
}