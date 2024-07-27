use std::cell::RefCell;

use candid::Principal;
use ic_stable_structures::{Memory, StableBTreeMap};

use crate::{
    errors::Error,
    paginator::Paginator,
    todo::{Priority, Todo, TodoId},
};

/// Type alias for the TodoStore, which is a StableBTreeMap with a tuple key of (Principal, TodoId) and value of Todo.
pub(crate) type TodoStore<M> = StableBTreeMap<(Principal, TodoId), Todo, M>;

/// Wrapper around the TodoStore to provide additional functionality.
pub(crate) struct TodoStoreWrapper<'a, M: Memory> {
    pub store: &'a RefCell<TodoStore<M>>,
}

impl<'a, M: Memory> TodoStoreWrapper<'a, M> {
    // pub(crate) fn new(store: &'a RefCell<TodoStore<M>>) -> Self {
    //     Self { store }
    // }

    /// Adds a new Todo item to the store.
    ///
    /// # Arguments
    ///
    /// * `principal` - The principal identifier.
    /// * `id` - The unique identifier for the Todo item.
    /// * `text` - The text description of the Todo item.
    pub(crate) fn add_todo(&self, principal: Principal, id: TodoId, description: String, priority: Priority) {
        let todo = Todo::new(id, description,priority);
        self.store.borrow_mut().insert((principal, id), todo);
    }

    /// Retrieves a Todo item from the store.
    ///
    /// # Arguments
    ///
    /// * `principal` - The principal identifier.
    /// * `id` - The unique identifier for the Todo item.
    ///
    /// # Returns
    ///
    /// An Option containing the Todo item if found, otherwise None.
    pub(crate) fn get_todo(&self, principal: Principal, id: TodoId) -> Option<Todo> {
        self.store.borrow().get(&(principal, id))
    }

    /// Lists Todo items for a given principal with pagination.
    ///
    /// # Arguments
    ///
    /// * `principal` - The principal identifier.
    /// * `paginator` - The paginator for controlling the list output.
    ///
    /// # Returns
    ///
    /// A vector of Todo items.
    pub(crate) fn list_todos(&self, principal: Principal, paginator: Paginator) -> Vec<Todo> {
        self.store
            .borrow()
            .range((principal, TodoId::MIN)..)
            .skip(paginator.skip())
            .take_while(|((p, _), _)| p == &principal)
            .take(paginator.limit())
            .map(|((_, _), todo)| todo.clone())
            .collect()
    }

    /// Updates the text of an existing Todo item.
    ///
    /// # Arguments
    ///
    /// * `principal` - The principal identifier.
    /// * `id` - The unique identifier for the Todo item.
    /// * `text` - The new text description of the Todo item.
    ///
    /// # Returns
    ///
    /// A Result indicating success or an Error if the Todo item is not found or the input is invalid.
    pub(crate) fn update_todo(
        &self,
        principal: Principal,
        id: TodoId,
        text: String,
    ) -> Result<(), Error> {
        if text.is_empty() {
            return Err(Error::InvalidInput("Text cannot be empty".to_string()));
        }
        match self.get_todo(principal, id) {
            Some(mut todo) => {
                todo.description = text;
                self.store.borrow_mut().insert((principal, id), todo);
                Ok(())
            }
            None => Err(Error::NotFound),
        }
    }

    /// Removes a Todo item from the store.
    ///
    /// # Arguments
    ///
    /// * `principal` - The principal identifier.
    /// * `id` - The unique identifier for the Todo item.
    pub(crate) fn remove_todo(&self, principal: Principal, id: TodoId) {
        self.store.borrow_mut().remove(&(principal, id));
    }

    /// Toggles the completion status of a Todo item.
    ///
    /// # Arguments
    ///
    /// * `principal` - The principal identifier.
    /// * `id` - The unique identifier for the Todo item.
    ///
    /// # Returns
    ///
    /// A Result indicating success or an Error if the Todo item is not found.
    pub(crate) fn toggle_todo_complete(
        &self,
        principal: Principal,
        id: TodoId,
    ) -> Result<(), Error> {
        match self.get_todo(principal, id) {
            Some(mut todo) => {
                todo.is_completed = !todo.is_completed;
                self.store.borrow_mut().insert((principal, id), todo);
                Ok(())
            }
            None => Err(Error::NotFound),
        }
    }

    /// Modifies the priority of an existing Todo item.
    ///
    /// # Arguments
    ///
    /// * `principal` - The principal identifier.
    /// * `id` - The unique identifier for the Todo item.
    /// * `priority` - The new priority for the Todo item.
    ///
    /// # Returns
    ///
    /// A Result indicating success or an Error if the Todo item is not found.
    pub(crate) fn modify_todo_priority(
        &self,
        principal: Principal,
        id: TodoId,
        priority: Priority,
    ) -> Result<(), Error> {
        match self.get_todo(principal, id) {
            Some(mut todo) => {
                todo.priority = priority;
                self.store.borrow_mut().insert((principal, id), todo);
                Ok(())
            }
            None => Err(Error::NotFound),
        }
    }
    
    /// Adds a tag to a Todo item.
    ///
    /// # Arguments
    ///
    /// * `principal` - The principal identifier.
    /// * `id` - The unique identifier for the Todo item.
    /// * `tag` - The tag to be added.
    ///
    /// # Returns
    ///
    /// A Result indicating success or an Error if the Todo item is not found.
    pub(crate) fn add_tag_to_todo(
        &self,
        principal: Principal,
        id: TodoId,
        tag: String,
    ) -> Result<(), Error> {
        match self.get_todo(principal, id) {
            Some(mut todo) => {
                todo.add_tag(tag);
                self.store.borrow_mut().insert((principal, id), todo);
                Ok(())
            }
            None => Err(Error::NotFound),
        }
    }

    /// Removes a tag from a Todo item.
    ///
    /// # Arguments
    ///
    /// * `principal` - The principal identifier.
    /// * `id` - The unique identifier for the Todo item.
    /// * `tag` - The tag to be removed.
    ///
    /// # Returns
    ///
    /// A Result indicating success or an Error if the Todo item is not found.
    pub(crate) fn remove_tag_from_todo(
        &self,
        principal: Principal,
        id: TodoId,
        tag: &str,
    ) -> Result<(), Error> {
        match self.get_todo(principal, id) {
            Some(mut todo) => {
                todo.remove_tag(tag);
                self.store.borrow_mut().insert((principal, id), todo);
                Ok(())
            }
            None => Err(Error::NotFound),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
   
    use std::cell::RefCell;
    use std::collections::HashMap;

    struct Store {
        store: RefCell<HashMap<(Principal, TodoId), Todo>>,
    }

    impl Store {
        fn new() -> Self {
            Self {
                store: RefCell::new(HashMap::new()),
            }
        }

        fn get_todo(
            &self,
            principal: Principal,
            id: TodoId,
        ) -> Option<Todo> {
            self.store.borrow().get(&(principal, id)).cloned()
        }

        fn add_tag_to_todo(
            &self,
            principal: Principal,
            id: TodoId,
            tag: String,
        ) -> Result<(), Error> {
            match self.get_todo(principal, id) {
                Some(mut todo) => {
                    todo.add_tag(tag);
                    self.store.borrow_mut().insert((principal, id), todo);
                    Ok(())
                }
                None => Err(Error::NotFound),
            }
        }

        fn remove_tag_from_todo(
            &self,
            principal: Principal,
            id: TodoId,
            tag: String,
        ) -> Result<(), Error> {
            match self.get_todo(principal, id) {
                Some(mut todo) => {
                    todo.remove_tag(&tag);
                    self.store.borrow_mut().insert((principal, id), todo);
                    Ok(())
                }
                None => Err(Error::NotFound),
            }
        }
    }

    #[derive(Debug, PartialEq)]
    enum Error {
        NotFound,
    }

    #[test]
    fn test_add_tag_to_todo() {
        let store = Store::new();
        let principal = Principal::anonymous();
        let todo = Todo::new(1, "Test Todo".to_string(), Priority::Medium);
        store.store.borrow_mut().insert((principal, 1), todo);

        assert!(store.add_tag_to_todo(principal, 1, "urgent".to_string()).is_ok());
        let updated_todo = store.get_todo(principal, 1).unwrap();
        assert_eq!(updated_todo.tags, vec!["urgent"]);
    }

    #[test]
    fn test_add_tag_to_nonexistent_todo() {
        let store = Store::new();
        let principal = Principal::anonymous();

        assert_eq!(
            store.add_tag_to_todo(principal, 1, "urgent".to_string()),
            Err(Error::NotFound)
        );
    }

    #[test]
    fn test_remove_tag_from_todo() {
        let store = Store::new();
        let principal = Principal::anonymous();
        let mut todo = Todo::new(1, "Test Todo".to_string(), Priority::Medium);
        todo.add_tag("urgent".to_string());
        store.store.borrow_mut().insert((principal, 1), todo);

        assert!(store.remove_tag_from_todo(principal, 1, "urgent".to_string()).is_ok());
        let updated_todo = store.get_todo(principal, 1).unwrap();
        assert!(updated_todo.tags.is_empty());
    }

    #[test]
    fn test_remove_tag_from_nonexistent_todo() {
        let store = Store::new();
        let principal = Principal::anonymous();

        assert_eq!(
            store.remove_tag_from_todo(principal, 1, "urgent".to_string()),
            Err(Error::NotFound)
        );
    }
}