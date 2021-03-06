//! This module provides a [stateroom::StateroomService] implementation that is backed by a
//! WebAssembly module. It is the counterpart to `stateroom-wasm`, which is used to
//! implement a compatible guest module.

use std::{
    error::Error,
    fmt::{Debug, Display},
};
pub use wasm_host::WasmHost;
pub use wasm_host_factory::WasmHostFactory;

mod wasm_host;
mod wasm_host_factory;

/// An error encountered while running WebAssembly.
#[derive(Debug)]
pub enum WasmRuntimeError {
    CouldNotImportMemory,
    CouldNotImportGlobal,
    InvalidApiVersion,
    InvalidProtocolVersion,
}

impl Display for WasmRuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self, f)
    }
}

impl Error for WasmRuntimeError {
    fn description(&self) -> &str {
        match self {
            Self::CouldNotImportMemory => "Could not import memory from wasm instance.",
            Self::CouldNotImportGlobal => "Could not read global variable from wasm instance.",
            Self::InvalidApiVersion => {
                "WebAssembly module has an incompatible Stateroom API version."
            }
            Self::InvalidProtocolVersion => {
                "WebAssembly module has an incompatible Stateroom protocol version."
            }
        }
    }
}
