use std::fs;
use std::fs::File;
use std::path::PathBuf;
use json_patch::merge;
use reqwest::IntoUrl;
use serde::{Serialize,Deserialize};
use serde_json::{json, Value};

#[derive(Serialize,Deserialize,Debug)]
pub struct VMessConfig {
    v: String,
    pub ps:String,
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
impl VMessConfig {
    pub fn to_v2fly_outbounds_json(&self) ->Value {
        let transport_setting = match self.net.as_str() {
            "ws" => {
              json!({
                  "wsSettings":{
                      "headers": {
                          "host": self.host,
                      },
                      "path": if self.path.is_empty() {"/"} else { &self.path},
                  }
              })
            },
            "tcp" => json!({}),
            _ => json!({}),
        };
        let tls_settings = json!({
            "allowInsecure": false,
            "serverName": if self.host.is_empty() {&self.sni} else {&self.host }
        });
        let mut stream_setting = json!({
            "network": self.net,
            "security": self.tls,
        });
        let tls_settings = if self.tls == "tls" {
            json!({
                "tlsSettings" : tls_settings
            })
        }else if self.tls == "xtls" {
            json!({
                "xtlsSettings": tls_settings
            })
        }else {
            json!({
                "tlsSettings": {"allowInsecure": true}
            })
        };
        merge(&mut stream_setting, &tls_settings);
        merge(&mut stream_setting, &transport_setting);

        json!({
            "remarks": self.ps,
            "outbounds": [{
                "settings":{
                    "vnext": [{
                        "address": self.add,
                        "port": self.port.parse::<i32>().unwrap(),
                        "users": [{
                            "id": self.id,
                            "level": 8,
                            "security": "auto",
                            "encryption": "auto"
                        }]
                    }]
                },
                "streamSettings": stream_setting,
                "protocol": "vmess",
                "tag": "proxy",
                "mux": {
                    "enabled":false,
                    "concurrency": 8
                },

            }]
        })
    }
}

pub async fn request_v2fly_config<T:IntoUrl>(url:T) -> anyhow::Result<Vec<VMessConfig>> {
    let base64_str = reqwest::get(url).await?
        .text()
        .await?;

    let config_str = String::from_utf8(base64::decode(base64_str)?)?;
    let config:Vec<&str> = config_str.trim_end().split('\n').collect();
    let mut v_mess_configs = vec![];
    for c  in config {
        if c.starts_with("vmess://") {
            let c = String::from_utf8(base64::decode(c.replace("vmess://",""))?)?;
            let v_mess_config:VMessConfig = serde_json::from_str(&c)?;
            v_mess_configs.push(v_mess_config);
            //println!("{:?}",v_mess_configD.to_v2fly_outbounds_json());
        //TODO: add vless
        }else {
            println!("only support vmess, original {}", c);
        }
    }
    Ok(v_mess_configs)
}
pub fn v2fly_config_write(data:&str, path:String) -> anyhow::Result<()>{
    let mut path = PathBuf::from(path);
    path.push("outbounds.json");
    let path = path.as_path();
    if !path.exists() {
        File::create(path)?;
    }
    fs::write(path, data)?;
    Ok(())

}