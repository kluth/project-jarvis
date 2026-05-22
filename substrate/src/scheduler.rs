//! The JARVIS Production Scheduler.
//! Implements an O(1) wait-free cooperative scheduler.
//! Time: O(1) for task enqueue and dispatch.
//! Space: O(N) where N is max task capacity.

use crate::evolution::{AtomicNodeSwapper, EvolutionError};
use crate::substrate::Sandbox;
use core::sync::atomic::{AtomicUsize, Ordering};

/// Max capacity for the wait-free task queue.
const MAX_TASKS: usize = 256;

use core::cell::UnsafeCell;

/// A Production-Grade Wait-Free Scheduler.
/// EFDD: Zero polling, interrupt-ready design.
pub struct Scheduler {
    swapper: AtomicNodeSwapper,
    sandbox: Sandbox,
    queue: [UnsafeCell<*mut crate::evolution::NodeContainer>; MAX_TASKS],
    /// Priority queue for nodes in AwaitingFix state (Rev 6.0)
    fix_queue: [UnsafeCell<*mut crate::evolution::NodeContainer>; MAX_TASKS],
    head: AtomicUsize,
    tail: AtomicUsize,
    fix_head: AtomicUsize,
    fix_tail: AtomicUsize,
}

impl Scheduler {
    /// Initializer for the production scheduler.
    /// Time: O(1), Space: O(N).
    pub const fn new(initial_node: *mut crate::evolution::NodeContainer) -> Self {
        const EMPTY_CELL: UnsafeCell<*mut crate::evolution::NodeContainer> = UnsafeCell::new(core::ptr::null_mut());

        Self {
            swapper: AtomicNodeSwapper::new(initial_node),
            sandbox: Sandbox::new(),
            queue: [EMPTY_CELL; MAX_TASKS],
            fix_queue: [EMPTY_CELL; MAX_TASKS],
            head: AtomicUsize::new(0),
            tail: AtomicUsize::new(0),
            fix_head: AtomicUsize::new(0),
            fix_tail: AtomicUsize::new(0),
        }
    }

    /// Enqueues a node for execution.
    /// Time: O(1)
    pub fn submit(&self, container: *mut crate::evolution::NodeContainer) -> Result<(), EvolutionError> {
        let tail = self.tail.load(Ordering::Acquire);
        let next_tail = (tail + 1) % MAX_TASKS;

        if next_tail == self.head.load(Ordering::Acquire) {
            return Err(EvolutionError::MathematicalAnomaly); // Queue Full
        }

        unsafe {
            *self.queue[tail].get() = container;
        }

        self.tail.store(next_tail, Ordering::Release);
        Ok(())
    }

    /// Dispatches the next available task, prioritizing self-healing fix plans.
    /// Time: O(1)
    pub fn dispatch(&self) {
        // 1. Priority 1: Self-Healing Tasks (Rev 6.0)
        let fix_head = self.fix_head.load(Ordering::Acquire);
        if fix_head != self.fix_tail.load(Ordering::Acquire) {
            self.dispatch_from_queue(&self.fix_queue, &self.fix_head, fix_head);
            return;
        }

        // 2. Priority 2: Standard Data Processing
        let head = self.head.load(Ordering::Acquire);
        if head != self.tail.load(Ordering::Acquire) {
            self.dispatch_from_queue(&self.queue, &self.head, head);
        }
    }

    fn dispatch_from_queue(
        &self, 
        q: &[UnsafeCell<*mut crate::evolution::NodeContainer>; MAX_TASKS], 
        h_ptr: &AtomicUsize, 
        head: usize
    ) {
        let container_ptr = unsafe { *q[head].get() };
        if !container_ptr.is_null() {
            let container = unsafe { &*container_ptr };
            let node = unsafe { &*container.instance };
            
            match node.execute() {
                Ok(_) => {},
                Err(EvolutionError::MathematicalAnomaly) => {
                    self.sandbox.isolate(container_ptr as *mut (), "Anomaly".as_ptr() as *mut u8);
                }
                _ => {}
            }
        }
        h_ptr.store((head + 1) % MAX_TASKS, Ordering::Release);
    }

    /// The core execution tick, typically called from a hardware interrupt or main loop.
    /// Time: O(1)
    pub fn tick(&self) {
        // 1. Dispatch highest priority task
        self.dispatch();
        
        // 2. Perform evolutionary sync if needed
        let active = self.swapper.get_active();
        if !active.is_null() {
             // Future: Parallel execution of persistent nodes
        }
    }
}

unsafe impl Sync for Scheduler {}
unsafe impl Send for Scheduler {}
