//! Identification of entities using groups.
//!
//! A group has a name and can contain multiple entities.   
//! An entity can belong to multiple groups.

use std::collections::{HashMap, HashSet};

use entity::{Entity, MetaEntity};

type Group = HashSet<Entity>;

/// A `GroupMap` is keeping track of entity groups.
pub struct GroupMap {
    groups: HashMap<String, Group>
}

impl GroupMap {
    /// Creates an empty `GroupMap`.
    pub fn new() -> GroupMap {
        GroupMap {
            groups: HashMap::new()
        }
    }

    /// Inserts an entity into a group.
    ///
    /// Creates the group if necessary.
    pub fn insert_in(&mut self, mentity: &mut MetaEntity, name: &str) {
        mentity.groups.insert(name.to_string());
        let entity = mentity.entity;

        self.ensure(name);
        self.groups.get_mut(name).unwrap().insert(entity);
    }

    /// Ensures the group presence.
    fn ensure(&mut self, name: &str) {
        if !self.groups.contains_key(name) {
            let empty = HashSet::new();
            self.groups.insert(name.to_string(), empty);
        }
    }

    /// Removes an entity from a group.
    pub fn remove_from(&mut self, mentity: &mut MetaEntity, name: &str) {
        let entity = mentity.entity;

        self.groups.get_mut(name).map(|group| group.remove(&entity));
        mentity.groups.remove(name);
    }

    /// Clears an entity groups.
    pub fn clear_entity(&mut self, mentity: &mut MetaEntity) {
        let entity = mentity.entity;

        for (_name, group) in self.groups.iter_mut() {
            group.remove(&entity);
        }
        mentity.groups.clear();
    }

    /// Returns a group of entity as a vector.
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

    /// Forgets an entity, removing it from the `GroupMap`
    /// without touching the meta entity data.
    pub fn forget(group_map: &mut GroupMap, mentity: &MetaEntity) {
        for name in mentity.groups.iter() {
            group_map.groups.remove(name);
        }
    }
}
