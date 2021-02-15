use actix_web::dev::BodyEncoding;
use actix_web::{
    get, http::header, http::ContentEncoding, http::StatusCode, web, App, HttpResponse, HttpServer,
    Result,
};
use actix_web::client::Client;
use actix_cors::Cors;
use cached::proc_macro::cached;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::{fs, os::linux::fs::MetadataExt, path::Path};
use textcode::gb2312;
use urlencoding;
use chrono::prelude::*;

//常量配置
const DISK_DIRECTORY: &str = "/www/wwwroot/pineapple.edgeless.top/disk";
const STATION_URL: &str = "https://pineapple.edgeless.top/disk";
const TOKEN: &str = "WDNMD";

//静态变量配置
static mut LAST_ALERT_TIME:i64=0; //上一次向Server酱发出警告的时间

//自定义Json结构
#[derive(Serialize, Deserialize, Clone)]
struct CateData {
    payload: Vec<String>,
}
#[derive(Serialize, Deserialize, Clone)]
struct ListData {
    payload: Vec<ListObj>,
}
#[derive(Serialize, Deserialize, Clone)]
struct ListObj {
    name: String,
    size: u64,
    node_type: String,
    url: String,
}

//自定义请求参数结构体
#[derive(Deserialize, Clone)]
struct EptAddrQueryStruct {
    name: String,
    cate: String,
    version: String,
    author: String,
}
#[derive(Deserialize, Clone)]
struct PluginListQueryStruct {
    name: String,
}
#[derive(Deserialize, Clone)]
struct TokenRequiredQueryStruct {
    token: String,
}

//工厂函数

#[get("/api/v2/alpha/{quest}")]
async fn factory_alpha(
    web::Path(quest): web::Path<String>,
    info: web::Query<TokenRequiredQueryStruct>,
) -> HttpResponse {
    //校验token
    if &info.token != TOKEN {
        return return_error_query(String::from("Invalid token : ") + &info.token);
    }
    return match &quest[..] {
        "version" => return_text_result(get_alpha_version()),
        "addr" => return_redirect_result(get_alpha_addr()),
        _ => return_error_query(format!("/alpha/{}", quest)),
    };
}

#[get("/api/v2/info/{quest}")]
async fn factory_info(web::Path(quest): web::Path<String>) -> HttpResponse {
    return match &quest[..] {
        "iso_version" => return_text_result(get_iso_version()),
        "iso_addr" => return_redirect_result(get_iso_addr()),
        "hub_version" => return_text_result(get_hub_version()),
        "hub_addr" => return_redirect_result(get_hub_addr()),
        "ventoy_plugin_addr" => {
            return_redirect_string(String::from(STATION_URL) + "/Socket/Hub/ventoy_wimboot.img")
        },
        "error"=>{
            return_error_internal(String::from("test error here"))
        },
        _ => return_error_query(quest),
    };
}

#[get("/api/v2/plugin/cateData")]
async fn factory_plugin_cate() -> HttpResponse {
    return return_json_result(get_plugin_cate());
}

#[get("/api/v2/plugin/listData")]
async fn factory_plugin_list(info: web::Query<PluginListQueryStruct>) -> HttpResponse {
    //判断目录是否存在
    if !Path::new(&(String::from(DISK_DIRECTORY) + "/插件包/" + &info.name.clone())).exists() {
        return return_error_query(String::from("No such cate"));
    }
    return return_json_result(get_plugin_list(info.name.clone()));
}

#[get("/api/v2/ept/index")]
async fn factory_ept_index() -> HttpResponse {
    return return_text_result_gb(get_ept_index());
}

#[get("/api/v2/ept/addr")]
async fn factory_ept_addr(info: web::Query<EptAddrQueryStruct>) -> HttpResponse {
    return return_redirect_string(get_ept_addr(
        info.cate.clone(),
        info.name.clone(),
        info.version.clone(),
        info.author.clone(),
    ));
}

#[get("/api/v2/misc/{quest}")]
async fn factory_misc(web::Path(quest): web::Path<String>) -> HttpResponse {
    return match &quest[..] {
        "ariang" => return_redirect_string(String::from(
            "https://www.edgeless.top/ariang/#!/settings/rpc/set/http/127.0.0.1/6800/jsonrpc",
        )),
        "sbl" => return_redirect_string(String::from("https://blog.gocrossthegfw.cf/")),
        _ => return_error_query(quest),
    };
}

//主函数
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let listen_addr = "127.0.0.1:3090";

    HttpServer::new(|| {
        App::new()
            .wrap(
                Cors::default()
                    .allowed_origin("http://localhost:8080")
                    .allowed_origin("https://*.edgeless.top")
                    .allowed_methods(vec!["GET"])
                    .max_age(3600)
            )
            .service(factory_info)
            .service(factory_alpha)
            .service(factory_plugin_cate)
            .service(factory_plugin_list)
            .service(factory_ept_index)
            .service(factory_ept_addr)
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

//发送GET请求
async fn request_get(url:String){
    let client=Client::default();
    let response=client.get(&url).send().await;
    println!("{:?}",response);
}

//按Text返回函数
fn return_text_result(content: Result<String, String>) -> HttpResponse {
    if let Err(error) = content {
        return return_error_internal(error);
    }
    return HttpResponse::Ok().body(format!("{}", content.unwrap()));
}
// fn return_text_result_gb(content: Result<String, String>) -> HttpResponse {
//     if let Err(error) = content {
//         return return_error_internal(error);
//     }
//     //编码转换为GB2312 Vec
//     let vec = gb2312::encode_to_vec(&content.unwrap());
//     //Vec转&[u8]
//     return HttpResponse::Ok()
//         .encoding(ContentEncoding::Identity)
//         .body(vec);
// }
// fn return_text_string(content: String) -> HttpResponse {
//     return HttpResponse::Ok().body(content);
// }
fn return_text_result_gb(content: Result<Vec<u8>, String>) -> HttpResponse {
    if let Err(error) = content {
        return return_error_internal(error);
    }
    return HttpResponse::Ok()
        .encoding(ContentEncoding::Identity)
        .body(content.unwrap());
}

//按Redirect返回函数
fn return_redirect_result(url: Result<String, String>) -> HttpResponse {
    if let Err(error) = url {
        return return_error_internal(error);
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

//按Json返回函数
fn return_json_result<T: Serialize>(data: Result<T, String>) -> HttpResponse {
    if let Err(error) = data {
        return return_error_internal(error);
    }
    return HttpResponse::Ok().json(data.unwrap());
}

//返回内部错误
fn return_error_internal(msg: String) -> HttpResponse {
    //判断是否需要发送通知
    unsafe {
        if Local::now().timestamp() -LAST_ALERT_TIME>3600{
            //通过Server酱发送通知
            let encoded=urlencoding::encode(&msg);
            let addr=String::from("https://sctapi.ftqq.com/SCT8221T9hGdL643mhj3cjUC6ao6L1uh.send?title=Server_Internal_Error&desp=")+&encoded;
            request_get(addr);

            //更新上次发送时间为现在
            LAST_ALERT_TIME=Local::now().timestamp();
            println!("{}",LAST_ALERT_TIME);
        }
    }

    //返回构造的HttpResponse
    return HttpResponse::Ok()
        .status(StatusCode::INTERNAL_SERVER_ERROR)
        .body(format!("Error: Internal\n{}", msg));
}

//返回查询错误
fn return_error_query(msg: String) -> HttpResponse {
    return HttpResponse::Ok()
        .status(StatusCode::BAD_REQUEST)
        .body(format!("Error: Quest\nUnknown quest:{}", msg));
}

//获取ISO版本号/info/iso_version
#[cached(time = 600)]
fn get_iso_version() -> Result<String, String> {
    //选中ISO文件
    let iso_name = file_selector(
        String::from(DISK_DIRECTORY) + "/Socket",
        String::from("^Edgeless.*iso$"),
    )?;
    //提取版本号
    let iso_version = version_extractor(iso_name, 2)?;
    return Ok(iso_version);
}

//获取ISO下载地址/info/iso_addr
#[cached(time = 600)]
fn get_iso_addr() -> Result<String, String> {
    //选中ISO文件
    let iso_name = file_selector(
        String::from(DISK_DIRECTORY) + "/Socket",
        String::from("^Edgeless.*iso$"),
    )?;
    //拼接并返回
    return Ok(STATION_URL.to_string() + "/Socket/" + &iso_name);
}

//获取Alpha版本wim文件版本号/info/alpha_version
#[cached(time = 600)]
fn get_alpha_version() -> Result<String, String> {
    //选中Alpha_xxx.wim文件
    let wim_name = file_selector(
        String::from(DISK_DIRECTORY) + "/Socket/Alpha",
        String::from("^Edgeless.*wim$"),
    )?;
    //提取版本号
    let wim_version = version_extractor(wim_name, 2)?;
    return Ok(wim_version);
}

//获取Alpha版本wim文件下载地址/info/alpha_addr
#[cached(time = 600)]
fn get_alpha_addr() -> Result<String, String> {
    //选中Alpha_xxx.wim文件
    let wim_name = file_selector(
        String::from(DISK_DIRECTORY) + "/Socket/Alpha",
        String::from("^Edgeless.*wim$"),
    )?;
    //拼接并返回
    return Ok(STATION_URL.to_string() + "/Socket/Alpha/" + &wim_name);
}

//获取Hub版本号/info/hub_version
#[cached(time = 600)]
fn get_hub_version() -> Result<String, String> {
    //选中Edgeless Hub_xxx.7z文件
    let hub_name = file_selector(
        String::from(DISK_DIRECTORY) + "/Socket/Hub",
        String::from("^Edgeless Hub.*7z$"),
    )?;
    //提取版本号
    let hub_version = version_extractor(hub_name, 2)?;
    return Ok(hub_version);
}

//获取Hub下载地址/info/hub_addr
#[cached(time = 600)]
fn get_hub_addr() -> Result<String, String> {
    //选中Edgeless Hub_xxx.7z文件
    let hub_name = file_selector(
        String::from(DISK_DIRECTORY) + "/Socket/Hub",
        String::from("^Edgeless Hub.*7z$"),
    )?;
    //拼接并返回
    return Ok(STATION_URL.to_string() + "/Socket/Hub/" + &hub_name);
}

//获取插件分类数组
#[cached(time = 600)]
fn get_plugin_cate() -> Result<CateData, String> {
    //扫描插件包目录
    let cate_list = fs::read_dir(DISK_DIRECTORY.to_string() + "/插件包");
    if let Err(_) = cate_list {
        return Err(String::from("get_plugin_cate:Fail to read : ") + &DISK_DIRECTORY + "/插件包");
    }

    //形成Vec<String>
    let mut result = Vec::new();
    for entry in cate_list.unwrap() {
        //解析node名称
        let file_name = entry.unwrap().file_name().clone();
        let true_name = file_name.to_str().unwrap();
        //判断是否为目录，是则push到Vector
        let path = String::from(DISK_DIRECTORY) + "/插件包/" + &true_name;
        if Path::new(&path).is_dir() {
            result.push(true_name.to_string());
        }
    }
    //println!("{:?}",result);
    return Ok(CateData { payload: result });
}

//获取分类详情
#[cached(time = 600)]
fn get_plugin_list(cate_name: String) -> Result<ListData, String> {
    //扫描分类目录
    let list = fs::read_dir(DISK_DIRECTORY.to_string() + "/插件包/" + &cate_name);
    if let Err(_) = list {
        return Err(String::from("get_plugin_list:Can't open as directory : ")
            + &DISK_DIRECTORY
            + "/插件包/"
            + &cate_name);
    }

    //形成Vec<ListObj>
    let mut result = Vec::new();
    for entry in list.unwrap() {
        //解析node名称
        let dir_entry = entry.unwrap();
        let file_name = &dir_entry.file_name().clone();
        let true_name = file_name.to_str().unwrap().to_string();

        //获取文件大小
        let meta_data = fs::metadata(&dir_entry.path());
        if let Err(_) = meta_data {
            return Err(String::from("get_plugin_list:Fail to read : ")
                + &DISK_DIRECTORY
                + "/插件包/"
                + &cate_name);
        }
        let file_size = meta_data.unwrap().st_size();

        //将后缀名为.7z的推入Vec
        if regex::is_match(".7z", &true_name).unwrap() {
            result.push(ListObj {
                name: true_name.clone(),
                size: file_size,
                node_type: String::from("FILE"),
                url: String::from(STATION_URL) + "/插件包/" + &cate_name + "/" + &true_name,
            })
        }
    }
    return Ok(ListData { payload: result });
}

//生成ept索引
#[cached(time = 600)]
fn get_ept_index() -> Result<Vec<u8>, String> {
    //获取分类
    let cate_data = get_plugin_cate()?;

    //生成文本
    let mut result = String::new();
    for cate_name in cate_data.payload {
        //对当前分类获取文件列表
        let list = get_plugin_list(cate_name.clone())?;

        //遍历列表，生成字段
        for plugin_info in list.payload {
            //去拓展名
            let plugin_name = &plugin_info.name[0..plugin_info.name.len() - 3];
            //生成字段
            let line = String::from(plugin_name) + "_" + &cate_name + "\n";
            //添加字段
            result.push_str(&line);
        }
    }
    return Ok(gb2312::encode_to_vec(&result));
}

//生成下载地址
fn get_ept_addr(cate: String, name: String, version: String, author: String) -> String {
    return String::from(STATION_URL)
        + "/插件包/"
        + &cate
        + "/"
        + &name
        + "_"
        + &version
        + "_"
        + &author
        + ".7z";
}
