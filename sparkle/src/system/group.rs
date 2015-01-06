use std::collections::HashSet;
use entity::{self, Entity, MetaEntity};
use space::SpaceProxy;
use system::{System, Processor};

pub struct FixedStepSystem<T> where T: Processor {
    group: String,
    entities: HashSet<Entity>,
    processor: T
}

impl<T> FixedStepSystem<T> where T: Processor {
    pub fn new(group: &str, processor: T) -> FixedStepSystem<T> {
        FixedStepSystem {
            group: group.to_string(),
            entities: HashSet::new(),
            processor: processor
        }
    }
}

impl<T> System for FixedStepSystem<T> where T: Processor {
    fn process(&mut self, space: &mut SpaceProxy, dt: f32) {
        self.processor.before_processing();
        self.processor.process(space, &self.entities, dt);
        self.processor.after_processing();
    }
}



impl<T> entity::Observer for FixedStepSystem<T> where T: Processor {
    fn on_created(&mut self, mentity: &MetaEntity) {
        if mentity.groups.contains(self.group[]) {
            self.entities.insert(mentity.entity);
        }
    }

    fn on_removed(&mut self, mentity: &MetaEntity) {
        self.entities.remove(&mentity.entity);
    }

    fn on_changed(&mut self, mentity: &MetaEntity) {
        let contains = self.entities.contains(&mentity.entity);
        let has_group = mentity.groups.contains(self.group[]);
        
        match (contains, has_group) {
            (true, false) => { 
                self.entities.remove(&mentity.entity);
            },
            (false, true) => {
                self.entities.insert(mentity.entity);
            },
            _ => {}
        }
    }
}