use actix_web::{get, http::header, http::StatusCode, web, App, HttpResponse, HttpServer, Result};
use regex::Regex;
use std::{fs, path::Path};

//常量配置
const DISK_DIRECTORY: &str = "E:\\Edgeless_Onedrive\\OneDrive - 洛阳科技职业学院";
const STATION_URL: &str = "https://pineapple.edgeless.top/disk";

#[get("/info/{quest}")]
async fn factory_info(web::Path(quest): web::Path<String>) -> HttpResponse {
    return match &quest[..] {
        "iso_version" => return_text_result(get_iso_version()),
        "iso_addr" => return_redirect_result(get_iso_addr()),
        "alpha_version" => return_text_result(get_alpha_version()),
        "alpha_addr" => return_redirect_result(get_alpha_addr()),
        "hub_version" => return_text_result(get_hub_version()),
        "hub_addr" => return_redirect_result(get_hub_addr()),
        "ventoy_plugin_addr" => {
            return_redirect_string(String::from(STATION_URL) + "/Socket/Hub/ventoy_wimboot.img")
        }
        _ => return_text_string(format!("Error: Quest\nUnknown quest:{}", quest)),
    };
}

#[get("/plugin/{quest}")]
async fn factory_plugin(web::Path(quest): web::Path<String>) -> HttpResponse {
    return_text_string(format!("Plugin, quest:{}", quest))
}

#[get("/ept/{quest}")]
async fn factory_ept(web::Path(quest): web::Path<String>) -> HttpResponse {
    return_text_string(format!("Ept, quest:{}", quest))
}

#[get("/misc/{quest}")]
async fn factory_misc(web::Path(quest): web::Path<String>) -> HttpResponse {
    return_text_string(format!("Misc, quest:{}", quest))
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
fn file_selector(path: String, exp: String) -> Result<String, String> {
    //校验路径是否存在
    if !Path::new(&path).exists() {
        return Err(String::from("file_selector:Can't find ") + &path);
    }

    //校验正则表达式是否有效
    let expression = Regex::new(&exp);
    if let Err(_) = expression {
        return Err(String::from("file_selector:Invalid expression: ") + &exp);
    }

    //列出文件列表
    let file_list = fs::read_dir(&path);
    if let Err(_) = file_list {
        return Err(String::from("file_selector:Can't read as directory: ") + &path);
    }

    //遍历匹配文件名
    for entry in file_list.unwrap() {
        let file_name = entry.unwrap().file_name().clone();
        let true_name = file_name.to_str().unwrap();
        //println!("checking {}", &true_name);
        if regex::is_match(&exp, true_name).unwrap() {
            //println!("match {}", &true_name);
            return Ok(String::from(true_name));
        }
    }

    return Err(
        String::from("file_selector:Matched nothing when looking into ") + &path + " for " + &exp,
    );
}

//版本号提取器函数
fn version_extractor(name: String, index: usize) -> Result<String, String> {
    //首次切割，获取拓展名的值及其长度
    let mut ext_name = "";
    let mut ext_len = 0;
    let result_ext: Vec<&str> = name.split(".").collect();
    if result_ext.len() > 1 {
        ext_name = result_ext[result_ext.len() - 1];
        ext_len = ext_name.len();
    }

    //再次切割（去拓展名切割），获取字段，将拓展名叠加到最后
    let mut result: Vec<&str> = name[0..name.len() - ext_len - 1].split("_").collect();
    result.push(ext_name);

    if index > result.len() {
        return Err(
            String::from("version_extractor:Index out of range when split ")
                + &name
                + ",got "
                + &index.to_string(),
        );
    }
    //println!("{:?}",result);
    return Ok(result[index].to_string());
}

//按Text返回函数
fn return_text_result(content: Result<String, String>) -> HttpResponse {
    if let Err(error) = content {
        return HttpResponse::Ok().body(format!("Error: Internal\n{}", error));
    }
    return HttpResponse::Ok().body(format!("{}", content.unwrap()));
}
fn return_text_string(content: String) -> HttpResponse {
    return HttpResponse::Ok().body(content);
}

//按Redirect返回函数
fn return_redirect_result(url: Result<String, String>) -> HttpResponse {
    if let Err(error) = url {
        return HttpResponse::Ok().body(format!("Error: Internal\n{}", error));
    }
    return HttpResponse::Ok()
        .status(StatusCode::TEMPORARY_REDIRECT)
        .header(header::LOCATION, url.unwrap())
        .finish();
}
fn return_redirect_string(url: String) -> HttpResponse {
    return HttpResponse::Ok()
        .status(StatusCode::TEMPORARY_REDIRECT)
        .header(header::LOCATION, url)
        .finish();
}

//获取ISO版本号/info/iso_version
fn get_iso_version() -> Result<String, String> {
    //选中ISO文件
    let iso_name = file_selector(
        String::from(DISK_DIRECTORY) + "\\Socket",
        String::from("^Edgeless.*iso$"),
    )?;
    //提取版本号
    let iso_version = version_extractor(iso_name, 2)?;
    return Ok(iso_version);
}

//获取ISO下载地址/info/iso_addr
fn get_iso_addr() -> Result<String, String> {
    //选中ISO文件
    let iso_name = file_selector(
        String::from(DISK_DIRECTORY) + "\\Socket",
        String::from("^Edgeless.*iso$"),
    )?;
    //拼接并返回
    return Ok(STATION_URL.to_string() + "/Socket/" + &iso_name);
}

//获取Alpha版本wim文件版本号/info/alpha_version
fn get_alpha_version() -> Result<String, String> {
    //选中Alpha_xxx.wim文件
    let wim_name = file_selector(
        String::from(DISK_DIRECTORY) + "\\Socket\\Alpha",
        String::from("^Edgeless.*wim$"),
    )?;
    //提取版本号
    let wim_version = version_extractor(wim_name, 2)?;
    return Ok(wim_version);
}

//获取Alpha版本wim文件下载地址/info/alpha_addr
fn get_alpha_addr() -> Result<String, String> {
    //选中Alpha_xxx.wim文件
    let wim_name = file_selector(
        String::from(DISK_DIRECTORY) + "\\Socket\\Alpha",
        String::from("^Edgeless.*wim$"),
    )?;
    //拼接并返回
    return Ok(STATION_URL.to_string() + "/Socket/Alpha/" + &wim_name);
}

//获取Hub版本号/info/hub_version
fn get_hub_version() -> Result<String, String> {
    //选中Edgeless Hub_xxx.7z文件
    let hub_name = file_selector(
        String::from(DISK_DIRECTORY) + "\\Socket\\Hub",
        String::from("^Edgeless Hub.*7z$"),
    )?;
    //提取版本号
    let hub_version = version_extractor(hub_name, 2)?;
    return Ok(hub_version);
}

//获取Hub下载地址/info/hub_addr
fn get_hub_addr() -> Result<String, String> {
    //选中Edgeless Hub_xxx.7z文件
    let hub_name = file_selector(
        String::from(DISK_DIRECTORY) + "\\Socket\\Hub",
        String::from("^Edgeless Hub.*7z$"),
    )?;
    //拼接并返回
    return Ok(STATION_URL.to_string() + "/Socket/Hub/" + &hub_name);
}
