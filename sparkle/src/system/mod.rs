use std::intrinsics::TypeId;

use space::Space;
use blackboard::SharedBlackboard;
use command::CommandSender;
use entity::{MetaEntity, EntityMapper, EntityObserver};
use component::ComponentMapper;

pub use self::filter::Filter;

pub mod filter;

pub trait System: 'static {
    fn update(&mut self, _em: &mut EntityMapper, _component: &mut ComponentMapper, _dt: f32) {}
    fn fixed_update(&mut self, _em: &mut EntityMapper, _component: &mut ComponentMapper) {}

    fn on_entity_changed(&mut self, _mentity: &MetaEntity) {}
    fn on_entity_removed(&mut self, _mentity: &MetaEntity) {}
}

pub struct SystemMapper {
    slots: Vec<SystemSlot>,
    cmd_sender: CommandSender<Space>,
    blackboard: SharedBlackboard
}

impl SystemMapper {
    pub fn new(cmd_sender: CommandSender<Space>, blackboard: SharedBlackboard) -> SystemMapper {
        SystemMapper {
            slots: Vec::new(),
            cmd_sender: cmd_sender,
            blackboard: blackboard
        }
    }

    pub fn insert<F, S>(&mut self, builder: F)
        where F: FnOnce(CommandSender<Space>, SharedBlackboard) -> S, S: System
    {
        self.slots.push(SystemSlot::new(
            builder(self.cmd_sender.clone(), self.blackboard.clone())
        ));
    }

    pub fn remove<S>(&mut self)
        where S: System
    {
        let type_id = TypeId::of::<S>();
        self.slots.retain(|slot| slot.type_id != type_id);
    }

    pub fn wake_up<S>(&mut self)
        where S: System
    {
        self.set_awake::<S>(true);
    }

    pub fn put_to_sleep<S>(&mut self)
        where S: System
    {
        self.set_awake::<S>(false);
    }

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

    pub fn update(&mut self, em: &mut EntityMapper, cm: &mut ComponentMapper, dt: f32) {
        self.update_with(em, cm, |slot, em, cm| slot.system.update(em, cm, dt));
    }

    pub fn fixed_update(&mut self, em: &mut EntityMapper, cm: &mut ComponentMapper) {
        self.update_with(em, cm, |slot, em, cm| slot.system.fixed_update(em, cm));
    }

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
    fn notify_changed(&mut self, mentity: &MetaEntity) {
        for slot in self.slots.iter_mut() {
            slot.system.on_entity_changed(mentity);
        }
    }

    fn notify_removed(&mut self, mentity: &MetaEntity) {
        for slot in self.slots.iter_mut() {
            slot.system.on_entity_removed(mentity);
        }
    }
}

struct SystemSlot {
    system: Box<System>,
    type_id: TypeId,
    is_awake: bool
}

impl SystemSlot {
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
