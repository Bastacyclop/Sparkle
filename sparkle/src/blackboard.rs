use std::collections::HashMap;
use std::collections::hash_map::Entry as HashMapEntry;
use std::rc::Rc;
use std::cell::RefCell;
use std::any::Any;

pub type Entry<T: 'static> = Rc<RefCell<T>>;

pub struct Blackboard {
    entries: HashMap<String, Box<Any>>
}

impl Blackboard {
    pub fn new() -> Blackboard {
        Blackboard {
            entries: HashMap::new()
        }
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

#[derive(Clone)]
pub struct SharedBlackboard(Rc<RefCell<Blackboard>>);

impl SharedBlackboard {
    pub fn new() -> SharedBlackboard {
        SharedBlackboard(Rc::new(RefCell::new(Blackboard::new())))
    }

    #[inline]
    pub fn insert<T: 'static>(&mut self, name: &str, entry: T) {
        self.0.borrow_mut().insert(name, entry);
    }

    #[inline]
    pub fn try_get<T: 'static>(&self, name: &str) -> Option<Entry<T>> {
        self.0.borrow().try_get(name)
    }

    #[inline]
    pub fn get<T: 'static>(&self, name: &str) -> Entry<T> {
        self.0.borrow().get(name)
    }
}