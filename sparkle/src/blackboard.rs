//! The Blackboard related structures

use std::collections::HashMap;
use std::collections::hash_map::Entry as HashMapEntry;
use std::rc::Rc;
use std::cell::RefCell;
use std::any::Any;

/// An `BlackboardEntry` is any value contained by the `Blackboard`
pub type BlackboardEntry<T: 'static> = Rc<RefCell<T>>;

/// A Blackboard is a container that can expose different values
/// to multiples readers. 
///
/// This is generally used when multiple systems need to work together.
pub struct Blackboard {
    entries: HashMap<String, Box<Any>>
}

impl Blackboard {
    /// Create a new empty `Blackboard`
    pub fn new() -> Blackboard {
        Blackboard {
            entries: HashMap::new()
        }
    }

    /// Insert a value in the blackboard with the given name.
    /// 
    /// This method panic if the name is already used.
    pub fn insert<T: 'static>(&mut self, name: &str, entry: T) {
        match self.entries.entry(name.to_string()) {
            HashMapEntry::Vacant(vacant) => {
                vacant.insert(Box::new(Rc::new(RefCell::new(entry))));
            },
            HashMapEntry::Occupied(_) => panic!("The name {} is already used", name)
        }
    }

    /// Try to retrieve a value with the given name from the blackboard.
    ///
    /// Returns None if the value doesn't exist.
    pub fn try_get<T: 'static>(&self, name: &str) -> Option<BlackboardEntry<T>> {
        self.entries.get(name).and_then(|any_entry| any_entry.downcast_ref::<BlackboardEntry<T>>())
                              .map(|entry| entry.clone())
    }

    /// Retrieve a value with the given name from the blackboard.
    ///
    /// This method panic if the value doesn't exist.
    pub fn get<T: 'static>(&self, name: &str) -> BlackboardEntry<T> {
        let any_entry = self.entries.get(name).expect(format!("missing entry {}.", name).as_slice());
        let entry = any_entry.downcast_ref::<BlackboardEntry<T>>().expect("invalid entry type.");

        entry.clone()
    }
}

/// A `SharedBlackboard` is a simple wrapper around
/// a `Blackboard` allocated with Rc. 
///
/// This permit to reduce the verbosity created by Rc and RefCell.
#[derive(Clone)]
pub struct SharedBlackboard(Rc<RefCell<Blackboard>>);

impl SharedBlackboard {
    /// Create a new empty `SharedBlackboard`.
    pub fn new() -> SharedBlackboard {
        SharedBlackboard(Rc::new(RefCell::new(Blackboard::new())))
    }

    /// Insert a value in the blackboard with the given name.
    /// 
    /// This method panic if the name is already used.
    #[inline]
    pub fn insert<T: 'static>(&mut self, name: &str, entry: T) {
        self.0.borrow_mut().insert(name, entry);
    }

    /// Try to retrieve a value with the given name from the blackboard.
    ///
    /// Returns None if the value doesn't exist.
    #[inline]
    pub fn try_get<T: 'static>(&self, name: &str) -> Option<BlackboardEntry<T>> {
        self.0.borrow().try_get(name)
    }

    /// Retrieve a value with the given name from the blackboard.
    ///
    /// This method panic if the value doesn't exist.
    #[inline]
    pub fn get<T: 'static>(&self, name: &str) -> BlackboardEntry<T> {
        self.0.borrow().get(name)
    }
}