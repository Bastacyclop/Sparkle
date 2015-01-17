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
            let group = group_map.groups.get_mut(name)
                                        .expect(format!("Failed to forget {}", name).as_slice());

            group.remove(&mentity.entity);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::GroupMap;
    use entity::MetaEntity;

    #[test]
    fn insert_in() {
        let mut group_map = GroupMap::new();
        let group_name = "aGroup";
        let mentity1 = &mut MetaEntity::new(0);
        let mentity2 = &mut MetaEntity::new(1);

        group_map.insert_in(mentity1, group_name);
        group_map.insert_in(mentity2, group_name);

        assert_eq!(1, mentity1.groups.len());
        assert_eq!(1, mentity2.groups.len());

        let group_string = group_name.to_string();
        let expected = Some(&group_string);
        assert_eq!(expected, mentity1.groups.iter().next());
        assert_eq!(expected, mentity2.groups.iter().next());

        let group = group_map.groups.get(group_name).unwrap();
        assert_eq!(2, group.len());
    }

    #[test]
    fn ensure() {
        let mut group_map = GroupMap::new();
        let group_name = "aGroup";

        group_map.ensure(group_name);

        assert_eq!(1, group_map.groups.len());
    }

    #[test]
    fn remove_from() {
        let mut group_map = GroupMap::new();
        let group_name = "aGroup";
        let mentity = &mut MetaEntity::new(0);

        group_map.insert_in(mentity, group_name);
        group_map.remove_from(mentity, group_name);

        assert_eq!(0, mentity.groups.len());

        let group = group_map.groups.get(group_name).unwrap();
        assert_eq!(0, group.len());
    }

    #[test]
    fn clear_entity() {
        let mut group_map = GroupMap::new();
        let group_name1 = "aGroup1";    
        let group_name2 = "aGroup2";
        let mentity = &mut MetaEntity::new(0);

        group_map.insert_in(mentity, group_name1);
        group_map.insert_in(mentity, group_name2);
        group_map.clear_entity(mentity);

        assert_eq!(0, mentity.groups.len());

        let group1 = group_map.groups.get(group_name1).unwrap();
        assert_eq!(0, group1.len());

        let group2 = group_map.groups.get(group_name2).unwrap();
        assert_eq!(0, group2.len());
    }

    #[test]
    fn get() {
        let mut group_map = GroupMap::new();
        let group_name = "aGroup";
        let mentity = &mut MetaEntity::new(0);  
    
        group_map.insert_in(mentity, group_name); 
    
        let entities = group_map.get(group_name);
        let expected = vec!(mentity.entity);

        assert_eq!(expected, entities);
    }

    #[test]
    fn forget() {
        let mut group_map = GroupMap::new();
        let group_name1 = "aGroup1";    
        let group_name2 = "aGroup2";
        let mentity = &mut MetaEntity::new(0);

        group_map.insert_in(mentity, group_name1);
        group_map.insert_in(mentity, group_name2);
        super::private::forget(&mut group_map, mentity);

        let group1 = group_map.groups.get(group_name1).unwrap();
        assert_eq!(0, group1.len());

        let group2 = group_map.groups.get(group_name2).unwrap();
        assert_eq!(0, group2.len());
    }
}