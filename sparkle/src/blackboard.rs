//! The blackboard related structures.

use std::collections::HashMap;
use std::collections::hash_map::Entry as HashMapEntry;
use std::rc::Rc;
use std::cell::RefCell;
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

    /// Tries to retrieve an entry from the blackboard with the given name.
    ///
    /// Returns `None` if the entry doesn't exist.
    pub fn try_get<T: 'static>(&self, name: &str) -> Option<BlackboardEntry<T>> {
        self.entries.get(name).and_then(|any_entry| any_entry.downcast_ref::<BlackboardEntry<T>>())
                              .map(|entry| entry.clone())
    }

    /// Retrieves an entry from the blackboard with the given name.
    ///
    /// This method panics if the entry doesn't exist.
    pub fn get<T: 'static>(&self, name: &str) -> BlackboardEntry<T> {
        let any_entry = self.entries.get(name).expect(format!("missing entry {}.", name).as_slice());
        let entry = any_entry.downcast_ref::<BlackboardEntry<T>>().expect("invalid entry type.");

        entry.clone()
    }
}

/// A simple wrapper around a `Blackboard`, allocated with Rc. 
///
/// This permits to reduce the verbosity created by Rc and RefCell.
#[derive(Clone)]
pub struct SharedBlackboard(Rc<RefCell<Blackboard>>);

impl SharedBlackboard {
    /// Creates an empty `SharedBlackboard`.
    pub fn new() -> SharedBlackboard {
        SharedBlackboard(Rc::new(RefCell::new(Blackboard::new())))
    }

    /// Behaves like [the original](struct.Blackboard.html#method.insert).
    #[inline]
    pub fn insert<T: 'static>(&mut self, name: &str, entry: T) {
        self.0.borrow_mut().insert(name, entry);
    }

    /// Behaves like [the original](struct.Blackboard.html#method.try_get).
    #[inline]
    pub fn try_get<T: 'static>(&self, name: &str) -> Option<BlackboardEntry<T>> {
        self.0.borrow().try_get(name)
    }

    /// Behaves like [the original](struct.Blackboard.html#method.get).
    #[inline]
    pub fn get<T: 'static>(&self, name: &str) -> BlackboardEntry<T> {
        self.0.borrow().get(name)
    }
}
