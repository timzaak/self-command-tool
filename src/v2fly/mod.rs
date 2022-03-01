use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::path::PathBuf;
use json_patch::merge;
use reqwest::{IntoUrl, Url};
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

        let stream_settings = stream_transport_setting(
            &self.net, Some(&self.typ), Some(&self.host),
            Some(&self.path), Some(&self.path),Some(&self.host),
            Some(&self.path), Some(&self.typ),Some(&self.path),
            &self.tls,false, Some(&self.sni)
            );
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
                "streamSettings": stream_settings,
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

fn stream_transport_setting<T:AsRef<str>>(transport:T, header_type:Option<T>, host:Option<T>,
                                          path:Option<T>,seed:Option<T>, quic_security:Option<T>,
                                          key:Option<T>,mode:Option<T>,service_name:Option<T>,
                                          stream_security:T,allow_insecure:bool, sni:Option<T>,
)  -> Value {
    let stream_security = stream_security.as_ref();
    let host = host.as_ref().map(|x|x.as_ref());
    let transport_setting = match transport.as_ref()  {
         "ws" => {
             json!({
                  "wsSettings":{
                      "headers": {
                          "host": host.unwrap_or(""),
                      },
                      "path": path.as_ref().map(|x|x.as_ref()).unwrap_or("/"),
                  }
              })
         },
         "tcp" => json!({}),
         _ => json!({}),
    };

    let mut stream_setting = json!({
            "network": transport.as_ref(),
            "security": stream_security,
        });
    let server_name = sni.as_ref().map(|x|x.as_ref()).filter(|_s| ! _s.is_empty()).or(host).unwrap();
    let tls_settings = json!({
                "serverName": server_name,
                "allowInsecure": allow_insecure
            });
    let tls_settings = if stream_security == "tls" {
        json!({
            "tlsSettings": tls_settings
        })
    } else if stream_security == "xtls" {
        json!({
                "xtlsSettings": tls_settings
            })
    }else {
        json!({
            "tlsSettings": tls_settings
        })
    };
    merge(&mut stream_setting, &tls_settings);
    merge(&mut stream_setting, &transport_setting);

    stream_setting
}


fn vless_v2fly_config(url:Url) ->(String, Value) {
    let config: HashMap<String,String> = url.query_pairs().into_owned().collect();
    let stream_settings = stream_transport_setting(
        config.get("type").unwrap_or(&String::from("tcp")),
        config.get("headerType"), config.get("host"),
        config.get("path"), config.get("seed"),
        config.get("quicSecurity"), config.get("key"),
        config.get("mode"), config.get("serviceName"),
        config.get("security").unwrap_or(&"".to_string()),
        false,None,
    );
    let data = json!({
        "remark": url.fragment().unwrap_or(""),
        "outbounds": [{
            "settings": {
                "vnext": [{
                    "address": url.host_str().unwrap(),
                    "port": url.port().unwrap(),
                    "users": [{
                        "id": url.username(),
                        "level": 8,
                        "security": "auto",
                        "encryption": config.get("encryption").unwrap_or(&String::from("auto")),
                        "flow": config.get("flow").unwrap_or(&String::from(""))
                    }]
                }]
            },
            "stream_settings": stream_settings,
            "protocol": "vless",
            "tag": "proxy",
            "mux": {
                "enabled":false,
                "concurrency": 8
            },
        }]
    });
    (config.get("serverName").unwrap_or(&"".to_string()).to_string(),data)
}
pub async fn request_v2fly_config<T:IntoUrl>(url:T) -> anyhow::Result<Vec<(String, Value)>> {
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
            let value = v_mess_config.to_v2fly_outbounds_json();
            v_mess_configs.push((v_mess_config.ps,value));
            //println!("{:?}",v_mess_configD.to_v2fly_outbounds_json());
        } else if c.starts_with("vless://") {
            v_mess_configs.push(vless_v2fly_config(Url::parse(c)?));
        } else {
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