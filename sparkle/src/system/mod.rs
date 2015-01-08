use std::collections::HashSet;
use entity::{self, Entity, MetaEntity};

pub use self::manager::Manager;
pub use self::filter::Filter;

pub mod manager;
pub mod filter;

pub trait System: 'static {
    fn update(&mut self, em: &mut entity::Manager, dt: f32);

    fn on_entity_created(&mut self, _mentity: &MetaEntity) {}
    fn on_entity_changed(&mut self, _mentity: &MetaEntity) {}
    fn on_entity_removed(&mut self, _mentity: &MetaEntity) {}
}

pub trait Processor: 'static {
    fn update(&mut self, em: &mut entity::Manager, entities: &HashSet<Entity>, dt: f32);

    fn on_entity_added(&mut self, _mentity: &MetaEntity) {}
    fn on_entity_removed(&mut self, _mentity: &MetaEntity) {}
}

impl<F> Processor for F
    where F: for<'a> FnMut<(&'a mut entity::Manager, &'a HashSet<Entity>, f32), ()> + 'static
{
    fn update(&mut self, em: &mut entity::Manager, entities: &HashSet<Entity>, dt: f32) {
        self.call_mut((em, entities, dt));
    }
}

macro_rules! add_entity {
    ($system:expr, $mentity:expr) => ({
        $system.entities.insert($mentity.entity);
        $system.processor.on_entity_added($mentity);
    })
}

macro_rules! remove_entity {
    ($system:expr, $mentity:expr) => ({
        $system.entities.remove(&$mentity.entity);
        $system.processor.on_entity_removed($mentity);
    })
}

macro_rules! impl_on_entity_created {
    () => (
        fn on_entity_created(&mut self, mentity: &MetaEntity) {
            if self.filter.pass(mentity) {
                add_entity!(self, mentity);
            }
        }
    )
}

macro_rules! impl_on_entity_changed {
    () => (
        fn on_entity_changed(&mut self, mentity: &MetaEntity) {
            let contains = self.entities.contains(&mentity.entity);
            let pass_filter = self.filter.pass(mentity);

            match (contains, pass_filter) {
                (true, false) => remove_entity!(self, mentity),
                (false, true) => add_entity!(self, mentity),
                _ => {}
            }
        }
    )
}

macro_rules! impl_on_entity_removed {
    () => (
        fn on_entity_removed(&mut self, mentity: &MetaEntity) {
            remove_entity!(self, mentity);
        }
    )
}

macro_rules! impl_on_entity_ {
    () => (
        impl_on_entity_created!();
        impl_on_entity_changed!();
        impl_on_entity_removed!();
    )
}

pub struct Framerate<P> where P: Processor {
    processor: P,
    filter: Filter,
    entities: HashSet<Entity>
}

impl<P> Framerate<P> where P: Processor {
    pub fn new(filter: Filter, processor: P) -> Framerate<P> {
        Framerate {
            processor: processor,
            filter: filter,
            entities: HashSet::new()
        }
    }
}

impl<P> System for Framerate<P> where P: Processor {
    fn update(&mut self, em: &mut entity::Manager, dt: f32) {
        self.processor.update(em, &self.entities, dt);
    }

    impl_on_entity_!();
}

pub struct FixedRate<P> where P: Processor {
    processor: P,
    filter: Filter,
    entities: HashSet<Entity>,
    rate: f32,
    accumulator: f32
}

impl<P> FixedRate<P> where P: Processor {
    pub fn new(filter: Filter, rate: f32, processor: P) -> FixedRate<P> {
        FixedRate {
            processor: processor,
            filter: filter,
            entities: HashSet::new(),
            rate: rate,
            accumulator: 0.
        }
    }
}

impl<P> System for FixedRate<P> where P: Processor {
    fn update(&mut self, em: &mut entity::Manager, dt: f32) {
        self.accumulator += dt;
        while self.accumulator >= self.rate {
            self.processor.update(em, &self.entities, dt);

            self.accumulator -= self.rate;
        }
    }

    impl_on_entity_!();
}
