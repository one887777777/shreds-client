use std::fmt;
use std::str::FromStr;
use solana_program::instruction::CompiledInstruction;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::transaction::VersionedTransaction;
use solana_sdk::message::legacy::Message as LegacyMessage;
use solana_sdk::message::v0::Message as V0Message;
use solana_sdk::message::VersionedMessage;

// 添加allow注解来消除警告
#[allow(dead_code)]
pub const PUMP_PROGRAM_ID: &str = "6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P";

// PUMP指令类型（根据IDL定义）
#[derive(Debug, PartialEq, Clone)]
#[allow(dead_code)]
pub enum PumpInstructionType {
    Unknown,
    Buy,          // 从绑定曲线购买代币
    Create,       // 创建新币种和绑定曲线
    ExtendAccount,// 扩展程序拥有的账户
    Initialize,   // 初始化全局状态
    Migrate,      // 如果绑定曲线完成，则将流动性迁移到pump_amm
    Sell,         // 卖出代币到绑定曲线
    SetParams,    // 设置全局参数
    UpdateGlobalAuthority, // 更新全局权限
    Withdraw,     // 如果绑定曲线完成，允许管理员提取流动性用于迁移
}

// PUMP指令的详细信息
#[derive(Debug, Clone)]
pub struct PumpInstruction {
    pub instruction_type: PumpInstructionType,
    pub accounts: Vec<String>,
    pub data: Vec<u8>,
}

impl fmt::Display for PumpInstruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        
        // 首先打印指令参数
        match &self.instruction_type {
            PumpInstructionType::Buy => {
                if self.data.len() >= 16 {
                    // 解析amount和max_sol_cost参数
                    let amount = u64::from_le_bytes([
                        self.data[8], self.data[9], self.data[10], self.data[11],
                        self.data[12], self.data[13], self.data[14], self.data[15]
                    ]);
                    
                    writeln!(f, "Token_Amount: {}", amount)?;
                    
                    if self.data.len() >= 24 {
                        let max_sol_cost = u64::from_le_bytes([
                            self.data[16], self.data[17], self.data[18], self.data[19],
                            self.data[20], self.data[21], self.data[22], self.data[23]
                        ]);
                        writeln!(f, "Max_SOL_Cost: {} ", max_sol_cost)?;
                    }
                }
                
                if self.accounts.len() >= 3 {
                    writeln!(f, "[0]Global: {}", self.accounts[0])?;
                    writeln!(f, "[1]Fee_Recipient: {}", self.accounts[1])?;
                    writeln!(f, "[2]Mint: {}", self.accounts[2])?;
                    
                    if self.accounts.len() >= 4 {
                        writeln!(f, "[3]Bonding_Curve: {}", self.accounts[3])?;
                    }
                    if self.accounts.len() >= 5 {
                        writeln!(f, "[4]Associated_Bonding_Curve: {}", self.accounts[4])?;
                    }
                    if self.accounts.len() >= 6 {
                        writeln!(f, "[5]Associated_User: {}", self.accounts[5])?;
                    }
                    if self.accounts.len() >= 7 {
                        writeln!(f, "[6]User: {}", self.accounts[6])?;
                    }
                    if self.accounts.len() >= 8 {
                        writeln!(f, "[7]System_Program: {}", self.accounts[7])?;
                    }
                    if self.accounts.len() >= 9 {
                        writeln!(f, "[8]Token_Program: {}", self.accounts[8])?;
                    }
                    if self.accounts.len() >= 10 {
                        writeln!(f, "[9]Rent: {}", self.accounts[9])?;
                    }
                    if self.accounts.len() >= 11 {
                        writeln!(f, "[10]Event_Authority: {}", self.accounts[10])?;
                    }
                    if self.accounts.len() >= 12 {
                        writeln!(f, "[11]Program: {}", self.accounts[11])?;
                    }
                }
            },
            PumpInstructionType::Create => {
                // Create指令的字符串参数在data[8..]之后
                if self.data.len() > 8 {
                    // 前8字节是discriminator，后面是参数数据
                    let mut offset = 8;
                    
                    // 解析name字段
                    if offset + 4 <= self.data.len() {
                        // 读取name字符串长度
                        let name_len = u32::from_le_bytes([
                            self.data[offset], self.data[offset+1], self.data[offset+2], self.data[offset+3]
                        ]) as usize;
                        offset += 4;
                        
                        // 读取name字符串内容
                        if offset + name_len <= self.data.len() {
                            let name = String::from_utf8_lossy(&self.data[offset..offset+name_len]);
                            writeln!(f, "name: {}", name)?;
                            offset += name_len;
                            
                            // 解析symbol字段
                            if offset + 4 <= self.data.len() {
                                // 读取symbol字符串长度
                                let symbol_len = u32::from_le_bytes([
                                    self.data[offset], self.data[offset+1], self.data[offset+2], self.data[offset+3]
                                ]) as usize;
                                offset += 4;
                                
                                // 读取symbol字符串内容
                                if offset + symbol_len <= self.data.len() {
                                    let symbol = String::from_utf8_lossy(&self.data[offset..offset+symbol_len]);
                                    writeln!(f, "symbol: {}", symbol)?;
                                    offset += symbol_len;
                                    
                                    // 解析URI字段
                                    if offset + 4 <= self.data.len() {
                                        // 读取URI字符串长度
                                        let uri_len = u32::from_le_bytes([
                                            self.data[offset], self.data[offset+1], self.data[offset+2], self.data[offset+3]
                                        ]) as usize;
                                        offset += 4;
                                        
                                        // 读取URI字符串内容
                                        if offset + uri_len <= self.data.len() {
                                            let uri = String::from_utf8_lossy(&self.data[offset..offset+uri_len]);
                                            writeln!(f, "uri: {}", uri)?;
                                            offset += uri_len;
                                            
                                            // 解析creator字段 (Pubkey是32字节)
                                            if offset + 32 <= self.data.len() {
                                                let creator = bs58::encode(&self.data[offset..offset+32]).into_string();
                                                writeln!(f, "creator: {}", creator)?;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                
                if self.accounts.len() >= 3 {
                    writeln!(f, "[0]Mint: {}", self.accounts[0])?;
                    writeln!(f, "[1]Mint_Authority: {}", self.accounts[1])?;
                    writeln!(f, "[2]Bonding_Curve: {}", self.accounts[2])?;
                    
                    if self.accounts.len() >= 4 {
                        writeln!(f, "[3]Associated_Bonding_Curve: {}", self.accounts[3])?;
                    }
                    if self.accounts.len() >= 5 {
                        writeln!(f, "[4]Global: {}", self.accounts[4])?;
                    }
                    if self.accounts.len() >= 6 {
                        writeln!(f, "[5]Mpl_Token_Metadata: {}", self.accounts[5])?;
                    }
                    if self.accounts.len() >= 7 {
                        writeln!(f, "[6]Metadata: {}", self.accounts[6])?;
                    }
                    if self.accounts.len() >= 8 {
                        writeln!(f, "[7]User: {}", self.accounts[7])?;
                    }
                    if self.accounts.len() >= 9 {
                        writeln!(f, "[8]System_Program: {}", self.accounts[8])?;
                    }
                    if self.accounts.len() >= 10 {
                        writeln!(f, "[9]Token_Program: {}", self.accounts[9])?;
                    }
                    if self.accounts.len() >= 11 {
                        writeln!(f, "[10]Associated_Token_Program: {}", self.accounts[10])?;
                    }
                    if self.accounts.len() >= 12 {
                        writeln!(f, "[11]Rent: {}", self.accounts[11])?;
                    }
                    if self.accounts.len() >= 13 {
                        writeln!(f, "[12]Event_Authority: {}", self.accounts[12])?;
                    }
                    if self.accounts.len() >= 14 {
                        writeln!(f, "[13]Program: {}", self.accounts[13])?;
                    }
                }
            },
            PumpInstructionType::Sell => {
                if self.data.len() >= 16 {
                    // 解析amount和min_sol_output参数
                    let amount = u64::from_le_bytes([
                        self.data[8], self.data[9], self.data[10], self.data[11],
                        self.data[12], self.data[13], self.data[14], self.data[15]
                    ]);
                    
                    writeln!(f, "Token_Amount: {}", amount)?;
                    
                    if self.data.len() >= 24 {
                        let min_sol_output = u64::from_le_bytes([
                            self.data[16], self.data[17], self.data[18], self.data[19],
                            self.data[20], self.data[21], self.data[22], self.data[23]
                        ]);
                        writeln!(f, "Min_SOL_Output: {} ", min_sol_output)?;
                    }
                }
                
                if self.accounts.len() >= 3 {
                    writeln!(f, "[0]Global: {}", self.accounts[0])?;
                    writeln!(f, "[1]Fee_Recipient: {}", self.accounts[1])?;
                    writeln!(f, "[2]Mint: {}", self.accounts[2])?;
                    
                    if self.accounts.len() >= 4 {
                        writeln!(f, "[3]Bonding_Curve: {}", self.accounts[3])?;
                    }
                    if self.accounts.len() >= 5 {
                        writeln!(f, "[4]Associated_Bonding_Curve: {}", self.accounts[4])?;
                    }
                    if self.accounts.len() >= 6 {
                        writeln!(f, "[5]Associated_User: {}", self.accounts[5])?;
                    }
                    if self.accounts.len() >= 7 {
                        writeln!(f, "[6]User: {}", self.accounts[6])?;
                    }
                    if self.accounts.len() >= 8 {
                        writeln!(f, "[7]System_Program: {}", self.accounts[7])?;
                    }
                    if self.accounts.len() >= 9 {
                        writeln!(f, "[8]Token_Program: {}", self.accounts[8])?;
                    }
                    if self.accounts.len() >= 10 {
                        writeln!(f, "[9]Rent: {}", self.accounts[9])?;
                    }
                    if self.accounts.len() >= 11 {
                        writeln!(f, "[10]Event_Authority: {}", self.accounts[10])?;
                    }
                    if self.accounts.len() >= 12 {
                        writeln!(f, "[11]Program: {}", self.accounts[11])?;
                    }
                }
            },
            PumpInstructionType::SetParams => {
                // SetParams有多个参数
                writeln!(f, "SetParams: 设置全局参数")?;
            },
            PumpInstructionType::Initialize => {
                writeln!(f, "Initialize: 初始化全局状态，无参数")?;
            },
            PumpInstructionType::Withdraw => {
                writeln!(f, "Withdraw: 提取流动性（管理员操作），无参数")?;
            },
            PumpInstructionType::Migrate => {
                writeln!(f, "Migrate: 迁移流动性到pump_amm")?;
            },
            PumpInstructionType::ExtendAccount => {
                writeln!(f, "ExtendAccount: 扩展程序拥有的账户大小")?;
            },
            PumpInstructionType::UpdateGlobalAuthority => {
                writeln!(f, "UpdateGlobalAuthority: 更新全局权限")?;
            },
            PumpInstructionType::Unknown => {
                writeln!(f, "Unknown: 未知指令")?;
            },
        }
        
        
        Ok(())
    }
}

// PUMP交易的解析结果
#[derive(Debug, Clone)]
pub struct PumpTransaction {
    pub signature: String,
    pub instructions: Vec<PumpInstruction>,
}

impl fmt::Display for PumpTransaction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Instructions_Count: {}", self.instructions.len())?;
        
        for (i, instruction) in self.instructions.iter().enumerate() {
            writeln!(f, "Instruction[{}]Type: {:?}", i, instruction.instruction_type)?;
            write!(f, "{}", instruction)?;
        }
        
        Ok(())
    }
}

// PUMP解析器
#[allow(dead_code)]
pub struct PumpParser;

impl PumpParser {
    // 解析交易，提取PUMP指令
    #[allow(dead_code)]
    pub fn parse_transaction(transaction: &VersionedTransaction) -> Option<PumpTransaction> {
        // 获取PUMP程序的Pubkey
        let pump_program_id = Pubkey::from_str(PUMP_PROGRAM_ID).ok()?;
        
        // 提取PUMP相关指令
        let pump_instructions = match &transaction.message {
            VersionedMessage::Legacy(message) => {
                Self::extract_pump_instructions_from_legacy(message, &pump_program_id)
            }
            VersionedMessage::V0(message) => {
                Self::extract_pump_instructions_from_v0(message, &pump_program_id)
            }
        };
        
        // 如果没有PUMP指令，则返回None
        if pump_instructions.is_empty() {
            return None;
        }
        
        // 获取交易签名
        let signature = if !transaction.signatures.is_empty() {
            transaction.signatures[0].to_string()
        } else {
            "No_Signature".to_string()
        };
        
        // 返回解析结果
        Some(PumpTransaction {
            signature,
            instructions: pump_instructions,
        })
    }
    
    // 从Legacy消息中提取PUMP指令
    fn extract_pump_instructions_from_legacy(
        message: &LegacyMessage,
        pump_program_id: &Pubkey,
    ) -> Vec<PumpInstruction> {
        let account_keys = &message.account_keys;
        
        message
            .instructions
            .iter()
            .filter_map(|ix| {
                let program_idx = ix.program_id_index as usize;
                if program_idx < account_keys.len() && account_keys[program_idx] == *pump_program_id {
                    Some(Self::compile_instruction_to_pump_instruction(ix, account_keys))
                } else {
                    None
                }
            })
            .collect()
    }
    
    // 从V0消息中提取PUMP指令
    fn extract_pump_instructions_from_v0(
        message: &V0Message,
        pump_program_id: &Pubkey,
    ) -> Vec<PumpInstruction> {
        let mut pump_instructions = Vec::new();
        
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
            
            // 检查是否是PUMP程序的指令
            if program_id == pump_program_id {
                pump_instructions.push(Self::compile_instruction_to_pump_instruction(
                    ix,
                    static_keys,
                ));
            }
        }
        
        pump_instructions
    }
    
    // 将编译后的指令转换为PUMP指令
    fn compile_instruction_to_pump_instruction(
        ix: &CompiledInstruction,
        account_keys: &[Pubkey],
    ) -> PumpInstruction {
        // 解析指令类型
        let instruction_type = if !ix.data.is_empty() && ix.data.len() >= 8 {
            // 根据IDL中的discriminator识别指令类型
            let discriminator = &ix.data[0..8];
            match discriminator {
                // Buy指令 - 从IDL中复制的discriminator
                [102, 6, 61, 18, 1, 218, 235, 234] => PumpInstructionType::Buy,
                [242, 35, 198, 137, 82, 225, 242, 182] => PumpInstructionType::Buy,
                
                // Create指令
                [24, 30, 200, 40, 5, 28, 7, 119] => PumpInstructionType::Create,
                [54, 49, 138, 255, 162, 99, 87, 199] => PumpInstructionType::Create,
                
                // ExtendAccount指令
                [234, 102, 194, 203, 150, 72, 62, 229] => PumpInstructionType::ExtendAccount,
                
                // Initialize指令
                [175, 175, 109, 31, 13, 152, 155, 237] => PumpInstructionType::Initialize,
                [103, 232, 80, 22, 46, 244, 138, 11] => PumpInstructionType::Initialize,
                
                // Migrate指令
                [155, 234, 231, 146, 236, 158, 162, 30] => PumpInstructionType::Migrate,
                
                // Sell指令
                [51, 230, 133, 164, 1, 127, 131, 173] => PumpInstructionType::Sell,
                [157, 141, 99, 91, 56, 32, 241, 199] => PumpInstructionType::Sell,
                
                // SetParams指令
                [27, 234, 178, 52, 147, 2, 187, 141] => PumpInstructionType::SetParams,
                [186, 127, 135, 21, 36, 67, 77, 55] => PumpInstructionType::SetParams,
                
                // UpdateGlobalAuthority指令
                [227, 181, 74, 196, 208, 21, 97, 213] => PumpInstructionType::UpdateGlobalAuthority,
                
                // Withdraw指令
                [183, 18, 70, 156, 148, 109, 161, 34] => PumpInstructionType::Withdraw,
                [16, 9, 233, 100, 246, 18, 249, 253] => PumpInstructionType::Withdraw,
                
                // 未知指令
                _ => PumpInstructionType::Unknown,
            }
        } else {
            PumpInstructionType::Unknown
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
        
        PumpInstruction {
            instruction_type,
            accounts,
            data: ix.data.clone(),
        }
    }
}

// 添加allow注解来消除警告
#[allow(dead_code)]
pub const COMPUTE_BUDGET_PROGRAM_ID: &str = "ComputeBudget111111111111111111111111111111";

// ComputeBudget指令类型
#[derive(Debug, PartialEq, Clone)]
#[allow(dead_code)]
pub enum ComputeBudgetInstructionType {
    Unknown,
    RequestHeapFrame,         // 请求堆帧大小
    RequestComputeUnits,      // 设置计算单元上限
    SetComputeUnitLimit,      // 设置计算单元上限（新版）
    SetComputeUnitPrice,      // 设置计算单元价格（优先费）
}

// ComputeBudget指令详细信息
#[derive(Debug, Clone)]
pub struct ComputeBudgetInstruction {
    pub instruction_type: ComputeBudgetInstructionType,
    pub data: Vec<u8>,
}

impl fmt::Display for ComputeBudgetInstruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.instruction_type {
            ComputeBudgetInstructionType::RequestHeapFrame => {
                if self.data.len() >= 5 {
                    let size = u32::from_le_bytes([
                        self.data[1], self.data[2], self.data[3], self.data[4]
                    ]);
                    writeln!(f, "Request_Heap_Frame_Size: {} 字节", size)?;
                }
            },
            ComputeBudgetInstructionType::RequestComputeUnits |
            ComputeBudgetInstructionType::SetComputeUnitLimit => {
                if self.data.len() >= 5 {
                    let units = u32::from_le_bytes([
                        self.data[1], self.data[2], self.data[3], self.data[4]
                    ]);
                    writeln!(f, "Set_Compute_Unit_Limit: {}", units)?;
                    
                    // 估算费用（按照默认没有优先费的情况）
                    let estimated_fee = (units as f64) * 0.0000005; // 0.0000005 lamports per CU
                    writeln!(f, "Estimated_Fee: {:.9} SOL (Based_on {} CU)", estimated_fee, units)?;
                }
            },
            ComputeBudgetInstructionType::SetComputeUnitPrice => {
                if self.data.len() >= 9 {
                    let price = u64::from_le_bytes([
                        self.data[1], self.data[2], self.data[3], self.data[4],
                        self.data[5], self.data[6], self.data[7], self.data[8]
                    ]);
                    writeln!(f, "Set_Compute_Unit_Price: {} 微lamports/计算单元", price)?;
                    
                    // 估算200K CU的费用
                    let estimated_cu = 200_000;
                    let estimated_fee = (price as f64) * (estimated_cu as f64) / 1_000_000.0 / 1_000_000_000.0;
                    writeln!(f, "Estimated_Fee: {:.9} SOL (Based_on {}K CU)", estimated_fee, estimated_cu/1000)?;
                }
            },
            ComputeBudgetInstructionType::Unknown => {
                writeln!(f, "Unknown: 未知的ComputeBudget指令")?;
            },
        }
        
        // 显示完整的指令数据格式
        writeln!(f, "Instruction_Data_Format:")?;
        for (i, byte) in self.data.iter().enumerate() {
            if i == 0 {
                writeln!(f, "  位置 {}: {} (指令码)", i, byte)?;
            } else if self.instruction_type == ComputeBudgetInstructionType::SetComputeUnitPrice && i <= 8 {
                if i == 1 {
                    writeln!(f, "  位置 {}-8: 价格参数 (u64, 小端序)", i)?;
                }
            } else if (self.instruction_type == ComputeBudgetInstructionType::RequestComputeUnits || 
                       self.instruction_type == ComputeBudgetInstructionType::SetComputeUnitLimit ||
                       self.instruction_type == ComputeBudgetInstructionType::RequestHeapFrame) && i <= 4 {
                if i == 1 {
                    writeln!(f, "  位置 {}-4: 单元/大小参数 (u32, 小端序)", i)?;
                }
            } else {
                writeln!(f, "  位置 {}: {}", i, byte)?;
            }
        }
        
        // 显示原始数据
        writeln!(f, "Raw_Data: {}", hex::encode(&self.data))?;
        
        Ok(())
    }
}

impl PumpParser {
    // 解析ComputeBudget指令
    #[allow(dead_code)]
    pub fn parse_compute_budget_instruction(data: &[u8]) -> ComputeBudgetInstruction {
        let instruction_type = if !data.is_empty() {
            match data[0] {
                0 => ComputeBudgetInstructionType::RequestHeapFrame,
                1 => ComputeBudgetInstructionType::RequestComputeUnits,
                2 => ComputeBudgetInstructionType::SetComputeUnitLimit,
                3 => ComputeBudgetInstructionType::SetComputeUnitPrice,
                _ => ComputeBudgetInstructionType::Unknown,
            }
        } else {
            ComputeBudgetInstructionType::Unknown
        };
        
        ComputeBudgetInstruction {
            instruction_type,
            data: data.to_vec(),
        }
    }
} 