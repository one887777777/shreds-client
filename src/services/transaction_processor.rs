use std::sync::{Arc, Mutex};
use rayon::prelude::*;
use solana_entry::entry::Entry;

use crate::models::{TransactionResults, PumpParser, PumpAmmParser, BoopParser};

// 批处理大小 - 可以根据系统性能调整
#[allow(dead_code)]
const BATCH_SIZE: usize = 200; // 增加批处理大小以提高并行效率

#[allow(dead_code)]
pub struct TransactionProcessor;

impl TransactionProcessor {
    // 处理slot中的所有交易
    #[allow(dead_code)]
    pub fn process_entries(entries: &[Entry], slot: u64) -> TransactionResults {
        let total_txs = entries.iter().map(|e| e.transactions.len()).sum::<usize>();
        
        // 预分配足够的空间
        let mut all_transactions = Vec::with_capacity(total_txs);
        
        // 使用并行集合处理
        let results = Arc::new(Mutex::new(TransactionResults::new()));
        
        // 设置当前slot
        {
            let mut tx_results = results.lock().unwrap();
            tx_results.set_current_slot(slot);
        }
        
        // 将所有交易收集到一个向量中，避免重复检查签名
        for entry in entries {
            for tx in &entry.transactions {
                if !tx.signatures.is_empty() {
                    all_transactions.push(tx);
                }
            }
        }
        
        // 如果没有交易，直接返回空结果
        if all_transactions.is_empty() {
            return Arc::try_unwrap(results)
                .expect("还有其他线程持有引用")
                .into_inner()
                .expect("无法获取内部值");
        }
        
        // 使用更大的批次进行并行处理
        all_transactions.par_chunks(BATCH_SIZE).for_each(|batch| {
            // 本地收集结果，减少锁争用
            let mut local_pump_results = Vec::new();
            let mut local_pumpamm_results = Vec::new();
            let mut local_boop_results = Vec::new();
            
            // 并行处理每个交易
            let pump_results: Vec<_> = batch
                .par_iter()
                .filter_map(|tx| {
                    PumpParser::parse_transaction(tx)
                })
                .collect();
                
            let pumpamm_results: Vec<_> = batch
                .par_iter()
                .filter_map(|tx| {
                    PumpAmmParser::parse_transaction(tx)
                })
                .collect();
            
            let boop_results: Vec<_> = batch
                .par_iter()
                .filter_map(|tx| {
                    BoopParser::parse_transaction(tx)
                })
                .collect();
            
            // 收集本地结果
            local_pump_results.extend(pump_results);
            local_pumpamm_results.extend(pumpamm_results);
            local_boop_results.extend(boop_results);
            
            // 一次性获取锁并添加所有结果，减少锁争用
            if !local_pump_results.is_empty() || !local_pumpamm_results.is_empty() || !local_boop_results.is_empty() {
                let mut tx_results = results.lock().unwrap();
                
                // 使用批量添加方法
                if !local_pump_results.is_empty() {
                    tx_results.add_pump_transactions(local_pump_results);
                }
                
                if !local_pumpamm_results.is_empty() {
                    tx_results.add_pumpamm_transactions(local_pumpamm_results);
                }
                
                if !local_boop_results.is_empty() {
                    tx_results.add_boop_transactions(local_boop_results);
                }
            }
        });
        
        // 返回处理结果
        Arc::try_unwrap(results)
            .expect("还有其他线程持有引用")
            .into_inner()
            .expect("无法获取内部值")
    }
} 