//! The command related features.

use std::rc::{Rc, Weak};
use std::cell::RefCell;
use std::collections::RingBuf;

/// Represents any command that can be runned.
pub trait Command<Args>: 'static {
    /// Runs the command.
    fn run(&mut self, args: &mut Args);
}

/// Creates a new command stream.
///
/// Note that this is not thread safe.
pub fn stream<Args>() -> (CommandSender<Args>, CommandReceiver<Args>) {
    let buffer = Rc::new(RefCell::new(CommandBuffer::new()));

    (CommandSender(buffer.clone().downgrade()), CommandReceiver(buffer.clone()))
}

/// Sends commands to a `CommandReceiver`.
///
/// You can have multiple senders per stream using the clone method.
pub struct CommandSender<Args>(WeakSharedBuffer<Args>);

impl<Args> Clone for CommandSender<Args> {
    fn clone(&self) -> CommandSender<Args> {
        CommandSender(self.0.clone())
    }
}

impl<Args> CommandSender<Args> {
    /// Sends a command to the linked `CommandReceiver`.
    ///
    /// If the receiver was dropped, this method does nothing.
    pub fn send<C>(&mut self, command: C)
        where C: Command<Args>
    {
        if let Some(buffer) = self.0.upgrade() {
            buffer.borrow_mut().push(command);    
        }
    }
}

/// Receives commands from `CommandSender`s.
///
/// You can only have one receiver per stream.
pub struct CommandReceiver<Args>(SharedBuffer<Args>);

impl<Args> CommandReceiver<Args> {
    /// Retrieves a command from the stream.
    ///
    /// The insertion order is preserved.
    ///
    /// Returns `None` if there is no more commands to retrieve.
    pub fn recv(&mut self) -> Option<Box<Command<Args>>> {
        self.0.borrow_mut().pop()
    }
}

type SharedBuffer<Args> = Rc<RefCell<CommandBuffer<Args>>>;
type WeakSharedBuffer<Args> = Weak<RefCell<CommandBuffer<Args>>>;

/// A simple buffer of commands using a `RingBuf`.
struct CommandBuffer<Args> {
    commands: RingBuf<Box<Command<Args>>>
}

impl<Args> CommandBuffer<Args> {
    /// Creates an empty `CommandBuffer`.
    fn new() -> CommandBuffer<Args> {
        CommandBuffer {
            commands: RingBuf::new()
        }
    }

    /// Pushes a new command into the buffer.
    fn push<C>(&mut self, command: C)
        where C: Command<Args>
    {
        self.commands.push_back(Box::new(command));
    }

    /// Tries to pop a command.
    ///
    /// Returns `None` if the buffer is empty.
    fn pop(&mut self) -> Option<Box<Command<Args>>> {
        self.commands.pop_front()
    }
}
