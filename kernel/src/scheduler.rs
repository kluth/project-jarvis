#![no_std]

use core::sync::atomic::{AtomicPtr, Ordering};
use core::ptr;

// --- Arena Allocator ---

/// Time: O(1)
/// Space: O(N) where N is the total arena size.
/// Mandate: EFDD-compliant contiguous storage with 64-byte cache alignment.
pub struct EfddArenaAllocator {
    start: *mut u8,
    end: *mut u8,
    cursor: AtomicPtr<u8>,
}

impl EfddArenaAllocator {
    /// Time: O(1), Space: O(1)
    pub const fn new(buffer: *mut u8, size: usize) -> Self {
        Self {
            start: buffer,
            end: unsafe { buffer.add(size) },
            cursor: AtomicPtr::new(buffer),
        }
    }

    /// Time: O(1), Space: O(1)
    /// Guarantees cache-line alignment (64 bytes) to minimize energy waste on bus fetches.
    pub fn alloc(&self, size: usize) -> Option<*mut u8> {
        let align = 64;
        let mut current = self.cursor.load(Ordering::Acquire);
        
        loop {
            let addr = current as usize;
            let aligned_addr = (addr + align - 1) & !(align - 1);
            let next = (aligned_addr + size) as *mut u8;

            if next > self.end {
                return None;
            }

            match self.cursor.compare_exchange_weak(
                current,
                next,
                Ordering::SeqCst,
                Ordering::Acquire,
            ) {
                Ok(_) => return Some(aligned_addr as *mut u8),
                Err(updated) => current = updated,
            }
        }
    }
}

// --- Scheduler & Swapper ---

pub trait StreamNode {
    fn execute(&self);
}

/// Time: O(1) for Swap
/// Space: O(1) overhead
/// Mandate: Wait-free RCU implementation. Zero-Waste CPU.
pub struct AtomicNodeSwapper {
    active_node: AtomicPtr<dyn StreamNode>,
    previous_node: AtomicPtr<dyn StreamNode>,
    epoch: core::sync::atomic::AtomicU64,
}

impl AtomicNodeSwapper {
    /// Time: O(1), Space: O(1)
    /// Ordering: Release ensures the new node's data is published globally.
    pub fn swap_node(&self, new_node: *mut dyn StreamNode) {
        // Store the old node for epoch-based reclamation
        let old = self.active_node.swap(new_node, Ordering::Release);
        self.previous_node.store(old, Ordering::Release);
        
        // Advance global epoch to signal a change in execution state
        self.epoch.fetch_add(1, Ordering::SeqCst);
    }

    /// Time: O(1), Space: O(1)
    /// Ordering: Acquire ensures we see the latest published code.
    pub fn get_active(&self) -> *mut dyn StreamNode {
        self.active_node.load(Ordering::Acquire)
    }
}
