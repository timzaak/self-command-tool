## Self Command Tools
自用。

配置文件: `~/self.conf`
```
// 切换 dockerRepo 帐号
// cm dr ${commandName}  
{
  dockerRepo {
    ${commandName} {
      name: ${DockerRepoUserName}
      password: ${DockerRepoPassword}
    }
  }
}
```