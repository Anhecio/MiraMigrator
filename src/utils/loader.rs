use std::{
    fs,
    io::{self, Read},
    path::Path,
};
use zip::ZipArchive;
use serde_json::Value;

#[derive(Debug, PartialEq)]
pub enum ModLoader {
    Forge,
    Fabric,
    Quilt,
    None,
}

/// 检测 JAR 文件的 mod 加载器类型
pub fn detect_mod(jar_path: &Path) -> io::Result<Option<ModLoader>> {
    let file = fs::File::open(jar_path)?;
    let mut zip = ZipArchive::new(file)?;

    let mut loader = Some(ModLoader::None);

    // Check the metadata files first
    for i in 0..zip.len() {
        let entry = zip.by_index(i)?;
        let entry_name = entry.name().to_lowercase();
        

        match entry_name.as_str() {
            "fabric.mod.json" => {
                loader = Some(ModLoader::Fabric);
                break;
            }
            "meta-inf/mods.toml" | "mcmod.info" => {
                loader = Some(ModLoader::Forge);
                break;
            }
            "quilt.mod.json" => {
                loader = Some(ModLoader::Quilt);
                break;
            }
            _ => {}
        }
    }

    if loader.is_none() && has_mod_annotation(&mut zip)? {
        loader = Some(ModLoader::Forge);
    }

    Ok(loader)
}


/// 检查类文件中的 @Mod 注解来识别 Forge 模组
fn has_mod_annotation(zip: &mut ZipArchive<fs::File>) -> io::Result<bool> {
    let annotation_marker = b"Lnet/minecraftforge/fml/common/Mod;";

    for i in 0..zip.len() {
        let mut entry = zip.by_index(i)?;
        if entry.name().ends_with(".class") {
            let mut buffer = Vec::new();
            entry.read_to_end(&mut buffer)?;

            if buffer
                .windows(annotation_marker.len())
                .any(|w| w == annotation_marker)
            {
                return Ok(true);
            }
        }
    }
    Ok(false)
}

/// 获取 mod 的版本信息
pub fn get_mod_version(jar_path: &Path) -> io::Result<Option<String>> {
    let file = fs::File::open(jar_path)?;
    let mut zip = ZipArchive::new(file)?;

    for i in 0..zip.len() {
        let mut entry = zip.by_index(i)?;
        let entry_name = entry.name().to_lowercase();

        match entry_name.as_str() {
            "fabric.mod.json" => {
                let mut contents = String::new();
                entry.read_to_string(&mut contents)?;
                if let Ok(json) = serde_json::from_str::<Value>(&contents) {
                    if let Some(version) = json.get("version").and_then(|v| v.as_str()) {
                        return Ok(Some(version.to_string()));
                    }
                }
            }
            "meta-inf/mods.toml" => {
                let mut contents = String::new();
                entry.read_to_string(&mut contents)?;
                if let Ok(toml) = contents.parse::<toml::Value>() {
                    if let Some(mods) = toml.get("mods").and_then(|m| m.as_array()) {
                        if let Some(first_mod) = mods.get(0) {
                            if let Some(version) = first_mod.get("version").and_then(|v| v.as_str()) {
                                return Ok(Some(version.to_string()));
                            }
                        }
                    }
                }
            }
            "mcmod.info" => {
                let mut contents = String::new();
                entry.read_to_string(&mut contents)?;
                if let Ok(json) = serde_json::from_str::<Value>(&contents) {
                    if let Some(arr) = json.as_array() {
                        if let Some(first) = arr.get(0) {
                            if let Some(version) = first.get("version").and_then(|v| v.as_str()) {
                                return Ok(Some(version.to_string()));
                            }
                        }
                    }
                }
            }
            "quilt.mod.json" => {
                let mut contents = String::new();
                entry.read_to_string(&mut contents)?;
                if let Ok(json) = serde_json::from_str::<Value>(&contents) {
                    if let Some(version) = json
                        .get("quilt_loader")
                        .and_then(|ql| ql.get("version"))
                        .and_then(|v| v.as_str())
                    {
                        return Ok(Some(version.to_string()));
                    }
                }
            }
            _ => {}
        }
    }
    Ok(None)
}

/// 获取 mod 的 ID
pub fn get_mod_id(jar_path: &Path) -> io::Result<Option<String>> {
    let file = fs::File::open(jar_path)?;
    let mut zip = ZipArchive::new(file)?;

    for i in 0..zip.len() {
        let mut entry = zip.by_index(i)?;
        let entry_name = entry.name().to_lowercase();

        match entry_name.as_str() {
            "fabric.mod.json" => {
                let mut contents = String::new();
                entry.read_to_string(&mut contents)?;
                if let Ok(json) = serde_json::from_str::<Value>(&contents) {
                    if let Some(name) = json.get("id").and_then(|v| v.as_str()) {
                        return Ok(Some(name.to_string()));
                    }
                }
            }
            "meta-inf/mods.toml" => {
                let mut contents = String::new();
                entry.read_to_string(&mut contents)?;
                if let Ok(toml) = contents.parse::<toml::Value>() {
                    if let Some(mods) = toml.get("mods").and_then(|m| m.as_array()) {
                        if let Some(first_mod) = mods.get(0) {
                            if let Some(name) = first_mod.get("modId").and_then(|v| v.as_str()) {
                                return Ok(Some(name.to_string()));
                            }
                        }
                    }
                }
            }
            "mcmod.info" => {
                let mut contents = String::new();
                entry.read_to_string(&mut contents)?;
                if let Ok(json) = serde_json::from_str::<Value>(&contents) {
                    if let Some(arr) = json.as_array() {
                        if let Some(first) = arr.get(0) {
                            if let Some(name) = first.get("modid").and_then(|v| v.as_str()) {
                                return Ok(Some(name.to_string()));
                            }
                        }
                    }
                }
            }
            "quilt.mod.json" => {
                let mut contents = String::new();
                entry.read_to_string(&mut contents)?;
                if let Ok(json) = serde_json::from_str::<Value>(&contents) {
                    if let Some(name) = json.get("id").and_then(|v| v.as_str()) {
                        return Ok(Some(name.to_string()));
                    }
                }
            }
            _ => {}
        }
    }
    Ok(None)
}

/// 判断Jar文件是否为有效的Mod文件
pub fn is_valid_mod(jar_file: &fs::DirEntry) -> bool {
    let path = jar_file.path();
    match detect_mod(&path) {
        Ok(Some(_)) => true,
        _ => false,
    }
}