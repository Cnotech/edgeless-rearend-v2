use actix_web::{get, web, App, HttpServer, Responder, error, Result, HttpResponse, http::StatusCode, Error};
// use derive_more::{Display, Error};
use std::{fs, ops::Add, path::Path, io};
use regex::{Regex, Captures};

#[get("/info/{quest}")]
async fn factory_info(web::Path(quest): web::Path<String>) -> impl Responder {
    //let file_name = file_selector(String::from("E:\\Edgeless_Onedrive\\OneDrive - 洛阳科技职业学院\\Socket"),String::from("^Edgeless.*iso$"));
    let file_name=version_extractor(String::from("Edgeless_Beta_3.1.0.iso"),3);
    if let Err(error) = file_name{
        return format!("Error: Internal\n{}",error);
    }
    return format!("{}",file_name.unwrap());
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

//文件选择器函数
fn file_selector(path:String,exp:String)->Result<String,String> {

    //校验路径是否存在
    if !Path::new(&path).exists() {
        return Err(String::from("file_selector:Can't find ")+&path);
    }

    //校验正则表达式是否有效
    let expression=Regex::new(&exp);
    if let Err(_)=expression{
        return Err(String::from("file_selector:Invalid expression: ")+&exp);
    }

    //列出文件列表
    let file_list=fs::read_dir(&path);
    if let Err(_)=file_list{
        return Err(String::from("file_selector:Can't read as directory: ")+&path);
    }

    //遍历匹配文件名
    for entry in file_list.unwrap(){
        let file_name=entry.unwrap().file_name().clone();
        let true_name=file_name.to_str().unwrap();
        //println!("checking {}", &true_name);
        if regex::is_match(&exp,true_name).unwrap(){
            //println!("match {}", &true_name);
            return Ok(String::from(true_name));
        }
    }

    return Err(String::from("file_selector:Matched nothing when looking into ")+&path+" for "+&exp);
}

//版本号提取器函数
fn version_extractor(name:String,index:usize)->Result<String,String>{
    let result:Vec<&str>=name.split("_").collect();
    if index> result.len()-1 {
        return Err(String::from("version_extractor:Index out of range when split ")+&name+",got "+&index.to_string());
    }
    println!("{:?}",result);
    return Ok(result[index].to_string());
}