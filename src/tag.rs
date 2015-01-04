use std::collections::HashMap;
use entity::{Entity, MetaEntity};

pub struct Manager {
    tags_to_entity: HashMap<String, Entity>,
    entity_to_tag: HashMap<Entity, String>
}

impl Manager {
    pub fn new() -> Manager {
        Manager {
            tags_to_entity: HashMap::new(),
            entity_to_tag: HashMap::new(),
        }
    }

    pub fn set(&mut self, tag: &str, mentity: &mut MetaEntity) {
        if !self.tags_to_entity.contains_key(tag) {
            let entity = mentity.entity;

            self.tags_to_entity.insert(tag.to_string(), entity);
            self.entity_to_tag.insert(entity, tag.to_string());
            mentity.tag = Some(tag.to_string());
        }
    }

    pub fn unset(&mut self, mentity: &mut MetaEntity) {
        let entity = mentity.entity;

        if let Some(tag) = self.entity_to_tag.remove(&entity) {
            self.tags_to_entity.remove(&tag);
            mentity.tag = None;
        }
    }

    pub fn get_with_tag(&mut self, tag: &str) -> Option<Entity> {
        self.tags_to_entity.get(tag).map(|entity| *entity)
    }
}