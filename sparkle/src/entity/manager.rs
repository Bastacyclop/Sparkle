use std::collections::VecMap;
use component::{StoreMap};
use command::{Command, CommandSender};
use entity::builder::{Builder, BuilderMap};
use entity::group::GroupMap;
use entity::tag::TagMap;
use entity::pool::Pool;
use entity::{Entity, MetaEntity};
use space::Space;

struct Entities {
    pool: Pool,
    mentities: VecMap<MetaEntity>,
    components: StoreMap
}

macro_rules! get_mentity_mut {
    ($entities:expr, $entity:expr) => (
        $entities.mentities.get_mut(&$entity)
                           .expect(format!("There is no meta information for {}", $entity).as_slice())
    )
}

impl Entities {
    pub fn new() -> Entities {
        Entities {
            pool: Pool::new(),
            mentities: VecMap::new(),
            components: StoreMap::new()
        }
    }
}

pub struct Manager {
    entities: Entities,
    groups: GroupMap,
    tags: TagMap,
    builders: BuilderMap,
    cmd_sender: CommandSender<Space>
}

impl Manager {
    pub fn new(cmd_sender: CommandSender<Space>) -> Manager {
        Manager {
            entities: Entities::new(),
            groups: GroupMap::new(),
            tags: TagMap::new(),
            builders: BuilderMap::new(),
            cmd_sender: cmd_sender
        }
    }

    pub fn create(&mut self) -> Entity {
        let meta_entity = self.entities.pool.get();
        let entity = meta_entity.entity;
        self.entities.mentities.insert(entity, meta_entity);

        self.cmd_sender.send(NotifyCreated(entity));

        entity
    }

    pub fn remove(&mut self, entity: Entity) {
        self.cmd_sender.send(NotifyRemoved(entity));
    }

    pub fn insert_group(&mut self, group: &str, entity: Entity) {
        let mentity = get_mentity_mut!(self.entities, entity);

        self.groups.insert(group, mentity);
        self.cmd_sender.send(NotifyChanged(entity));
    }

    pub fn remove_from_group(&mut self, group_name: &str, entity: Entity) {
        let mentity = get_mentity_mut!(self.entities, entity);

        self.groups.remove_from(group_name, mentity);
        self.cmd_sender.send(NotifyChanged(entity));
    }

    pub fn get_from_group(&self, group_name: &str) -> Vec<Entity> {
        self.groups.get(group_name)
    }

    pub fn insert_tag(&mut self, tag: &str, entity: Entity) {
        let mentity = get_mentity_mut!(self.entities, entity);

        self.tags.insert(tag, mentity);
        self.cmd_sender.send(NotifyChanged(entity));
    }

    pub fn remove_tag(&mut self, entity: Entity) {
        let mentity = get_mentity_mut!(self.entities, entity);

        self.tags.remove(mentity);
        self.cmd_sender.send(NotifyChanged(entity));
    }

    pub fn get_with_tag(&self, tag: &str) -> Option<Entity> {
        self.tags.get(tag)
    }

    pub fn insert_builder<T>(&mut self, name: &str, builder: T) where T: Builder {
        self.builders.insert(name, builder);
    }

    pub fn build_entity_with(&mut self, name: &str) -> Entity
    {
        let entity = self.create();
        let Manager { ref mut entities, ref mut groups, ref mut tags, ref mut builders, .. } = *self;

        let mentity = get_mentity_mut!(entities, entity);
        builders.get_builder_mut(name).map(|builder| {
            builder.create_entity(mentity, groups, tags)
        }).expect(format!("No template with the name {} was found.", name).as_slice())
    }
}

struct NotifyCreated(pub Entity);
impl Command<Space> for NotifyCreated {
    fn run(&mut self, space: &mut Space) {
        let mentity = get_mentity_mut!(space.em.entities, self.0);

        space.sm.notify_entity_created(mentity);
    }
}

struct NotifyChanged(pub Entity);
impl Command<Space> for NotifyChanged {
    fn run(&mut self, space: &mut Space) {
        let mentity = get_mentity_mut!(space.em.entities, self.0);

        space.sm.notify_entity_changed(mentity);
    }
}

struct NotifyRemoved(pub Entity);
impl Command<Space> for NotifyRemoved {
    fn run(&mut self, space: &mut Space) {
        {
            let mentity = get_mentity_mut!(space.em.entities, self.0);

            space.em.entities.components.remove_all(mentity);
            space.em.groups.clear_entity(mentity);
            space.em.tags.remove(mentity);

            space.sm.notify_entity_removed(mentity);
        }
        space.em.entities.mentities.remove(&self.0);
    }
}
