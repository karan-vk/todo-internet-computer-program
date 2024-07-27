mod errors;
mod memory;
mod paginator;
mod store;
mod todo;

use errors::Error;
use memory::{LAST_TODO_ID, TODO_STORE};
use paginator::Paginator;
use store::TodoStoreWrapper;
use todo::{Priority, Todo, TodoId};

/// Adds a new Todo item.
///
/// # Arguments
///
/// * `text` - The text description of the Todo item.
///
/// # Returns
///
/// The unique identifier for the newly created Todo item.
#[ic_cdk::update]
fn add_todo_item(description: String, priority: Option<Priority>) -> TodoId {
    let principal = ic_cdk::caller();
    let id = generate_next_id();
    let priority = priority.unwrap_or_default();
    TODO_STORE.with(|store| TodoStoreWrapper{store}.add_todo(principal, id, description, priority));
    id
}



/// Retrieves a Todo item.
///
/// # Arguments
///
/// * `id` - The unique identifier for the Todo item.
///
/// # Returns
///
/// A Result containing the Todo item if found, otherwise an Error.
#[ic_cdk::query]
fn get_todo_item(id: TodoId) -> Result<Todo, Error> {
    let principal = ic_cdk::caller();
    TODO_STORE
        .with(|store| TodoStoreWrapper{store}.get_todo(principal, id))
        .ok_or(Error::NotFound)
}

/// Lists Todo items with pagination.
///
/// # Arguments
///
/// * `paginator` - Optional paginator for controlling the list output.
///
/// # Returns
///
/// A vector of Todo items.
#[ic_cdk::query]
fn list_todo_items(paginator: Option<Paginator>) -> Vec<Todo> {
    let principal = ic_cdk::caller();
    let paginator = paginator.unwrap_or_default();
    TODO_STORE.with(|store| TodoStoreWrapper{store}.list_todos(principal, paginator))
}

/// Updates the text of an existing Todo item.
///
/// # Arguments
///
/// * `id` - The unique identifier for the Todo item.
/// * `text` - The new text description of the Todo item.
///
/// # Returns
///
/// A Result indicating success or an Error if the Todo item is not found or the input is invalid.
#[ic_cdk::update]
fn update_todo_item(id: TodoId, text: String) -> Result<(), Error> {
    let principal = ic_cdk::caller();
    TODO_STORE.with(|store| TodoStoreWrapper{store}.update_todo(principal, id, text))
}

/// Deletes a Todo item.
///
/// # Arguments
///
/// * `id` - The unique identifier for the Todo item.
#[ic_cdk::update]
fn delete_todo_item(id: TodoId) {
    let principal = ic_cdk::caller();
    TODO_STORE.with(|store| TodoStoreWrapper{store}.remove_todo(principal, id));
}

/// Marks a Todo item as complete.
///
/// # Arguments
///
/// * `id` - The unique identifier for the Todo item.
///
/// # Returns
///
/// A Result indicating success or an Error if the Todo item is not found.
#[ic_cdk::update]
fn toggle_todo_complete(id: TodoId) -> Result<(), Error> {
    let principal = ic_cdk::caller();
    TODO_STORE.with(|store| TodoStoreWrapper{store}.toggle_todo_complete(principal, id))
}

/// Modifies the priority of a Todo item.
///
/// # Arguments
///
/// * `id` - The unique identifier for the Todo item.
/// * `priority` - The new priority to be set.
///
/// # Returns
///
/// A Result indicating success or an Error if the Todo item is not found.
#[ic_cdk::update]
fn modify_todo_priority(id: TodoId, priority: Priority) -> Result<(), Error> {
    let principal = ic_cdk::caller();
    TODO_STORE.with(|store| TodoStoreWrapper { store }.modify_todo_priority(principal, id, priority))
}

/// Adds a tag to a Todo item.
///
/// # Arguments
///
/// * `id` - The unique identifier for the Todo item.
/// * `tag` - The tag to be added.
///
/// # Returns
///
/// A Result indicating success or an Error if the Todo item is not found.
#[ic_cdk::update]
fn add_tag_to_todo_item(id: TodoId, tag: String) -> Result<(), Error> {
    let principal = ic_cdk::caller();
    TODO_STORE.with(|store| TodoStoreWrapper { store }.add_tag_to_todo(principal, id, tag))
}

/// Removes a tag from a Todo item.
///
/// # Arguments
///
/// * `id` - The unique identifier for the Todo item.
/// * `tag` - The tag to be removed.
///
/// # Returns
///
/// A Result indicating success or an Error if the Todo item is not found.
#[ic_cdk::update]
fn remove_tag_from_todo_item(id: TodoId, tag: String) -> Result<(), Error> {
    let principal = ic_cdk::caller();
    TODO_STORE.with(|store| TodoStoreWrapper { store }.remove_tag_from_todo(principal, id, &tag))
}

/// Generates the next unique identifier for a Todo item.
///
/// # Returns
///
/// The next unique identifier for a Todo item.
fn generate_next_id() -> TodoId {
    LAST_TODO_ID.with(|id| {
        let mut id = id.borrow_mut();
        let new_id = *id.get() + 1;
        id.set(new_id).unwrap()
    })
}



ic_cdk::export_candid!();
