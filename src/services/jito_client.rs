use jito_protos::shredstream::{
    shredstream_proxy_client::ShredstreamProxyClient, SubscribeEntriesRequest,
};
use tokio::runtime::Runtime;
use bincode;
use std::io;
use std::time::Instant;

use crate::services::transaction_processor::TransactionProcessor;

#[allow(dead_code)]
pub struct JitoClient;

impl JitoClient {
    // 连接到Jito服务器并开始处理数据流
    #[allow(dead_code)]
    pub async fn connect_and_process(jito_url: String) -> Result<(), io::Error> {
        // 创建更高级别的client配置
        let mut client = ShredstreamProxyClient::connect(jito_url)
            .await
            .map_err(|e| io::Error::new(io::ErrorKind::ConnectionRefused, e.to_string()))?;
            
        let mut stream = client
            .subscribe_entries(SubscribeEntriesRequest {})
            .await
            .map_err(|e| io::Error::new(io::ErrorKind::ConnectionAborted, e.to_string()))?
            .into_inner();

        // 预先创建一个字符串缓冲区，避免重复分配
        let mut tx_message_buf = String::with_capacity(1024);
        
        while let Some(slot_entry) = stream.message().await.map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()))? {
            let start_time = Instant::now();
            
            let entries =
                match bincode::deserialize::<Vec<solana_entry::entry::Entry>>(&slot_entry.entries) {
                    Ok(e) => e,
                    Err(e) => {
                        eprintln!("反序列化失败: {e}");
                        continue;
                    }
                };
                
            // 处理该slot中的所有交易，并传递slot信息
            let results = TransactionProcessor::process_entries(&entries, slot_entry.slot);
            
            // 只在有交易结果时才打印信息
            if results.has_results() {
                let _processing_time = start_time.elapsed();
                
                // 打印结果
                results.print();
                
                if cfg!(debug_assertions) {
                    //println!("处理时间: {:?}", processing_time);
                }
            }
            
            // 清空主缓冲区以便下次循环重用
            tx_message_buf.clear();
        }
        
        Ok(())
    }
    
    // 创建一个同步方法启动客户端，方便在main中调用
    #[allow(dead_code)]
    pub fn start(jito_url: String) -> Result<(), io::Error> {
        // 配置tokio运行时以获得最佳性能
        let rt = Runtime::new()
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;
        
        // 设置最大工作线程数
        let thread_count = num_cpus::get();
        println!("使用 {} 线程处理交易", thread_count);
        
        // 启动处理循环
        rt.block_on(Self::connect_and_process(jito_url))
    }
} 