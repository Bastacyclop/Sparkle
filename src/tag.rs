use std::collections::HashMap;
use entity::{Entity, MetaEntityMap};

pub struct Manager {
    tags_to_entity: HashMap<String, Entity>,
    entity_to_tag: HashMap<Entity, String>,
    mentities: MetaEntityMap
}

impl Manager {
    pub fn new(mentities: MetaEntityMap) -> Manager {
        Manager {
            tags_to_entity: HashMap::new(),
            entity_to_tag: HashMap::new(),
            mentities: mentities
        }
    }

    pub fn set(&mut self, tag: &str, entity: &Entity) {
        if !self.tags_to_entity.contains_key(tag) {
            self.tags_to_entity.insert(tag.to_string(), *entity);
            self.entity_to_tag.insert(*entity, tag.to_string());

            self.mentities.apply_to(entity, |mentity| {
                mentity.tag = Some(tag.to_string());
            });
        }
    }

    pub fn unset(&mut self, entity: &Entity) {
        if let Some(tag) = self.entity_to_tag.remove(entity) {
            self.tags_to_entity.remove(&tag);
        }

        self.mentities.apply_to(entity, |mentity| {
            mentity.tag = None;
        });
    }

    pub fn get_with_tag(&mut self, tag: &str) -> Option<Entity> {
        self.tags_to_entity.get(tag).map(|entity| *entity)
    }
}