use command::CommandSender;
use entity::{self, MetaEntity};
use space::Space;
use system::System;
use entity::event;


pub struct Manager {
    systems: Vec<Box<System>>,
    cmd_sender: CommandSender<Space>
}

impl Manager {
    pub fn new(cmd_sender: CommandSender<Space>) -> Manager {
        Manager {
            systems: Vec::new(),
            cmd_sender: cmd_sender
        }
    }

    pub fn insert<F, S>(&mut self, builder: F)
        where F: FnOnce(CommandSender<Space>) -> S, S: System
    {
        self.systems.push(Box::new(builder(self.cmd_sender.clone())));
    }

    pub fn update(&mut self, em: &mut entity::Manager, dt: f32) {
        for i in range(0, self.systems.len()) {
            self.notify_events(em);
            self.systems[i].update(em, dt);
        }
    }

    pub fn fixed_update(&mut self, em: &mut entity::Manager) {
        for i in range(0, self.systems.len()) {
            self.notify_events(em);
            self.systems[i].fixed_update(em);
        }
    }

    fn notify_events(&mut self, em: &mut entity::Manager) {
        while let Some(event) = em.pop_event() {
            let mentity = em.get_mentity(event.1);
            match event.0 {
                event::Created => self.notify_entity_created(mentity),
                event::Removed => self.notify_entity_removed(mentity),
            }
        }
    }

    fn notify_entity_created(&mut self, mentity: &MetaEntity) {
        for system in self.systems.iter_mut() {
            system.on_entity_created(mentity);
        }
    }

    fn notify_entity_removed(&mut self, mentity: &MetaEntity) {
        for system in self.systems.iter_mut() {
            system.on_entity_removed(mentity);
        }
    }
}
