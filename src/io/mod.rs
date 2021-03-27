use crate::drivers::{ns16550a::NS16550A, Uart};
use lazy_static::lazy_static;
use spin::Mutex;

lazy_static! {
	pub static ref WRITER: Mutex<NS16550A> = Mutex::new({
		let uart = unsafe { NS16550A::new(0x1000_0000) };
		uart.init();
		uart
	});
}

#[macro_export]
macro_rules! kprint {
	($($arg:tt)+) => ({
		use core::fmt::Write;

		let mut writer = crate::io::WRITER.lock();
		let _ = writer.write_fmt(format_args!($($arg)+));
	})
}

#[macro_export]
macro_rules! kprintln {
	() => ({
		kprint!("\n");
	});
	($($arg:tt)+) => ({
		crate::kprint!("{}\n", format_args!($($arg)+));
	})
}
