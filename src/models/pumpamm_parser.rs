use std::fmt;
use std::str::FromStr;
use solana_program::instruction::CompiledInstruction;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::transaction::VersionedTransaction;
use solana_sdk::message::legacy::Message as LegacyMessage;
use solana_sdk::message::v0::Message as V0Message;
use solana_sdk::message::VersionedMessage;
use crate::utils::get_pumpamm_account_labels;

// 添加allow注解来消除警告
#[allow(dead_code)]
pub const PUMP_AMM_PROGRAM_ID: &str = "pAMMBay6oceH9fJKBRHGP5D4bD4sWpmSwMn52FMfXEA";

// PUMP_AMM指令类型（根据IDL定义）
#[derive(Debug, PartialEq, Clone)]
#[allow(dead_code)]
pub enum PumpAmmInstructionType {
    Unknown,
    Buy,             // 从流动性池买入代币
    CreateConfig,    // 创建全局配置
    CreatePool,      // 创建新的流动性池
    Deposit,         // 向流动性池存入代币
    Disable,         // 禁用某些功能
    ExtendAccount,   // 扩展账户
    Sell,            // 向流动性池卖出代币
    UpdateAdmin,     // 更新管理员
    UpdateFeeConfig, // 更新费用配置
    Withdraw,        // 从流动性池取出代币
}

// PUMP_AMM指令的详细信息
#[derive(Debug, Clone)]
pub struct PumpAmmInstruction {
    pub instruction_type: PumpAmmInstructionType,
    pub accounts: Vec<String>,
    pub data: Vec<u8>,
}

impl fmt::Display for PumpAmmInstruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Instruction_Type: {:?}", self.instruction_type)?;
        
        // 获取账户标签
        let account_labels = get_pumpamm_account_labels();
        
        // 根据指令类型显示相关信息
        match &self.instruction_type {
            PumpAmmInstructionType::Buy => {
                if self.data.len() >= 24 {
                    // 解析base_amount_out和max_quote_amount_in参数
                    let base_amount_out = u64::from_le_bytes([
                        self.data[8], self.data[9], self.data[10], self.data[11],
                        self.data[12], self.data[13], self.data[14], self.data[15]
                    ]);
                    
                    let max_quote_amount_in = u64::from_le_bytes([
                        self.data[16], self.data[17], self.data[18], self.data[19],
                        self.data[20], self.data[21], self.data[22], self.data[23]
                    ]);
                    
                    writeln!(f, "Base Amount Out: {}", base_amount_out)?;
                    writeln!(f, "Max Quote Amount In: {}", max_quote_amount_in)?;
                }
                
                // 显示账户
                for (i, account) in self.accounts.iter().enumerate() {
                    let label = account_labels.get(&i).unwrap_or(&"Unknown");
                    writeln!(f, "[{}]:{}: {}", i, label, account)?;
                }
            },
            PumpAmmInstructionType::CreateConfig => {
                if self.data.len() >= 24 {
                    // 解析lp_fee_basis_points和protocol_fee_basis_points参数
                    let lp_fee_basis_points = u64::from_le_bytes([
                        self.data[8], self.data[9], self.data[10], self.data[11],
                        self.data[12], self.data[13], self.data[14], self.data[15]
                    ]);
                    
                    let protocol_fee_basis_points = u64::from_le_bytes([
                        self.data[16], self.data[17], self.data[18], self.data[19],
                        self.data[20], self.data[21], self.data[22], self.data[23]
                    ]);
                    
                    writeln!(f, "LP Fee Basis Points: {}", lp_fee_basis_points)?;
                    writeln!(f, "Protocol Fee Basis Points: {}", protocol_fee_basis_points)?;
                }
                
                // 显示账户
                for (i, account) in self.accounts.iter().enumerate() {
                    let label = account_labels.get(&i).unwrap_or(&"Unknown");
                    writeln!(f, "[{}]:{}: {}", i, label, account)?;
                }
            },
            PumpAmmInstructionType::CreatePool => {
                if self.data.len() >= 24 {
                    // 解析index, base_amount_in和quote_amount_in参数
                    let index = u16::from_le_bytes([
                        self.data[8], self.data[9]
                    ]);
                    
                    let base_amount_in = u64::from_le_bytes([
                        self.data[10], self.data[11], self.data[12], self.data[13],
                        self.data[14], self.data[15], self.data[16], self.data[17]
                    ]);
                    
                    let quote_amount_in = u64::from_le_bytes([
                        self.data[18], self.data[19], self.data[20], self.data[21],
                        self.data[22], self.data[23], self.data[24], self.data[25]
                    ]);
                    
                    writeln!(f, "Index: {}", index)?;
                    writeln!(f, "Base Amount In: {}", base_amount_in)?;
                    writeln!(f, "Quote Amount In: {}", quote_amount_in)?;
                }
                
                // 显示账户
                for (i, account) in self.accounts.iter().enumerate() {
                    let label = account_labels.get(&i).unwrap_or(&"Unknown");
                    writeln!(f, "[{}]:{}: {}", i, label, account)?;
                }
            },
            PumpAmmInstructionType::Deposit => {
                if self.data.len() >= 32 {
                    // 解析lp_token_amount_out, max_base_amount_in和max_quote_amount_in参数
                    let lp_token_amount_out = u64::from_le_bytes([
                        self.data[8], self.data[9], self.data[10], self.data[11],
                        self.data[12], self.data[13], self.data[14], self.data[15]
                    ]);
                    
                    let max_base_amount_in = u64::from_le_bytes([
                        self.data[16], self.data[17], self.data[18], self.data[19],
                        self.data[20], self.data[21], self.data[22], self.data[23]
                    ]);
                    
                    let max_quote_amount_in = u64::from_le_bytes([
                        self.data[24], self.data[25], self.data[26], self.data[27],
                        self.data[28], self.data[29], self.data[30], self.data[31]
                    ]);
                    
                    writeln!(f, "LP Token Amount Out: {}", lp_token_amount_out)?;
                    writeln!(f, "Max Base Amount In: {}", max_base_amount_in)?;
                    writeln!(f, "Max Quote Amount In: {}", max_quote_amount_in)?;
                }
                
                // 显示账户
                for (i, account) in self.accounts.iter().enumerate() {
                    let label = account_labels.get(&i).unwrap_or(&"Unknown");
                    writeln!(f, "账户[{}]:{}: {}", i, label, account)?;
                }
            },
            PumpAmmInstructionType::Sell => {
                if self.data.len() >= 24 {
                    // 解析base_amount_in和min_quote_amount_out参数
                    let base_amount_in = u64::from_le_bytes([
                        self.data[8], self.data[9], self.data[10], self.data[11],
                        self.data[12], self.data[13], self.data[14], self.data[15]
                    ]);
                    
                    let min_quote_amount_out = u64::from_le_bytes([
                        self.data[16], self.data[17], self.data[18], self.data[19],
                        self.data[20], self.data[21], self.data[22], self.data[23]
                    ]);
                    
                    writeln!(f, "Base Amount In: {}", base_amount_in)?;
                    writeln!(f, "Min Quote Amount Out: {}", min_quote_amount_out)?;
                }
                
                // 显示账户
                for (i, account) in self.accounts.iter().enumerate() {
                    let label = account_labels.get(&i).unwrap_or(&"Unknown");
                    writeln!(f, "[{}]:{}: {}", i, label, account)?;
                }
            },
            PumpAmmInstructionType::Withdraw => {
                if self.data.len() >= 32 {
                    // 解析lp_token_amount_in, min_base_amount_out和min_quote_amount_out参数
                    let lp_token_amount_in = u64::from_le_bytes([
                        self.data[8], self.data[9], self.data[10], self.data[11],
                        self.data[12], self.data[13], self.data[14], self.data[15]
                    ]);
                    
                    let min_base_amount_out = u64::from_le_bytes([
                        self.data[16], self.data[17], self.data[18], self.data[19],
                        self.data[20], self.data[21], self.data[22], self.data[23]
                    ]);
                    
                    let min_quote_amount_out = u64::from_le_bytes([
                        self.data[24], self.data[25], self.data[26], self.data[27],
                        self.data[28], self.data[29], self.data[30], self.data[31]
                    ]);
                    
                    writeln!(f, "LP Token Amount In: {}", lp_token_amount_in)?;
                    writeln!(f, "Min Base Amount Out: {}", min_base_amount_out)?;
                    writeln!(f, "Min Quote Amount Out: {}", min_quote_amount_out)?;
                }
                
                // 显示账户
                for (i, account) in self.accounts.iter().enumerate() {
                    let label = account_labels.get(&i).unwrap_or(&"Unknown");
                    writeln!(f, "账户[{}]:{}: {}", i, label, account)?;
                }
            },
            _ => {
                // 对于其他指令类型，只显示账户列表
                for (i, account) in self.accounts.iter().enumerate() {
                    let label = account_labels.get(&i).unwrap_or(&"Unknown");
                    writeln!(f, "账户[{}]:{}: {}", i, label, account)?;
                }
            }
        }
        
        Ok(())
    }
}

// PUMP_AMM交易结构
#[derive(Debug, Clone)]
pub struct PumpAmmTransaction {
    pub signature: String,
    pub instructions: Vec<PumpAmmInstruction>,
}

impl fmt::Display for PumpAmmTransaction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Instructions_Count: {}", self.instructions.len())?;
        
        for (i, instruction) in self.instructions.iter().enumerate() {
            writeln!(f, "Instruction[{}]Type: {:?}", i, instruction.instruction_type)?;
            write!(f, "{}", instruction)?;
        }
        
        Ok(())
    }
}

// PUMP_AMM解析器
#[allow(dead_code)]
pub struct PumpAmmParser;

impl PumpAmmParser {
    // 解析交易，提取PUMP_AMM指令
    #[allow(dead_code)]
    pub fn parse_transaction(transaction: &VersionedTransaction) -> Option<PumpAmmTransaction> {
        // 获取PUMP_AMM程序的Pubkey
        let pump_amm_program_id = Pubkey::from_str(PUMP_AMM_PROGRAM_ID).ok()?;
        
        // 提取PUMP_AMM相关指令
        let pump_amm_instructions = match &transaction.message {
            VersionedMessage::Legacy(message) => {
                Self::extract_pump_amm_instructions_from_legacy(message, &pump_amm_program_id)
            }
            VersionedMessage::V0(message) => {
                Self::extract_pump_amm_instructions_from_v0(message, &pump_amm_program_id)
            }
        };
        
        // 如果没有PUMP_AMM指令，则返回None
        if pump_amm_instructions.is_empty() {
            return None;
        }
        
        // 获取交易签名
        let signature = if !transaction.signatures.is_empty() {
            transaction.signatures[0].to_string()
        } else {
            "No_Signature".to_string()
        };
        
        // 返回解析结果
        Some(PumpAmmTransaction {
            signature,
            instructions: pump_amm_instructions,
        })
    }
    
    // 从Legacy消息中提取PUMP_AMM指令
    fn extract_pump_amm_instructions_from_legacy(
        message: &LegacyMessage,
        pump_amm_program_id: &Pubkey,
    ) -> Vec<PumpAmmInstruction> {
        let account_keys = &message.account_keys;
        
        message
            .instructions
            .iter()
            .filter_map(|ix| {
                let program_idx = ix.program_id_index as usize;
                if program_idx < account_keys.len() && account_keys[program_idx] == *pump_amm_program_id {
                    Some(Self::compile_instruction_to_pump_amm_instruction(ix, account_keys))
                } else {
                    None
                }
            })
            .collect()
    }
    
    // 从V0消息中提取PUMP_AMM指令
    fn extract_pump_amm_instructions_from_v0(
        message: &V0Message,
        pump_amm_program_id: &Pubkey,
    ) -> Vec<PumpAmmInstruction> {
        let mut pump_amm_instructions = Vec::new();
        
        // 获取静态账户和地址查找表
        let static_keys = &message.account_keys;
        let lookup_tables = &message.address_table_lookups;
        
        // 如果有地址查找表，需要解析完整的账户列表（比较复杂，简化处理）
        if !lookup_tables.is_empty() {
            // 这里简化处理，实际情况下需要根据地址查找表解析完整的账户列表
            // 完整解析需要访问Solana状态数据，这里只处理静态账户
            return Vec::new();
        }
        
        // 遍历指令
        for ix in &message.instructions {
            if ix.program_id_index as usize >= static_keys.len() {
                continue;
            }
            
            let program_id = &static_keys[ix.program_id_index as usize];
            
            // 检查是否是PUMP_AMM程序的指令
            if program_id == pump_amm_program_id {
                pump_amm_instructions.push(Self::compile_instruction_to_pump_amm_instruction(
                    ix,
                    static_keys,
                ));
            }
        }
        
        pump_amm_instructions
    }
    
    // 将编译后的指令转换为PUMP_AMM指令
    fn compile_instruction_to_pump_amm_instruction(
        ix: &CompiledInstruction,
        account_keys: &[Pubkey],
    ) -> PumpAmmInstruction {
        // 解析指令类型
        let instruction_type = if !ix.data.is_empty() && ix.data.len() >= 8 {
            // 根据IDL中的discriminator识别指令类型
            let discriminator = &ix.data[0..8];
            match discriminator {
                // Buy指令
                [102, 6, 61, 18, 1, 218, 235, 234] => PumpAmmInstructionType::Buy,
                
                // CreateConfig指令
                [201, 207, 243, 114, 75, 111, 47, 189] => PumpAmmInstructionType::CreateConfig,
                
                // CreatePool指令
                [233, 146, 209, 142, 207, 104, 64, 188] => PumpAmmInstructionType::CreatePool,
                
                // Deposit指令
                [242, 35, 198, 137, 82, 225, 242, 182] => PumpAmmInstructionType::Deposit,
                
                // Disable指令
                [185, 173, 187, 90, 216, 15, 238, 233] => PumpAmmInstructionType::Disable,
                
                // ExtendAccount指令
                [234, 102, 194, 203, 150, 72, 62, 229] => PumpAmmInstructionType::ExtendAccount,
                
                // Sell指令
                [51, 230, 133, 164, 1, 127, 131, 173] => PumpAmmInstructionType::Sell,
                
                // UpdateAdmin指令
                [161, 176, 40, 213, 60, 184, 179, 228] => PumpAmmInstructionType::UpdateAdmin,
                
                // UpdateFeeConfig指令
                [104, 184, 103, 242, 88, 151, 107, 20] => PumpAmmInstructionType::UpdateFeeConfig,
                
                // Withdraw指令
                [183, 18, 70, 156, 148, 109, 161, 34] => PumpAmmInstructionType::Withdraw,
                
                // 未知指令
                _ => PumpAmmInstructionType::Unknown,
            }
        } else {
            PumpAmmInstructionType::Unknown
        };
        
        // 获取账户地址
        let accounts = ix
            .accounts
            .iter()
            .filter_map(|account_idx| {
                account_keys
                    .get(*account_idx as usize)
                    .map(|pubkey| pubkey.to_string())
            })
            .collect();
        
        PumpAmmInstruction {
            instruction_type,
            accounts,
            data: ix.data.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use solana_sdk::hash::Hash;
    use solana_sdk::signature::{Keypair, Signature};
    use solana_sdk::signer::Signer;
    
    // 创建一个模拟的PUMP_AMM交易
    fn create_mock_pump_amm_transaction() -> VersionedTransaction {
        // 创建一些账户
        let from = Keypair::new();
        let to = Pubkey::new_unique();
        let program_id = Pubkey::from_str(PUMP_AMM_PROGRAM_ID).unwrap();
        
        // 创建一个Legacy消息
        let message = LegacyMessage::new_with_compiled_instructions(
            1,
            0,
            0,
            vec![from.pubkey(), to, program_id],
            Hash::default(),
            vec![CompiledInstruction {
                program_id_index: 2,
                accounts: vec![0, 1],
                data: vec![102, 6, 61, 18, 1, 218, 235, 234, 1, 0, 0, 0, 0, 0, 0, 0, 10, 0, 0, 0, 0, 0, 0, 0],
            }],
        );
        
        // 创建一个签名
        let signature = Signature::new_unique();
        
        // 创建一个VersionedTransaction
        VersionedTransaction {
            signatures: vec![signature],
            message: VersionedMessage::Legacy(message),
        }
    }
    
    #[test]
    fn test_pump_amm_transaction_parsing() {
        let transaction = create_mock_pump_amm_transaction();
        let pump_amm_transaction = PumpAmmParser::parse_transaction(&transaction);
        
        assert!(pump_amm_transaction.is_some());
        
        let pump_amm_transaction = pump_amm_transaction.unwrap();
        assert_eq!(pump_amm_transaction.instructions.len(), 1);
        assert_eq!(pump_amm_transaction.instructions[0].instruction_type, PumpAmmInstructionType::Buy);
    }
    
    #[test]
    fn test_non_pump_amm_transaction() {
        // 创建一个非PUMP_AMM交易
        let from = Keypair::new();
        let to = Pubkey::new_unique();
        let program_id = Pubkey::new_unique();
        
        let message = LegacyMessage::new_with_compiled_instructions(
            1,
            0,
            0,
            vec![from.pubkey(), to, program_id],
            Hash::default(),
            vec![CompiledInstruction {
                program_id_index: 2,
                accounts: vec![0, 1],
                data: vec![0, 1, 2, 3],
            }],
        );
        
        let signature = Signature::new_unique();
        
        let transaction = VersionedTransaction {
            signatures: vec![signature],
            message: VersionedMessage::Legacy(message),
        };
        
        let pump_amm_transaction = PumpAmmParser::parse_transaction(&transaction);
        assert!(pump_amm_transaction.is_none());
    }
}
