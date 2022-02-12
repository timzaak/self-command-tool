# Self Command Tools
自用。配置文件: `~/self.conf`


## 切换 dockerRepo 帐号
`cm dr {commandName}`
配置文件：
```
{
  dockerRepo {
    ${commandName} {
      name: ${DockerRepoUserName}
      password: ${DockerRepoPassword}
    }
  }
}
```
## 切换 k8s 配置文件
`cm ke {filePrefix}` just like shell `cp ~/.kube/config.{filePrefix} ~/.kube/config`

## 同步dashboard
`cm cs server_start` 开启同步服务

`cm cs sync` 同步剪贴板

配置文件：
```
clipboardSync.remoteUrl="http://127.0.0.1:3001/clipboard"

```

## v2fly 订阅
命令：`cm v2fly sync`
根据链接${v2fly.url}获取配置，根据用户选择生成 outbounds.json 文件到指定目录${v2fly.path}，并重启名为 ${v2fly.dockerName|v2fly} 的镜像。

### v2fly 镜像命令
basePath 文件里存放 v2fly 的本地配置即可，例如指定日志等级、暴露端口等。
```
docker run --net=host --restart=always -v ${basePath}:/etc/v2ray/config.json -v${outboundsPath}:/etc/v2ray/outbounds.json --name=v2fly v2fly/v2fly-core:latest v2ray -confdir /etc/v2ray
```
MacOS 貌似不能用 net=host 需要自行指定端口。 

配置文件:
```
v2fly {
  url = "http://xxx"
  configPath = "/abc/xx"
  dockerName = v2fly
}
```

## 安装
```bash
cargo install --path .
```
