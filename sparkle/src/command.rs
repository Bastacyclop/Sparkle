//! The command related features.

use std::rc::{Rc, Weak};
use std::cell::RefCell;
use std::collections::RingBuf;

/// Represents any command that can be runned.
pub trait Command: 'static {
    type Args;

    /// Runs the command.
    fn run(&mut self, args: <Self as Command>::Args);
}

/// Creates a new command stream.
///
/// Note that this is not thread safe.
pub fn stream<C>() -> (CommandSender<C>, CommandReceiver<C>) {
    let buffer = Rc::new(RefCell::new(CommandBuffer::new()));

    (CommandSender(buffer.clone().downgrade()), CommandReceiver(buffer.clone()))
}

/// Sends commands to a `CommandReceiver`.
///
/// You can have multiple senders per stream using the clone method.
pub struct CommandSender<C>(WeakSharedBuffer<C>);

impl<C> Clone for CommandSender<C> {
    fn clone(&self) -> CommandSender<C> {
        CommandSender(self.0.clone())
    }
}

impl<C> CommandSender<C> {
    /// Sends a command to the linked `CommandReceiver`.
    ///
    /// If the receiver was dropped, this method does nothing.
    pub fn send(&mut self, command: C) {
        if let Some(buffer) = self.0.upgrade() {
            buffer.borrow_mut().push(command);    
        }
    }
}

/// Receives commands from `CommandSender`s.
///
/// You can only have one receiver per stream.
pub struct CommandReceiver<C>(SharedBuffer<C>);

impl<C> CommandReceiver<C> {
    /// Retrieves a command from the stream.
    ///
    /// The insertion order is preserved.
    ///
    /// Returns `None` if there is no more commands to retrieve.
    pub fn recv(&mut self) -> Option<C> {
        self.0.borrow_mut().pop()
    }
}

type SharedBuffer<C> = Rc<RefCell<CommandBuffer<C>>>;
type WeakSharedBuffer<C> = Weak<RefCell<CommandBuffer<C>>>;

/// A simple buffer of commands using a `RingBuf`.
struct CommandBuffer<C> {
    commands: RingBuf<C>
}

impl<C> CommandBuffer<C> {
    /// Creates an empty `CommandBuffer`.
    fn new() -> CommandBuffer<C> {
        CommandBuffer {
            commands: RingBuf::new()
        }
    }

    /// Pushes a new command into the buffer.
    fn push(&mut self, command: C) {
        self.commands.push_back(command);
    }

    /// Tries to pop a command.
    ///
    /// Returns `None` if the buffer is empty.
    fn pop(&mut self) -> Option<C> {
        self.commands.pop_front()
    }
}
