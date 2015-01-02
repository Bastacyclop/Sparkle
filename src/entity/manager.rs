use std::collections::{VecMap, RingBuf};
use component::{Component, ComponentType, StoreMap};
use entity::{TagMap, GroupMap, Pool, Entity, MetaEntity, Update, Observer, Record};

struct Entities {
    pub pool: Pool,
    pub actives: VecMap<MetaEntity>,
    pub removed: RingBuf<Entity>,
    pub groups: GroupMap,
    pub tags: TagMap
}

impl Entities {
    pub fn new() -> Entities {
        Entities {
            pool: Pool::new(),
            actives: VecMap::new(),
            removed: RingBuf::new(),
            groups: GroupMap::new(),
            tags: TagMap::new()
        }
    }
}

pub struct Manager {
    entities: Entities,
    components: StoreMap,
    updates_record: Record
}

impl Manager{
    pub fn new() -> Manager {
        Manager {
            entities: Entities::new(),
            components: StoreMap::new(),
            updates_record: Record::new()
        }
    }

    pub fn create(&mut self) -> Entity {
        let meta_entity = self.entities.pool.get();
        let entity = meta_entity.entity;

        self.entities.actives.insert(entity, meta_entity);
        self.updates_record.add(Update::new_created(entity));
        entity
    }

    pub fn remove(&mut self, entity: &Entity) {
        self.entities.removed.push_back(*entity);
        self.updates_record.add(Update::new_removed(*entity));
    }

    pub fn attach_component<T>(&mut self, entity: &Entity, component: T) where T: Component {
        let type_index = ComponentType::get_index_of::<T>();

        self.components.attach_component(entity, component);
        self.entities.actives.get_mut(entity).map(|mentity| mentity.component_bits.insert(type_index));
        self.updates_record.add(Update::new_changed(*entity));
    }

    pub fn detach_component<T>(&mut self, entity: &Entity) where T: Component {
        let type_index = ComponentType::get_index_of::<T>();

        self.components.detach_component::<T>(entity);
        self.entities.actives.get_mut(entity).map(|mentity| mentity.component_bits.remove(&type_index));
        self.updates_record.add(Update::new_changed(*entity));
    }

    #[inline]
    pub fn get_component<T>(&self, entity: &Entity) -> Option<&T> where T: Component {
        self.components.get_component::<T>(entity)
    }

    #[inline]
    pub fn get_mut_component<T>(&mut self, entity: &Entity) -> Option<&mut T> where T: Component {
        self.components.get_mut_component::<T>(entity)
    }

    pub fn set_group(&mut self, group: &str, entity: &Entity) {
        self.entities.actives.get_mut(entity).map(|mentity| mentity.groups.insert(group.to_string()));
        self.entities.groups.insert(group, entity);

        self.updates_record.add(Update::new_changed(*entity));
    }

    pub fn unset_group(&mut self, group: &str, entity: &Entity) {
        self.entities.groups.remove_from(group, entity);

        self.updates_record.add(Update::new_changed(*entity));
    }

    pub fn get_from_group(&mut self, group: &str) -> Vec<Entity> {
        self.entities.groups.get(group)
    }

    pub fn set_tag(&mut self, tag: &str, entity: &Entity) {
        self.entities.tags.set_tag(tag, entity);
    }

    pub fn unset_tag(&mut self, entity: &Entity) {
        self.entities.tags.unset_tag(entity);
    }

    pub fn get_with_tag(&mut self, tag: &str) -> Option<Entity> {
        self.entities.tags.get_with_tag(tag)
    }

    pub fn notify_observer_and_flush<T>(&mut self, observer: &mut T) where T: Observer {
        self.updates_record.notify_and_flush(&self.entities.actives, observer);
    }

    pub fn flush_removed(&mut self) {
        while let Some(removed) = self.entities.removed.pop_back() {
            self.entities.actives.remove(&removed).map(|mentity| self.entities.pool.put(mentity));
            self.entities.groups.clear_entity(&removed);
            self.entities.tags.unset_tag(&removed);
            self.components.detach_components(&removed);
        }
    }
}