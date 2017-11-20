//! Standard library for parity wasm contracts

#![cfg_attr(not(feature="std"), no_std)]
#![warn(missing_docs)]
#![cfg_attr(feature = "strict", deny(warnings))]
#![feature(lang_items)]
#![feature(link_args)]
#![feature(alloc)]
#![feature(macro_reexport)]


#[cfg(feature="std")]
extern crate core;

#[cfg(not(feature="std"))]
extern crate wasm_alloc;

#[cfg(not(feature="std"))]
extern crate wasm_libc;

#[macro_use]
extern crate cfg_if;

cfg_if! {
	if #[cfg(all(feature="panic_with_msg", not(feature="std")))] {
		#[macro_use]
		#[macro_reexport(vec, format)]
		extern crate alloc;
	} else {
		#[macro_reexport(vec, format)]
		extern crate alloc;
	}
}

extern crate byteorder;

pub extern crate bigint;
pub extern crate parity_hash;
extern crate tiny_keccak;

use byteorder::{LittleEndian, ByteOrder};

pub use alloc::boxed::Box;
pub use alloc::string::String;
pub use alloc::str;
pub use alloc::vec::Vec;
pub use parity_hash as hash;

// Wrapper over storage read/write externs
pub mod storage;

// Safe wrapper around debug logging
pub mod logger;

// Safe wrapper around externalities invokes
pub mod ext;

// Wrapper for arguments and result
mod wrapped;

// Crypto functions
mod crypto;

mod panic;

pub use wrapped::{WrappedArgs, WrappedResult, parse_args};
pub use crypto::keccak;

/// Read u32 using native endianness
pub fn read_u32(slc: &[u8]) -> u32 {
	LittleEndian::read_u32(slc)
}

/// Write u32 using native endianness
pub fn write_u32(dst: &mut [u8], val: u32) {
	LittleEndian::write_u32(dst, val)
}

/// Write ptr using native endianness
pub fn write_ptr(dst: &mut [u8], ptr: *mut u8) {
	// todo: consider: add assert that arch is 32bit
	write_u32(dst, ptr as usize as u32);
}

/// Read u64 using native endianness
pub fn read_u64(slc: &[u8]) -> u64 {
	LittleEndian::read_u64(slc)
}

/// Write u64 using native endianness
pub fn write_u64(dst: &mut [u8], val: u64) {
	LittleEndian::write_u64(dst, val)
}
