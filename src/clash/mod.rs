use std::collections::BTreeMap;
use std::fs;
use std::fs::File;
use std::path::PathBuf;
use reqwest::IntoUrl;
use warp::header::exact_ignore_case;

pub async fn request_clash_config<T:IntoUrl>(url:T) -> anyhow::Result<String>{
    let base64_str = reqwest::get(url).await?
        .text()
        .await?;

    let config_str = String::from_utf8(base64::decode(base64_str)?)?;
    let config:BTreeMap<String,serde_yaml::Value> = serde_yaml::from_str(&config_str)?;

    let mut new_config = BTreeMap::new();
    new_config.insert("proxies", config.get("proxies").unwrap());
    new_config.insert("proxy-groups", config.get("proxy-groups").unwrap());
    new_config.insert("rules", config.get("rules").unwrap());

    Ok(serde_yaml::to_string(&new_config)?)
}

pub fn clash_config_write(data:&str, path:String) -> anyhow::Result<()>{
    let mut path = PathBuf::from(path);
    path.push("outbounds.yaml");
    let path = path.as_path();
    if !path.exists() {
        File::create(path)?;
    }
    fs::write(path, data)?;
    Ok(())

}