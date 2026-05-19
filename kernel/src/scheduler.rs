//! The JARVIS Stream-Graph Scheduler.
//! Responsible for polling nodes, managing the evolution cycle, 
//! and enforcing Algorithmic Homeostasis (AHD).

use crate::evolution::{AtomicNodeSwapper, EvolutionError};
use crate::substrate::Sandbox;

pub struct Scheduler {
    swapper: AtomicNodeSwapper,
    sandbox: Sandbox,
}

impl Scheduler {
    pub const fn new(initial_node: *mut crate::evolution::NodeContainer) -> Self {
        Self {
            swapper: AtomicNodeSwapper::new(initial_node),
            sandbox: Sandbox::new(),
        }
    }

    /// The main execution loop of the scheduler.
    /// Time: O(1) dispatch per node.
    pub fn tick(&self) {
        let container_ptr = self.swapper.get_active();
        if !container_ptr.is_null() {
            let container = unsafe { &*container_ptr };
            let node = unsafe { &*container.instance };
            
            // Execute node and monitor for AHD violations
            match node.execute() {
                Ok(_) => {},
                Err(EvolutionError::MathematicalAnomaly) => {
                    // AHD: Isolate the node in the sandbox and signal NCI
                    self.sandbox.isolate(container_ptr as *mut (), "Anomaly Detected".as_ptr() as *mut u8);
                }
                Err(e) => {
                    // Fatal evolution errors result in toxic rejection
                    // In a production kernel, this would trigger a system-wide safety halt
                    let _ = e;
                }
            }
        }
    }
}
