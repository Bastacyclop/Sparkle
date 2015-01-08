use std::rc::Rc;
use std::cell::RefCell;
use std::collections::RingBuf;

pub use self::space_commands::CreateEntity;
pub use self::space_commands::RemoveEntity;

pub mod space_commands;

pub trait Command<Args>: 'static {
    fn run(&mut self, args: &mut Args);
}

pub struct CommandBuffer<Args> {
    commands: RingBuf<Box<Command<Args>>>
}

impl<Args> CommandBuffer<Args> {
    pub fn new() -> CommandBuffer<Args> {
        CommandBuffer {
            commands: RingBuf::new()
        }
    }

    pub fn push<C>(&mut self, command: C) where C: Command<Args> {
        self.commands.push_back(box command);
    }

    pub fn pop<'a>(&'a mut self) -> Option<Box<Command<Args>>> {
        self.commands.pop_front()
    }
}

type SharedBuffer<Args> = Rc<RefCell<CommandBuffer<Args>>>;

pub struct CommandSender<Args>(SharedBuffer<Args>);

impl<Args> Clone for CommandSender<Args> {
    fn clone(&self) -> CommandSender<Args> {
        CommandSender(self.0.clone())
    }
}

impl<Args> CommandSender<Args> {
    pub fn send<C>(&mut self, command: C) where C: Command<Args> {
        self.0.borrow_mut().push(command);
    }
}

pub struct CommandReceiver<Args>(SharedBuffer<Args>);

impl<Args> CommandReceiver<Args> {
    pub fn recv(&mut self) -> Option<Box<Command<Args>>> {
        self.0.borrow_mut().pop()
    }
}

pub fn stream<Args>() -> (CommandSender<Args>, CommandReceiver<Args>) {
    let buffer = Rc::new(RefCell::new(CommandBuffer::new()));

    (CommandSender(buffer.clone()), CommandReceiver(buffer.clone()))
}
