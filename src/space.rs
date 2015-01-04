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
        Space {
            events: EventQueue::new(),
            entities: EntityManager::new(),
            groups: GroupManager::new(),
            tags: TagManager::new(),
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
        while let Some(event) = self.events.poll_event() {
            let remove_flag = self.handle_event(event);
            if remove_flag { self.entities.remove(&event.entity); }
        }
    }

    fn handle_event(&mut self, event: Event) -> bool {
        let mentity = self.entities.get_mentity(&event.entity).unwrap();
                
        match event.event_type {
            EventType::Created => self.systems.on_created(mentity),
            EventType::Changed => self.systems.on_changed(mentity),
            EventType::Removed => {
                self.systems.on_removed(mentity);
                return true;
            }
        }

        false
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
        let mentity = self.entities.get_mut_mentity(entity).unwrap();

        self.groups.set_group(group, mentity);
        self.events.add(Event::new_changed(*entity));
    }

    pub fn remove_from_group(&mut self, group: &str, entity: &Entity) {
        let mentity = self.entities.get_mut_mentity(entity).unwrap();

        self.groups.remove_from(group, mentity);
        self.events.add(Event::new_changed(*entity));
    }

    pub fn set_tag(&mut self, tag: &str, entity: &Entity) {
        let mentity = self.entities.get_mut_mentity(entity).unwrap();

        self.tags.set(tag, mentity);
        self.events.add(Event::new_changed(*entity));
    }

    pub fn unset_tag(&mut self, entity: &Entity) {
        let mentity = self.entities.get_mut_mentity(entity).unwrap();

        self.tags.unset(mentity);
        self.events.add(Event::new_changed(*entity));
    }
}