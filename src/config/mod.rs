use std::fs;
use std::io::{Error, ErrorKind};

// 定义要查找的程序ID (Base58格式)
pub const PUMP_PROGRAM_ID: &str = "6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P";
pub const PUMPAMM_PROGRAM_ID: &str = "pAMMBay6oceH9fJKBRHGP5D4bD4sWpmSwMn52FMfXEA";

// 批处理大小 - 每批处理的交易数量
pub const BATCH_SIZE: usize = 100;

// 从env.toml文件中读取JITO服务器URL
pub fn read_jito_url() -> Result<String, Error> {
    let env_content = fs::read_to_string("env.toml")
        .map_err(|_| Error::new(ErrorKind::NotFound, "无法读取env.toml文件"))?;
    
    for line in env_content.lines() {
        if line.starts_with('#') || line.trim().is_empty() {
            continue; // 跳过注释和空行
        }
        
        if let Some((key, value)) = line.split_once('=') {
            if key.trim() == "JITO-SHRED-URL" {
                return Ok(value.trim().to_string());
            }
        }
    }
    
    Err(Error::new(ErrorKind::NotFound, "未找到JITO-SHRED-URL配置"))
} 