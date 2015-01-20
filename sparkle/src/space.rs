//! Convenient object of the library.

use command::{self, CommandReceiver, CommandSender, Command};
use component::ComponentMapper;
use entity::EntityMapper;
use system::SystemMapper;

pub type SpaceCommand = Box<for<'a> Command<Args = &'a Space>>;

/// Regroups the three essential mappers and the blackboard.
///
/// It's also responsible of updates and command execution.
pub struct Space {
    cmd_receiver: CommandReceiver<SpaceCommand>,
    pub em: EntityMapper,
    pub cm: ComponentMapper,
    pub sm: SystemMapper
}

impl Space {
    /// Creates a new `Space`.
    pub fn new() -> (Space, CommandSender<SpaceCommand>) {
        let (sender, receiver) = command::stream();

        (Space {
            cmd_receiver: receiver,
            em: EntityMapper::new(),
            cm: ComponentMapper::new(),
            sm: SystemMapper::new()
        },
        sender)
    }

    /// Runs pending commands and updates systems according to the
    /// given delta time. This should be called every frame.
    pub fn update(&mut self, dt: f32) {
        self.run_commands();
        self.sm.update(&mut self.em, &mut self.cm, dt);
    }

    /// Runs pending commands and updates systems. This should be called
    /// at a fixed timestep.
    pub fn fixed_update(&mut self) {
        self.run_commands();
        self.sm.fixed_update(&mut self.em, &mut self.cm);
    }

    /// Runs pending commands.
    fn run_commands(&mut self) {
        while let Some(mut command) = self.cmd_receiver.recv() {
            command.run(self)
        }
    }
}
