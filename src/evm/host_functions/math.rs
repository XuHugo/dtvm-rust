// Copyright (C) 2021-2025 the DTVM authors. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

//! Mathematical operation host functions

use crate::core::instance::ZenInstance;
use crate::evm::error::HostFunctionResult;
use crate::evm::traits::EvmHost;
use crate::evm::utils::{format_hex, validate_bytes32_param, MemoryAccessor};
use crate::{host_error, host_info};

/// Modular addition: (a + b) % n
/// Computes the modular addition of two 256-bit numbers
///
/// Parameters:
/// - instance: WASM instance pointer
/// - a_offset: Memory offset of the first 32-byte operand
/// - b_offset: Memory offset of the second 32-byte operand
/// - n_offset: Memory offset of the 32-byte modulus
/// - result_offset: Memory offset where the 32-byte result should be written
pub fn addmod<T>(
    instance: &ZenInstance<T>,
    a_offset: i32,
    b_offset: i32,
    n_offset: i32,
    result_offset: i32,
) -> HostFunctionResult<()>
where
    T: EvmHost,
{
    host_info!(
        "addmod called: a_offset={}, b_offset={}, n_offset={}, result_offset={}",
        a_offset,
        b_offset,
        n_offset,
        result_offset
    );

    let memory = MemoryAccessor::new(instance);

    // Validate all parameters
    let a_offset_u32 = validate_bytes32_param(instance, a_offset)?;
    let b_offset_u32 = validate_bytes32_param(instance, b_offset)?;
    let n_offset_u32 = validate_bytes32_param(instance, n_offset)?;
    let result_offset_u32 = validate_bytes32_param(instance, result_offset)?;

    // Read operands
    let a_bytes = memory.read_bytes32(a_offset_u32).map_err(|e| {
        host_error!("Failed to read operand A at offset {}: {}", a_offset, e);
        e
    })?;

    let b_bytes = memory.read_bytes32(b_offset_u32).map_err(|e| {
        host_error!("Failed to read operand B at offset {}: {}", b_offset, e);
        e
    })?;

    let n_bytes = memory.read_bytes32(n_offset_u32).map_err(|e| {
        host_error!("Failed to read modulus N at offset {}: {}", n_offset, e);
        e
    })?;

    host_info!(
        "addmod operands: a=0x{}, b=0x{}, n=0x{}",
        format_hex(&a_bytes),
        format_hex(&b_bytes),
        format_hex(&n_bytes)
    );

    let evmhost = &instance.extra_ctx;
    let result_bytes: [u8; 32] = evmhost.addmod(a_bytes, b_bytes, n_bytes);

    host_info!("addmod result: 0x{}", format_hex(&result_bytes));

    // Write the result to memory
    memory
        .write_bytes32(result_offset_u32, &result_bytes)
        .map_err(|e| {
            host_error!(
                "Failed to write addmod result at offset {}: {}",
                result_offset,
                e
            );
            e
        })?;

    host_info!(
        "addmod completed: result written to offset {}",
        result_offset
    );
    Ok(())
}

/// Modular multiplication: (a * b) % n
/// Computes the modular multiplication of two 256-bit numbers
///
/// Parameters:
/// - instance: WASM instance pointer
/// - a_offset: Memory offset of the first 32-byte operand
/// - b_offset: Memory offset of the second 32-byte operand
/// - n_offset: Memory offset of the 32-byte modulus
/// - result_offset: Memory offset where the 32-byte result should be written
pub fn mulmod<T>(
    instance: &ZenInstance<T>,
    a_offset: i32,
    b_offset: i32,
    n_offset: i32,
    result_offset: i32,
) -> HostFunctionResult<()>
where
    T: EvmHost,
{
    host_info!(
        "mulmod called: a_offset={}, b_offset={}, n_offset={}, result_offset={}",
        a_offset,
        b_offset,
        n_offset,
        result_offset
    );

    let memory = MemoryAccessor::new(instance);

    // Validate all parameters
    let a_offset_u32 = validate_bytes32_param(instance, a_offset)?;
    let b_offset_u32 = validate_bytes32_param(instance, b_offset)?;
    let n_offset_u32 = validate_bytes32_param(instance, n_offset)?;
    let result_offset_u32 = validate_bytes32_param(instance, result_offset)?;

    // Read operands
    let a_bytes = memory.read_bytes32(a_offset_u32).map_err(|e| {
        host_error!("Failed to read operand A at offset {}: {}", a_offset, e);
        e
    })?;

    let b_bytes = memory.read_bytes32(b_offset_u32).map_err(|e| {
        host_error!("Failed to read operand B at offset {}: {}", b_offset, e);
        e
    })?;

    let n_bytes = memory.read_bytes32(n_offset_u32).map_err(|e| {
        host_error!("Failed to read modulus N at offset {}: {}", n_offset, e);
        e
    })?;

    host_info!(
        "mulmod operands: a=0x{}, b=0x{}, n=0x{}",
        format_hex(&a_bytes),
        format_hex(&b_bytes),
        format_hex(&n_bytes)
    );

    let evmhost = &instance.extra_ctx;
    let result_bytes: [u8; 32] = evmhost.mulmod(a_bytes, b_bytes, n_bytes);

    host_info!("mulmod result: 0x{}", format_hex(&result_bytes));

    // Write the result to memory
    memory
        .write_bytes32(result_offset_u32, &result_bytes)
        .map_err(|e| {
            host_error!(
                "Failed to write mulmod result at offset {}: {}",
                result_offset,
                e
            );
            e
        })?;

    host_info!(
        "mulmod completed: result written to offset {}",
        result_offset
    );
    Ok(())
}

/// Modular exponentiation: (base ^ exponent) % modulus
/// Computes the modular exponentiation of 256-bit numbers using efficient algorithms
///
/// Parameters:
/// - instance: WASM instance pointer
/// - base_offset: Memory offset of the 32-byte base
/// - exp_offset: Memory offset of the 32-byte exponent
/// - mod_offset: Memory offset of the 32-byte modulus
/// - result_offset: Memory offset where the 32-byte result should be written
pub fn expmod<T>(
    instance: &ZenInstance<T>,
    base_offset: i32,
    exp_offset: i32,
    mod_offset: i32,
    result_offset: i32,
) -> HostFunctionResult<()>
where
    T: EvmHost,
{
    host_info!(
        "expmod called: base_offset={}, exp_offset={}, mod_offset={}, result_offset={}",
        base_offset,
        exp_offset,
        mod_offset,
        result_offset
    );

    let memory = MemoryAccessor::new(instance);

    // Validate all parameters
    let base_offset_u32 = validate_bytes32_param(instance, base_offset)?;
    let exp_offset_u32 = validate_bytes32_param(instance, exp_offset)?;
    let mod_offset_u32 = validate_bytes32_param(instance, mod_offset)?;
    let result_offset_u32 = validate_bytes32_param(instance, result_offset)?;

    // Read operands
    let base_bytes = memory.read_bytes32(base_offset_u32).map_err(|e| {
        host_error!("Failed to read base at offset {}: {}", base_offset, e);
        e
    })?;

    let exp_bytes = memory.read_bytes32(exp_offset_u32).map_err(|e| {
        host_error!("Failed to read exponent at offset {}: {}", exp_offset, e);
        e
    })?;

    let mod_bytes = memory.read_bytes32(mod_offset_u32).map_err(|e| {
        host_error!("Failed to read modulus at offset {}: {}", mod_offset, e);
        e
    })?;

    host_info!(
        "expmod operands: base=0x{}, exp=0x{}, mod=0x{}",
        format_hex(&base_bytes),
        format_hex(&exp_bytes),
        format_hex(&mod_bytes)
    );

    let evmhost = &instance.extra_ctx;
    let result_bytes: [u8; 32] = evmhost.expmod(base_bytes, exp_bytes, mod_bytes);

    host_info!("expmod result: 0x{}", format_hex(&result_bytes));

    // Write the result to memory
    memory
        .write_bytes32(result_offset_u32, &result_bytes)
        .map_err(|e| {
            host_error!(
                "Failed to write expmod result at offset {}: {}",
                result_offset,
                e
            );
            e
        })?;

    host_info!(
        "expmod completed: result written to offset {}",
        result_offset
    );
    Ok(())
}

/// Helper function to validate modular arithmetic parameters
#[allow(dead_code)]
fn validate_modular_params(
    a_offset: i32,
    b_offset: i32,
    n_offset: i32,
    result_offset: i32,
) -> HostFunctionResult<()> {
    let offsets = [a_offset, b_offset, n_offset, result_offset];
    let names = ["operand A", "operand B", "modulus N", "result"];

    for (i, &offset) in offsets.iter().enumerate() {
        if offset < 0 {
            return Err(crate::evm::error::out_of_bounds_error(
                offset as u32,
                32,
                &format!("negative offset for {}", names[i]),
            ));
        }
    }

    Ok(())
}
