use std::collections::HashMap;
use entity::{self, MetaEntity};
use space::SpaceProxy;
use system::System;

pub struct Manager {
    systems: HashMap<String, Box<System>> 
}

impl Manager {
    pub fn new() -> Manager {
        Manager {
            systems: HashMap::new()
        }
    }

    pub fn insert_system<T>(&mut self, name: &str, system: T) where T: System {
        self.systems.insert(name.to_string(), box system);
    }

    pub fn remove_system(&mut self, name: &str) {
        self.systems.remove(name);
    }

    pub fn process_systems<'a>(&mut self, space: &mut SpaceProxy<'a>, dt: f32) {
        for (_name, system) in self.systems.iter_mut() {
            system.process(space, dt);
        }
    }
}

impl entity::Observer for Manager {
    fn on_created(&mut self, mentity: &MetaEntity) {
        for (_name, system) in self.systems.iter_mut() {
            system.on_created(mentity);
        }
    }

    fn on_removed(&mut self, mentity: &MetaEntity) {
        for (_name, system) in self.systems.iter_mut() {
            system.on_removed(mentity);
        }
    }

    fn on_changed(&mut self, mentity: &MetaEntity) {
        for (_name, system) in self.systems.iter_mut() {
            system.on_changed(mentity);
        }
    }
}