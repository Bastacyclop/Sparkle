use component::{Component, ComponentIndex};
use entity::{Entity, Event};
use entity::event::Type as EventType;
use entity::Queue as EventQueue;
use entity::Manager as EntityManager;
use entity::Observer;
use group::Manager as GroupManager;
use tag::Manager as TagManager;
use system::Manager as SystemManager;
use system::System;

pub struct Space {
    events: EventQueue,
    entities: EntityManager,
    groups: GroupManager,
    tags: TagManager,
    systems: SystemManager
}

impl Space {
    pub fn new() -> Space {
        let entities = EntityManager::new();
        let mentities = entities.get_meta_entities();

        Space {
            events: EventQueue::new(),
            entities: entities,
            groups: GroupManager::new(mentities.clone()),
            tags: TagManager::new(mentities.clone()),
            systems: SystemManager::new()
        }
    }

    pub fn get_proxy<'a>(&'a mut self) -> SpaceProxy<'a> {
        SpaceProxy {
            events: &mut self.events,
            entities: &mut self.entities,
            groups: &mut self.groups,
            tags: &mut self.tags
        }
    }

    pub fn insert_system<T>(&mut self, name: &str, system: T) where T: System {
        self.systems.insert_system(name, system);
    }

    pub fn update(&mut self) {
        self.poll_events();

        let mut proxy = SpaceProxy {
            events: &mut self.events,
            entities: &mut self.entities,
            groups: &mut self.groups,
            tags: &mut self.tags
        };
        self.systems.process_systems(&mut proxy);
    }

    fn poll_events(&mut self) {
        let mentities = self.entities.get_meta_entities();

        while let Some(event) = self.events.poll_event() {
            let mentity = mentities.get(&event.entity);

            match event.event_type {
                EventType::Created => self.systems.on_created(mentity.unwrap()),
                EventType::Changed => self.systems.on_changed(mentity.unwrap()),
                EventType::Removed => {
                    self.systems.on_removed(mentity.unwrap());
                    self.entities.remove(&event.entity);
                }
            }
            
        }
    }
}

pub struct SpaceProxy<'a> {
    events: &'a mut EventQueue,
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