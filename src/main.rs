#![feature(asm, naked_functions)]
#![no_std]
#![no_main]

use fdt::Fdt;
use mm::pmm::Pmm;

mod drivers;
mod io;
mod macros;
mod mm;

#[naked]
#[no_mangle]
#[link_section = ".init.rust"]
unsafe extern "C" fn _start() -> ! {
	asm!(
		"
        .option push
        .option norelax
        lla     gp, __global_pointer$
        .option pop

        lla t0, __bss_start
        lla t1, __bss_end

        # taken from repnop/vanadinite
        zero_bss:
            beq     t0, t1, zero_bss_end
            sd      zero, (t0)
            addi    t0, t0, 8
            j       zero_bss
        zero_bss_end:

        lla     sp, __stack_top
        mv      fp, sp

        j       kmain
        ",
		options(noreturn)
	);
}

#[no_mangle]
extern "C" fn kmain(hart: usize, fdt_ptr: *const u8) -> ! {
	kprintln!("Hello, world from hart {}!", hart);
	let fdt = unsafe {
		Fdt::from_ptr(fdt_ptr).expect("Failed to construct FDT structure")
	};

	let region = fdt.memory().regions().next().expect("???wut???");

	let start = region.starting_address as *mut u8;
	let end = unsafe { start.add(region.size.unwrap()) };

	let mut pmm = Pmm::new(start, end);
	unsafe {
		pmm.init();
	}

	kprintln!("lol?");

	loop {}
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
	kprintln!("Kernel {}", info);
	loop {}
}
