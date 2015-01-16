//! The Command related types

use std::rc::{Rc, Weak};
use std::cell::RefCell;
use std::collections::RingBuf;

/// The Command trait represent any command
/// that can be runned.
pub trait Command<Args>: 'static {
    fn run(&mut self, args: &mut Args);
}

/// Create a new stream of commands.
/// Note that this is not thread safe.
pub fn stream<Args>() -> (CommandSender<Args>, CommandReceiver<Args>) {
    let buffer = Rc::new(RefCell::new(CommandBuffer::new()));

    (CommandSender(buffer.clone().downgrade()), CommandReceiver(buffer.clone()))
}

/// A `CommandSender` is a type that send commands
/// to the `CommandReceiver`.
///
/// You can have multiple sender using the clone method.
pub struct CommandSender<Args>(WeakSharedBuffer<Args>);

impl<Args> Clone for CommandSender<Args> {
    fn clone(&self) -> CommandSender<Args> {
        CommandSender(self.0.clone())
    }
}

impl<Args> CommandSender<Args> {
    /// Send a command to the linked `CommandReceiver`.
    /// If the receiver is dropped this method does nothing.
    pub fn send<C>(&mut self, command: C)
        where C: Command<Args>
    {
        if let Some(buffer) = self.0.upgrade() {
            buffer.borrow_mut().push(command);    
        }
    }
}

/// A `CommandReceiver` is a type that receive commands
/// from the `CommandSender`.
///
/// You can only have one Receiver per stream.
pub struct CommandReceiver<Args>(SharedBuffer<Args>);

impl<Args> CommandReceiver<Args> {
    /// Retrieve a command from the pendings command.
    /// The insertion order is preserved.
    ///
    /// Returns None if no command are waiting.
    pub fn recv(&mut self) -> Option<Box<Command<Args>>> {
        self.0.borrow_mut().pop()
    }
}

type SharedBuffer<Args> = Rc<RefCell<CommandBuffer<Args>>>;
type WeakSharedBuffer<Args> = Weak<RefCell<CommandBuffer<Args>>>;

/// A simple buffer of commands using a RingBuffer.
struct CommandBuffer<Args> {
    commands: RingBuf<Box<Command<Args>>>
}

impl<Args> CommandBuffer<Args> {
    /// Create a new empty `CommandBuffer`
    fn new() -> CommandBuffer<Args> {
        CommandBuffer {
            commands: RingBuf::new()
        }
    }

    /// Push a new command to the buffer
    fn push<C>(&mut self, command: C)
        where C: Command<Args>
    {
        self.commands.push_back(Box::new(command));
    }

    /// Try to pop command. Returns None if there is no 
    /// commands in the buffer.
    fn pop(&mut self) -> Option<Box<Command<Args>>> {
        self.commands.pop_front()
    }
}
