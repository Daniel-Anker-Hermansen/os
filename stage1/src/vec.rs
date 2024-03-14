use core::ops::Index;

pub struct Vec<T> {
    raw: *mut T,
    capactity: usize,
    len: usize,
}

impl<T> Vec<T> {
    pub fn new() -> Self {
        Self {
            raw: core::ptr::null_mut(),
            capactity: 0,
            len: 0,
        }
    }

    pub fn push(&mut self, val: T) {
        if self.len == self.capactity {
            let new_cap = (self.capactity * 2).max(1);
            let layout = alloc::alloc::Layout::array::<T>(self.capactity).unwrap();
            self.raw = unsafe { alloc::alloc::realloc(self.raw.cast(), layout, new_cap * core::mem::size_of::<T>()).cast() };
        }
        unsafe { self.raw.add(self.len).write(val); }
        self.len += 1;
    }
}

impl<T> Index<usize> for Vec<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        if index > self.len {
            panic!();
        }
        unsafe {
            self.raw.add(index).as_ref().unwrap_unchecked()
        }
    }
}
