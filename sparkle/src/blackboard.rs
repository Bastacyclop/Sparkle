use std::collections::HashMap;
use std::collections::hash_map::Entry as HashMapEntry;
use std::rc::Rc;
use std::cell::RefCell;
use std::any::Any;

pub type Entry<T: 'static> = Rc<RefCell<T>>;
pub type SharedBlackboard = Rc<RefCell<Blackboard>>;

pub struct Blackboard {
    entries: HashMap<String, Box<Any>>
}

impl Blackboard {
    pub fn new() -> Blackboard {
        Blackboard {
            entries: HashMap::new()
        }
    }

    pub fn new_shared() -> SharedBlackboard {
        Rc::new(RefCell::new(Blackboard::new()))
    }

    pub fn insert<T: 'static>(&mut self, name: &str, entry: T) {
        match self.entries.entry(name.to_string()) {
            HashMapEntry::Vacant(vacant) => {
                vacant.insert(Box::new(Rc::new(RefCell::new(entry))));
            },
            HashMapEntry::Occupied(_) => panic!("The name {} is already used", name)
        }
    }

    pub fn try_get<T: 'static>(&self, name: &str) -> Option<Entry<T>> {
        self.entries.get(name).and_then(|any_entry| any_entry.downcast_ref::<Entry<T>>())
                              .map(|entry| entry.clone())
    }

    pub fn get<T: 'static>(&self, name: &str) -> Entry<T> {
        let any_entry = self.entries.get(name).expect(format!("missing entry {}.", name).as_slice());
        let entry = any_entry.downcast_ref::<Entry<T>>().expect("invalid entry type.");
        entry.clone()
    }
}