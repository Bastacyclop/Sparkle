//! Main object of the library

use command::{self, CommandReceiver, Command};
use blackboard::SharedBlackboard;
use component::ComponentMapper;
use entity::EntityMapper;
use system::SystemMapper;

/// A Space is a type that regroups the three essential mappers
/// and the blackboard. It's also responsible for updating the world
/// and run commands from command senders.
pub struct Space {
    cmd_receiver: CommandReceiver<Space>,
    pub blackboard: SharedBlackboard,
    pub em: EntityMapper,
    pub cm: ComponentMapper,
    pub sm: SystemMapper
}

impl Space {
    /// Create a new `Space`
    pub fn new(blackboard: SharedBlackboard) -> Space {
        let (sender, receiver) = command::stream();

        Space {
            cmd_receiver: receiver,
            blackboard: blackboard.clone(),
            em: EntityMapper::new(),
            cm: ComponentMapper::new(),
            sm: SystemMapper::new(sender, blackboard)
        }
    }

    /// Run pending commands and update systems according to the
    /// given delta time (dt). This should be called every frame.
    pub fn update(&mut self, dt: f32) {
        self.run_commands();
        self.sm.update(&mut self.em, &mut self.cm, dt);
    }

    /// Run pending commands and update systems. This should be called
    /// at a fixed timestep.
    pub fn fixed_update(&mut self) {
        self.run_commands();
        self.sm.fixed_update(&mut self.em, &mut self.cm);
    }

    /// Run pending commands
    fn run_commands(&mut self) {
        while let Some(mut command) = self.cmd_receiver.recv() {
            command.run(self)
        }
    }
}
