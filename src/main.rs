// 主程序入口
// 现在只负责程序的启动和调用模块，不再包含具体业务逻辑

// 引入所需的库
use std::io::Error;
use shreds_client::{JitoClient, read_jito_url};

fn main() -> Result<(), Error> {

    
    // 读取配置并启动客户端
    let jito_url = read_jito_url()?;
    JitoClient::start(jito_url)
}