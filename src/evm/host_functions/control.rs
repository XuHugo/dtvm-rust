// Copyright (C) 2021-2025 the DTVM authors. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! Execution control host functions

use crate::core::instance::ZenInstance;
use crate::evm::error::HostFunctionResult;
use crate::evm::traits::EvmHost;
use crate::evm::utils::{validate_address_param, validate_data_param, MemoryAccessor};
use crate::{host_error, host_info, host_warn};

/// Finish execution and return data (RETURN opcode)
/// Terminates execution successfully and returns the specified data
///
/// Parameters:
/// - instance: WASM instance pointer
/// - data_offset: Memory offset of the return data
/// - length: Length of the return data
///
/// Note: This function should cause the WASM execution to terminate
pub fn finish<T>(instance: &ZenInstance<T>, data_offset: i32, length: i32) -> HostFunctionResult<()>
where
    T: EvmHost,
{
    host_info!(
        "finish called: data_offset={}, length={}",
        data_offset,
        length
    );

    let memory = MemoryAccessor::new(instance);

    // Validate parameters
    let (data_offset_u32, length_u32) = validate_data_param(instance, data_offset, length)?;

    // Read the return data
    let return_data = memory
        .read_bytes_vec(data_offset_u32, length_u32)
        .map_err(|e| {
            host_error!(
                "Failed to read return data at offset {} length {}: {}",
                data_offset,
                length,
                e
            );
            e
        })?;

    host_info!(
        "finish: execution completed successfully with {} bytes of return data",
        return_data.len()
    );

    // Store the return data in the Mockevmhost so it can be accessed externally
    let evmhost = &instance.extra_ctx;
    evmhost.finish(return_data.clone());

    host_info!(
        "finish: return data stored in evmhost, hex: 0x{}",
        hex::encode(&return_data)
    );

    // Successfully finish execution - exit with code 0 (success)
    host_info!("finish: execution completed successfully, exiting with code 0");
    instance.exit(0);

    // This should not be reached, but return Ok for completeness
    Ok(())
}

/// Revert execution and return data (REVERT opcode)
/// Terminates execution with failure and returns the specified error data
///
/// Parameters:
/// - instance: WASM instance pointer
/// - data_offset: Memory offset of the revert data
/// - length: Length of the revert data
///
/// Note: This function should cause the WASM execution to terminate with revert
pub fn revert<T>(instance: &ZenInstance<T>, data_offset: i32, length: i32) -> HostFunctionResult<()>
where
    T: EvmHost,
{
    host_info!(
        "revert called: data_offset={}, length={}",
        data_offset,
        length
    );

    let memory = MemoryAccessor::new(instance);

    // Validate parameters
    let (data_offset_u32, length_u32) = validate_data_param(instance, data_offset, length)?;

    // Read the revert data
    let revert_data = memory
        .read_bytes_vec(data_offset_u32, length_u32)
        .map_err(|e| {
            host_error!(
                "Failed to read revert data at offset {} length {}: {}",
                data_offset,
                length,
                e
            );
            e
        })?;

    host_warn!(
        "revert: execution reverted with {} bytes of revert data",
        revert_data.len()
    );

    // Store the revert data in the Mockevmhost so it can be accessed externally
    let evmhost = &instance.extra_ctx;
    evmhost.revert(revert_data.clone());

    host_info!(
        "revert: revert data stored in evmhost, hex: 0x{}",
        hex::encode(&revert_data)
    );

    // Revert execution - exit with code 1 (revert)
    host_warn!("revert: execution reverted, exiting with code 1");
    instance.exit(1);

    // This should not be reached, but return Ok for completeness
    Ok(())
}

/// Invalid operation (INVALID opcode)
/// Terminates execution with an invalid operation error
///
/// Parameters:
/// - instance: WASM instance pointer
///
/// Note: This function should cause the WASM execution to terminate with error
pub fn invalid<T>(instance: &ZenInstance<T>) -> HostFunctionResult<()>
where
    T: EvmHost,
{
    host_info!("invalid called");

    host_error!("invalid: EVM invalid operation encountered");

    // Store the revert data in the Mockevmhost so it can be accessed externally
    let evmhost = &instance.extra_ctx;
    evmhost.invalid();

    // Invalid operation - exit with code 2 (invalid operation)
    host_error!("invalid: invalid EVM operation, exiting with code 2");
    instance.exit(2);

    // This should not be reached, but return Ok for completeness
    Ok(())
}

/// Self-destruct the contract (SELFDESTRUCT opcode)
/// Destroys the current contract and sends its balance to the specified address
///
/// Parameters:
/// - instance: WASM instance pointer
/// - addr_offset: Memory offset of the 20-byte recipient address
///
/// Note: This function should cause the WASM execution to terminate
pub fn self_destruct<T>(instance: &ZenInstance<T>, addr_offset: i32) -> HostFunctionResult<()>
where
    T: EvmHost,
{
    host_info!("self_destruct called: addr_offset={}", addr_offset);

    let evmhost = &instance.extra_ctx;
    let memory = MemoryAccessor::new(instance);

    // Validate the address parameter
    let addr_offset_u32 = validate_address_param(instance, addr_offset)?;

    // Read the recipient address
    let recipient_address = memory.read_address(addr_offset_u32).map_err(|e| {
        host_error!(
            "Failed to read recipient address at offset {}: {}",
            addr_offset,
            e
        );
        e
    })?;

    host_info!(
        "    💥 Self-destructing contract, recipient: 0x{}",
        hex::encode(&recipient_address)
    );

    // Perform the self-destruct operation - let the evmhost handle the details
    let transferred_balance = evmhost.self_destruct(&recipient_address);
    let transferred_amount = u64::from_be_bytes([
        transferred_balance[24],
        transferred_balance[25],
        transferred_balance[26],
        transferred_balance[27],
        transferred_balance[28],
        transferred_balance[29],
        transferred_balance[30],
        transferred_balance[31],
    ]);

    host_info!(
        "    ✅ Balance transferred: {} wei to 0x{}",
        transferred_amount,
        hex::encode(&recipient_address)
    );

    // Self-destruct - exit with code 3 (self-destruct)
    host_warn!("self_destruct: contract self-destructed, exiting with code 3");
    instance.exit(3);

    // This should not be reached, but return Ok for completeness
    Ok(())
}

/// Get the size of the return data from the last call
/// Returns the size of the return data buffer
///
/// This function returns the size of the return data that was set by the last
/// contract call or the current contract's execution (via finish/revert).
///
/// Parameters:
/// - instance: WASM instance pointer
///
/// Returns:
/// - The size of the return data as i32
pub fn get_return_data_size<T>(instance: &ZenInstance<T>) -> i32
where
    T: EvmHost,
{
    let evmhost = &instance.extra_ctx;
    let return_data_size = evmhost.get_return_data_size() as i32;

    host_info!(
        "get_return_data_size called, returning: {} bytes",
        return_data_size
    );
    return_data_size
}

/// Copy return data from the last call to memory
/// Copies the return data from the last external call to the specified memory location
///
/// This function copies return data that was set by the last contract call or
/// the current contract's execution (via finish/revert) to the specified memory location.
///
/// Parameters:
/// - instance: WASM instance pointer
/// - result_offset: Memory offset where the return data should be copied
/// - data_offset: Offset within the return data to start copying from
/// - length: Number of bytes to copy
pub fn return_data_copy<T>(
    instance: &ZenInstance<T>,
    result_offset: i32,
    data_offset: i32,
    length: i32,
) -> HostFunctionResult<()>
where
    T: EvmHost,
{
    host_info!(
        "return_data_copy called: result_offset={}, data_offset={}, length={}",
        result_offset,
        data_offset,
        length
    );

    let evmhost = &instance.extra_ctx;
    let memory = MemoryAccessor::new(instance);

    // Validate parameters
    let (result_offset_u32, length_u32) = validate_data_param(instance, result_offset, length)?;

    if data_offset < 0 {
        return Err(crate::evm::error::out_of_bounds_error(
            data_offset as u32,
            length_u32,
            "negative return data offset",
        ));
    }

    // Get the return data from the evmhost
    let return_data = evmhost.return_data_copy();
    let data_offset_usize = data_offset as usize;

    host_info!("    📤 Available return data: {} bytes", return_data.len());

    // Prepare buffer for copying
    let mut buffer = vec![0u8; length_u32 as usize];

    // Copy from return data with bounds checking
    let available_bytes = if data_offset_usize < return_data.len() {
        return_data.len() - data_offset_usize
    } else {
        0
    };

    let copy_len = std::cmp::min(available_bytes, length_u32 as usize);
    if copy_len > 0 {
        buffer[..copy_len]
            .copy_from_slice(&return_data[data_offset_usize..data_offset_usize + copy_len]);
        host_info!("    📋 Copied {} bytes from return data", copy_len);
    } else {
        host_info!("    📋 No bytes copied (offset beyond return data length or no return data)");
    }

    // Write the buffer to memory
    memory
        .write_bytes(result_offset_u32, &buffer)
        .map_err(|e| {
            host_error!(
                "Failed to write return data to memory at offset {}: {}",
                result_offset,
                e
            );
            e
        })?;

    host_info!(
        "return_data_copy completed: copied {} bytes from return data offset {} to memory offset {}",
        copy_len,
        data_offset,
        result_offset
    );
    Ok(())
}
