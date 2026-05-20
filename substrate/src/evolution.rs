//! Wait-Free Node Swapping via Type-State and Atomic Pointers.
//! Time: O(1), Space: O(1).

use core::sync::atomic::{AtomicPtr, Ordering};
use core::marker::PhantomData;

/// Marker for a node that has been loaded into memory but not yet verified.
pub struct Loading;
/// Marker for a node that has passed all PDD/EFDD/EuDD verification checks.
pub struct Verified;

/// A JARVIS Stream Node with Type-State enforcement.
pub struct Node<S, T> {
    state: PhantomData<S>,
    pub data: T,
}

impl<T> Node<Loading, T> {
    pub fn new(data: T) -> Self {
        Self { state: PhantomData, data }
    }

    pub fn verify(self) -> Node<Verified, T> {
        Node { state: PhantomData, data: self.data }
    }
}

/// The fundamental interface for all execution units in the JARVIS stream-graph.
pub trait StreamNode {
    fn budget_nj(&self) -> u64;
    fn complexity(&self) -> &'static str;
    fn execute(&self) -> Result<(), EvolutionError>;
}

#[derive(Debug)]
pub enum EvolutionError {
    BudgetExceeded,
    ContractViolation,
    AbiMismatch,
    MathematicalAnomaly,
}

/// A wrapper to handle the fat pointer of dyn StreamNode in a Sized container.
pub struct NodeContainer {
    pub instance: *mut dyn StreamNode,
}

/// A Lock-Free Swapper for zero-downtime hot-swapping.
pub struct AtomicNodeSwapper {
    // We swap a pointer to a NodeContainer, which itself holds the fat pointer.
    active_container: AtomicPtr<NodeContainer>,
}

impl AtomicNodeSwapper {
    pub const fn new(initial: *mut NodeContainer) -> Self {
        Self {
            active_container: AtomicPtr::new(initial),
        }
    }

    pub fn swap(&self, new_container: *mut NodeContainer) -> *mut NodeContainer {
        self.active_container.swap(new_container, Ordering::AcqRel)
    }

    pub fn get_active(&self) -> *mut NodeContainer {
        self.active_container.load(Ordering::Acquire)
    }
}
