use std::fmt;
use std::str::FromStr;
use solana_program::instruction::CompiledInstruction;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::transaction::VersionedTransaction;
use solana_sdk::message::legacy::Message as LegacyMessage;
use solana_sdk::message::v0::Message as V0Message;
use solana_sdk::message::VersionedMessage;
use crate::utils::get_boop_account_labels_by_instruction;

// 添加allow注解来消除警告
#[allow(dead_code)]
pub const BOOP_PROGRAM_ID: &str = "boop8hVGQGqehUK2iVEMEnMrL5RbjywRzHKBmBE7ry4";

// BOOP指令类型（根据IDL定义）
#[derive(Debug, PartialEq, Clone)]
#[allow(dead_code)]
pub enum BoopInstructionType {
    Unknown,
    // 新指令类型
    BuyToken,      // 购买代币
    SellToken,     // 卖出代币
    CreateToken,   // 创建代币
    DeployBondingCurve, // 部署绑定曲线
    
    // 原始指令类型
    Create,        // 创建新币种
    Sell,          // 卖出代币
    Initialize,    // 初始化全局状态
    SetParams,     // 设置全局参数
    UpdateAuthority, // 更新权限
}

// BOOP指令的详细信息
#[derive(Debug, Clone)]
pub struct BoopInstruction {
    pub instruction_type: BoopInstructionType,
    pub accounts: Vec<String>,
    pub data: Vec<u8>,
}

impl fmt::Display for BoopInstruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // 首先打印指令参数
        match &self.instruction_type {
            BoopInstructionType::BuyToken => {
                if self.data.len() >= 16 {
                    // 解析amount参数（跳过前8字节的鉴别器）
                    let amount = u64::from_le_bytes([
                        self.data[8], self.data[9], self.data[10], self.data[11],
                        self.data[12], self.data[13], self.data[14], self.data[15]
                    ]);
                    
                    writeln!(f, "Token_Amount: {}", amount)?;
                    
                    // 解析max_sol_cost参数（如果存在）
                    if self.data.len() >= 24 {
                        let max_sol_cost = u64::from_le_bytes([
                            self.data[16], self.data[17], self.data[18], self.data[19],
                            self.data[20], self.data[21], self.data[22], self.data[23]
                        ]);
                        writeln!(f, "Max_SOL_Cost: {} ", max_sol_cost)?;
                    }
                }
                
                // 获取账户标签
                let account_labels = get_boop_account_labels_by_instruction("BuyToken");
                
                // 打印账户信息
                for (i, account) in self.accounts.iter().enumerate() {
                    let label = account_labels.get(&i).unwrap_or(&"Unknown");
                    writeln!(f, "[{}]{}: {}", i, label, account)?;
                }
            },
            BoopInstructionType::SellToken => {
                if self.data.len() >= 16 {
                    // 解析amount参数（跳过前8字节的鉴别器）
                    let amount = u64::from_le_bytes([
                        self.data[8], self.data[9], self.data[10], self.data[11],
                        self.data[12], self.data[13], self.data[14], self.data[15]
                    ]);
                    
                    writeln!(f, "Token_Amount: {}", amount)?;
                    
                    // 解析min_sol_out参数（如果存在）
                    if self.data.len() >= 24 {
                        let min_sol_out = u64::from_le_bytes([
                            self.data[16], self.data[17], self.data[18], self.data[19],
                            self.data[20], self.data[21], self.data[22], self.data[23]
                        ]);
                        writeln!(f, "Min_SOL_Output: {} ", min_sol_out)?;
                    }
                }
                
                // 获取账户标签
                let account_labels = get_boop_account_labels_by_instruction("SellToken");
                
                // 打印账户信息
                for (i, account) in self.accounts.iter().enumerate() {
                    let label = account_labels.get(&i).unwrap_or(&"Unknown");
                    writeln!(f, "[{}]{}: {}", i, label, account)?;
                }
            },
            BoopInstructionType::CreateToken => {
                // 解析CreateToken指令的参数数据
                if self.data.len() >= 16 { // 至少包含鉴别器(8字节)和部分数据
                    // 跳过前8字节的鉴别器
                    let mut offset = 8;
                    
                    // 首先解析 salt (u64, 8字节)
                    if offset + 8 <= self.data.len() {
                        let salt = u64::from_le_bytes([
                            self.data[offset], self.data[offset+1], self.data[offset+2], self.data[offset+3],
                            self.data[offset+4], self.data[offset+5], self.data[offset+6], self.data[offset+7]
                        ]);
                        writeln!(f, "Salt: {}", salt)?;
                        offset += 8;
                    }
                    
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
                            writeln!(f, "Name: {}", name)?;
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
                                    writeln!(f, "Symbol: {}", symbol)?;
                                    offset += symbol_len;
                                    
                                    // 解析uri字段
                                    if offset + 4 <= self.data.len() {
                                        // 读取uri字符串长度
                                        let uri_len = u32::from_le_bytes([
                                            self.data[offset], self.data[offset+1], self.data[offset+2], self.data[offset+3]
                                        ]) as usize;
                                        offset += 4;
                                        
                                        // 读取uri字符串内容
                                        if offset + uri_len <= self.data.len() {
                                            let uri = String::from_utf8_lossy(&self.data[offset..offset+uri_len]);
                                            writeln!(f, "URI: {}", uri)?;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                
                // 获取账户标签
                let account_labels = get_boop_account_labels_by_instruction("CreateToken");
                
                // 打印账户信息
                for (i, account) in self.accounts.iter().enumerate() {
                    let label = account_labels.get(&i).unwrap_or(&"Unknown");
                    writeln!(f, "[{}]{}: {}", i, label, account)?;
                }
            },
            BoopInstructionType::DeployBondingCurve => {
                // 解析DeployBondingCurve指令的参数数据
                if self.data.len() >= 16 { // 鉴别器(8字节) + 至少8字节数据
                    // 跳过前8字节的鉴别器
                    let mut offset = 8;
                    
                    // 检查第一个参数是u64还是pubkey
                    // 如果数据长度足够长，并且后面有足够的数据来存储一个u64，我们假设第一个参数是creator
                    if self.data.len() >= 8 + 32 + 8 { // 鉴别器 + creator(32字节) + salt(8字节)
                        // 尝试读取creator (32字节的公钥)
                        let creator = bs58::encode(&self.data[offset..offset+32]).into_string();
                        writeln!(f, "Creator: {}", creator)?;
                        offset += 32;
                        
                        // 读取salt
                        if offset + 8 <= self.data.len() {
                            let salt = u64::from_le_bytes([
                                self.data[offset], self.data[offset+1], self.data[offset+2], self.data[offset+3],
                                self.data[offset+4], self.data[offset+5], self.data[offset+6], self.data[offset+7]
                            ]);
                            writeln!(f, "Salt: {}", salt)?;
                        }
                    } else {
                        // 假设第一个参数是salt (8字节)
                        let salt = u64::from_le_bytes([
                            self.data[offset], self.data[offset+1], self.data[offset+2], self.data[offset+3],
                            self.data[offset+4], self.data[offset+5], self.data[offset+6], self.data[offset+7]
                        ]);
                        writeln!(f, "Salt: {}", salt)?;
                        offset += 8;
                        
                        // 如果还有更多数据，尝试读取creator
                        if offset + 32 <= self.data.len() {
                            let creator = bs58::encode(&self.data[offset..offset+32]).into_string();
                            writeln!(f, "Creator: {}", creator)?;
                        }
                    }
                }
                
                // 获取账户标签
                let account_labels = get_boop_account_labels_by_instruction("DeployBondingCurve");
                
                // 打印账户信息
                for (i, account) in self.accounts.iter().enumerate() {
                    let label = account_labels.get(&i).unwrap_or(&"Unknown");
                    writeln!(f, "[{}]{}: {}", i, label, account)?;
                }
            },
            BoopInstructionType::Create => {
                // Create指令的字符串参数在data[8..]之后
                if self.data.len() > 8 {
                    // 前8字节是discriminator，后面是参数数据
                    let mut offset = 8;
                    
                    // 首先解析 salt (u64, 8字节)
                    if offset + 8 <= self.data.len() {
                        let salt = u64::from_le_bytes([
                            self.data[offset], self.data[offset+1], self.data[offset+2], self.data[offset+3],
                            self.data[offset+4], self.data[offset+5], self.data[offset+6], self.data[offset+7]
                        ]);
                        writeln!(f, "Salt: {}", salt)?;
                        offset += 8;
                    }
                    
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
                            writeln!(f, "Name: {}", name)?;
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
                                    writeln!(f, "Symbol: {}", symbol)?;
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
                                            writeln!(f, "URI: {}", uri)?;
                                            offset += uri_len;
                                            
                                            // 解析creator字段 (Pubkey是32字节)
                                            if offset + 32 <= self.data.len() {
                                                let creator = bs58::encode(&self.data[offset..offset+32]).into_string();
                                                writeln!(f, "Creator: {}", creator)?;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                
                // 获取账户标签
                let account_labels = get_boop_account_labels_by_instruction("Create");
                
                // 打印账户信息
                for (i, account) in self.accounts.iter().enumerate() {
                    let label = account_labels.get(&i).unwrap_or(&"Unknown");
                    writeln!(f, "[{}]{}: {}", i, label, account)?;
                }
            },
            BoopInstructionType::Sell => {
                if self.data.len() >= 16 {
                    // 解析amount参数（跳过前8字节的鉴别器）
                    let amount = u64::from_le_bytes([
                        self.data[8], self.data[9], self.data[10], self.data[11],
                        self.data[12], self.data[13], self.data[14], self.data[15]
                    ]);
                    
                    writeln!(f, "Token_Amount: {}", amount)?;
                    
                    // 解析min_sol_out参数（如果存在）
                    if self.data.len() >= 24 {
                        let min_sol_out = u64::from_le_bytes([
                            self.data[16], self.data[17], self.data[18], self.data[19],
                            self.data[20], self.data[21], self.data[22], self.data[23]
                        ]);
                        writeln!(f, "Min_SOL_Output: {} ", min_sol_out)?;
                    }
                }
                
                // 获取账户标签
                let account_labels = get_boop_account_labels_by_instruction("Sell");
                
                // 打印账户信息
                for (i, account) in self.accounts.iter().enumerate() {
                    let label = account_labels.get(&i).unwrap_or(&"Unknown");
                    writeln!(f, "[{}]{}: {}", i, label, account)?;
                }
            },
            BoopInstructionType::Initialize => {
                // 获取账户标签
                let account_labels = get_boop_account_labels_by_instruction("Initialize");
                
                // 打印账户信息
                for (i, account) in self.accounts.iter().enumerate() {
                    let label = account_labels.get(&i).unwrap_or(&"Unknown");
                    writeln!(f, "[{}]{}: {}", i, label, account)?;
                }
            },
            BoopInstructionType::SetParams => {
                // 获取账户标签
                let account_labels = get_boop_account_labels_by_instruction("SetParams");
                
                // 打印账户信息
                for (i, account) in self.accounts.iter().enumerate() {
                    let label = account_labels.get(&i).unwrap_or(&"Unknown");
                    writeln!(f, "[{}]{}: {}", i, label, account)?;
                }
            },
            BoopInstructionType::UpdateAuthority => {
                // 获取账户标签
                let account_labels = get_boop_account_labels_by_instruction("UpdateAuthority");
                
                // 打印账户信息
                for (i, account) in self.accounts.iter().enumerate() {
                    let label = account_labels.get(&i).unwrap_or(&"Unknown");
                    writeln!(f, "[{}]{}: {}", i, label, account)?;
                }
            },
            BoopInstructionType::Unknown => {
                writeln!(f, "未知指令类型，数据长度: {}", self.data.len())?;
                if !self.data.is_empty() {
                    writeln!(f, "前8字节: {:?}", &self.data[0..std::cmp::min(8, self.data.len())])?;
                }
                
                // 尝试解析一些常见的参数格式
                if self.data.len() >= 8 + 8 { // 鉴别器 + u64参数
                    let value = u64::from_le_bytes([
                        self.data[8], self.data[9], self.data[10], self.data[11],
                        self.data[12], self.data[13], self.data[14], self.data[15]
                    ]);
                    writeln!(f, "可能的u64参数: {}", value)?;
                }

                // 打印所有账户
                for (i, account) in self.accounts.iter().enumerate() {
                    writeln!(f, "[{}]账户: {}", i, account)?;
                }
            }
        }
        
        Ok(())
    }
}

// BOOP交易定义
#[derive(Debug)]
pub struct BoopTransaction {
    pub signature: String,
    pub instructions: Vec<BoopInstruction>,
}

impl fmt::Display for BoopTransaction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Instruction_Count: {}", self.instructions.len())?;
        
        for (i, instruction) in self.instructions.iter().enumerate() {
            writeln!(f, "Instruction #{}: {:?}", i + 1, instruction.instruction_type)?;
            write!(f, "{}", instruction)?;
        }
        
        Ok(())
    }
}

pub struct BoopParser;

impl BoopParser {
    pub fn parse_transaction(transaction: &VersionedTransaction) -> Option<BoopTransaction> {
        let boop_program_id = Pubkey::from_str(BOOP_PROGRAM_ID).unwrap();
        
        let (instructions, signature) = match &transaction.message {
            VersionedMessage::Legacy(message) => {
                let instructions = Self::extract_boop_instructions_from_legacy(message, &boop_program_id);
                
                if instructions.is_empty() {
                    return None;
                }
                
                let signature = bs58::encode(&transaction.signatures[0]).into_string();
                (instructions, signature)
            }
            VersionedMessage::V0(message) => {
                let instructions = Self::extract_boop_instructions_from_v0(message, &boop_program_id);
                
                if instructions.is_empty() {
                    return None;
                }
                
                let signature = bs58::encode(&transaction.signatures[0]).into_string();
                (instructions, signature)
            }
        };
        
        Some(BoopTransaction {
            signature,
            instructions,
        })
    }
    
    fn extract_boop_instructions_from_legacy(
        message: &LegacyMessage,
        boop_program_id: &Pubkey,
    ) -> Vec<BoopInstruction> {
        let account_keys = &message.account_keys;
        let mut boop_instructions = Vec::new();
        
        for ix in &message.instructions {
            if account_keys.get(ix.program_id_index as usize) == Some(boop_program_id) {
                let boop_instruction = Self::compile_instruction_to_boop_instruction(ix, account_keys);
                boop_instructions.push(boop_instruction);
            }
        }
        
        boop_instructions
    }
    
    fn extract_boop_instructions_from_v0(
        message: &V0Message,
        boop_program_id: &Pubkey,
    ) -> Vec<BoopInstruction> {
        let account_keys: Vec<Pubkey> = message
            .account_keys
            .iter()
            .map(|key| key.clone())
            .collect();
            
        let mut boop_instructions = Vec::new();
        
        for ix in &message.instructions {
            if ix.program_id_index as usize >= account_keys.len() {
                continue;
            }
            
            if account_keys.get(ix.program_id_index as usize) == Some(boop_program_id) {
                let boop_instruction = Self::compile_instruction_to_boop_instruction(ix, &account_keys);
                boop_instructions.push(boop_instruction);
            }
        }
        
        boop_instructions
    }
    
    fn compile_instruction_to_boop_instruction(
        ix: &CompiledInstruction,
        account_keys: &[Pubkey],
    ) -> BoopInstruction {
        // 转换账户索引到账户地址的字符串表示
        let accounts: Vec<String> = ix
            .accounts
            .iter()
            .filter_map(|&idx| {
                account_keys.get(idx as usize).map(|key| key.to_string())
            })
            .collect();
            
        // 确定指令类型
        let instruction_type = if ix.data.len() >= 8 {
            // 检查指令数据的前8个字节来确定指令类型
            match &ix.data[0..8] {
                // 根据IDL定义的指令鉴别器
                [138, 127, 14, 91, 38, 87, 115, 105] => BoopInstructionType::BuyToken,  // buy_token指令
                [109, 61, 40, 187, 230, 176, 135, 174] => BoopInstructionType::SellToken, // sell_token指令
                [253, 184, 126, 199, 235, 232, 172, 162] => BoopInstructionType::CreateToken, // create_token指令
                [53, 230, 172, 84, 77, 174, 22, 61] => BoopInstructionType::DeployBondingCurve, // deploy_bonding_curve指令
                [84, 52, 204, 228, 24, 140, 234, 75] => BoopInstructionType::CreateToken, // 新发现的create_token指令鉴别器
                [180, 89, 199, 76, 168, 236, 217, 138] => BoopInstructionType::DeployBondingCurve, // 新发现的deploy_bonding_curve指令鉴别器
                
                // 原始指令的鉴别器
                [191, 19, 103, 26, 245, 85, 112, 105] => BoopInstructionType::Create, // create指令
                [25, 169, 76, 76, 84, 153, 195, 216] => BoopInstructionType::Sell,    // sell指令
                [175, 175, 109, 31, 13, 152, 155, 237] => BoopInstructionType::Initialize, // initialize指令
                [235, 129, 153, 118, 219, 194, 131, 246] => BoopInstructionType::SetParams, // setParams指令
                [167, 17, 172, 137, 241, 116, 201, 161] => BoopInstructionType::UpdateAuthority, // updateAuthority指令
                
                _ => {
                    println!("未知鉴别器: {:?}", &ix.data[0..8]);
                    BoopInstructionType::Unknown
                },
            }
        } else {
            BoopInstructionType::Unknown
        };
        
        BoopInstruction {
            instruction_type,
            accounts,
            data: ix.data.clone(),
        }
    }
}