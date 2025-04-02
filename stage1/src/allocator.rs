use core::cell::SyncUnsafeCell;

struct Allocator;

#[global_allocator]
static ALLOCATOR: Allocator = Allocator;

static PTR: SyncUnsafeCell<usize> = SyncUnsafeCell::new(0x1000000);

unsafe impl Send for Allocator {}

unsafe impl Sync for Allocator {}

unsafe impl alloc::alloc::GlobalAlloc for Allocator {
	unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
		unsafe {
			*PTR.get() += if *PTR.get() % layout.align() == 0 {
				0
			} else {
				layout.align() - (*PTR.get() % layout.align())
			};
			let ret = *PTR.get() as *mut u8;
			*PTR.get() += layout.size();
			ret
		}
	}

	unsafe fn dealloc(&self, _ptr: *mut u8, _layout: core::alloc::Layout) {}
}
