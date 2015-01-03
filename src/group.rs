use std::collections::{HashMap, HashSet};
use entity::{Entity, MetaEntityMap};

pub type Group = HashSet<Entity>;

pub struct GroupManager {
    mentities: MetaEntityMap,
    groups: HashMap<String, Group>
}
    
impl GroupManager {
    pub fn new(mentities: MetaEntityMap) -> GroupManager {
        GroupManager {
            mentities: mentities,
            groups: HashMap::new()
        }
    }

    pub fn insert(&mut self, group_name: &str, entity: &Entity) {
        if let Some(group) = self.groups.get_mut(group_name) {
            group.insert(*entity);
            return;
        }
        self.insert_new_group_with(group_name, *entity); 
    }

    fn insert_new_group_with(&mut self, group_name: &str, entity: Entity) {
        let mut group = HashSet::new();
        group.insert(entity);

        self.groups.insert(group_name.to_string(), group);
    }

    pub fn remove(&mut self, group_name: &str) {
        self.groups.remove(group_name);
    }

    pub fn remove_from(&mut self, group_name: &str, entity: &Entity) {
        self.groups.get_mut(group_name).map(|group| group.remove(entity));
    }

    pub fn clear_entity(&mut self, entity: &Entity) {
        for (_name, group) in self.groups.iter_mut() {
            group.remove(entity);
        }
    }

    pub fn get(&self, group_name: &str) -> Vec<Entity> {
        match self.groups.get(group_name) {
            Some(group) => group.iter().map(|entity| *entity).collect(),
            None => Vec::new()
        }
    }
}