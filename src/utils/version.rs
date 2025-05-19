/// 验证版本格式是否合法
pub fn validate_version(version: &str) -> bool {
    if version.is_empty() {
        return false;
    } else {
        let verion_re = regex::Regex::new(r"^\d+\.\d+\.\d+$").unwrap();
        return verion_re.is_match(version);
    }
}
