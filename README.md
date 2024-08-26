# Cileamzh-frame

Cileamzh-frame 是一个由rust开发的简易的后端框架,

得益于rust先天多线程的支持和性能cileamzh-web服务器有优

秀的高并发处理能力，同时支持二进制传输

如果你想快速搭建一个简单的服务器，

cileamzh-web是一个不错的选择

Getting Start
-----------------------------------
将cileamzh-web 放入你的rust项目依赖中。
通过HttpServer::new()来快速获得一个HttpServer实例

.listen()方法用于设置要监听的ip同时启动服务器


通过.add_get()方法可快速添加一个get请求的服务器接口

将下列代码添加到你的main.rs中
------------------------------------
```rust
use cileamzh-web::HttpServer;

fn main(){
    let httpserver=HttpServer::new()?;

    httpserver.add_get("/",|req,res|{
    
    res.set_body("hello_world");
    });

    httpserver.listen("127.0.0.1:8080")?;

}
```
api文档
=========

cileamzh.top/cileamzh-frame
--------------------------------

目前正在更新中
