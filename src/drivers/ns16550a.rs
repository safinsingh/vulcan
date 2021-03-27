use core::fmt::{self, Write};

use super::Uart;
use register::{
	mmio::{ReadWrite, WriteOnly},
	register_bitfields, register_structs,
};

register_bitfields! {
	u8,
	// Interrupt enable register
	IER [
		// Enable recieved data available interrupt
		ERBFI OFFSET(0) NUMBITS(1) []
	],
	// FIFO control register
	FCR [
		// FIFO is enabled
		FIFOEN OFFSET(0) NUMBITS(1) []
	],
	// Line control register
	LCR [
		// Word length
		WLS OFFSET(0) NUMBITS(2) [
			FiveBits = 0b00,
			SixBits = 0b01,
			SevenBits = 0b10,
			EightBits = 0b11
		]
	],
	// Line status register
	LSR [
		// Data ready
		DR OFFSET(0) NUMBITS(1) [],
		// Transmitter empty
		TEMT OFFSET(6) NUMBITS(1) []
	]
}

register_structs! {
   #[allow(non_snake_case)]
   pub Registers {
	  (0x00 => RBRTHR: ReadWrite<u8>),
	  (0x01 => IER: ReadWrite<u8, IER::Register>),
	  (0x02 => FCR: WriteOnly<u8, FCR::Register>),
	  (0x03 => LCR: ReadWrite<u8, LCR::Register>),
	  (0x05 => LSR: ReadWrite<u8, LSR::Register>),
	  (0x07 => @END),
   }
}

pub struct NS16550A {
	base: usize,
}

impl NS16550A {
	pub const unsafe fn new(base: usize) -> Self { Self { base } }

	pub fn registers(&self) -> &Registers {
		unsafe { &*(self.base as *const Registers) }
	}
}

impl Uart for NS16550A {
	fn init(&self) {
		let registers = self.registers();
		registers.LCR.write(LCR::WLS::EightBits);
		registers.FCR.write(FCR::FIFOEN::SET);
		registers.IER.write(IER::ERBFI::SET);
	}

	fn read(&self) -> Option<u8> {
		let registers = self.registers();
		if registers.LSR.is_set(LSR::DR) {
			Some(registers.RBRTHR.get())
		} else {
			None
		}
	}

	fn write(&self, ch: u8) {
		let registers = self.registers();
		while registers.LSR.is_set(LSR::TEMT) {}
		registers.RBRTHR.set(ch);
	}
}

impl Write for NS16550A {
	fn write_str(&mut self, s: &str) -> fmt::Result {
		for byte in s.bytes() {
			self.write(byte)
		}
		Ok(())
	}
}
