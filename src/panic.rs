#![cfg(not(feature = "std"))]

use Vec;
use byteorder::{LittleEndian, ByteOrder};

#[lang = "panic_fmt"]
pub fn panic_fmt(_fmt: ::core::fmt::Arguments, file: &'static str, line: u32, col: u32) -> ! {
	extern "C" {
		fn panic(payload_ptr: *const u8, payload_len: u32) -> !;
	}

	#[cfg(feature = "panic_with_msg")]
	let msg = format!("{}", _fmt);

	#[cfg(not(feature = "panic_with_msg"))]
	let msg = ::alloc::String::new();

	let mut sink = Sink::new(msg.as_bytes().len() + file.as_bytes().len() + 8);
	sink.write_str(msg.as_bytes());
	sink.write_str(file.as_bytes());
	sink.write_u32(line);
	sink.write_u32(col);

	unsafe {
		panic(sink.as_ptr(), sink.len() as u32);
	}
}

struct Sink {
	buf: Vec<u8>,
	pos: usize
}

impl Sink {
	#[inline(always)]
	fn new(capacity: usize) -> Sink {
		Sink {
			buf: Vec::with_capacity(capacity),
			pos: 0,
		}
	}

	#[inline(always)]
	fn reserve(&mut self, len: usize) -> &mut [u8] {
		let dst = &mut self.buf[self.pos..self.pos+len];
		self.pos += len;
		dst
	}

	#[inline(always)]
	fn write_u32(&mut self, val: u32) {
		LittleEndian::write_u32(self.reserve(4), val);
	}

	#[inline(always)]
	fn write_str(&mut self, bytes: &[u8]) {
		self.write_u32(bytes.len() as u32);
		self.reserve(bytes.len()).copy_from_slice(bytes)
	}
}

impl ::core::ops::Deref for Sink {
	type Target = [u8];
	fn deref(&self) -> &[u8] {
		&self.buf
	}
}

#[lang = "eh_personality"]
extern "C" fn eh_personality() {}
