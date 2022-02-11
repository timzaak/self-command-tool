mod clipboard;
mod v2fly;
use clap::{App, Arg};
use std::process::Command;
use hocon::HoconLoader;
use std::env::home_dir;
use crate::clipboard::{start_clip_server, clipboard_sync};
use crate::v2fly::request_v2fly_config;


const DOCKER_REPO_COMMAND:&str = "docker_repo";
const KUBECTL_EXCHANGE_COMMAND:&str = "kubectl_exchange";
const CLIPBOARD_SYNC_COMMAND: &str = "clipboard_sync";
const SERVER_START_SUBCOMMAND: &str = "server_start";
const SYNC_SUBCOMMAND: &str = "sync";
//
const V2FLY_SUBCOMMAND: &str = "v2fly";

fn docker_repo_command<'a>() ->App<'a> {
    App::new(DOCKER_REPO_COMMAND).alias("dr").arg(Arg::new("name"))
}
fn kubectl_exchange_command<'a>() -> App<'a> {
    App::new(KUBECTL_EXCHANGE_COMMAND).alias("ke").arg(Arg::new("prefix"))
}
fn sync_clipboard_command<'a>() -> App<'a> {
    App::new(CLIPBOARD_SYNC_COMMAND).alias("cs")
        .subcommand(App::new(SERVER_START_SUBCOMMAND).alias("ss"))
        .subcommand(App::new(SYNC_SUBCOMMAND).alias("s"))

}

fn v2fly_command<'a>() ->App<'a> {
    App::new(V2FLY_SUBCOMMAND)
        .subcommand(App::new(SERVER_START_SUBCOMMAND).alias("ss").arg(Arg::new("url").required(true)))
}

#[tokio::main]
async fn main() ->Result<(), Box<dyn std::error::Error>> {
    let config = &HoconLoader::new().load_file(home_dir().unwrap().join("self.conf"))?.hocon()?;
    let matches = App::new("cm").version("1.0").author("zsy.evan@gmail.com").about("self shell, save life")
        .subcommand(docker_repo_command())
        .subcommand(kubectl_exchange_command())
        .subcommand(sync_clipboard_command())
        .subcommand(v2fly_command())
        .get_matches();
    match matches.subcommand() {
        Some((DOCKER_REPO_COMMAND,args)) => {
            let name = args.value_of("name").unwrap();
            let c = &config["dockerRepo"][name];
            let cm = format!("echo {}| docker login uhub.service.ucloud.cn -u {} --password-stdin",c["password"].as_string().unwrap(), c["user"].as_string().unwrap());
            let r = Command::new("bash").arg("-c").arg(cm).output()?;
            println!("{:?}",String::from_utf8_lossy(&r.stdout));
        },
        Some((KUBECTL_EXCHANGE_COMMAND,args)) => {
            let prefix = args.value_of("prefix").unwrap();
            let kube_base_dir = home_dir().unwrap().join(".kube");
            std::fs::copy(kube_base_dir.join(format!("config.{}", prefix)), kube_base_dir.join("config")).unwrap();
            println!("exchange kubectl config file")
        }
        Some((CLIPBOARD_SYNC_COMMAND, args)) => {
            match args.subcommand() {
                Some((SERVER_START_SUBCOMMAND, _)) => {
                    start_clip_server().await
                }
                Some((SYNC_SUBCOMMAND, _)) => {
                    let s = &config["clipboardSync"]["remoteUrl"].as_string().unwrap();
                    clipboard_sync(s.as_str()).await;
                }
                _ => panic!("does not match any command")
            }
        }
        Some((V2FLY_SUBCOMMAND, args)) => {
            match args.subcommand() {
                Some((SERVER_START_SUBCOMMAND,args2)) => {
                    let url = args2.value_of("url").unwrap();
                    let config = request_v2fly_config(url).await?;
                    println!("{}", config)
                }
                _ => panic!("does not match any command")
            }
        }
        _ => panic!("does not match any command")
    }
    println!("ok");
    Ok(())
}
