## Self Command Tools
自用。配置文件: `~/self.conf`


#### 切换 dockerRepo 帐号
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
#### 切换 k8s 配置文件
`cm ke {filePrefix}` just like shell `cp ~/.kube/config.{filePrefix} ~/.kube/config`
