use crate::x86;

#[cfg(not(test))]
use core::panic::PanicInfo;

#[cfg(not(test))]
#[panic_handler]
fn panic_handler(_: &PanicInfo) -> ! {
	loop {
		x86::hlt();
	}
}
