use std::intrinsics::TypeId;

use command::CommandSender;
use blackboard::SharedBlackboard;
use entity::{self, MetaEntity};
use component;
use entity::event;
use space::Space;
use system::System;

struct SystemHandle {
    system: Box<System>,
    type_id: TypeId,
    is_awake: bool
}

impl SystemHandle {
    fn new<S>(system: S) -> SystemHandle where S: System {
        SystemHandle {
            system: Box::new(system),
            type_id: TypeId::of::<S>(),
            is_awake: true
        }
    }
}

pub struct Mapper {
    handles: Vec<SystemHandle>,
    cmd_sender: CommandSender<Space>,
    blackboard: SharedBlackboard
}

impl Mapper {
    pub fn new(cmd_sender: CommandSender<Space>, blackboard: SharedBlackboard) -> Mapper {
        Mapper {
            handles: Vec::new(),
            cmd_sender: cmd_sender,
            blackboard: blackboard
        }
    }

    pub fn insert<F, S>(&mut self, builder: F)
        where F: FnOnce(CommandSender<Space>, SharedBlackboard) -> S, S: System
    {
        self.handles.push(SystemHandle::new(
            builder(self.cmd_sender.clone(), self.blackboard.clone())
        ));
    }

    pub fn remove<S>(&mut self) where S: System {
        let type_id = TypeId::of::<S>();
        self.handles.retain(|handle| handle.type_id != type_id);
    }

    pub fn wake_up<S>(&mut self) where S: System {
        self.set_awake::<S>(true);
    }

    pub fn put_to_sleep<S>(&mut self) where S: System {
        self.set_awake::<S>(false);
    }

    fn set_awake<S>(&mut self, is_awake: bool) where S: System {
        let type_id = TypeId::of::<S>();
        for handle in self.handles.iter_mut() {
            if handle.type_id == type_id {
                handle.is_awake = is_awake;
                break;
            }
        }
    }

    pub fn update(&mut self, em: &mut entity::Mapper, cm: &mut component::Mapper, dt: f32) {
        self.update_with(em, cm, |handle, em, cm| handle.system.update(em, cm, dt));
    }

    pub fn fixed_update(&mut self, em: &mut entity::Mapper, cm: &mut component::Mapper) {
        self.update_with(em, cm, |handle, em, cm| handle.system.fixed_update(em, cm));
    }

    fn update_with<F>(&mut self, em: &mut entity::Mapper, cm: &mut component::Mapper, mut func: F)
        where F: FnMut(&mut SystemHandle, &mut entity::Mapper, &mut component::Mapper)
    {
        for i in range(0, self.handles.len()) {
            em.notify_events(cm, self);
            let handle = &mut self.handles[i];
            if handle.is_awake {
                func(handle, em, cm);
            }
        }
    }
}

impl event::Observer for Mapper {
    /// NOTE: notify only awake systems ?
    fn notify_changed(&mut self, mentity: &MetaEntity) {
        for handle in self.handles.iter_mut() {
            handle.system.on_entity_changed(mentity);
        }
    }

    /// NOTE: notify only awake systems ?
    fn notify_removed(&mut self, mentity: &MetaEntity) {
        for handle in self.handles.iter_mut() {
            handle.system.on_entity_removed(mentity);
        }
    }
}
