// 工具函数模块，用于放置通用的工具函数
use std::collections::HashMap;

// 格式化显示进度条
#[allow(dead_code)]
pub fn format_progress(current: usize, total: usize, width: usize) -> String {
    let percent = current as f64 / total as f64;
    let filled_width = (width as f64 * percent) as usize;
    let empty_width = width - filled_width;
    
    let filled = "=".repeat(filled_width);
    let empty = " ".repeat(empty_width);
    
    format!("[{}{}] {:.1}%", filled, empty, percent * 100.0)
}

// PUMP_AMM指令账户索引标签映射
#[allow(dead_code)]
pub fn get_pumpamm_account_labels() -> HashMap<usize, &'static str> {
    let mut labels = HashMap::new();
    
    // Buy指令账户标签 (英文标签不翻译)
    labels.insert(0, "Pool");
    labels.insert(1, "User");
    labels.insert(2, "Global Config");
    labels.insert(3, "Base Mint");
    labels.insert(4, "Quote Mint");
    labels.insert(5, "User Base Token Account");
    labels.insert(6, "User Quote Token Account");
    labels.insert(7, "Pool Base Token Account");
    labels.insert(8, "Pool Quote Token Account");
    labels.insert(9, "Protocol Fee Recipient");
    labels.insert(10, "Protocol Fee Recipient Token Account");
    labels.insert(11, "Base Token Program");
    labels.insert(12, "Quote Token Program");
    labels.insert(13, "System Program");
    labels.insert(14, "Associated Token Program");
    labels.insert(15, "Event Authority");
    labels.insert(16, "Program");
    
    labels
}

// 可以在此添加更多通用工具函数 