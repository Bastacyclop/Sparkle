use std::collections::HashMap;
use entity::{Entity, MetaEntity};
use entity::group::GroupMap;
use entity::tag::TagMap;

pub trait Builder: 'static {
    fn create_entity(&mut self, 
                     mentity: &mut MetaEntity, 
                     groups: &mut GroupMap, 
                     tags: &mut TagMap) -> Entity;
}

impl<F> Builder for F 
    where F: for<'a> FnMut<(&'a mut MetaEntity, 
                            &'a mut GroupMap, 
                            &'a mut TagMap), Entity> + 'static
{
    fn create_entity(&mut self,
                     mentity: &mut MetaEntity, 
                     groups: &mut GroupMap, 
                     tags: &mut TagMap) -> Entity 
    {
        self(mentity, groups, tags)
    }
}

pub struct BuilderMap {
    builders: HashMap<String, Box<Builder>>
}

impl BuilderMap {
    pub fn new() -> BuilderMap {
        BuilderMap {
            builders: HashMap::new()
        }
    }

    pub fn insert<T>(&mut self, name: &str, builder: T) where T: Builder {
        self.builders.insert(name.to_string(), Box::new(builder));
    }

    pub fn get_builder_mut(&mut self, name: &str) -> Option<&mut Box<Builder>> {
        self.builders.get_mut(name)
    }
}