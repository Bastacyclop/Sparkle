use component;
use entity::{Entity, MetaEntity, MetaEntityMap, GroupMap, TagMap};
use entity::event;

pub struct Mapper {
    mentities: MetaEntityMap,
    groups: GroupMap,
    tags: TagMap
}

impl Mapper {
    pub fn new() -> Mapper {
        Mapper {
            mentities: MetaEntityMap::new(),
            groups: GroupMap::new(),
            tags: TagMap::new()
        }
    }

    pub fn get_mentity(&self, entity: Entity) -> &MetaEntity {
        self.mentities.get(entity)
    }

    pub fn get_mentity_mut(&mut self, entity: Entity) -> &mut MetaEntity {
        self.mentities.get_mut(entity)
    }

    pub fn set_group(&mut self, entity: Entity, group: &str) {
        self.groups.insert_in(self.mentities.get_mut(entity), group);
    }

    pub fn unset_group(&mut self,  entity: Entity, group: &str) {
        self.groups.remove_from(self.mentities.get_mut(entity), group);
    }

    pub fn get_group(&mut self, group: &str) -> Vec<Entity> {
        self.groups.get(group)
    }

    pub fn set_tag(&mut self, tag: &str, entity: Entity) {
        self.tags.insert(self.mentities.get_mut(entity), tag);
    }

    pub fn unset_tag(&mut self, entity: Entity) {
        self.tags.remove(self.mentities.get_mut(entity))
    }

    pub fn get_tag(&self, tag: &str) -> Option<Entity> {
        self.tags.get(tag)
    }

    pub fn notify_events<O>(&mut self, cm: &mut component::Mapper, obs: &mut O) where O: event::Observer {
        let Mapper { ref mut mentities, .. } = *self;

        mentities.drain_events_with(|(kind, mentity)| {
            match kind {
                event::Changed => obs.notify_changed(mentity),
                event::Removed => {
                    obs.notify_removed(mentity);
                    component::mapper::private::forget(cm, mentity);
                }
            }
        });
    }
}
