use std::collections::RingBuf;
use std::collections::ring_buf::Drain;

pub trait Command {
    type Arg;

    fn run(arg: Self::Arg);
}

pub struct CommandBuffer<C> where C: Command {
    commands: RingBuf<Box<C>>
}

impl<C> CommandBuffer<C> where C: Command {
    pub fn push(&mut self, command: Box<C>) {
        self.commands.push_back(command);
    }

    pub fn drain<'a>(&'a mut self) -> Drain<'a, Box<C>> {
        self.commands.drain()
    }
}
