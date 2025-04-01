use core::cell::SyncUnsafeCell;

use crate::x86;

static TIME: SyncUnsafeCell<u128> = SyncUnsafeCell::new(0);

pub fn get_time_millis() -> u64 {
	unsafe { (TIME.get().read() / 1000) as u64 }
}

fn tick() {
	unsafe {
		*TIME.get() += 54925;
	}
}

#[unsafe(no_mangle)]
pub extern "C" fn pit_interupt_handler() {
	tick();
	x86::pic1_eoi();
}
