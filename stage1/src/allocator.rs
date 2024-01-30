struct Allocator;

#[global_allocator]
static ALLOCATOR: Allocator = Allocator;

static mut PTR: usize = 0x1000000;

unsafe impl Send for Allocator { }

unsafe impl Sync for Allocator { }

#[no_mangle]
#[inline(never)]
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

    #[inline(never)]
    unsafe fn realloc(&self, ptr: *mut u8, layout: core::alloc::Layout, new_size: usize) -> *mut u8 {
        // SAFETY: the caller must ensure that the `new_size` does not overflow.
        // `layout.align()` comes from a `Layout` and is thus guaranteed to be valid.
        let new_layout = unsafe { core::alloc::Layout::from_size_align_unchecked(new_size, layout.align()) };
        // SAFETY: the caller must ensure that `new_layout` is greater than zero.
        let new_ptr = unsafe { self.alloc(new_layout) };
        if !new_ptr.is_null() {
            // SAFETY: the previously allocated block cannot overlap the newly allocated block.
            // The safety contract for `dealloc` must be upheld by the caller.
            unsafe {
                core::ptr::copy_nonoverlapping(ptr, new_ptr, 0x100);
                self.dealloc(ptr, layout);
            }
        }
        new_ptr
    }


}
