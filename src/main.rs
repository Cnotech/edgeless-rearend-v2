use actix_web::{get, web, App, HttpServer, Responder,error, Result,HttpResponse,http::StatusCode};
// use derive_more::{Display, Error};
use std::{fs, ops::Add, path::Path};
//自定义错误结构体实现错误处理转换为HTTP响应
// #[derive(Debug, Display, Error)]
// #[display(fmt = "Server error: {}", name)]
// struct MyError {
//     name: &'static str,
// }
// impl error::ResponseError for MyError {
//     fn error_response(&self) -> Response<Body>{
//         println!("run error response");
//         return HttpResponse::body(&self);
//     }
//     fn status_code(&self) -> StatusCode{
//         return StatusCode::INTERNAL_SERVER_ERROR;
//     }
// }

#[get("/info/{quest}")]
async fn factory_info(web::Path(quest): web::Path<String>) -> impl Responder {
    let file_name = file_selector(String::from("D:\\Desktop\\Projects\\edgeless-rearend-rust"),String::from("2233.txt"));
    if let Err(msg) = file_name{
        return format!("Server error: {}",msg);
    }
    return format!("Success");
}

#[get("/plugin/{quest}")]
async fn factory_plugin(web::Path(quest): web::Path<String>) -> impl Responder {
    println!("run plugin factory");
    format!("Plugin, quest:{}", quest)
}

#[get("/ept/{quest}")]
async fn factory_ept(web::Path(quest): web::Path<String>) -> impl Responder {
    format!("Ept, quest:{}", quest)
}

#[get("/misc/{quest}")]
async fn factory_misc(web::Path(quest): web::Path<String>) -> impl Responder {
    format!("Misc, quest:{}", quest)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let listen_addr = "127.0.0.1:8080";

    HttpServer::new(|| {
        App::new()
            .service(factory_info)
            .service(factory_plugin)
            .service(factory_ept)
            .service(factory_misc)
    })
    .bind(listen_addr)?
    .run()
    .await
}

//文件扫描选择函数
fn file_selector(path:String,exp:String)->Result<String, String> {
    //println!("run selector");
    //校验路径是否存在
    if !Path::new(&path).exists() {
        // Err(MyError { name: "path not found" }:error);
        return Err(String::from("file_selector-Path not found:")+&path);
    }
    //列出文件列表
    let file_list=fs::read_dir(&path).unwrap();

    println!("{:?}",file_list);

    //遍历匹配文件
    // for i in file_list{
    //     println!("{:?}",i);
    // }
    return Ok(String::from("fuck rust"));
}