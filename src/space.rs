use component::{Component, ComponentIndex};
use entity::{Entity, Event};
use entity::Queue as EventQueue;
use entity::Manager as EntityManager;
use group::Manager as GroupManager;
use tag::Manager as TagManager;

pub struct Space {
    entities: EntityManager,
    groups: GroupManager,
    tags: TagManager
}

impl Space {
    pub fn new() -> Space {
        let entities = EntityManager::new();
        let mentities = entities.get_meta_entities();

        Space {
            entities: entities,
            groups: GroupManager::new(mentities.clone()),
            tags: TagManager::new(mentities.clone())
        }
    }

    pub fn get_proxy<'a>(&'a mut self) -> SpaceProxy<'a> {
        SpaceProxy {
            events: EventQueue::new(),
            entities: &mut self.entities,
            groups: &mut self.groups,
            tags: &mut self.tags
        }
    }
}

pub struct SpaceProxy<'a> {
    events: EventQueue,
    entities: &'a mut EntityManager,
    groups: &'a mut GroupManager,
    tags: &'a mut TagManager
}

impl<'a> SpaceProxy<'a> {
    pub fn create_entity(&mut self) -> Entity {
        let entity = self.entities.create();        
        self.events.add(Event::new_created(entity));

        entity
    }

    pub fn remove_entity(&mut self, entity: &Entity) {
        self.entities.remove(entity);
        self.events.add(Event::new_removed(*entity));
    }

    pub fn attach_component<T>(&mut self, entity: &Entity, component: T) 
        where T: Component + ComponentIndex 
    {
        self.entities.attach_component(entity, component);
        self.events.add(Event::new_changed(*entity));
    }

    pub fn detach_component<T>(&mut self, entity: &Entity, component: T) 
        where T: Component + ComponentIndex 
    {
        self.entities.attach_component(entity, component);
        self.events.add(Event::new_changed(*entity));
    }

    #[inline]
    pub fn get_component<T>(&self, entity: &Entity) -> Option<&T> 
        where T: Component + ComponentIndex 
    {
        self.entities.get_component::<T>(entity)
    }

    #[inline]
    pub fn get_mut_component<T>(&mut self, entity: &Entity) -> Option<&mut T> 
        where T: Component + ComponentIndex 
    {
        self.entities.get_mut_component::<T>(entity)
    }

    pub fn set_group(&mut self, group: &str, entity: &Entity) {
        self.groups.set_group(group, entity);
        self.events.add(Event::new_changed(*entity));
    }

    pub fn remove_from_group(&mut self, group: &str, entity: &Entity) {
        self.groups.remove_from(group, entity);
        self.events.add(Event::new_changed(*entity));
    }

    pub fn set_tag(&mut self, tag: &str, entity: &Entity) {
        self.tags.set(tag, entity);
        self.events.add(Event::new_changed(*entity));
    }

    pub fn unset_tag(&mut self, entity: &Entity) {
        self.tags.unset(entity);
        self.events.add(Event::new_changed(*entity));
    }
}