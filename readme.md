# Edgeless REST Api Reference

## Information for Edgeless Hub
|Name|Location|Return type|Return demo|Description|
|--|--|--|--|--|
|iso data|/api/v2/info/iso|json|<details>`{"version":"3.2.0","name":"Edgeless_Beta_3.2.0.iso","url":"https://pineapple.edgeless.top/disk/Socket/Edgeless_Beta_3.2.0.iso"}`</details>|Only return the latest one|
|hub data|/api/v2/info/hub|json|<details>`{"miniupdate_pack_addr":"https://pineapple.edgeless.top/disk/Socket/Hub/Update/miniupdate.7z","update_pack_addr":"https://pineapple.edgeless.top/disk/Socket/Hub/Update/update.7z","full_update_redirect":"https://down.edgeless.top","update_info":{"dependencies_requirement":"1.6","wide_gaps":[]}}`</details>|Hub self information for updating etc|
|hub version|/api/v2/info/hub_version|string|2.02||
|hub download address|/api/v2/info/hub_addr|redirect|https://pineapple.edgeless.top/disk/Socket/Hub/Edgeless%20Hub_Beta_2.02.7z|Only return the latest one|
|ventoy plugin download address|/api/v2/info/ventoy_plugin_addr|redirect|https://pineapple.edgeless.top/disk/Socket/Hub/ventoy_wimboot.img|Ventoy plugin for .wim support|
|ventoy download address|/api/v2/info/ventoy_addr|redirect|https://pineapple.edgeless.top/disk/Socket/Ventoy/ventoy-1.0.46-windows.zip|Fetch release everyday by GitHub Actions|
|ventoy zip name|/api/v2/info/ventoy_name|string|ventoy-1.0.49-windows.zip

## Edgeless Alpha
Token required! Use `?token=` to provide valid token

|Name|Location|Return type|Return demo|Description|
|--|--|--|--|--|
|alpha data|/api/v2/alpha/data|json|<details>`{"iso_version":"3.2.1","iso_name":"Edgeless_Alpha_3.2.1.wim","iso_url":"https://pineapple.edgeless.top/disk/Socket/Alpha/Edgeless_Alpha_3.2.1.wim","pack_require":"4.0.0","pack_name":"Edgeless.7z","pack_url":"https://pineapple.edgeless.top/disk/Socket/Alpha/Edgeless.7z"}`</details>|May got invalid address when "wim version" equals "0.0.0",cause that means no alpha avaliable now|

## Edgeless Plugin
|Name|Location|Return type|Return demo|Description|
|--|--|--|--|--|
|categories|/api/v2/plugin/cateData|json|<details>`{"payload":["实用工具","开发辅助","配置检测","资源管理","办公编辑","输入法","录屏看图","磁盘数据","安全急救","即时通讯","安装备份","游戏娱乐","运行环境","压缩镜像","美化增强","驱动管理","下载上传","浏览器","影音播放","远程连接"]}`</details>||
|list|/api/v2/plugin/listData?name=|json|<details>`{"payload":[{"name":"Listary_5.0.2843.0_Fir.7z","size":3224692,"node_type":"FILE","url":"https://pineapple.edgeless.top/disk/插件包/资源管理/Listary_5.0.2843.0_Fir.7z"},{"name":"Everything_1.4.1.1002_Horatio Shaw.7z","size":1479652,"node_type":"FILE","url":"https://pineapple.edgeless.top/disk/插件包/资源管理/Everything_1.4.1.1002_Horatio Shaw.7z"},{"name":"ReNamer_7.3.0.0_Cno（bot）.7z","size":3245271,"node_type":"FILE","url":"https://pineapple.edgeless.top/disk/插件包/资源管理/ReNamer_7.3.0.0_Cno（bot）.7z"},{"name":"Listary_3.51.858.0_Cno（bot）.7z","size":5263228,"node_type":"FILE","url":"https://pineapple.edgeless.top/disk/插件包/资源管理/Listary_3.51.858.0_Cno（bot）.7z"},{"name":"FastCopy_3.63.0.0_Cno（bot）.7z","size":743254,"node_type":"FILE","url":"https://pineapple.edgeless.top/disk/插件包/资源管理/FastCopy_3.63.0.0_Cno（bot）.7z"}]}`</details>|Parameter `name` required,the value should be included inside "categories"|

## EPT (Batch version)
|Name|Location|Return type|Return demo|Description|
|--|--|--|--|--|
|index|/api/v2/ept/index|string|<details>Vitomu_2.0.1_Cpl.Kerry_实用工具<br/>Qemu启动测试器_3.14.7.31_chenbx_实用工具<br/>软媒设置大师_3.7.2.0_汪凯_实用工具<br/>360清理大师_1.0.0.1001_汪凯_实用工具</details>|Use character set GB2312|
|address parser|/api/v2/ept/addr?cate=&name=&version=&author=|redirect|--|Four parameters required|
