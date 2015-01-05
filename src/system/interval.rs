use std::collections::HashSet;
use entity::{self, Entity, MetaEntity};
use space::SpaceProxy;
use system::{Filter, System, Processor};

pub struct FramerateSystem<T> where T: Processor {
    filter: Filter,
    entities: HashSet<Entity>,
    processor: T
}

impl<T> FramerateSystem<T> where T: Processor {
    pub fn new(filter: Filter, processor: T) -> FramerateSystem<T> {
        FramerateSystem {
            filter: filter,
            entities: HashSet::new(),
            processor: processor
        }
    }
}

impl<T> System for FramerateSystem<T> where T: Processor {
    fn process(&mut self, space: &mut SpaceProxy) {
        self.processor.before_processing();
        self.processor.process(space, &self.entities);
        self.processor.after_processing();
    }
}

impl<T> entity::Observer for FramerateSystem<T> where T: Processor {
    fn on_created(&mut self, mentity: &MetaEntity) {
        if self.filter.check(mentity) {
            self.entities.insert(mentity.entity);
        }
    }

    fn on_removed(&mut self, mentity: &MetaEntity) {
        self.entities.remove(&mentity.entity);
    }

    fn on_changed(&mut self, mentity: &MetaEntity) {
        let contains = self.entities.contains(&mentity.entity);
        let pass_filter = self.filter.check(mentity);
        
        match (contains, pass_filter) {
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