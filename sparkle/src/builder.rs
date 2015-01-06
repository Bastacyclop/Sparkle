use std::collections::HashMap;
use entity::{self, Entity};
use group::GroupMap;
use tag::TagMap;

pub trait Builder: 'static {
    fn create_entity(&mut self, 
                     em: &mut entity::Manager, 
                     groups: &mut GroupMap, 
                     tags: &mut TagMap) -> Entity;
}

impl<F> Builder for F 
    where F: for<'a> FnMut<(&'a mut entity::Manager, 
                            &'a mut GroupMap, 
                            &'a mut TagMap), Entity> + 'static
{
    fn create_entity(&mut self, 
                     em: &mut entity::Manager, 
                     groups: &mut GroupMap, 
                     tags: &mut TagMap) -> Entity 
    {
        self(em, groups, tags)
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

    pub fn insert<T>(&mut self, name: &str, template: T) where T: Builder {
        self.builders.insert(name.to_string(), box template);
    }

    pub fn build_entity_with(&mut self,
                             name: &str, 
                             em: &mut entity::Manager, 
                             groups: &mut GroupMap, 
                             tags: &mut TagMap) -> Entity 
    {
        self.builders.get_mut(name).map(|builder| {
            builder.create_entity(em, groups, tags)
        }).expect(format!("No template with the name {} was found.", name)[])
    }
}