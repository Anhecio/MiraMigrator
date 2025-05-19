use super::Interface;
use crate::LOGO;
use crate::VERSION;
use crate::api::courseforge::download_curseforge_mod;
use crate::api::modrinth::download_modrinth_mod;
use crate::scan::scan;
use crate::utils::loader::{self, detect_mod, get_mod_id, get_mod_version};
use crate::utils::version::validate_version;
use std::time::Instant;

pub struct EnUsInterface;

impl EnUsInterface {
    pub fn new() -> Self {
        EnUsInterface
    }
}

impl Interface for EnUsInterface {
    fn init(&self) {
        // Initialization
        println!("Initializing MiraMigrator...");
        scan::create_backup_folder();
        println!("MiraMigrator initialization complete.");
    }

    fn start(&self) {
        println!("{}", LOGO);
        println!("Welcome to MiraMigrator v{}", VERSION);
        println!("Created by: Lumira | QQ: 2301385546 | QQ Group: 000000000");
        println!("Please enter the target Minecraft version (e.g., 1.20.6):");
        let mut version = String::new();
        loop {
            version.clear();
            std::io::stdin()
                .read_line(&mut version)
                .expect("Failed to read user input.");
            if validate_version(version.trim()) {
                break;
            } else {
                println!("Invalid version, please re-enter:");
            }
        }
        print!("Target Minecraft version: {}", version);
        // Scanning Mods directory
        println!("Scanning Mods directory...");
        let jar_files = scan::get_jar_files();
        if jar_files.is_empty() {
            println!("No Jar files found, please check the Mods directory.");
            return;
        }
        let jar_files = scan::filter_valid_mods(jar_files);
        if jar_files.is_empty() {
            println!("No valid mods found, please check the Mods directory.");
            return;
        }
        println!("Found {} mod files, listed below:", jar_files.len());
        for jar_file in &jar_files {
            println!(" - {}", jar_file.file_name().to_string_lossy());
        }
        println!(
            "Do you confirm to proceed with the version migration? The original mods will be backed up to the 'backup' folder in the current directory (y/n):"
        );
        let mut confirm = String::new();
        loop {
            confirm.clear();
            std::io::stdin()
                .read_line(&mut confirm)
                .expect("Failed to read user input.");
            if confirm.trim().eq_ignore_ascii_case("y") {
                break;
            } else if confirm.trim().eq_ignore_ascii_case("n") {
                println!("Version migration canceled.");
                return;
            } else {
                println!("Invalid input, please re-enter (y/n):");
            }
        }
        // Backing up Mod files
        println!("Starting to back up Mod files...");
        let start_time = Instant::now();
        for jar_file in &jar_files {
            let file_name_osstr = jar_file.file_name();
            let file_name = file_name_osstr.to_string_lossy();
            let backup_path = format!("backup/{}", file_name);
            std::fs::copy(jar_file.path(), backup_path).expect("Failed to back up Mod file.");
        }
        let elapsed_time = start_time.elapsed();
        println!(
            "File backup complete, took: {:.2?}s",
            elapsed_time.as_secs_f64()
        );

        // Executing version migration
        println!("Starting version migration...");
        // Failed mods list
        let mut failed_mods = Vec::new();
        // Successful mods list
        let mut success_mods = Vec::new();
        // Cache directory
        let cache_path = std::path::Path::new("cache");
        let start_time = Instant::now();
        for jar_file in &jar_files {
            let jar_file_path = jar_file.path();
            let mod_id = get_mod_id(&jar_file_path).ok().flatten();
            let mod_version = get_mod_version(&jar_file_path).ok().flatten();
            let mod_loader = detect_mod(&jar_file_path).ok().flatten();
            if mod_id.is_none() && mod_version.is_none() {
                std::fs::remove_file(jar_file.path()).expect("Failed to delete original Jar file.");
                failed_mods.push(jar_file);
            } else {
                std::fs::remove_file(jar_file.path()).expect("Failed to delete original Jar file.");
                let old_mod_id = mod_id.unwrap();
                let old_mod_version = mod_version.unwrap();
                let loader = mod_loader.unwrap();
                let cache_path = std::path::Path::new("cache");
                let time = Instant::now();
                let new_mod_version;
                // Downloading Mod
                // CourseForge API Key
                let api_key = "";
                match loader {
                    loader::ModLoader::Fabric => {
                        println!("Downloading Fabric Mod {}...", old_mod_id);
                        let result =
                            download_modrinth_mod(&old_mod_id, &version, "fabric", cache_path);
                        match result {
                            Ok(path) => {
                                new_mod_version = get_mod_version(&path).unwrap();
                            }
                            Err(error) => {
                                println!("Failed to fetch from Modrinth: {:?}", error);
                                println!("Attempting to fetch from CurseForge...");
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
                                        println!("Failed to fetch from CurseForge: {:?}", error);
                                        println!(
                                            "Fabric Mod {} download failed, please migrate manually.",
                                            old_mod_id
                                        );
                                        failed_mods.push(jar_file);
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                    loader::ModLoader::Forge => {
                        println!("Downloading Forge Mod {}...", old_mod_id);
                        let result =
                            download_modrinth_mod(&old_mod_id, &version, "forge", cache_path);
                        match result {
                            Ok(path) => {
                                new_mod_version = get_mod_version(&path).unwrap();
                            }
                            Err(error) => {
                                println!("Failed to fetch from Modrinth: {:?}", error);
                                println!("Attempting to fetch from CurseForge...");
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
                                        println!("Failed to fetch from CurseForge: {:?}", error);
                                        println!(
                                            "Forge Mod {} download failed, please migrate manually.",
                                            old_mod_id
                                        );
                                        failed_mods.push(jar_file);
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                    loader::ModLoader::Quilt => {
                        println!("Downloading Quilt Mod {}...", old_mod_id);
                        let result =
                            download_modrinth_mod(&old_mod_id, &version, "quilt", cache_path);
                        match result {
                            Ok(path) => {
                                new_mod_version = get_mod_version(&path).unwrap();
                            }
                            Err(error) => {
                                println!("Failed to fetch from Modrinth: {:?}", error);
                                println!("Attempting to fetch from CurseForge...");
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
                                        println!("Failed to fetch from CurseForge: {:?}", error);
                                        println!(
                                            "Quilt Mod {} download failed, please migrate manually.",
                                            old_mod_id
                                        );
                                        failed_mods.push(jar_file);
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                    loader::ModLoader::None => {
                        println!("Unknown mod loader type: {:?}", loader);
                        failed_mods.push(jar_file);
                        continue;
                    }
                }

                let end_time = time.elapsed();
                println!(
                    "{}-{} => {}-{} took {:.2?}s",
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
            eprintln!("Version migration failed: {}", e);
        } else {
            let elapsed_time = start_time.elapsed();
            println!(
                "\nVersion migration complete, took: {:.2?}s",
                elapsed_time.as_secs_f64()
            );
            println!(
                "Successfully migrated {} mods, failed {} mods.",
                success_mods.len(),
                failed_mods.len()
            );
            if !failed_mods.is_empty() {
                println!("Failed mods list (please migrate manually):");
                for jar_file in &failed_mods {
                    println!(" - {}", jar_file.file_name().to_string_lossy());
                }
            }
        }
    }

    fn exit(&self) {
        println!("Press any key to exit MiraMigrator...");
        std::io::stdin()
            .read_line(&mut String::new())
            .expect("Failed to read line");
    }
}
