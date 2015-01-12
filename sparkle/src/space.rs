use command::{self, CommandReceiver, Command};
use component::StoreMap;
use blackboard::Blackboard;
use std::rc::Rc;
use std::cell::RefCell;
use entity;
use system;

pub struct Space {
    cmd_receiver: CommandReceiver<Space>,
    pub blackboard: Rc<RefCell<Blackboard>>,
    pub em: entity::Manager,
    pub cm: StoreMap,
    pub sm: system::Manager
}

impl Space {
    pub fn new(blackboard: Rc<RefCell<Blackboard>>) -> Space {
        let (sender, receiver) = command::stream();

        Space {
            cmd_receiver: receiver,
            blackboard: blackboard,
            em: entity::Manager::new(),
            cm: StoreMap::new(),
            sm: system::Manager::new(sender)
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
