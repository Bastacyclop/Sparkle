use entity::{self, MetaEntity};
use space::SpaceProxy;
use system::System;

pub struct Manager {
    systems: Vec<Box<System>> 
}

impl Manager {
    pub fn new() -> Manager {
        Manager {
            systems: Vec::new()
        }
    }

    pub fn push<T>(&mut self, system: T) where T: System {
        self.systems.push(box system);
    }

    pub fn process_all<'a>(&mut self, space: &mut SpaceProxy<'a>, dt: f32) {
        for system in self.systems.iter_mut() {
            system.process(space, dt);
        }
    }
}

impl entity::Observer for Manager {
    fn on_created(&mut self, mentity: &MetaEntity) {
        for system in self.systems.iter_mut() {
            system.on_created(mentity);
        }
    }

    fn on_removed(&mut self, mentity: &MetaEntity) {
        for system in self.systems.iter_mut() {
            system.on_removed(mentity);
        }
    }

    fn on_changed(&mut self, mentity: &MetaEntity) {
        for system in self.systems.iter_mut() {
            system.on_changed(mentity);
        }
    }
}