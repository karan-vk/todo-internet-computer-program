use std::borrow::Cow;


use candid::{CandidType, Decode, Deserialize, Encode};
use ic_stable_structures::{storable::Bound, Storable};

/// Type alias for the unique identifier of a Todo item.
pub(crate) type TodoId = u32;

/// Represents the priority level of a Todo item.
#[derive(CandidType, Deserialize, Clone, Copy)]
pub(crate) enum Priority {
    Low,
    Medium,
    High,
}

impl Default for Priority {
    fn default() -> Self {
        Priority::Medium
    }
}

/// Represents a Todo item with an ID, text description, and completion status.
#[derive(CandidType, Deserialize, Clone)]
pub(crate) struct Todo {
    /// Unique identifier for the Todo item.
    pub(crate) id: TodoId,
    /// Text description of the Todo item.
    pub(crate) description: String,
    /// Completion status of the Todo item.
    pub(crate) is_completed: bool,
    /// Priority level of the Todo item.
    pub(crate) priority: Priority,
    /// Tags associated with the Todo item.
    pub(crate) tags: Vec<String>,

}

impl Todo {
    /// Creates a new Todo item.
    ///
    /// # Arguments
    ///
    /// * `id` - The unique identifier for the Todo item.
    /// * `description` - The text description of the Todo item.
    ///
    /// # Returns
    ///
    /// A new instance of `Todo`.
    pub(crate) fn new(id: TodoId, description: String, priority: Priority) -> Self {
        
        Self {
            id,
            description,
            is_completed: false,
            priority: priority,
            tags: Vec::new(),
        }
    }

    /// Adds a tag to the Todo item.
    ///
    /// # Arguments
    ///
    /// * `tag` - The tag to be added.
    pub(crate) fn add_tag(&mut self, tag: String) {
        self.tags.push(tag);
    }

    /// Removes a tag from the Todo item.
    ///
    /// # Arguments
    ///
    /// * `tag` - The tag to be removed.
    pub(crate) fn remove_tag(&mut self, tag: &str) {
        self.tags.retain(|t| t != tag);
    }

    

    
}

impl Storable for Todo {
    const BOUND: Bound = Bound::Unbounded;

    /// Converts the `Todo` instance to a byte array.
    ///
    /// # Returns
    ///
    /// A `Cow<[u8]>` containing the byte representation of the `Todo` instance.
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    /// Creates a `Todo` instance from a byte array.
    ///
    /// # Arguments
    ///
    /// * `bytes` - A `Cow<[u8]>` containing the byte representation of a `Todo` instance.
    ///
    /// # Returns
    ///
    /// A `Todo` instance.
    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}
