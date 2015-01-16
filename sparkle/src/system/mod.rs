//! The System related types.

use std::intrinsics::TypeId;

use space::Space;
use blackboard::SharedBlackboard;
use command::CommandSender;
use entity::{MetaEntity, EntityMapper, EntityObserver};
use component::ComponentMapper;

pub use self::filter::Filter;

pub mod filter;

/// The trait for systems
pub trait System: 'static {
    /// Perform an update of components in the space according to the given delta time.
    ///
    /// This method is called every frame.
    fn update(&mut self, _em: &mut EntityMapper, _component: &mut ComponentMapper, _dt: f32) {}

    /// Perform an update of components in the space.
    ///
    /// This method is called at a fixed timestep.
    fn fixed_update(&mut self, _em: &mut EntityMapper, _component: &mut ComponentMapper) {}

    /// Called when an entity has been changed.
    ///
    /// If you want a default implementation you can use the sparkle_default_system_filtering macro.
    fn on_entity_changed(&mut self, _mentity: &MetaEntity) {}

    /// Called when an entity has been removed from Space.
    /// For convenience all meta datas are kept during this method. However
    /// when all systems has been notifyied entity meta datas are cleared.
    ///
    /// If you want a default implementation you can use the sparkle_default_system_filtering macro.
    fn on_entity_removed(&mut self, _mentity: &MetaEntity) {}
}

/// A `SystemMapper` using TypeId as systems identifier
pub struct SystemMapper {
    slots: Vec<SystemSlot>,
    cmd_sender: CommandSender<Space>,
    blackboard: SharedBlackboard
}

impl SystemMapper {
    /// Create a new empty `SystemMapper` with the geiven cmd_sender and blackboard.
    /// Those will be passed around each system at insertion.
    pub fn new(cmd_sender: CommandSender<Space>, blackboard: SharedBlackboard) -> SystemMapper {
        SystemMapper {
            slots: Vec::new(),
            cmd_sender: cmd_sender,
            blackboard: blackboard
        }
    }

    /// Insert a new system in the mapper. The builder fonction receive a 
    /// cmd_sender and a blackboard free to use for systems which require them.
    pub fn insert<F, S>(&mut self, builder: F)
        where F: FnOnce(CommandSender<Space>, SharedBlackboard) -> S, S: System
    {
        self.slots.push(SystemSlot::new(
            builder(self.cmd_sender.clone(), self.blackboard.clone())
        ));
    }

    /// Remove a system from the mapper.
    ///
    /// Note that this method is O(n).
    pub fn remove<S>(&mut self)
        where S: System
    {
        let type_id = TypeId::of::<S>();
        self.slots.retain(|slot| slot.type_id != type_id);
    }

    /// Enable a system present in the mapper.
    /// Update methods of the system will anew called.
    ///
    /// Note that this method is O(n).
    pub fn wake_up<S>(&mut self)
        where S: System
    {
        self.set_awake::<S>(true);
    }

    /// Disable a system present in the mapper.
    /// Update methods of the system won't be called anymore
    /// but the system will still get update of entities.
    ///
    /// Note that this method is O(n).
    pub fn put_to_sleep<S>(&mut self)
        where S: System
    {
        self.set_awake::<S>(false);
    }

    /// Enable or disable the given system.
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

    /// Update systems with the given delta times.
    /// Every change applied to entities are kept updated after each system update call.
    pub fn update(&mut self, em: &mut EntityMapper, cm: &mut ComponentMapper, dt: f32) {
        self.update_with(em, cm, |slot, em, cm| slot.system.update(em, cm, dt));
    }

    /// Update systems at a fixed timestep
    /// Every change applied to entities are kept updated after each system update call.
    pub fn fixed_update(&mut self, em: &mut EntityMapper, cm: &mut ComponentMapper) {
        self.update_with(em, cm, |slot, em, cm| slot.system.fixed_update(em, cm));
    }

    /// Update all systems with the given function.
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
    /// Notify all systems that an entity has changed.
    fn notify_changed(&mut self, mentity: &MetaEntity) {
        for slot in self.slots.iter_mut() {
            slot.system.on_entity_changed(mentity);
        }
    }

    /// Notify all systems that an entity has been removed.
    fn notify_removed(&mut self, mentity: &MetaEntity) {
        for slot in self.slots.iter_mut() {
            slot.system.on_entity_removed(mentity);
        }
    }
}

/// A `SystemSlot` is a wrapper around a system
/// that keep some extra information
struct SystemSlot {
    system: Box<System>,
    type_id: TypeId,
    is_awake: bool
}

impl SystemSlot {
    /// Returns a new `SystemSlot` with the given system
    /// By default, a system is awaken.
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
