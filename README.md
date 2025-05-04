# Shreds Client

## 项目介绍

Shreds Client是一个用Rust开发的Solana交易监控工具，专注于解析和监控PUMP、PUMP_AMM和BOOP智能合约的交易。该工具连接到Jito网络的数据流，实时捕获交易并进行解析，提供交易的详细信息和统计数据。

## 打赏作者
 感谢您的打赏,让开源更和谐.
 Solana:BwPzuiSNf6bDDYuosUfCFVJUn4J3g2nMZGvBxwy42mcZ
## 主要功能

- 实时监控Solana区块链上的交易
- 解析PUMP、PUMP_AMM和BOOP智能合约的交易指令
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

### boop交易指令类型

- **BuyToken** - 从绑定曲线购买代币
- **SellToken** - 向绑定曲线卖出代币
- **CreateToken** - 创建新代币
- **DeployBondingCurve** - 部署绑定曲线
- **Create** - 创建新币种（原始指令）
- **Sell** - 卖出代币（原始指令）
- **Initialize** - 初始化全局状态
- **SetParams** - 设置全局参数
- **UpdateAuthority** - 更新权限

## 技术栈

- Rust 1.85+
- Tokio异步运行时
- Solana SDK
- Jito Shredstream API
- Rayon并行计算库

## 项目结构

项目按照功能模块化组织，目录结构如下：

```
src/
├── config/              # 配置相关代码
│   └── mod.rs           # 定义常量和配置读取功能
├── jito_protos/         # Jito网络协议定义
│   ├── build.rs         # 构建脚本，用于编译protobuf文件
│   ├── Cargo.toml       # 子项目依赖配置
│   ├── protos/          # protobuf定义文件
│   └── src/             # 生成的Rust代码
├── models/              # 数据模型和解析器
│   ├── boop_parser.rs   # BOOP交易解析器
│   ├── mod.rs           # 模块导出
│   ├── pump_parser.rs   # PUMP交易解析器
│   ├── pumpamm_parser.rs# PUMP_AMM交易解析器
│   └── transaction.rs   # 交易结果数据结构
├── services/            # 服务层
│   ├── jito_client.rs   # Jito网络客户端
│   ├── mod.rs           # 模块导出
│   └── transaction_processor.rs # 交易处理逻辑
├── utils/               # 工具函数
│   └── mod.rs           # 通用工具和辅助函数
├── lib.rs               # 库入口点，导出公共API
└── main.rs              # 程序入口点
└── keypair.json         # JitoShreds私钥
```

### 主要组件说明

1. **解析器 (Parsers)**
   - `pump_parser.rs`: 专门解析PUMP协议交易，支持Buy、Sell、Create等指令类型
   - `pumpamm_parser.rs`: 解析PUMP_AMM协议交易，包括流动性池交易指令
   - `boop_parser.rs`: 解析BOOP协议交易，支持绑定曲线和代币创建指令

2. **数据模型 (Models)**
   - `transaction.rs`: 定义`TransactionResults`结构，用于存储和管理解析后的交易信息
   - 各类交易结构体: 每个解析器中定义了对应的交易和指令结构体

3. **服务 (Services)**
   - `jito_client.rs`: 负责与Jito Shredstream API通信，接收交易数据
   - `transaction_processor.rs`: 处理接收到的交易，协调解析和结果管理

4. **配置和工具 (Config & Utils)**
   - `config/mod.rs`: 程序配置，包括程序ID和批处理大小
   - `utils/mod.rs`: 提供辅助功能，如账户标签映射和其他工具函数

### 数据流

1. 主程序通过`jito_client`连接到Jito Shredstream API
2. 接收到交易数据后，传递给`transaction_processor`
3. `transaction_processor`使用并行处理将交易分配给对应的解析器
4. 各解析器识别和解析交易指令，生成结构化的交易信息
5. 解析结果收集到`TransactionResults`中并输出 

## 安装与使用

## 服务端安装教程
```bash
git clone https://github.com/jito-labs/shredstream-proxy.git
cd shredstream-proxy
#启动
RUST_LOG=info cargo run --release --bin jito-shredstream-proxy -- shredstream \
    --block-engine-url https://mainnet.block-engine.jito.wtf \ //
    --auth-keypair keypair.json \  //jito_shred 私钥
    --desired-regions amsterdam,ny \  //要接受的区域
    --dest-ip-ports 127.0.0.1:8001,10.0.0.1:8001
    --grpc-service-port 9999
```

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

### PUMP交易示例

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

### PUMPAMM交易示例

```
--------------------------------------------------------
Parser:PUMPAMM
Slot:337165008
Signature:66gYvubBCxaMgaiXxZiVypdU7sgghbQKWyCYcwZuovi32kQrD4aBxoQxbVgwPDJb5mUj1jBwmTSgPZBmPFDCWqPJ
Instructions_Count: 1
Instruction[0]Type: Buy
Base Amount Out: 21771559280
Max Quote Amount In: 63871932
[0]:Pool: EA4xjkrKYy1XmQ9A63p4FS5bvGqPZjuDrSS8x1RNaegx
[1]:User: HZYfBiKmgd9jtCbQHfPz13YVrhHxA8YMqrUi7acQoBn6
[2]:Global_Config: ADyA8hdefvWN2dbGGWFotbzWxrAvLW83WG6QCVXvJKqw
[3]:Base_Mint: 9jUb38tuMqsCe328tM7kxFEm3wRpbo3MbXPJoFiTpump
[4]:Quote_Mint: So11111111111111111111111111111111111111112
[5]:User_Base_Token_Account: FigWgWAXCiVTYZoKCrDD9NGnGyWy2boVxcaKWDwcdXpH
[6]:User_Quote_Token_Account: 3Z3JJeh4qtGAhb28Wk39DGEMuffxRg5y2S3ja8AXHEQR
[7]:Pool_Base_Token_Account: CJKhBiCQcGSGua8uDwskGffrKJ9rNJdKdHBLw8ePZm3B
[8]:Pool_Quote_Token_Account: 9E22AHbxXVUVoPhH6aVZ8Y5XEmuGo7Jvq76ReWToDsAa
[9]:Protocol_Fee_Recipient: 7hTckgnGnLQR6sdH7YkqFTAA7VwTfYFaZ6EhEsU3saCX
[10]:Protocol_Fee_Recipient_Token_Account: X5QPJcpph4mBAJDzc4hRziFftSbcygV59kRb2Fu6Je1
[11]:Base_Token_Program: TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA
[12]:Quote_Token_Program: TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA
[13]:System_Program: 11111111111111111111111111111111
[14]:Associated_Token_Program: ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL
[15]:Event_Authority: GS4CU59F31iL7aR2Q8zVS8DRrcRnXX1yjQ66TqNVQnaR
[16]:Program: pAMMBay6oceH9fJKBRHGP5D4bD4sWpmSwMn52FMfXEA

--------------------------------------------------------
```

### boop交易示例

```
--------------------------------------------------------
Parser:BOOP
Slot:337174693
Signature:PrdmTS47Y4YnBSaNFKmjWoiMbidRoB1cFV69Bmhtw8bB9FHvWGk69qTrtRrFNz4n7fo1QSWxnSe3Ttg8NMZZoUq
Instruction_Count: 3
Instruction #1: CreateToken
Salt: 12396577615618075214
Name: neep
Symbol: neep
URI: https://mainnet-boop.s3.us-east-1.amazonaws.com/LjJAYJ1UqH6gXS9suLErM3FFbs9UsP6yZDqcBDBboop/metadata.json
[0]Mint: AbgFqRWjGWgUaVrZrLLWU5HDY5dktmAL6zT9aacQW7y1
[1]Mint_Authority: AwXCsBCyoeZzPEaG8QcBV2LbLrVZWmBKnYMwnDpKfRiz
[2]Payer: LjJAYJ1UqH6gXS9suLErM3FFbs9UsP6yZDqcBDBboop
[3]Config: 7NipRu6zZxWhLd8pjwsdRHhq3ort2GCuHANaUStKmnip
[4]Rent: SysvarRent111111111111111111111111111111111
[5]Metadata: 11111111111111111111111111111111
[6]System_Program: TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA
[7]Token_Program: metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s
Instruction #2: DeployBondingCurve
Creator: 7NipRu6zZxWhLd8pjwsdRHhq3ort2GCuHANaUStKmnip
Salt: 12396577615618075214
[0]Mint: LjJAYJ1UqH6gXS9suLErM3FFbs9UsP6yZDqcBDBboop
[1]Vault_Authority: GVVUi6DaocSEAp8ATnXFAPNF5irCWjCvmPCzoaGAf5eJ
[2]Bonding_Curve: 5fh5Nu1GBYTqMZ6mGZGg17qngjaCEX4e58o8a1PYQ1oT
[3]Bonding_Curve_Sol_Vault: 8sh55uMzh91ofVGgCQBTkY13p2tjaG2ecbBWQQnepcaF
[4]Bonding_Curve_Vault: 3KgfeLvg21u8EtTp6pGQnqY7PNfqBm6fJjbwKtheapSD
[5]Config: AbgFqRWjGWgUaVrZrLLWU5HDY5dktmAL6zT9aacQW7y1
[6]Payer: 7NipRu6zZxWhLd8pjwsdRHhq3ort2GCuHANaUStKmnip
[7]System_Program: 11111111111111111111111111111111
[8]Token_Program: TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA
[9]Associated_Token_Program: ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL
Instruction #3: BuyToken
Max_SOL_Cost: 616845883 
Token_Amount: 20000000000000000
[0]Mint: LjJAYJ1UqH6gXS9suLErM3FFbs9UsP6yZDqcBDBboop
[1]Bonding Curve: 5fh5Nu1GBYTqMZ6mGZGg17qngjaCEX4e58o8a1PYQ1oT
[2]Trading_Fees_Vault: CJYwvp6gRHzTJmen7u69CJNT9WvVRXbBz6JNtm5cDDfw
[3]Bonding_Curve_Vault: 3KgfeLvg21u8EtTp6pGQnqY7PNfqBm6fJjbwKtheapSD
[4]Bonding_Curve_Sol_Vault: 8sh55uMzh91ofVGgCQBTkY13p2tjaG2ecbBWQQnepcaF
[5]Recipient_Token_Account: BCSaMHPs9hWdtuHxXrwCxDseKGTHyyvvT44fFddGJNJe
[6]Buyer: 7NipRu6zZxWhLd8pjwsdRHhq3ort2GCuHANaUStKmnip
[7]Config: AbgFqRWjGWgUaVrZrLLWU5HDY5dktmAL6zT9aacQW7y1
[8]Vault_Authority: GVVUi6DaocSEAp8ATnXFAPNF5irCWjCvmPCzoaGAf5eJ
[9]Wsol: So11111111111111111111111111111111111111112
[10]System_Program: 11111111111111111111111111111111
[11]Token_Program: TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA
[12]Associated_Token_Program: ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL

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
pub const BOOP_PROGRAM_ID: &str = "boop8hVGQGqehUK2iVEMEnMrL5RbjywRzHKBmBE7ry4";
```

## 许可证

[MIT License](LICENSE) 

