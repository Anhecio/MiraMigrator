use reqwest::blocking::Client;
use reqwest::header;
use sanitize_filename::sanitize;
use serde_json::Value;
use std::error::Error;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::time::Duration;

pub fn download_curseforge_mod(
    mod_id: &str, // CurseForge mod ID
    mc_version: &str,
    loader: &str,
    output_dir: &Path,
    api_key: &str, // CurseForge API Key
) -> Result<PathBuf, Box<dyn std::error::Error>> {
    // 1. 配置HTTP客户端
    let mut headers = header::HeaderMap::new();
    headers.insert(
        header::ACCEPT,
        header::HeaderValue::from_static("application/json"),
    );
    headers.insert(
        header::USER_AGENT,
        header::HeaderValue::from_static("MiraMigrator/1.0"),
    );
    headers.insert(
        header::AUTHORIZATION,
        header::HeaderValue::from_str(&format!("Bearer {}", api_key))?,
    );

    let client = Client::builder()
        .default_headers(headers)
        .timeout(Duration::from_secs(30))
        .build()?;

    // 2. 验证项目存在
    let project_url = format!("https://addons-ecs.forgesvc.net/api/v2/addon/{}", mod_id);
    println!("Requesting mod data from: {}", project_url); // Debugging log
    let project_res = client.get(&project_url).send()?;

    println!("Response Status: {}", project_res.status()); // Debugging log

    let project_status = project_res.status();
    if !project_status.is_success() {
        let error_text = project_res.text()?;
        println!("Error Response: {}", error_text); // Debugging log
        return Err(format!(
            "The mod does not exist or error occurred: {} {}",
            project_status,
            error_text
        )
        .into());
    }

    // 3. 获取兼容版本
    let versions_url = format!("https://addons-ecs.forgesvc.net/api/v2/addon/{}/files", mod_id);
    println!("Requesting mod versions from: {}", versions_url); // Debugging log
    let versions_res = client.get(&versions_url).send()?;

    let versions_status = versions_res.status();
    println!("Response Status: {}", versions_status); // Debugging log
    if !versions_status.is_success() {
        let error_text = versions_res.text()?;
        println!("Error Response: {}", error_text); // Debugging log
        return Err(format!(
            "Failed to get mod versions: {} {}",
            versions_status,
            error_text
        )
        .into());
    }

    let versions_text = versions_res.text()?;
    let versions: Vec<Value> = serde_json::from_str(&versions_text).map_err(|e| {
        format!(
            "JSON parsing failed: {}\nResponse content: {}",
            e, versions_text
        )
    })?;

    // 4. 选择与目标版本兼容的最新版本
    let version = get_latest_compatible_version(&versions, mc_version, loader)?;

    // 5. 获取可下载文件
    let file = version["file"]
        .as_object()
        .and_then(|file| file.get("downloadUrl"))
        .and_then(|url| url.as_str())
        .ok_or("Invalid download URL")?;

    // 6. 安全下载文件
    let file_name = sanitize(format!(
        "{}-{}.jar",
        mod_id,
        version["name"].as_str().unwrap_or("unknown")
    ));

    let save_path = output_dir.join(file_name);
    download_file(&client, file, &save_path)?;

    Ok(save_path)
}

// 下载文件函数
fn download_file(
    client: &Client,
    url: &str,
    path: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Downloading from URL: {}", url); // Debugging log
    let mut response = client.get(url).send()?;
    let mut file = File::create(path)?;
    std::io::copy(&mut response, &mut file)?;
    Ok(())
}

fn get_latest_compatible_version(
    versions: &[Value],
    mc_version: &str,
    loader: &str,
) -> Result<Value, Box<dyn Error>> {
    // 按照发布时间降序排序版本
    let mut sorted_versions = versions.to_vec();
    sorted_versions.sort_by(|a, b| {
        let a_time = a["dateAdded"].as_str().unwrap_or("");
        let b_time = b["dateAdded"].as_str().unwrap_or("");
        b_time.cmp(a_time) // 降序排列
    });

    // 查找最新版本，筛选支持目标 Minecraft 版本和加载器
    sorted_versions
        .iter()
        .find(|v| {
            let game_versions = v["gameVersions"]
                .as_array()
                .map(|arr| arr.as_slice())
                .unwrap_or(&[]);
            let loaders = v["loaders"]
                .as_array()
                .map(|arr| arr.as_slice())
                .unwrap_or(&[]);
            // 确保 mc_version 和 loader 都是 &str 类型
            let contains_mc_version = game_versions
                .iter()
                .any(|v| v.as_str().unwrap_or("").trim() == mc_version.trim());
            let contains_loader = loaders
                .iter()
                .any(|v| v.as_str().unwrap_or("").trim() == loader.trim());
            contains_mc_version && contains_loader
        })
        .cloned() // 克隆出匹配的版本
        .ok_or_else(|| "No compatible version found.".into())
}
