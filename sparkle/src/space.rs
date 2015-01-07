use std::ops::Deref;
use component::{Component, ComponentIndex};
use entity::{self, Entity};
use builder::Builder;
use system;
use command::{self, CommandReceiver, Command};

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
}