// Copyright (C) 2021-2025 the DTVM authors. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! Code related host functions

use crate::core::instance::ZenInstance;
use crate::evm::error::HostFunctionResult;
use crate::evm::traits::EvmHost;
use crate::evm::utils::{
    validate_address_param, validate_bytes32_param, validate_data_param, MemoryAccessor,
};
use crate::{host_error, host_info};

/// Get the size of the current contract's code
/// Returns the size of the contract code including the 4-byte length prefix
///
/// Parameters:
/// - instance: WASM instance pointer
///
/// Returns:
/// - The size of the contract code as i32
pub fn get_code_size<T>(instance: &ZenInstance<T>) -> i32
where
    T: EvmHost,
{
    let evmhost = &instance.extra_ctx;
    let code_size = evmhost.get_code_size();

    host_info!("get_code_size called, returning: {}", code_size);
    code_size
}

/// Copy contract code to memory
/// Copies a portion of the current contract's code to the specified memory location
///
/// Parameters:
/// - instance: WASM instance pointer
/// - result_offset: Memory offset where the code should be copied
/// - code_offset: Offset within the contract code to start copying from
/// - length: Number of bytes to copy
pub fn code_copy<T>(
    instance: &ZenInstance<T>,
    result_offset: i32,
    code_offset: i32,
    length: i32,
) -> HostFunctionResult<()>
where
    T: EvmHost,
{
    host_info!(
        "code_copy called: result_offset={}, code_offset={}, length={}",
        result_offset,
        code_offset,
        length
    );

    let evmhost = &instance.extra_ctx;
    let memory = MemoryAccessor::new(instance);

    // Validate parameters
    let (result_offset_u32, length_u32) = validate_data_param(instance, result_offset, length)?;

    if code_offset < 0 {
        return Err(crate::evm::error::out_of_bounds_error(
            code_offset as u32,
            length_u32,
            "negative code offset",
        ));
    }

    // Get a mutable buffer to write to
    let mut buffer = vec![0u8; length_u32 as usize];

    // Copy code using the evmhost's copy_code method
    let code = evmhost.code_copy();
    let dest_len = buffer.len();

    // Calculate how much we can actually copy
    let available_from_offset = if (code_offset as usize) < code.len() {
        code.len() - code_offset as usize
    } else {
        0
    };

    let copied_bytes = std::cmp::min(
        std::cmp::min(length_u32 as usize, available_from_offset),
        dest_len,
    );

    if copied_bytes > 0 {
        buffer[..copied_bytes]
            .copy_from_slice(&code[code_offset as usize..code_offset as usize + copied_bytes]);
    }

    // Fill remaining buffer with zeros if needed
    if copied_bytes < dest_len && copied_bytes < length_u32 as usize {
        let zero_fill_len =
            std::cmp::min(length_u32 as usize - copied_bytes, dest_len - copied_bytes);
        if zero_fill_len > 0 {
            buffer[copied_bytes..copied_bytes + zero_fill_len].fill(0);
        }
    }
    // Write the copied data to memory
    memory
        .write_bytes(result_offset_u32, &buffer[..copied_bytes])
        .map_err(|e| {
            host_error!(
                "Failed to write code to memory at offset {}: {}",
                result_offset,
                e
            );
            e
        })?;

    host_info!(
        "code_copy completed: copied {} bytes from code offset {} to memory offset {}",
        copied_bytes,
        code_offset,
        result_offset
    );
    Ok(())
}

/// Get the size of an external contract's code
/// Returns the size of the specified external contract's code
///
/// This function queries the external code size using the ExternalCodeProvider trait,
/// allowing users to implement custom external contract code lookup logic.
///
/// Parameters:
/// - instance: WASM instance pointer
/// - addr_offset: Memory offset of the 20-byte contract address
///
/// Returns:
/// - The size of the external contract's code as i32, or 0 if not found
pub fn get_external_code_size<T>(
    instance: &ZenInstance<T>,
    addr_offset: i32,
) -> HostFunctionResult<i32>
where
    T: EvmHost,
{
    host_info!("get_external_code_size called: addr_offset={}", addr_offset);

    let evmhost = &instance.extra_ctx;
    let memory = MemoryAccessor::new(instance);

    // Validate the address parameter
    let addr_offset_u32 = validate_address_param(instance, addr_offset)?;

    // Read the address
    let address = memory.read_address(addr_offset_u32).map_err(|e| {
        host_error!("Failed to read address at offset {}: {}", addr_offset, e);
        e
    })?;

    host_info!(
        "    🔍 Querying external code size for address: 0x{}",
        hex::encode(&address)
    );

    // Query the external code size using the ExternalCodeProvider trait
    match evmhost.get_external_code_size(&address) {
        Some(size) => {
            host_info!("    📏 Retrieved external code size: {}", size);
            Ok(size)
        }
        None => {
            host_info!("    ❌ External contract not found, returning size 0");
            Ok(0) // Return 0 for non-existent contracts
        }
    }
}

/// Get the hash of an external contract's code
/// Writes the 32-byte code hash of the specified external contract to memory
///
/// This function queries the external code hash using the ExternalCodeProvider trait,
/// allowing users to implement custom external contract code lookup logic.
///
/// Parameters:
/// - instance: WASM instance pointer
/// - addr_offset: Memory offset of the 20-byte contract address
/// - result_offset: Memory offset where the 32-byte hash should be written
pub fn get_external_code_hash<T>(
    instance: &ZenInstance<T>,
    addr_offset: i32,
    result_offset: i32,
) -> HostFunctionResult<()>
where
    T: EvmHost,
{
    host_info!(
        "get_external_code_hash called: addr_offset={}, result_offset={}",
        addr_offset,
        result_offset
    );

    let evmhost = &instance.extra_ctx;
    let memory = MemoryAccessor::new(instance);

    // Validate parameters
    let addr_offset_u32 = validate_address_param(instance, addr_offset)?;
    let result_offset_u32 = validate_bytes32_param(instance, result_offset)?;

    // Read the address
    let address = memory.read_address(addr_offset_u32).map_err(|e| {
        host_error!("Failed to read address at offset {}: {}", addr_offset, e);
        e
    })?;

    host_info!(
        "    🔍 Querying external code hash for address: 0x{}",
        hex::encode(&address)
    );

    // Query the external code hash using the ExternalCodeProvider trait
    match evmhost.get_external_code_hash(&address) {
        Some(hash) => {
            host_info!(
                "    🔐 Retrieved external code hash: 0x{}",
                hex::encode(&hash)
            );

            // Write the hash to memory
            memory
                .write_bytes32(result_offset_u32, &hash)
                .map_err(|e| {
                    host_error!(
                        "Failed to write code hash at offset {}: {}",
                        result_offset,
                        e
                    );
                    e
                })?;

            host_info!(
                "get_external_code_hash completed: hash written to offset {}",
                result_offset
            );
            Ok(())
        }
        None => {
            host_info!("    ❌ External contract not found, writing zero hash");

            // Write zero hash for non-existent contracts
            let zero_hash = [0u8; 32];
            memory
                .write_bytes32(result_offset_u32, &zero_hash)
                .map_err(|e| {
                    host_error!(
                        "Failed to write zero hash at offset {}: {}",
                        result_offset,
                        e
                    );
                    e
                })?;

            host_info!(
                "get_external_code_hash completed: zero hash written for non-existent contract"
            );
            Ok(())
        }
    }
}

/// Copy external contract code to memory
/// Copies a portion of an external contract's code to the specified memory location
///
/// This function queries the external code using the ExternalCodeProvider trait,
/// allowing users to implement custom external contract code lookup logic.
///
/// Parameters:
/// - instance: WASM instance pointer
/// - addr_offset: Memory offset of the 20-byte contract address
/// - result_offset: Memory offset where the code should be copied
/// - code_offset: Offset within the external contract code to start copying from
/// - length: Number of bytes to copy
pub fn external_code_copy<T>(
    instance: &ZenInstance<T>,
    addr_offset: i32,
    result_offset: i32,
    code_offset: i32,
    length: i32,
) -> HostFunctionResult<()>
where
    T: EvmHost,
{
    host_info!(
        "external_code_copy called: addr_offset={}, result_offset={}, code_offset={}, length={}",
        addr_offset,
        result_offset,
        code_offset,
        length
    );

    let evmhost = &instance.extra_ctx;
    let memory = MemoryAccessor::new(instance);

    // Validate parameters
    let addr_offset_u32 = validate_address_param(instance, addr_offset)?;
    let (result_offset_u32, length_u32) = validate_data_param(instance, result_offset, length)?;

    if code_offset < 0 {
        return Err(crate::evm::error::out_of_bounds_error(
            code_offset as u32,
            length_u32,
            "negative external code offset",
        ));
    }

    // Read the address
    let address = memory.read_address(addr_offset_u32).map_err(|e| {
        host_error!("Failed to read address at offset {}: {}", addr_offset, e);
        e
    })?;

    host_info!(
        "    🔍 Querying external code for address: 0x{}",
        hex::encode(&address)
    );

    // Query the external code using the ExternalCodeProvider trait
    match evmhost.external_code_copy(&address) {
        Some(external_code) => {
            host_info!(
                "    📄 Retrieved external code: {} bytes",
                external_code.len()
            );

            let mut buffer = vec![0u8; length_u32 as usize];

            // Copy from external code with bounds checking
            let code_offset_usize = code_offset as usize;
            let available_bytes = if code_offset_usize < external_code.len() {
                external_code.len() - code_offset_usize
            } else {
                0
            };

            let copy_len = std::cmp::min(available_bytes, length_u32 as usize);
            if copy_len > 0 {
                buffer[..copy_len].copy_from_slice(
                    &external_code[code_offset_usize..code_offset_usize + copy_len],
                );
                host_info!("    📋 Copied {} bytes from external code", copy_len);
            } else {
                host_info!("    📋 No bytes copied (offset beyond code length)");
            }

            // Write the copied data to memory
            memory
                .write_bytes(result_offset_u32, &buffer)
                .map_err(|e| {
                    host_error!(
                        "Failed to write external code to memory at offset {}: {}",
                        result_offset,
                        e
                    );
                    e
                })?;

            host_info!(
                "external_code_copy completed: copied {} bytes from external code offset {} to memory offset {}",
                copy_len,
                code_offset,
                result_offset
            );
            Ok(())
        }
        None => {
            host_info!("    ❌ External contract not found, writing zeros");

            // Write zeros for non-existent contracts
            let buffer = vec![0u8; length_u32 as usize];
            memory
                .write_bytes(result_offset_u32, &buffer)
                .map_err(|e| {
                    host_error!(
                        "Failed to write zeros to memory at offset {}: {}",
                        result_offset,
                        e
                    );
                    e
                })?;

            host_info!(
                "external_code_copy completed: wrote {} zero bytes for non-existent contract",
                length_u32
            );
            Ok(())
        }
    }
}
