use component::{Component, ComponentIndex, Store};
use entity::{self, event, Event, Observer, Entity};
use system::{self, System};
use group::GroupMap;
use tag::TagMap;
use builder::{Builder, BuilderMap};

struct Maps {
    pub groups: GroupMap,
    pub tags: TagMap,
    pub builders: BuilderMap
}

impl Maps {
    pub fn new() -> Maps {
        Maps {
            groups: GroupMap::new(),
            tags: TagMap::new(),
            builders: BuilderMap::new()
        }
    }
}

pub struct Space {
    events: event::Queue,
    maps: Maps,
    entities: entity::Manager,
    systems: system::Manager
}

impl Space {
    pub fn new() -> Space {
        Space {
            events: event::Queue::new(),
            maps: Maps::new(),
            entities: entity::Manager::new(),
            systems: system::Manager::new()
        }
    }

    pub fn get_proxy<'a>(&'a mut self) -> SpaceProxy<'a> {
        SpaceProxy {
            events: &mut self.events,
            maps: &mut self.maps,
            entities: &mut self.entities
        }
    }

    pub fn push_system<T>(&mut self, system: T) where T: System {
        self.systems.push(system);
    }

    pub fn update(&mut self, dt: f32) {
        self.poll_events();

        let mut proxy = SpaceProxy {
            events: &mut self.events,
            maps: &mut self.maps,
            entities: &mut self.entities,
        };
        self.systems.process_all(&mut proxy, dt);
    }

    fn poll_events(&mut self) {
        while let Some(event) = self.events.poll_event() {
            let remove_flag = self.handle_event(event);
            if remove_flag { self.entities.remove_entity(&event.entity); }
        }
        self.events.reset();
    }

    fn handle_event(&mut self, event: Event) -> bool {
        let mentity = self.entities.get_mentity(&event.entity);
                
        match event.kind {
            event::Kind::Created => self.systems.on_created(mentity),
            event::Kind::Changed => self.systems.on_changed(mentity),
            event::Kind::Removed => {
                self.systems.on_removed(mentity);
                return true;
            }
        }

        false
    }
}

pub struct SpaceProxy<'a> {
    events: &'a mut event::Queue,
    maps: &'a mut Maps,
    entities: &'a mut entity::Manager,
}

impl<'a> SpaceProxy<'a> {
    pub fn create_entity(&mut self) -> Entity {
        let entity = self.entities.create_entity();        
        self.events.add(Event::created(entity));

        entity
    }

    pub fn remove_entity(&mut self, entity: &Entity) {
        self.events.add(Event::removed(*entity));
    }

    pub fn attach_component<T>(&mut self, entity: &Entity, component: T) 
        where T: Component + ComponentIndex 
    {
        self.entities.attach_component(entity, component);
        self.events.add(Event::changed(*entity));
    }

    pub fn detach_component<T>(&mut self, entity: &Entity, component: T) 
        where T: Component + ComponentIndex 
    {
        self.entities.attach_component(entity, component);
        self.events.add(Event::changed(*entity));
    }

    #[inline]
    pub fn get_component<T>(&self, entity: &Entity) -> Option<&T> 
        where T: Component + ComponentIndex 
    {
        self.entities.get_component::<T>(entity)
    }

    #[inline]
    pub fn get_component_mut<T>(&mut self, entity: &Entity) -> Option<&mut T> 
        where T: Component + ComponentIndex 
    {
        self.entities.get_component_mut::<T>(entity)
    }

    pub fn get_store<T>(&self) -> Option<&Store<T>> 
        where T: Component + ComponentIndex
    {
        self.entities.get_store::<T>()
    }

    pub fn get_store_mut<T>(&mut self) -> Option<&mut Store<T>> 
        where T: Component + ComponentIndex
    {
        self.entities.get_store_mut::<T>()
    }

    pub fn set_group(&mut self, group: &str, entity: &Entity) {
        let mentity = self.entities.get_mentity_mut(entity);

        self.maps.groups.insert(group, mentity);
        self.events.add(Event::changed(*entity));
    }

    pub fn remove_from_group(&mut self, group: &str, entity: &Entity) {
        let mentity = self.entities.get_mentity_mut(entity);

        self.maps.groups.remove_from(group, mentity);
        self.events.add(Event::changed(*entity));
    }

    pub fn get_entities_from_group(&self, group: &str) -> Vec<Entity> {
        self.maps.groups.get(group)
    }

    pub fn set_tag(&mut self, tag: &str, entity: &Entity) {
        let mentity = self.entities.get_mentity_mut(entity);

        self.maps.tags.insert(tag, mentity);
        self.events.add(Event::changed(*entity));
    }

    pub fn unset_tag(&mut self, entity: &Entity) {
        let mentity = self.entities.get_mentity_mut(entity);

        self.maps.tags.remove(mentity);
        self.events.add(Event::changed(*entity));
    }

    pub fn get_entity_with_tag(&self, tag: &str) -> Option<Entity> {
        self.maps.tags.get(tag)
    }

}