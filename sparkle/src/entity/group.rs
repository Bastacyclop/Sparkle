use std::collections::{HashMap, HashSet};
use entity::{Entity, MetaEntity};

pub type Group = HashSet<Entity>;

pub struct GroupMap {
    groups: HashMap<String, Group>
}

impl GroupMap {
    pub fn new() -> GroupMap {
        GroupMap {
            groups: HashMap::new()
        }
    }

    pub fn insert_in(&mut self, mentity: &mut MetaEntity, name: &str) {
        mentity.groups.insert(name.to_string());
        let entity = mentity.entity;

        self.ensure(name);
        self.groups.get_mut(name).unwrap().insert(entity);
    }

    fn ensure(&mut self, name: &str) {
        if !self.groups.contains_key(name) {
            let empty = HashSet::new();
            self.groups.insert(name.to_string(), empty);
        }
    }

    pub fn remove_from(&mut self, mentity: &mut MetaEntity, name: &str) {
        let entity = mentity.entity;

        self.groups.get_mut(name).map(|group| group.remove(&entity));
        mentity.groups.remove(name);
    }

    pub fn clear_entity(&mut self, mentity: &mut MetaEntity) {
        let entity = mentity.entity;

        for (_name, group) in self.groups.iter_mut() {
            group.remove(&entity);
        }
        mentity.groups.clear();
    }

    pub fn get(&self, name: &str) -> Vec<Entity> {
        match self.groups.get(name) {
            Some(group) => group.iter().map(|entity| *entity).collect(),
            None => Vec::new()
        }
    }
}

#[doc(hidden)]
pub mod private {
    use super::GroupMap;
    use entity::MetaEntity;

    pub fn forget(group_map: &mut GroupMap, mentity: &MetaEntity) {
        for name in mentity.groups.iter() {
            group_map.groups.remove(name);
        }
    }
}
