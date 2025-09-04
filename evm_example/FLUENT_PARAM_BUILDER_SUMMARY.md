# 🔧 生成式 ParamBuilder：链式构建参数

## ✅ 新的生成式设计

基于你的建议，我们重新设计了 ParamBuilder，采用生成式（fluent）API，让用户可以链式调用来构建复杂的参数列表。

## 🏗️ 新的 ParamBuilder 设计

### 核心结构
```rust
#[derive(Debug, Clone, Default)]
pub struct ParamBuilder {
    tokens: Vec<Token>,
}
```

### 链式方法
```rust
impl ParamBuilder {
    pub fn new() -> Self
    pub fn uint256(self, value: u64) -> Self
    pub fn address(self, address: &[u8; 20]) -> Self
    pub fn bytes(self, data: &[u8]) -> Self
    pub fn fixed_bytes(self, data: &[u8]) -> Self
    pub fn string(self, s: &str) -> Self
    pub fn bool(self, value: bool) -> Self
    pub fn int256(self, value: i64) -> Self
    pub fn uint256_array(self, values: &[u64]) -> Self
    pub fn address_array(self, addresses: &[[u8; 20]]) -> Self
    pub fn custom_token(self, token: Token) -> Self
    pub fn build(self) -> Vec<Token>  // 最终构建
}
```

## 📊 使用示例

### 简单参数：
```rust
// 无参数
let params = ParamBuilder::new().build();

// 单个 uint256
let params = ParamBuilder::new().uint256(123).build();

// 单个地址
let params = ParamBuilder::new().address(&address).build();
```

### 复合参数（链式调用）：
```rust
// 地址 + uint256
let params = ParamBuilder::new()
    .address(&to_address)
    .uint256(amount)
    .build();

// 地址 + 两个 uint256
let params = ParamBuilder::new()
    .address(&contract_address)
    .uint256(value1)
    .uint256(value2)
    .build();

// 三个 uint256
let params = ParamBuilder::new()
    .uint256(a)
    .uint256(b)
    .uint256(c)
    .build();
```

### 复杂参数：
```rust
// 混合类型参数
let params = ParamBuilder::new()
    .address(&contract_address)
    .uint256(amount)
    .bytes(&data)
    .bool(true)
    .string("hello")
    .build();

// 数组参数
let params = ParamBuilder::new()
    .address(&owner)
    .uint256_array(&[100, 200, 300])
    .address_array(&[addr1, addr2, addr3])
    .build();

// 自定义 Token
let params = ParamBuilder::new()
    .address(&contract)
    .custom_token(Token::Tuple(vec![
        Token::Uint(123.into()),
        Token::String("test".to_string()),
    ]))
    .bytes(&call_data)
    .build();
```

## 🎯 实际使用场景

### ERC20 Transfer：
```rust
// transfer(address to, uint256 amount)
let params = ParamBuilder::new()
    .address(&to_address)
    .uint256(amount)
    .build();

set_call_data_with_params(context, &TRANSFER_SELECTOR, params);
```

### 复杂合约调用：
```rust
// complexFunction(address owner, uint256[] amounts, bytes data, bool flag)
let params = ParamBuilder::new()
    .address(&owner_address)
    .uint256_array(&[100, 200, 300])
    .bytes(&call_data)
    .bool(true)
    .build();

set_call_data_with_params(context, &COMPLEX_SELECTOR, params);
```

### Uniswap 风格的调用：
```rust
// swapExactTokensForTokens(uint amountIn, uint amountOutMin, address[] path, address to, uint deadline)
let params = ParamBuilder::new()
    .uint256(amount_in)
    .uint256(amount_out_min)
    .address_array(&[token_a, token_b])
    .address(&recipient)
    .uint256(deadline)
    .build();

set_call_data_with_params(context, &SWAP_SELECTOR, params);
```

## 🔄 与便利函数的结合

便利函数现在内部使用 ParamBuilder：

```rust
/// 便利函数内部实现
pub fn params_address_and_uint256(address: &[u8; 20], amount: u64) -> Vec<Token> {
    ParamBuilder::new().address(address).uint256(amount).build()
}

/// 用户可以选择使用便利函数
let params = params_address_and_uint256(&address, 100);

/// 或者直接使用 ParamBuilder
let params = ParamBuilder::new().address(&address).uint256(100).build();
```

## 🎯 设计优势

### 1. **灵活性**
- 用户可以按任意顺序添加任意类型的参数
- 支持所有 ABI 类型：uint256, address, bytes, string, bool, arrays, tuples

### 2. **可读性**
- 链式调用清晰表达参数构建过程
- 方法名直观反映参数类型

### 3. **类型安全**
- 每个方法都有明确的类型约束
- 编译时检查参数类型

### 4. **渐进复杂度**
- 简单场景：`ParamBuilder::new().uint256(123).build()`
- 复杂场景：链式调用多个不同类型的参数
- 超复杂场景：使用 `custom_token()` 添加任意 Token

### 5. **一致性**
- 所有参数构建都使用相同的模式
- 与现有 API 完全兼容

## 📈 完整的使用流程

```rust
// 1. 创建 ParamBuilder
let params = ParamBuilder::new()
    // 2. 链式添加参数
    .address(&contract_address)
    .uint256(amount)
    .bytes(&data)
    // 3. 构建最终的 Vec<Token>
    .build();

// 4. 传递给统一函数
set_call_data_with_params(context, &selector, params);
```

## 📝 总结

新的生成式 ParamBuilder 实现了：

1. **完全的灵活性**：用户可以构建任意复杂的参数组合
2. **优雅的 API**：链式调用，代码可读性强
3. **类型安全**：每个参数都有明确的类型
4. **易于扩展**：添加新的参数类型只需要添加新方法
5. **向后兼容**：与现有代码完全兼容

这个设计完美地满足了你的需求：**每种类型单独占用一个位置，用户可以自主选择添加的个数与类型，最后 build 输出 Vec<Token>！**

✅ **所有测试通过，API 优雅灵活！**