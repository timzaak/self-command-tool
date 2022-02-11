use reqwest::IntoUrl;
use serde::{Serialize,Deserialize};
mod v2fly_config;

#[derive(Serialize,Deserialize,Debug)]
struct VMessConfig {
    v: String,
    ps:String,
    add:String,
    port:String,
    id:String,
    aid:String,
    net:String,
    #[serde(rename= "type")]
    typ:String,
    host:String,
    path:String,
    tls:String,
    sni:String,
}




pub async fn request_v2fly_config<T:IntoUrl>(url:T) -> anyhow::Result<String> {
    let base64_str = reqwest::get(url).await?
        .text()
        .await?;

    let config_str = String::from_utf8(base64::decode(base64_str)?)?;
    let config:Vec<&str> = config_str.split('\n').collect();
    for c  in config {
        if c.starts_with("vmess://") {
            let c = String::from_utf8(base64::decode(c.replace("vmess://",""))?)?;
            let v_mess_config:VMessConfig = serde_json::from_str(&c)?;

        }else {
            println!("only support vmess, original {}", c);
        }

    }
    Ok("".to_string())
}