struct Allocator;

#[global_allocator]
static ALLOCATOR: Allocator = Allocator;

static mut PTR: usize = 0x1000000;

unsafe impl Send for Allocator { }

unsafe impl Sync for Allocator { }

unsafe extern "C" fn real_alloc(size: usize) -> *mut u8 {
    let ret = PTR as *mut u8;
    PTR = PTR + size + 0x100;
    core::arch::asm!(
        "nop",
        in("eax") ret,
    );
    ret
}

unsafe impl alloc::alloc::GlobalAlloc for Allocator {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        real_alloc(layout.size())
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: core::alloc::Layout) {
        
    }
}
