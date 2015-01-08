use command::{self, CommandReceiver, Command};
use entity;
use system;

pub struct Space {
    cmd_receiver: CommandReceiver<Space>,
    pub em: entity::Manager,
    pub sm: system::Manager
}

impl Space {
    pub fn new() -> Space {
        let (sender, receiver) = command::stream();

        Space {
            cmd_receiver: receiver,
            em: entity::Manager::new(sender.clone()),
            sm: system::Manager::new(sender)
        }
    }

    pub fn update(&mut self, dt: f32) {
        while let Some(mut command) = self.cmd_receiver.recv() {
            command.run(self)
        }
        self.sm.update(&mut self.em, dt);
    }
}
