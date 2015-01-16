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
    /// If the entity was already tagged, the previous tag will be overriden.
    /// If the tag was already used, the previously tagged entity is returned.
    pub fn insert(&mut self, mentity: &mut MetaEntity, tag: &str) -> Option<Entity> {
        if let Some(previous_tag) = mem::replace(&mut mentity.tag, Some(tag.to_string())) {
            self.tags.remove(&previous_tag);
        }
        self.tags.insert(tag.to_string(), mentity.entity)
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
