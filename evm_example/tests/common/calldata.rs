// Copyright (C) 2021-2025 the DTVM authors. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

#![allow(dead_code)]

use crate::calculate_selector;
use crate::random_test_address;
use ethabi::{encode, Token};

/// Unified function to set call data with selector and parameters
/// This replaces all the specific set_function_call_data_* functions
pub fn set_call_data_unified(
    context: &mut super::MockContext,
    selector: &[u8; 4],
    params: Vec<Token>,
) {
    let mut call_data = selector.to_vec();
    
    if !params.is_empty() {
        call_data.extend_from_slice(&encode(&params));
    }
    
    context.set_call_data(call_data);
    
    // Debug output
    if params.is_empty() {
        println!(
            "   📋 Set call data with function selector: 0x{}",
            hex::encode(selector)
        );
    } else {
        println!(
            "   📋 Set call data with function selector: 0x{} and {} parameters",
            hex::encode(selector),
            params.len()
        );
    }
}

/// Parameter builder enum for common parameter patterns
#[derive(Debug, Clone)]
pub enum ParamBuilder {
    None,
    Uint256(u64),
    Address([u8; 20]),
    AddressAndUint256([u8; 20], u64),
    AddressAndTwoUint256([u8; 20], u64, u64),
    ThreeUint256(u64, u64, u64),
    Bytes(Vec<u8>),
    Custom(Vec<Token>),
}

impl ParamBuilder {
    /// Convert ParamBuilder to Vec<Token>
    pub fn to_tokens(self) -> Vec<Token> {
        match self {
            ParamBuilder::None => vec![],
            ParamBuilder::Uint256(value) => vec![Token::Uint(value.into())],
            ParamBuilder::Address(address) => vec![Token::Address(address.into())],
            ParamBuilder::AddressAndUint256(address, amount) => vec![
                Token::Address(address.into()),
                Token::Uint(amount.into()),
            ],
            ParamBuilder::AddressAndTwoUint256(address, a, b) => vec![
                Token::Address(address.into()),
                Token::Uint(a.into()),
                Token::Uint(b.into()),
            ],
            ParamBuilder::ThreeUint256(a, b, c) => vec![
                Token::Uint(a.into()),
                Token::Uint(b.into()),
                Token::Uint(c.into()),
            ],
            ParamBuilder::Bytes(data) => vec![Token::Bytes(data)],
            ParamBuilder::Custom(tokens) => tokens,
        }
    }
}

/// Convenience functions to build common parameter patterns
/// Users can use these to easily construct Vec<Token> for common cases

/// Build no parameters
pub fn params_none() -> Vec<Token> {
    vec![]
}

/// Build single uint256 parameter
pub fn params_uint256(value: u64) -> Vec<Token> {
    vec![Token::Uint(value.into())]
}

/// Build single address parameter
pub fn params_address(address: &[u8; 20]) -> Vec<Token> {
    vec![Token::Address((*address).into())]
}

/// Build address + uint256 parameters
pub fn params_address_and_uint256(address: &[u8; 20], amount: u64) -> Vec<Token> {
    vec![
        Token::Address((*address).into()),
        Token::Uint(amount.into()),
    ]
}

/// Build address + two uint256 parameters
pub fn params_address_and_two_uint256(address: &[u8; 20], a: u64, b: u64) -> Vec<Token> {
    vec![
        Token::Address((*address).into()),
        Token::Uint(a.into()),
        Token::Uint(b.into()),
    ]
}

/// Build three uint256 parameters
pub fn params_three_uint256(a: u64, b: u64, c: u64) -> Vec<Token> {
    vec![
        Token::Uint(a.into()),
        Token::Uint(b.into()),
        Token::Uint(c.into()),
    ]
}

/// Build single bytes parameter
pub fn params_bytes(data: &[u8]) -> Vec<Token> {
    vec![Token::Bytes(data.to_vec())]
}

/// Unified function to set call data with selector and tokens
/// This is the main function users should use - pass Vec<Token> directly
pub fn set_call_data_with_params(
    context: &mut super::MockContext,
    selector: &[u8; 4],
    tokens: Vec<Token>,
) {
    set_call_data_unified(context, selector, tokens);
}

/// Legacy function - now uses the unified approach
pub fn set_function_call_data(context: &mut super::MockContext, selector: &[u8; 4]) {
    set_call_data_with_params(context, selector, params_none());
}

/// Convenience function for single uint256 parameter
pub fn set_call_data_with_uint256(
    context: &mut super::MockContext,
    selector: &[u8; 4],
    value: u64,
) {
    set_call_data_with_params(context, selector, params_uint256(value));
}

/// Convenience function for address parameter
pub fn set_call_data_with_address(
    context: &mut super::MockContext,
    selector: &[u8; 4],
    address: &[u8; 20],
) {
    set_call_data_with_params(context, selector, params_address(address));
}

/// Convenience function for address + uint256 parameters
pub fn set_call_data_with_address_and_uint256(
    context: &mut super::MockContext,
    selector: &[u8; 4],
    address: &[u8; 20],
    amount: u64,
) {
    set_call_data_with_params(context, selector, params_address_and_uint256(address, amount));
}

/// Convenience function for address + two uint256 parameters
pub fn set_call_data_with_address_and_two_uint256(
    context: &mut super::MockContext,
    selector: &[u8; 4],
    address: &[u8; 20],
    a: u64,
    b: u64,
) {
    set_call_data_with_params(context, selector, params_address_and_two_uint256(address, a, b));
}

/// Convenience function for three uint256 parameters
pub fn set_call_data_with_three_uint256(
    context: &mut super::MockContext,
    selector: &[u8; 4],
    a: u64,
    b: u64,
    c: u64,
) {
    set_call_data_with_params(context, selector, params_three_uint256(a, b, c));
}

/// Convenience function for bytes parameter
pub fn set_call_data_with_bytes(
    context: &mut super::MockContext,
    selector: &[u8; 4],
    data: &[u8],
) {
    set_call_data_with_params(context, selector, params_bytes(data));
}

/// Legacy function - replaced by unified approach
pub fn set_function_call_data_with_address_and_amount(
    context: &mut super::MockContext,
    selector: &[u8; 4],
    address: &[u8; 20],
    amount: u64,
) {
    set_call_data_with_address_and_uint256(context, selector, address, amount);
}

/// Legacy function - replaced by unified approach
pub fn set_function_call_data_with_address(
    context: &mut super::MockContext,
    selector: &[u8; 4],
    address: &[u8; 20],
) {
    set_call_data_with_address(context, selector, address);
}

/// Legacy function - replaced by unified approach
pub fn set_function_call_data_with_address_and_two_uint256(
    context: &mut super::MockContext,
    selector: &[u8; 4],
    address: &[u8; 20],
    a: u64,
    b: u64,
) {
    set_call_data_with_address_and_two_uint256(context, selector, address, a, b);
}

/// Legacy function - replaced by unified approach
pub fn set_function_call_data_with_three_uint256(
    context: &mut super::MockContext,
    selector: &[u8; 4],
    a: u64,
    b: u64,
    c: u64,
) {
    set_call_data_with_three_uint256(context, selector, a, b, c);
}

/// Legacy function - replaced by unified approach
pub fn set_function_call_data_with_uint256(
    context: &mut super::MockContext,
    selector: &[u8; 4],
    value: u64,
) {
    set_call_data_with_uint256(context, selector, value);
}

/// Legacy function - replaced by unified approach
pub fn set_function_call_data_with_bytes(
    context: &mut super::MockContext,
    selector: &[u8; 4],
    data: &[u8],
) {
    set_call_data_with_bytes(context, selector, data);
}
/// Helper function to set call data for testCall(address target, bytes data)
pub fn set_function_call_data_with_call(context: &mut super::MockContext) {
    // Prepare call data for setValue(100) on the target contract
    let target_call_data = {
        let mut data = calculate_selector("setValue(uint256)").to_vec();
        data.extend_from_slice(&encode(&params_uint256(100)));
        data
    };

    // Direct Vec<Token> for testCall(address, bytes)
    let target_address = random_test_address(20);
    let params = vec![
        Token::Address(target_address.into()),
        Token::Bytes(target_call_data),
    ];
    
    set_call_data_with_params(context, &calculate_selector("testCall(address,bytes)"), params);
}

/// Helper function to set call data for testStaticCall(address target, bytes data)
pub fn set_function_call_data_with_static_call(context: &mut super::MockContext) {
    // Prepare call data for getValue() - no parameters needed
    let target_call_data = calculate_selector("getValue()").to_vec();

    // Direct Vec<Token> for testStaticCall(address, bytes)
    let target_address = random_test_address(20);
    let params = vec![
        Token::Address(target_address.into()),
        Token::Bytes(target_call_data),
    ];
    
    set_call_data_with_params(context, &calculate_selector("testStaticCall(address,bytes)"), params);
}

/// Helper function to set call data for testCreate2(uint256 _value, bytes32 salt)
pub fn set_function_call_data_with_create2(context: &mut super::MockContext) {
    let salt = [
        0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc, 0xde, 0xf0, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77,
        0x88, 0x99, 0xaa, 0xbb, 0xcc, 0xdd, 0xee, 0xff, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06,
        0x07, 0x08,
    ];

    // Direct Vec<Token> for testCreate2(uint256, bytes32)
    let params = vec![
        Token::Uint(456u64.into()),
        Token::FixedBytes(salt.to_vec()),
    ];
    
    set_call_data_with_params(context, &calculate_selector("testCreate2(uint256,bytes32)"), params);
}
