use command::CommandSender;
use blackboard::SharedBlackboard;
use entity::{self, MetaEntity};
use component;
use entity::event;
use space::Space;
use system::System;

pub struct Manager {
    systems: Vec<Box<System>>,
    cmd_sender: CommandSender<Space>,
    blackboard: SharedBlackboard
}

impl Manager {
    pub fn new(cmd_sender: CommandSender<Space>, blackboard: SharedBlackboard) -> Manager {
        Manager {
            systems: Vec::new(),
            cmd_sender: cmd_sender,
            blackboard: blackboard
        }
    }

    pub fn insert<F, S>(&mut self, builder: F)
        where F: FnOnce(CommandSender<Space>, SharedBlackboard) -> S, S: System
    {
        self.systems.push(Box::new(builder(self.cmd_sender.clone(), self.blackboard.clone())));
    }

    pub fn update(&mut self, em: &mut entity::Manager, cm: &mut component::Manager, dt: f32) {
        for i in range(0, self.systems.len()) {
            em.notify_events(cm, self);
            self.systems[i].update(em, cm, dt);
        }
    }

    pub fn fixed_update(&mut self, em: &mut entity::Manager, cm: &mut component::Manager) {
        for i in range(0, self.systems.len()) {
            em.notify_events(cm, self);
            self.systems[i].fixed_update(em, cm);
        }
    }
}

impl event::Observer for Manager {
    fn notify_changed(&mut self, mentity: &MetaEntity) {
        for system in self.systems.iter_mut() {
            system.on_entity_changed(mentity);
        }
    }

    fn notify_removed(&mut self, mentity: &MetaEntity) {
        for system in self.systems.iter_mut() {
            system.on_entity_removed(mentity);
        }
    }
}
