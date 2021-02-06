use clap::{App, Arg};
use std::process::Command;
use hocon::HoconLoader;
use std::error::Error;
use std::env::home_dir;

const DOCKER_REPO_COMMAND:&str = "docker_repo";
const KUBECTL_EXCHANGE_COMMAND:&str = "kubectl_exchange";
fn docker_repo_command() ->App {
    App::new(DOCKER_REPO_COMMAND).alias("dr").arg(Arg::new("name"))
}
fn kubectl_exchange_command() -> App {
    App::new(KUBECTL_EXCHANGE_COMMAND).alias("ke".arg(Arg::new("prefix")))
}
fn main() ->Result<(), dyn Error> {
    let config = &HoconLoader::new().load_file(home_dir().unwrap().join("self.conf"))?.hocon()?;
    let matches = App::new("cm").version("1.0").author("zsy.evan@gmail.com").about("self shell, save life")
        .subcommand(docker_repo_command())
        .subcommand(kubectl_exchange_command())
        .get_matches();
    match matches.subcommand() {
        Some((DOCKER_REPO_COMMAND,args)) => {
            let name = args.value_of("name")?;
            let c = &config["dockerRepo"][name];
            let cm = format!("echo {}| docker login uhub.service.ucloud.cn -u {} --password-stdin",c["password"].as_string()?, c["user"].as_string()?);
            let r = Command::new("bash").arg("-c").arg(cm).output()?;
            println!("{:?}",String::from_utf8_lossy(&r.stdout));
        },
        Some((KUBECTL_EXCHANGE_COMMAND,args)) => {
            let prefix = args.value_of("prefix")?;
            let kube_base_dir = home_dir().unwrap().join(".kube");
            std::fs::copy(kube_base_dir.join(format!("config.{}", prefix)), kube_base_dir.join("config"))?;
            println!("exchange kubectl config file")
        }
        _ => panic!("does not match any command")
    }
    println!("ok");
    Ok(())
}
