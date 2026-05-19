//! The JARVIS Stream-Graph Scheduler.
//! Responsible for polling nodes and managing the evolution cycle.

use crate::evolution::AtomicNodeSwapper;

pub struct Scheduler {
    swapper: AtomicNodeSwapper,
}

impl Scheduler {
    pub const fn new(initial: *mut crate::evolution::NodeContainer) -> Self {
        Self {
            swapper: AtomicNodeSwapper::new(initial),
        }
    }

    /// The main execution loop of the scheduler.
    /// Time: O(1) dispatch per node.
    pub fn tick(&self) {
        let container_ptr = self.swapper.get_active();
        if !container_ptr.is_null() {
            let container = unsafe { &*container_ptr };
            let node = unsafe { &*container.instance };
            let _ = node.execute();
        }
    }
}
