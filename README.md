# Shreds Client

## 项目介绍

Shreds Client是一个用Rust开发的Solana交易监控工具，专注于解析和监控PUMP和PUMP_AMM智能合约的交易。该工具连接到Jito网络的数据流，实时捕获交易并进行解析，提供交易的详细信息和统计数据。

## 主要功能

- 实时监控Solana区块链上的交易
- 解析PUMP和PUMP_AMM智能合约的交易指令
- 详细显示交易的参数和账户信息
- 使用多线程并行处理以提高性能
- 结构化的日志输出，便于数据分析和监控

## 支持的交易类型

### PUMP交易指令类型

- **Buy** - 从绑定曲线购买代币
- **Sell** - 向绑定曲线卖出代币
- **Create** - 创建新币种和绑定曲线
- **Initialize** - 初始化全局状态
- **SetParams** - 设置全局参数
- **Withdraw** - 如果绑定曲线完成，允许管理员提取流动性用于迁移
- **Migrate** - 如果绑定曲线完成，则将流动性迁移到pump_amm
- **ExtendAccount** - 扩展程序拥有的账户
- **UpdateGlobalAuthority** - 更新全局权限

### PUMP_AMM交易指令类型

- **Buy** - 从流动性池买入代币
- **Sell** - 向流动性池卖出代币
- **CreateConfig** - 创建全局配置
- **CreatePool** - 创建新的流动性池
- **Deposit** - 向流动性池存入代币
- **Withdraw** - 从流动性池取出代币
- **Disable** - 禁用某些功能
- **ExtendAccount** - 扩展账户
- **UpdateAdmin** - 更新管理员
- **UpdateFeeConfig** - 更新费用配置

## 技术栈

- Rust 1.85+
- Tokio异步运行时
- Solana SDK
- Jito Shredstream API
- Rayon并行计算库

## 安装与使用

### 前置条件

- 安装Rust和Cargo（1.85+版本）
- 获取Jito Shredstream API的访问地址

### 安装

```bash
# 克隆仓库
git clone https://github.com/vnxfsc/shreds-client.git
cd shreds-client

# 编译项目
cargo build --release
```

### 配置

创建一个`env.toml`文件在项目根目录下，内容如下：

```toml
# Jito Shredstream API地址
JITO-SHRED-URL = "http://127.0.0.1:9999"
```

### 运行

```bash
cargo run --release
```

## 输出示例

```
--------------------------------------------------------
Parser:PUMP
Slot:336893571
Signature:2idv2zaXWJjesRPQzGeGRGoUuZVwkYkZaUU3khc86tWLY96AG4wRDxZ1yLrux6RE7q1igBJFpAgVdjKCfDVojWJh
Instructions_Count: 1
Instruction[0]Type: Sell
Token_Amount: 11813479435461
Min_SOL_Output: 121299504 
[0]Global: 4wTV1YmiEkRvAtNtsSGPtUrqRYQMe5SKy2uB4Jjaxnjf
[1]Fee_Recipient: G5UZAVbAf46s7cKWoyKu8kYTip9DGTpbLZ2qa9Aq69dP
[2]Mint: C8aJbg6eHEvvoZsZyfWZVwPiYRKAqjNbopagk7gGpump
[3]Bonding_Curve: 8nygcweT7bBtmHTwDMBQe2tSiszx1bVujiTzfVRHwRo5
[4]Associated_Bonding_Curve: 6T81AcGzpPW3FjgUjB4pVSdtqgio47TSZZmeWKgzfDgM
[5]Associated_User: 284xL36A3NgMB87ir1mx1NVrJmysp4Pi4FPs1b6Z8BDV
[6]User: GiTjPLMYngDKZYxyc2MBFBwsS1Zt3NBCjJtAgsGb3AJv
[7]System_Program: 11111111111111111111111111111111
[8]Token_Program: ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL
[9]Rent: TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA
[10]Event_Authority: Ce6TQqeHC9p8KetsN6JsjHK7UTZk7nasjjnr7XxXp9F1
[11]Program: 6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P
--------------------------------------------------------
```

## 性能优化

- 批处理交易以减少锁争用
- 使用多线程并行处理交易
- 对容器进行预分配以减少内存重分配
- 使用本地结果收集机制提高并行效率

## 程序ID配置

在`src/config/mod.rs`文件中配置要监控的程序ID：

```rust
// 定义要查找的程序ID (Base58格式)
pub const PUMP_PROGRAM_ID: &str = "6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P";
pub const PUMPAMM_PROGRAM_ID: &str = "pAMMBay6oceH9fJKBRHGP5D4bD4sWpmSwMn52FMfXEA";
```

## 许可证

[MIT License](LICENSE) 