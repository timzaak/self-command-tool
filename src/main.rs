use clap::{App, Arg};
use std::process::Command;
use hocon::HoconLoader;
use std::error::Error;


fn docker_repo_command() ->App {
    App::new("docker_repo").alias("dr").arg(Arg::new("target")).arg(Arg::new("file"))
}
fn main() ->Result<(), dyn Error> {
    let config = &HoconLoader::new().load_file(std::env::home_dir().unwrap().join("self.conf"))?.hocon()?;
    let matches = App::new("cm").version("1.0").author("zsy.evan@gmail.com").about("self shell, save life")
        .subcommand(docker_repo_command())
        .get_matches();
    match matches.subcommand() {
        Some(("docker_repo",args))=> {
            let name = args.value_of("name")?;
            let c = &config["dockerRepo"][name];
            let cm = format!("echo {}| docker login uhub.service.ucloud.cn -u {} --password-stdin",c["password"].as_string()?, c["user"].as_string()?);
            let r = Command::new("bash").arg("-c").arg(cm).output()?;
            println!("{:?}",String::from_utf8_lossy(&r.stdout));
        },
        _ => panic!("does not match any command")
    }
    //HoconLoader::new().load_file("")?;

    println!("ok");
    Ok(())
}
