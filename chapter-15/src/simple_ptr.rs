use std::alloc;
use std::alloc::Layout;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};

/// A simple and probably badly implemented simple pointer. It will allocate space for `T` and
/// drop that memory when dropped.
pub struct SimplePtr<T> {
    /// The raw pointer can be represented by a `*u8`. We don't need to care about the size of the
    /// thing pointed to by the pointer because we will allocate and deallocate the correct amount
    /// of space starting at that address. Note: This might not be the correct way to do it! It
    /// seems to work OK.
    ptr: *mut u8,
    layout: Layout,
    /// When we have a generic `T` and nowhere to "put" it, we can use `PhantomData` to hold the
    /// "typeness" of the object.
    _t: PhantomData<T>,
}

// By implementing the `drop` trait, we ensure our memory always gets cleaned up.
// See https://doc.rust-lang.org/book/ch15-03-drop.html#running-code-on-cleanup-with-the-drop-trait
impl<T> Drop for SimplePtr<T> {
    fn drop(&mut self) {
        println!(
            "SimplePtr: freeing memory with this layout: {:?}",
            self.layout
        );
        unsafe { alloc::dealloc(self.ptr, self.layout) }
    }
}

impl<T> SimplePtr<T> {
    pub fn new(item: T) -> Self {
        let layout = Layout::for_value(&item);
        let ptr: *mut u8;
        unsafe {
            println!(
                "SimplePtr: allocating memory with this layout: {:?}",
                layout
            );
            ptr = alloc::alloc(layout);
            let p = ptr as *mut T;
            *p = item;
        }

        Self {
            ptr,
            layout,
            _t: PhantomData::default(),
        }
    }

    pub fn get_mut(&mut self) -> &mut T {
        unsafe {
            let p = self.ptr as *mut T;
            &mut *p
        }
    }

    pub fn get(&self) -> &T {
        unsafe {
            let p = self.ptr as *const T;
            &*p
        }
    }
}

// We can treat the object like a `&T` with `Deref` and `DerefMut`. See the following section
// for more: https://doc.rust-lang.org/book/ch15-02-deref.html#treating-smart-pointers-like-regular
impl<T> Deref for SimplePtr<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.get()
    }
}

impl<T> DerefMut for SimplePtr<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.get_mut()
    }
}
