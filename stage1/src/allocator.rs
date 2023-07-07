struct Allocator(*mut u8);

#[global_allocator]
static mut ALLOCATOR: Allocator = Allocator(0x100000 as *mut u8);

unsafe impl Send for Allocator { }

unsafe impl Sync for Allocator { }

unsafe impl alloc::alloc::GlobalAlloc for Allocator {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        let ptr_sz = self.0 as usize;
        let ptr_sz_aligned = (ptr_sz + layout.align() - 1) % layout.align();
        let start = ptr_sz_aligned as *mut u8;
        ALLOCATOR.0 = start.add(layout.size());
        start
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: core::alloc::Layout) {
        
    }
}
