#![cfg_attr(not(test), no_std)]

#[cfg(not(test))]
use core::{alloc::GlobalAlloc, sync::atomic::AtomicBool};
#[cfg(test)]
use std::{sync::atomic::AtomicBool, alloc::GlobalAlloc};

extern crate alloc;

pub struct BuddyAllocator {
    initialized: AtomicBool
}

impl BuddyAllocator {
    pub const fn new() -> Self {
        Self {
            initialized: AtomicBool::new(false)
        }
    }
}

unsafe impl GlobalAlloc for BuddyAllocator {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        todo!()
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: core::alloc::Layout) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::BuddyAllocator;

    /// Test that the `new` function works.
    /// Just creating a new blank `BuddyAllocator`.
    #[test]
    fn test_init() {
        BuddyAllocator::new();
    }
}
