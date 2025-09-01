// Copyright (C) 2021-2025 the DTVM authors. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! Contract interaction host functions

use crate::core::instance::ZenInstance;
use crate::evm::error::HostFunctionResult;
use crate::evm::traits::EvmHost;
use crate::evm::utils::{
    validate_address_param, validate_bytes32_param, validate_data_param, MemoryAccessor,
};
use crate::{host_error, host_info};

/// Call another contract (CALL opcode)
/// Performs a call to another contract with the specified parameters
///
/// This function uses the ContractCallProvider trait to execute the call,
/// allowing users to implement custom contract execution logic.
///
/// Parameters:
/// - instance: WASM instance pointer
/// - gas: Gas limit for the call
/// - addr_offset: Memory offset of the 20-byte target contract address
/// - value_offset: Memory offset of the 32-byte value to send
/// - data_offset: Memory offset of the call data
/// - data_length: Length of the call data
///
/// Returns:
/// - 1 if the call succeeded, 0 if it failed
pub fn call_contract<T>(
    instance: &ZenInstance<T>,
    gas: i64,
    addr_offset: i32,
    value_offset: i32,
    data_offset: i32,
    data_length: i32,
) -> HostFunctionResult<i32>
where
    T: EvmHost,
{
    host_info!(
        "call_contract called: gas={}, addr_offset={}, value_offset={}, data_offset={}, data_length={}",
        gas,
        addr_offset,
        value_offset,
        data_offset,
        data_length
    );

    let evmhost = &instance.extra_ctx;
    let memory = MemoryAccessor::new(instance);

    // Validate parameters
    let addr_offset_u32 = validate_address_param(instance, addr_offset)?;
    let value_offset_u32 = validate_bytes32_param(instance, value_offset)?;
    let (data_offset_u32, data_length_u32) =
        validate_data_param(instance, data_offset, data_length)?;

    // Read the target address
    let target_address = memory.read_address(addr_offset_u32).map_err(|e| {
        host_error!(
            "Failed to read target address at offset {}: {}",
            addr_offset,
            e
        );
        e
    })?;

    // Read the value to send
    let call_value = memory.read_bytes32(value_offset_u32).map_err(|e| {
        host_error!(
            "Failed to read call value at offset {}: {}",
            value_offset,
            e
        );
        e
    })?;

    // Read the call data
    let call_data = memory
        .read_bytes_vec(data_offset_u32, data_length_u32)
        .map_err(|e| {
            host_error!(
                "Failed to read call data at offset {} length {}: {}",
                data_offset,
                data_length,
                e
            );
            e
        })?;

    // Get the caller address from evmhost
    let caller_address = evmhost.get_caller();

    host_info!(
        "    📞 Calling contract: target=0x{}, caller=0x{}, value=0x{}, data_len={}",
        hex::encode(&target_address),
        hex::encode(&caller_address),
        hex::encode(&call_value),
        call_data.len()
    );

    // Execute the contract call using the provider
    let result = evmhost.call_contract(
        &target_address,
        &caller_address,
        &call_value,
        &call_data,
        gas,
    );

    let success_code = if result.success { 1 } else { 0 };
    host_info!(
        "call_contract completed: success={}, return_data_len={}, gas_used={}",
        result.success,
        result.return_data.len(),
        result.gas_used
    );

    Ok(success_code)
}

/// Call another contract with current contract's code (CALLCODE opcode)
/// Similar to call_contract but uses the current contract's code
///
/// This function uses the ContractCallProvider trait to execute the call,
/// allowing users to implement custom contract execution logic.
///
/// Parameters:
/// - instance: WASM instance pointer
/// - gas: Gas limit for the call
/// - addr_offset: Memory offset of the 20-byte target contract address
/// - value_offset: Memory offset of the 32-byte value to send
/// - data_offset: Memory offset of the call data
/// - data_length: Length of the call data
///
/// Returns:
/// - 1 if the call succeeded, 0 if it failed
pub fn call_code<T>(
    instance: &ZenInstance<T>,
    gas: i64,
    addr_offset: i32,
    value_offset: i32,
    data_offset: i32,
    data_length: i32,
) -> HostFunctionResult<i32>
where
    T: EvmHost,
{
    host_info!(
        "call_code called: gas={}, addr_offset={}, value_offset={}, data_offset={}, data_length={}",
        gas,
        addr_offset,
        value_offset,
        data_offset,
        data_length
    );

    let evmhost = &instance.extra_ctx;
    let memory = MemoryAccessor::new(instance);

    // Validate parameters (same as call_contract)
    let addr_offset_u32 = validate_address_param(instance, addr_offset)?;
    let value_offset_u32 = validate_bytes32_param(instance, value_offset)?;
    let (data_offset_u32, data_length_u32) =
        validate_data_param(instance, data_offset, data_length)?;

    // Read parameters
    let target_address = memory.read_address(addr_offset_u32).map_err(|e| {
        host_error!(
            "Failed to read target address at offset {}: {}",
            addr_offset,
            e
        );
        e
    })?;

    let call_value = memory.read_bytes32(value_offset_u32).map_err(|e| {
        host_error!(
            "Failed to read call value at offset {}: {}",
            value_offset,
            e
        );
        e
    })?;

    let call_data = memory
        .read_bytes_vec(data_offset_u32, data_length_u32)
        .map_err(|e| {
            host_error!(
                "Failed to read call data at offset {} length {}: {}",
                data_offset,
                data_length,
                e
            );
            e
        })?;

    // Get the caller address from evmhost
    let caller_address = evmhost.get_caller();

    host_info!(
        "    📞 Call code: target=0x{}, caller=0x{}, value=0x{}, data_len={}",
        hex::encode(&target_address),
        hex::encode(&caller_address),
        hex::encode(&call_value),
        call_data.len()
    );

    // Execute the call code using the provider
    let result = evmhost.call_code(
        &target_address,
        &caller_address,
        &call_value,
        &call_data,
        gas,
    );

    let success_code = if result.success { 1 } else { 0 };
    host_info!(
        "call_code completed: success={}, return_data_len={}, gas_used={}",
        result.success,
        result.return_data.len(),
        result.gas_used
    );

    Ok(success_code)
}

/// Delegate call to another contract (DELEGATECALL opcode)
/// Calls another contract but preserves the current contract's evmhost
///
/// This function uses the ContractCallProvider trait to execute the call,
/// allowing users to implement custom contract execution logic.
///
/// Parameters:
/// - instance: WASM instance pointer
/// - gas: Gas limit for the call
/// - addr_offset: Memory offset of the 20-byte target contract address
/// - data_offset: Memory offset of the call data
/// - data_length: Length of the call data
///
/// Returns:
/// - 1 if the call succeeded, 0 if it failed
pub fn call_delegate<T>(
    instance: &ZenInstance<T>,
    gas: i64,
    addr_offset: i32,
    data_offset: i32,
    data_length: i32,
) -> HostFunctionResult<i32>
where
    T: EvmHost,
{
    host_info!(
        "call_delegate called: gas={}, addr_offset={}, data_offset={}, data_length={}",
        gas,
        addr_offset,
        data_offset,
        data_length
    );

    let evmhost = &instance.extra_ctx;
    let memory = MemoryAccessor::new(instance);

    // Validate parameters
    let addr_offset_u32 = validate_address_param(instance, addr_offset)?;
    let (data_offset_u32, data_length_u32) =
        validate_data_param(instance, data_offset, data_length)?;

    // Read parameters
    let target_address = memory.read_address(addr_offset_u32).map_err(|e| {
        host_error!(
            "Failed to read target address at offset {}: {}",
            addr_offset,
            e
        );
        e
    })?;

    let call_data = memory
        .read_bytes_vec(data_offset_u32, data_length_u32)
        .map_err(|e| {
            host_error!(
                "Failed to read call data at offset {} length {}: {}",
                data_offset,
                data_length,
                e
            );
            e
        })?;

    // Get the caller address from evmhost (for delegate call, this preserves the original caller)
    let caller_address = evmhost.get_caller();

    host_info!(
        "    📞 Delegate call: target=0x{}, caller=0x{}, data_len={}",
        hex::encode(&target_address),
        hex::encode(&caller_address),
        call_data.len()
    );

    // Execute the delegate call using the provider
    let result = evmhost.call_delegate(&target_address, &caller_address, &call_data, gas);

    let success_code = if result.success { 1 } else { 0 };
    host_info!(
        "call_delegate completed: success={}, return_data_len={}, gas_used={}",
        result.success,
        result.return_data.len(),
        result.gas_used
    );

    Ok(success_code)
}

/// Static call to another contract (STATICCALL opcode)
/// Calls another contract without allowing state modifications
///
/// This function uses the ContractCallProvider trait to execute the call,
/// allowing users to implement custom contract execution logic.
///
/// Parameters:
/// - instance: WASM instance pointer
/// - gas: Gas limit for the call
/// - addr_offset: Memory offset of the 20-byte target contract address
/// - data_offset: Memory offset of the call data
/// - data_length: Length of the call data
///
/// Returns:
/// - 1 if the call succeeded, 0 if it failed
pub fn call_static<T>(
    instance: &ZenInstance<T>,
    gas: i64,
    addr_offset: i32,
    data_offset: i32,
    data_length: i32,
) -> HostFunctionResult<i32>
where
    T: EvmHost,
{
    host_info!(
        "call_static called: gas={}, addr_offset={}, data_offset={}, data_length={}",
        gas,
        addr_offset,
        data_offset,
        data_length
    );

    let evmhost = &instance.extra_ctx;
    let memory = MemoryAccessor::new(instance);

    // Validate parameters
    let addr_offset_u32 = validate_address_param(instance, addr_offset)?;
    let (data_offset_u32, data_length_u32) =
        validate_data_param(instance, data_offset, data_length)?;

    // Read parameters
    let target_address = memory.read_address(addr_offset_u32).map_err(|e| {
        host_error!(
            "Failed to read target address at offset {}: {}",
            addr_offset,
            e
        );
        e
    })?;

    let call_data = memory
        .read_bytes_vec(data_offset_u32, data_length_u32)
        .map_err(|e| {
            host_error!(
                "Failed to read call data at offset {} length {}: {}",
                data_offset,
                data_length,
                e
            );
            e
        })?;

    // Get the caller address from evmhost
    let caller_address = evmhost.get_caller();

    host_info!(
        "    📞 Static call: target=0x{}, caller=0x{}, data_len={}",
        hex::encode(&target_address),
        hex::encode(&caller_address),
        call_data.len()
    );

    // Execute the static call using the provider
    let result = evmhost.call_static(&target_address, &caller_address, &call_data, gas);

    let success_code = if result.success { 1 } else { 0 };
    host_info!(
        "call_static completed: success={}, return_data_len={}, gas_used={}",
        result.success,
        result.return_data.len(),
        result.gas_used
    );

    Ok(success_code)
}

/// Create a new contract (CREATE opcode)
/// Creates a new contract with the specified code and constructor data
///
/// This function uses the ContractCallProvider trait to execute the creation,
/// allowing users to implement custom contract creation logic.
///
/// Parameters:
/// - instance: WASM instance pointer
/// - value_offset: Memory offset of the 32-byte value to send to constructor
/// - code_offset: Memory offset of the contract creation code
/// - code_length: Length of the creation code
/// - data_offset: Memory offset of the constructor data
/// - data_length: Length of the constructor data
/// - result_offset: Memory offset where the 20-byte new contract address should be written
///
/// Returns:
/// - 1 if contract creation succeeded, 0 if it failed
pub fn create_contract<T>(
    instance: &ZenInstance<T>,
    value_offset: i32,
    code_offset: i32,
    code_length: i32,
    data_offset: i32,
    data_length: i32,
    salt_offset: i32,
    is_create2: i32,
    result_offset: i32,
) -> HostFunctionResult<i32>
where
    T: EvmHost,
{
    host_info!(
        "create_contract called: value_offset={}, code_offset={}, code_length={}, data_offset={}, data_length={}, salt_offset={}, is_create2={}, result_offset={}",
        value_offset,
        code_offset,
        code_length,
        data_offset,
        data_length,
        salt_offset,
        is_create2,
        result_offset
    );

    let evmhost = &instance.extra_ctx;
    let memory = MemoryAccessor::new(instance);

    // Validate parameters
    let value_offset_u32 = validate_bytes32_param(instance, value_offset)?;
    let (code_offset_u32, code_length_u32) =
        validate_data_param(instance, code_offset, code_length)?;
    let (data_offset_u32, data_length_u32) =
        validate_data_param(instance, data_offset, data_length)?;
    let salt_offset_u32 = if is_create2 != 0 {
        Some(validate_bytes32_param(instance, salt_offset)?)
    } else {
        None
    };
    let result_offset_u32 = validate_address_param(instance, result_offset)?;

    // Read parameters
    let value = memory.read_bytes32(value_offset_u32).map_err(|e| {
        host_error!("Failed to read value at offset {}: {}", value_offset, e);
        e
    })?;

    let creation_code = memory
        .read_bytes_vec(code_offset_u32, code_length_u32)
        .map_err(|e| {
            host_error!(
                "Failed to read creation code at offset {} length {}: {}",
                code_offset,
                code_length,
                e
            );
            e
        })?;

    let constructor_data = memory
        .read_bytes_vec(data_offset_u32, data_length_u32)
        .map_err(|e| {
            host_error!(
                "Failed to read constructor data at offset {} length {}: {}",
                data_offset,
                data_length,
                e
            );
            e
        })?;

    // Read salt if this is CREATE2 (for future use)
    let _salt = if let Some(salt_offset_u32) = salt_offset_u32 {
        Some(memory.read_bytes32(salt_offset_u32).map_err(|e| {
            host_error!("Failed to read salt at offset {}: {}", salt_offset, e);
            e
        })?)
    } else {
        None
    };

    // Get the creator address from evmhost
    let creator_address = evmhost.get_address();

    // Execute the contract creation using the provider
    // Note: All logic is implemented in Mockevmhost::create_contract
    let gas = 1000000; // Default gas for creation
    let is_create2_bool = is_create2 != 0;
    let result = evmhost.create_contract(
        &creator_address,
        &value,
        &creation_code,
        &constructor_data,
        gas,
        _salt,
        is_create2_bool,
    );

    // Write the contract address to memory (or zero address if failed)
    let address_to_write = result.contract_address.unwrap_or([0u8; 20]);
    memory
        .write_address(result_offset_u32, &address_to_write)
        .map_err(|e| {
            host_error!(
                "Failed to write contract address at offset {}: {}",
                result_offset,
                e
            );
            e
        })?;

    let success_code = if result.success { 1 } else { 0 };
    host_info!(
        "create_contract completed: success={}, address=0x{}, return_data_len={}, gas_used={}",
        result.success,
        hex::encode(&address_to_write),
        result.return_data.len(),
        result.gas_used
    );

    Ok(success_code)
}
