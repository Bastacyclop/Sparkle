use std::collections::HashMap;
use entity::Entity;

pub struct TagMap {
    tags_to_entity: HashMap<String, Entity>,
    entity_to_tag: HashMap<Entity, String>
}

impl TagMap {
    pub fn new() -> TagMap {
        TagMap {
            tags_to_entity: HashMap::new(),
            entity_to_tag: HashMap::new()
        }
    }

    pub fn set_tag(&mut self, tag: &str, entity: &Entity) {
        if !self.tags_to_entity.contains_key(tag) {
            self.tags_to_entity.insert(tag.to_string(), *entity);
            self.entity_to_tag.insert(*entity, tag.to_string());
        }
    }

    pub fn unset_tag(&mut self, entity: &Entity) {
        if let Some(tag) = self.entity_to_tag.remove(entity) {
            self.tags_to_entity.remove(&tag);
        }
    }

    pub fn get_with_tag(&mut self, tag: &str) -> Option<Entity> {
        self.tags_to_entity.get(tag).map(|entity| *entity)
    }
}