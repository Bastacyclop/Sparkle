//! Identification of entities using tags.
//!
//! A tag has a name and can only be applied to one entity at a time.  
//! An entity can only have one tag.

use std::mem;
use std::collections::HashMap;

use entity::{Entity, MetaEntity};

/// A `TagMap` is keeping track of entity tags.
pub struct TagMap {
    tags: HashMap<String, Entity>
}

impl TagMap {
    /// Creates an empty `TagMap`.
    pub fn new() -> TagMap {
        TagMap {
            tags: HashMap::new()
        }
    }

    /// Inserts an entity tag.
    ///
    /// If the entity was already tagged, the previous tag will be overriden and returned.
    /// Panics if the tag was already used.
    pub fn insert(&mut self, mentity: &mut MetaEntity, tag: &str) -> Option<String> {
        let previous_tag = mem::replace(&mut mentity.tag, Some(tag.to_string()));
        previous_tag.as_ref().map(|t| self.tags.remove(t));
        
        if let Some(already_tagged) = self.tags.insert(tag.to_string(), mentity.entity) {
            panic!("the tag '{}' was already tagging entity {}", tag, already_tagged);
        }
        
        previous_tag
    }

    /// Removes the tag of an entity.
    pub fn remove(&mut self, mentity: &mut MetaEntity) {
        mentity.tag.take().map(|tag| self.tags.remove(&tag));
    }

    /// Returns the entity tagged by `tag` if it exists.
    pub fn get(&self, tag: &str) -> Option<Entity> {
        self.tags.get(tag).map(|entity| *entity)
    }
}

#[doc(hidden)]
pub mod private {
    use super::TagMap;
    use entity::MetaEntity;

    /// Forgets an entity, removing it from the `TagMap`
    /// without touching the meta entity data.
    pub fn forget(tag_map: &mut TagMap, mentity: &MetaEntity) {
        mentity.tag.as_ref().map(|tag| tag_map.tags.remove(tag));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use MetaEntity;
    
    #[test]
    fn regular_insertion() {
        let mut tag_map = TagMap::new();
        let orc = &mut MetaEntity::new(0);
        let tag = "tag";
        
        tag_map.insert(orc, tag);
        assert_eq!(tag_map.tags.get(tag), Some(&orc.entity));
        assert_eq!(orc.tag.as_ref().map(|t| t.as_slice()), Some(tag));
    }
    
    #[test]
    #[should_fail]
    fn double_tag_insertion() {
        let mut tag_map = TagMap::new();
        let orc = &mut MetaEntity::new(0);
        let troll = &mut MetaEntity::new(1);
        
        let tag = "tag";
        tag_map.insert(orc, tag);
        tag_map.insert(troll, tag);
    }
    
    #[test]
    fn double_entity_insertion() {
        let mut tag_map = TagMap::new();
        let orc = &mut MetaEntity::new(0);
        let tag = "tag";
        let new_tag = "gat";
        
        tag_map.insert(orc, tag);
        tag_map.insert(orc, new_tag);
        assert_eq!(tag_map.tags.get(new_tag), Some(&orc.entity));
        assert_eq!(orc.tag.as_ref().map(|t| t.as_slice()), Some(new_tag));
        assert!(tag_map.tags.get(tag).is_none());
    }
    
    #[test]
    fn get_some() {
        let mut tag_map = TagMap::new();
        let orc = &mut MetaEntity::new(0);
        let tag = "tag";
        
        tag_map.insert(orc, tag);
        assert_eq!(tag_map.get(tag), Some(orc.entity));
    }
    
    #[test]
    fn get_none() {
        let tag_map = TagMap::new();
        let tag = "tag";
        
        assert_eq!(tag_map.get(tag), None);
    }
    
    #[test]
    fn regular_removal() {
        let mut tag_map = TagMap::new();
        let orc = &mut MetaEntity::new(0);
        let tag = "tag";
        
        tag_map.insert(orc, tag);
        tag_map.remove(orc);
        assert_eq!(tag_map.tags.get(tag), None);
        assert_eq!(orc.tag, None);
    }
    
    #[test]
    fn pointless_removal() {
        let mut tag_map = TagMap::new();
        let orc = &mut MetaEntity::new(0);
        
        tag_map.remove(orc);
        assert_eq!(orc.tag, None);
    }
    
    #[test]
    fn regular_forgetting() {
        let mut tag_map = &mut TagMap::new();
        let orc = &mut MetaEntity::new(0);
        let tag = "tag";
        
        tag_map.insert(orc, tag);
        private::forget(tag_map, orc);
        assert_eq!(tag_map.tags.get(tag), None);
        assert_eq!(orc.tag.as_ref().map(|t| t.as_slice()), Some(tag));
    }
    
    #[test]
    fn pointless_forgetting() {
        let mut tag_map = &mut TagMap::new();
        let orc = &mut MetaEntity::new(0);
        
        private::forget(tag_map, orc);
        assert_eq!(orc.tag, None);
    }
}
