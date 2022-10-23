# goods-server

## 项目初衷

因为偶尔搞点硬件相关的小玩具，需要对各种硬件进行管理，但是嘛，没找到适合自己的开源项目，于是就边学 rust 边写对应的后端和前端。
这是对应的后端部分，使用的是 Clion 作为 IDE，开发语言采用的是 Rust。

## 对应文件
[user](src/route/user) 对应的是用户请求相关

[good](src/route/good) 对应的是货物请求相关

[data](src/data.rs) 一些自行配置的参数

默认用户 agoines
默认密码 qwer1234

在 static 插入 vue 生成的 css 和 js 文件，
命名为 index.css 和 index.js

### 对应的 HTTP 测试文件
[货物的测试文件](good.http)

[用户的测试文件](user.http)

### 对应的参数
[货物的参数](doc/good.md)

[用户的参数](doc/user.md)