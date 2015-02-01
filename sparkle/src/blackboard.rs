//! The blackboard related features.

use std::collections::HashMap;
use std::collections::hash_map::Entry as HashMapEntry;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;
use std::cell::{Ref, RefMut, RefCell};
use std::any::Any;

/// Contains a value exposed by the `Blackboard`.
pub type BlackboardEntry<T: 'static> = Rc<RefCell<T>>;

/// A container exposing different values to multiples readers. 
///
/// This is generally used when multiple systems need to work together.
pub struct Blackboard {
    entries: HashMap<String, Box<Any>>
}

impl Blackboard {
    /// Creates an empty `Blackboard`.
    pub fn new() -> Blackboard {
        Blackboard {
            entries: HashMap::new()
        }
    }

    /// Inserts an entry in the blackboard with the given name and value.
    /// 
    /// This method panics if the name is already used.
    pub fn insert<T: 'static>(&mut self, name: &str, value: T) {
        match self.entries.entry(name.to_string()) {
            HashMapEntry::Vacant(vacant) => {
                vacant.insert(Box::new(Rc::new(RefCell::new(value))));
            },
            HashMapEntry::Occupied(_) => panic!("The name {} is already used", name)
        }
    }

    /// Tries to retrieve a reference to an entry from the blackboard with the given name.
    ///
    /// Returns `None` if the entry doesn't exist.
    pub fn try_get<'a, T: 'static>(&'a self, name: &str) -> Option<Ref<'a, T>> {
        self.entries.get(name).and_then(|any_entry| any_entry.downcast_ref::<BlackboardEntry<T>>())
                              .map(|entry| entry.borrow())
    }

    /// Retrieves a reference to an entry from the blackboard with the given name.
    ///
    /// This method panics if the entry doesn't exist.
    pub fn get<'a, T: 'static>(&'a self, name: &str) -> Ref<'a, T> {
        self.try_get(name).expect(format!("Failed to get {}", name).as_slice())
    }

    /// Tries to retrieve an entry from the blackboard with the given name.
    ///
    /// Returns `None` if the entry doesn't exist.
    pub fn try_get_entry<T: 'static>(&self, name: &str) -> Option<BlackboardEntry<T>> {
        self.entries.get(name).and_then(|any_entry| any_entry.downcast_ref::<BlackboardEntry<T>>())
                              .map(|entry| entry.clone())    
    }

    /// Retrieves an entry from the blackboard with the given name.
    ///
    /// This method panics if the entry doesn't exist.
    pub fn get_entry<T: 'static>(&self, name: &str) -> BlackboardEntry<T> {
        self.try_get_entry(name).expect(format!("Failed to get {}", name).as_slice())
    }

    /// Tries to retrieve a mutable reference to an entry from the blackboard with the given name.
    ///
    /// Returns `None` if the entry doesn't exist.
    pub fn try_get_mut<'a, T: 'static>(&'a self, name: &str) -> Option<RefMut<'a, T>> {
        self.entries.get(name).and_then(|any_entry| any_entry.downcast_ref::<BlackboardEntry<T>>())
                              .map(|entry| entry.borrow_mut())
    }

    /// Retrieves a mutable reference to an entry from the blackboard with the given name.
    ///
    /// This method panics if the entry doesn't exist.
    pub fn get_mut<'a, T: 'static>(&'a self, name: &str) -> RefMut<'a, T> {
        self.try_get_mut(name).expect(format!("Failed to get {}", name).as_slice())
    }
}

/// A simple wrapper around a `Blackboard`, allocated with Rc. 
///
/// This permits to reduce the verbosity created by Rc and RefCell.
#[derive(Clone)]
pub struct SharedBlackboard(Rc<RefCell<Blackboard>>);

impl SharedBlackboard {
    pub fn new() -> SharedBlackboard {
        SharedBlackboard(Rc::new(RefCell::new(Blackboard::new())))
    }
}

impl Deref for SharedBlackboard {
    type Target = Rc<RefCell<Blackboard>>;

    fn deref(&self) -> &Rc<RefCell<Blackboard>> {
        &self.0
    }
}

impl DerefMut for SharedBlackboard { 
    fn deref_mut(&mut self) -> &mut Rc<RefCell<Blackboard>> {
        &mut self.0
    }
}