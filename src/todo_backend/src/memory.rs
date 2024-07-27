use std::cell::RefCell;

use ic_stable_structures::{
    memory_manager::{MemoryId, MemoryManager, VirtualMemory},
    DefaultMemoryImpl, StableBTreeMap, StableCell,
};

use crate::{store::TodoStore, todo::TodoId};

/// Type alias for the virtual memory used in the stable structures.
type Memory = VirtualMemory<DefaultMemoryImpl>;

/// Memory ID for storing the last Todo ID.
const LAST_TODO_ID_MEMORY_ID: MemoryId = MemoryId::new(0);

/// Memory ID for storing the Todo items.
const TODO_STORE_MEMORY_ID: MemoryId = MemoryId::new(1);

thread_local! {
    /// Global memory manager for stable structures.
    static GLOBAL_MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
        RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));

    /// Stable cell for storing the last Todo ID.
    pub(crate) static LAST_TODO_ID: RefCell<StableCell<TodoId, Memory>> = RefCell::new(
        StableCell::init(
            GLOBAL_MEMORY_MANAGER.with(|manager| manager.borrow().get(LAST_TODO_ID_MEMORY_ID)), 0,
        ).unwrap()
    );

    /// Stable BTreeMap for storing Todo items.
    pub(crate) static TODO_STORE: RefCell<TodoStore<Memory>> = RefCell::new(
        StableBTreeMap::init(
            GLOBAL_MEMORY_MANAGER.with(|manager| manager.borrow().get(TODO_STORE_MEMORY_ID))
        )
    );
}
