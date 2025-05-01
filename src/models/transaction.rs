use std::collections::HashSet;
use crate::models::pump_parser::PumpTransaction;
use crate::models::pumpamm_parser::PumpAmmTransaction;
use crate::models::boop_parser::BoopTransaction;

// 交易结果容器，性能优化版本
#[derive(Default, Debug)]
pub struct TransactionResults {
    #[allow(dead_code)]
    pub pump_signatures: HashSet<String>,
    #[allow(dead_code)]
    pub pumpamm_signatures: HashSet<String>,
    #[allow(dead_code)]
    pub boop_signatures: HashSet<String>,
    #[allow(dead_code)]
    pub pump_transactions: Vec<PumpTransaction>, // 存储PUMP交易的详细信息
    #[allow(dead_code)]
    pub pumpamm_transactions: Vec<PumpAmmTransaction>, // 存储PUMP_AMM交易的详细信息
    #[allow(dead_code)]
    pub boop_transactions: Vec<BoopTransaction>, // 存储boop交易的详细信息
    #[allow(dead_code)]
    pub current_slot: u64, // 存储当前处理的slot
}

impl TransactionResults {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            // 使用较大的初始容量减少重新分配
            pump_signatures: HashSet::with_capacity(128),
            pumpamm_signatures: HashSet::with_capacity(128),
            boop_signatures: HashSet::with_capacity(128),
            pump_transactions: Vec::with_capacity(128),
            pumpamm_transactions: Vec::with_capacity(128),
            boop_transactions: Vec::with_capacity(128),
            current_slot: 0,
        }
    }

    #[allow(dead_code)]
    #[inline]
    pub fn has_results(&self) -> bool {
        !self.pump_signatures.is_empty() || !self.pumpamm_signatures.is_empty() || !self.boop_signatures.is_empty()
    }
    
    // 设置当前slot
    #[allow(dead_code)]
    #[inline]
    pub fn set_current_slot(&mut self, slot: u64) {
        self.current_slot = slot;
    }
    
    // 添加解析后的PUMP交易
    #[allow(dead_code)]
    #[inline]
    pub fn add_pump_transaction(&mut self, transaction: PumpTransaction) {
        // 添加签名到签名集合
        self.pump_signatures.insert(transaction.signature.clone());
        // 添加交易详情到交易列表
        self.pump_transactions.push(transaction);
    }
    
    // 添加解析后的PUMP_AMM交易
    #[allow(dead_code)]
    #[inline]
    pub fn add_pumpamm_transaction(&mut self, transaction: PumpAmmTransaction) {
        // 添加签名到签名集合
        self.pumpamm_signatures.insert(transaction.signature.clone());
        // 添加交易详情到交易列表
        self.pumpamm_transactions.push(transaction);
    }
    
    // 添加解析后的boop交易
    #[allow(dead_code)]
    #[inline]
    pub fn add_boop_transaction(&mut self, transaction: BoopTransaction) {
        // 添加签名到签名集合
        self.boop_signatures.insert(transaction.signature.clone());
        // 添加交易详情到交易列表
        self.boop_transactions.push(transaction);
    }

    // 批量添加PUMP交易
    #[allow(dead_code)]
    #[inline]
    pub fn add_pump_transactions(&mut self, transactions: Vec<PumpTransaction>) {
        for tx in transactions {
            self.add_pump_transaction(tx);
        }
    }
    
    // 批量添加PUMP_AMM交易
    #[allow(dead_code)]
    #[inline]
    pub fn add_pumpamm_transactions(&mut self, transactions: Vec<PumpAmmTransaction>) {
        for tx in transactions {
            self.add_pumpamm_transaction(tx);
        }
    }
    
    // 批量添加boop交易
    #[allow(dead_code)]
    #[inline]
    pub fn add_boop_transactions(&mut self, transactions: Vec<BoopTransaction>) {
        for tx in transactions {
            self.add_boop_transaction(tx);
        }
    }

    #[allow(dead_code)]
    pub fn print(&self) {
        if !self.has_results() {
            return;
        }
        

        // 打印PUMP交易
        for tx in &self.pump_transactions {
            println!("--------------------------------------------------------");
            println!("Parser:PUMP");
            println!("Slot:{}", self.current_slot);
            println!("Signature:{}", tx.signature);
            println!("{}", tx);
            println!("--------------------------------------------------------");
        }
        
        // 打印PUMP_AMM交易
        for tx in &self.pumpamm_transactions {
            println!("--------------------------------------------------------");
            println!("Parser:PUMPAMM");
            println!("Slot:{}", self.current_slot);
            println!("Signature:{}", tx.signature);
            println!("{}", tx);
            println!("--------------------------------------------------------");
        }

        
        // 打印boop交易
        for tx in &self.boop_transactions {
            println!("--------------------------------------------------------");
            println!("Parser:BOOP");
            println!("Slot:{}", self.current_slot);
            println!("Signature:{}", tx.signature);
            println!("{}", tx);
            println!("--------------------------------------------------------");
        }
    }
} 