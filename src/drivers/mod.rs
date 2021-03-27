pub mod ns16550a;

use core::fmt::Write;

pub trait Uart: Write + 'static {
	fn init(&self);
	fn read(&self) -> Option<u8>;
	fn write(&self, ch: u8);
}
