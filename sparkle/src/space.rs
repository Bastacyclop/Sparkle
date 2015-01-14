use command::{self, CommandReceiver, Command};
use blackboard::SharedBlackboard;
use component;
use entity;
use system;

pub struct Space {
    cmd_receiver: CommandReceiver<Space>,
    pub blackboard: SharedBlackboard,
    pub em: entity::Mapper,
    pub cm: component::Mapper,
    pub sm: system::Mapper
}

impl Space {
    pub fn new(blackboard: SharedBlackboard) -> Space {
        let (sender, receiver) = command::stream();

        Space {
            cmd_receiver: receiver,
            blackboard: blackboard.clone(),
            em: entity::Mapper::new(),
            cm: component::Mapper::new(),
            sm: system::Mapper::new(sender, blackboard)
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.run_commands();
        self.sm.update(&mut self.em, &mut self.cm, dt);
    }

    pub fn fixed_update(&mut self) {
        self.run_commands();
        self.sm.fixed_update(&mut self.em, &mut self.cm);
    }

    fn run_commands(&mut self) {
        while let Some(mut command) = self.cmd_receiver.recv() {
            command.run(self)
        }    
    }
}
