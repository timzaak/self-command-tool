use serde::{Deserialize,Serialize};
pub struct User {
    id:String,
    encryption:String,

}
pub struct VNext {
    address:String,
    port:i32,
    user:User,

}

pub struct StreamSettings {

}
#[derive(Deserialize,Debug)]
pub struct Outbounds {
    protocol:String,
    tag:String,
    settings:String,
    #[serde(rename="streamSetting")]
    stream_setting:StreamSettings,

}
