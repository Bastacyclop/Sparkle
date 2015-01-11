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

    pub fn insert(&mut self, group_name: &str, mentity: &mut MetaEntity) {
        mentity.groups.insert(group_name.to_string());
        let entity = mentity.entity;

        if let Some(group) = self.groups.get_mut(group_name) {
            group.insert(entity);
            return;
        }
        self.insert_new_group_with(group_name, entity);
    }

    fn insert_new_group_with(&mut self, group_name: &str, entity: Entity) {
        let mut group = HashSet::new();
        group.insert(entity);

        self.groups.insert(group_name.to_string(), group);
    }

    pub fn remove_from(&mut self, group_name: &str, mentity: &mut MetaEntity) {
        let entity = mentity.entity;

        self.groups.get_mut(group_name).map(|group| group.remove(&entity));
        mentity.groups.remove(group_name);
    }

    pub fn clear_entity(&mut self, mentity: &mut MetaEntity) {
        let entity = mentity.entity;

        for (_name, group) in self.groups.iter_mut() {
            group.remove(&entity);
        }
        mentity.groups.clear();
    }

    pub fn get(&self, group_name: &str) -> Vec<Entity> {
        match self.groups.get(group_name) {
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
