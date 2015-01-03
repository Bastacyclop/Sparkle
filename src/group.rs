use std::collections::{HashMap, HashSet};
use entity::{Entity, MetaEntityMap};

pub type Group = HashSet<Entity>;

pub struct Manager {
    mentities: MetaEntityMap,
    groups: HashMap<String, Group>
}
    
impl Manager {
    pub fn new(mentities: MetaEntityMap) -> Manager {
        Manager {
            mentities: mentities,
            groups: HashMap::new()
        }
    }

    pub fn set_group(&mut self, group_name: &str, entity: &Entity) {
        if let Some(group) = self.groups.get_mut(group_name) {
            group.insert(*entity);
            return;
        }
        self.insert_new_group_with(group_name, *entity); 

        self.mentities.apply_to(entity, |mentity| { 
            mentity.groups.insert(group_name.to_string()); 
        });
    }

    fn insert_new_group_with(&mut self, group_name: &str, entity: Entity) {
        let mut group = HashSet::new();
        group.insert(entity);

        self.groups.insert(group_name.to_string(), group);
    }

    pub fn remove(&mut self, group_name: &str) {
        self.groups.remove(group_name).map(|group| {
            for entity in group.into_iter() {
                self.mentities.apply_to(&entity, |mentity| {
                    mentity.groups.remove(group_name);
                });
            }
        });
    }

    pub fn remove_from(&mut self, group_name: &str, entity: &Entity) {
        self.groups.get_mut(group_name).map(|group| group.remove(entity));

        self.mentities.apply_to(entity, |mentity| {
            mentity.groups.remove(group_name);
        });
    }

    pub fn clear_entity(&mut self, entity: &Entity) {
        for (_name, group) in self.groups.iter_mut() {
            group.remove(entity);
        }

        self.mentities.apply_to(entity, |mentity| {
            mentity.groups.clear();
        })
    }

    pub fn get(&self, group_name: &str) -> Vec<Entity> {
        match self.groups.get(group_name) {
            Some(group) => group.iter().map(|entity| *entity).collect(),
            None => Vec::new()
        }
    }
}