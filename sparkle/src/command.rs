use std::rc::Rc;
use std::cell::RefCell;
use std::collections::RingBuf;

pub trait Command<Args>: 'static {
    fn run(&mut self, args: &mut Args);
}

pub fn stream<Args>() -> (CommandSender<Args>, CommandReceiver<Args>) {
    let buffer = Rc::new(RefCell::new(CommandBuffer::new()));

    (CommandSender(buffer.clone()), CommandReceiver(buffer.clone()))
}

pub struct CommandSender<Args>(SharedBuffer<Args>);

impl<Args> Clone for CommandSender<Args> {
    fn clone(&self) -> CommandSender<Args> {
        CommandSender(self.0.clone())
    }
}

impl<Args> CommandSender<Args> {
    pub fn send<C>(&mut self, command: C)
        where C: Command<Args>
    {
        self.0.borrow_mut().push(command);
    }
}

pub struct CommandReceiver<Args>(SharedBuffer<Args>);

impl<Args> CommandReceiver<Args> {
    pub fn recv(&mut self) -> Option<Box<Command<Args>>> {
        self.0.borrow_mut().pop()
    }
}

type SharedBuffer<Args> = Rc<RefCell<CommandBuffer<Args>>>;

struct CommandBuffer<Args> {
    commands: RingBuf<Box<Command<Args>>>
}

impl<Args> CommandBuffer<Args> {
    fn new() -> CommandBuffer<Args> {
        CommandBuffer {
            commands: RingBuf::new()
        }
    }

    fn push<C>(&mut self, command: C)
        where C: Command<Args>
    {
        self.commands.push_back(Box::new(command));
    }

    fn pop(&mut self) -> Option<Box<Command<Args>>> {
        self.commands.pop_front()
    }
}
