use super::PAGE_SIZE;
use core::mem;

pub struct Pmm {
	bitmap_ptr: *mut u8,
	bitmap_size: usize,
	mem_start: *mut u8,
	mem_end: *mut u8,
}

impl Pmm {
	pub fn new(mem_start: *mut u8, mem_end: *mut u8) -> Self {
		Self {
			bitmap_ptr: mem_start,
			bitmap_size: 0,
			mem_start,
			mem_end,
		}
	}

	fn slice(&mut self) -> &mut [u8] {
		unsafe {
			core::slice::from_raw_parts_mut(self.bitmap_ptr, self.bitmap_size)
		}
	}

	pub unsafe fn init(&mut self) {
		let pages =
			(self.mem_end as usize - self.mem_start as usize) / PAGE_SIZE;
		self.bitmap_size = pages / 8;
		self.slice().fill(0);

		for page in (0..self.bitmap_size).step_by(PAGE_SIZE) {
			self.set_used(self.mem_start.add(page * PAGE_SIZE));
		}
	}

	pub unsafe fn set_unused(&mut self, addr: *mut u8) {
		let bit = (addr as usize - self.mem_start as usize) / PAGE_SIZE;
		let byte = bit / 8;

		self.slice()[byte] &= !(1 << bit % 8);
	}

	pub unsafe fn set_used(&mut self, addr: *mut u8) {
		let bit = (addr as usize - self.mem_start as usize) / PAGE_SIZE;
		let byte = bit / 8;

		self.slice()[byte] |= 1 << bit % 8;
	}
}
