use actix_web::{get, web, App, HttpServer, Responder};
use std::{fs, path::Path};

#[get("/info/{quest}")]
async fn factory_info(web::Path(quest): web::Path<String>) -> impl Responder {
    let txt = read_txt();
    println!("{}", txt);
    format!("Info, quest:{}, {}", quest, txt)
}

#[get("/plugin/{quest}")]
async fn factory_plugin(web::Path(quest): web::Path<String>) -> impl Responder {
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

// 文件读取Demo
fn read_txt() -> String {
    //let a=Path::new("does_not_exist.txt").exists();
    return fs::read_to_string("./1.txt").unwrap();
}
