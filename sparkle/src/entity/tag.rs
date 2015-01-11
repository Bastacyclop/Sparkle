use std::collections::HashMap;
use entity::{Entity, MetaEntity};

pub struct TagMap {
    tags: HashMap<String, Entity>
}

impl TagMap {
    pub fn new() -> TagMap {
        TagMap {
            tags: HashMap::new()
        }
    }

    pub fn insert(&mut self, tag: &str, mentity: &mut MetaEntity) {
        if !self.tags.contains_key(tag) {
            let entity = mentity.entity;

            self.tags.insert(tag.to_string(), entity);
            mentity.tag = Some(tag.to_string());
        }
    }

    pub fn remove(&mut self, mentity: &mut MetaEntity) {
        mentity.tag.take().map(|tag| self.tags.remove(&tag));
    }

    pub fn get(&self, tag: &str) -> Option<Entity> {
        self.tags.get(tag).map(|entity| *entity)
    }
}

#[doc(hidden)]
pub mod private {
    use super::TagMap;
    use entity::MetaEntity;

    pub fn forget(tag_map: &mut TagMap, mentity: &MetaEntity) {
        mentity.tag.as_ref().map(|tag| tag_map.tags.remove(tag));
    }
}
