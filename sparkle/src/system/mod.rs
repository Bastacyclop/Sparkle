//! The system related features.

use std::any::TypeId;

use entity::{MetaEntity, EntityMapper, EntityObserver};
use component::ComponentMapper;

pub use self::filter::{EntityView, StandardEntityView, EntityFilter, StandardEntityFilter};

pub mod filter;

/// The trait for systems.
pub trait System: 'static {
    /// Performs an update of the system according to the given delta time.
    ///
    /// This method is called every frame.
    fn update(&mut self, _em: &mut EntityMapper, _component: &mut ComponentMapper, _dt: f32) {}

    /// Performs an update of of the system.
    ///
    /// This method is called at a fixed timestep.
    fn fixed_update(&mut self, _em: &mut EntityMapper, _component: &mut ComponentMapper) {}

    /// Called when an entity has been changed.
    fn on_entity_changed(&mut self, _cm: &ComponentMapper, _mentity: &MetaEntity) {}

    /// Called when an entity has been removed.
    ///
    /// For convenience, entity metadatas clearing is delayed
    /// until all systems have been notified.
    fn on_entity_removed(&mut self, _cm: &ComponentMapper, _mentity: &MetaEntity) {}
}

/// Maps systems using `TypeId`s as identifiers.
pub struct SystemMapper {
    slots: Vec<SystemSlot>,
}

impl SystemMapper {
    /// Creates an empty `SystemMapper`.
    pub fn new() -> SystemMapper {
        SystemMapper {
            slots: Vec::new(),
        }
    }

    /// Inserts a system in the mapper.
    ///
    /// The system will be awake by default.
    pub fn insert<S>(&mut self, system: S)
        where S: System
    {
        self.slots.push(SystemSlot::new(system));
    }

    /// Removes a system from the mapper.
    ///
    /// Note that this method is O(n).
    pub fn remove<S>(&mut self)
        where S: System
    {
        let type_id = TypeId::of::<S>();
        self.slots.retain(|slot| slot.type_id != type_id);
    }

    /// Enables a system, resuming updates.
    ///
    /// Note that this method is O(n).
    pub fn wake_up<S>(&mut self)
        where S: System
    {
        self.set_awake::<S>(true);
    }

    /// Disables a system, interrupting updates.
    ///
    /// The system will be kept informed of entity changes.
    ///
    /// Note that this method is O(n).
    pub fn put_to_sleep<S>(&mut self)
        where S: System
    {
        self.set_awake::<S>(false);
    }

    /// Enables or disables a system.
    fn set_awake<S>(&mut self, is_awake: bool)
        where S: System
    {
        let type_id = TypeId::of::<S>();
        for slot in self.slots.iter_mut() {
            if slot.type_id == type_id {
                slot.is_awake = is_awake;
                break;
            }
        }
    }

    /// Updates systems with the given delta time.
    ///
    /// The systems are kept informed of entity changes between each system update.
    pub fn update(&mut self, em: &mut EntityMapper, cm: &mut ComponentMapper, dt: f32) {
        self.update_with(em, cm, |slot, em, cm| slot.system.update(em, cm, dt));
    }

    /// Updates systems at a fixed timestep.
    ///
    /// The systems are kept informed of entity changes between each system update.
    pub fn fixed_update(&mut self, em: &mut EntityMapper, cm: &mut ComponentMapper) {
        self.update_with(em, cm, |slot, em, cm| slot.system.fixed_update(em, cm));
    }

    /// Updates systems with the given function.
    ///
    /// The systems are kept informed of entity changes between each system update.
    fn update_with<F>(&mut self, em: &mut EntityMapper, cm: &mut ComponentMapper, mut func: F)
        where F: FnMut(&mut SystemSlot, &mut EntityMapper, &mut ComponentMapper)
    {
        for i in range(0, self.slots.len()) {
            em.notify_events(cm, self);
            let slot = &mut self.slots[i];
            if slot.is_awake {
                func(slot, em, cm);
            }
        }
    }
}

impl EntityObserver for SystemMapper {
    /// Notifies systems that an entity has changed.
    fn notify_changed(&mut self, cm: &ComponentMapper, mentity: &MetaEntity) {
        for slot in self.slots.iter_mut() {
            slot.system.on_entity_changed(cm, mentity);
        }
    }

    /// Notifies systems that an entity has been removed.
    fn notify_removed(&mut self, cm: &ComponentMapper, mentity: &MetaEntity) {
        for slot in self.slots.iter_mut() {
            slot.system.on_entity_removed(cm, mentity);
        }
    }
}

/// Hosts a system and keeps some extra informations.
struct SystemSlot {
    system: Box<System>,
    type_id: TypeId,
    is_awake: bool
}

impl SystemSlot {
    /// Returns a new `SystemSlot` hosting the given system.
    ///
    /// By default, a system is awake.
    fn new<S>(system: S) -> SystemSlot
        where S: System
    {
        SystemSlot {
            system: Box::new(system),
            type_id: TypeId::of::<S>(),
            is_awake: true
        }
    }
}
