use reqwest::{
    blocking::Client,
    header,
};
use std::fs::File;
use std::path::PathBuf;
use std::path::Path;
use std::time::Duration;
use serde_json::{json, Value};
use sanitize_filename::sanitize;

pub fn download_modrinth_mod(
    mod_id: &str,
    mc_version: &str,
    loader: &str,
    output_dir: &Path,
) -> Result<PathBuf, Box<dyn std::error::Error>> {
    // 1. 配置HTTP客户端
    let mut headers = header::HeaderMap::new();
    headers.insert(header::ACCEPT, header::HeaderValue::from_static("application/json"));
    headers.insert(header::USER_AGENT, header::HeaderValue::from_static("MiraMigrator/1.0"));

    let client = Client::builder()
        .default_headers(headers)
        .timeout(Duration::from_secs(30))
        .build()?;

    // 2. 验证项目存在
    let project_url = format!("https://api.modrinth.com/v2/project/{}", mod_id);
    let project_res = client.get(&project_url).send()?;
    if !project_res.status().is_success() {
        return Err(format!(
            "Mod不存在: {} {}",
            project_res.status(),
            project_res.text()?
        ).into());
    }

    // 3. 获取兼容版本
    let versions_url = format!("https://api.modrinth.com/v2/project/{}/version", mod_id);
    let versions_res = client.get(&versions_url)
        .query(&[
            ("game_versions", json!(vec![mc_version])),
            ("loaders", json!(vec![loader])),
        ])
        .send()?;

    let versions_text = versions_res.text()?;
    let versions: Vec<Value> = serde_json::from_str(&versions_text).map_err(|e| {
        format!(
            "JSON解析失败: {}\n响应内容: {}",
            e,
            versions_text
        )
    })?;

    // 4. 选择最新版本
    let version = versions
        .iter()
        .max_by_key(|v| v["date_published"].as_str().unwrap_or(""))
        .ok_or("找不到兼容版本")?;

    // 5. 获取可下载文件
    let file = version["files"]
        .as_array()
        .and_then(|files| files.iter().find(|f| f["primary"].as_bool().unwrap_or(false)))
        .ok_or("找不到主文件")?;

    // 6. 安全下载文件
    let download_url = file["url"].as_str().ok_or("无效下载URL")?;
    let file_name = sanitize(
        file["filename"].as_str().map(|s| s.to_string()).unwrap_or_else(|| {
            format!("{}-{}.jar", mod_id, version["version_number"].as_str().unwrap_or("unknown"))
        })
    );
    
    let save_path = output_dir.join(file_name);
    download_file(&client, download_url, &save_path)?;

    Ok(save_path)
}
// 添加下载文件函数
fn download_file(client: &Client, url: &str, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let mut response = client.get(url).send()?;
    let mut file = File::create(path)?;
    std::io::copy(&mut response, &mut file)?;
    Ok(())
}