pub use self::manager::Manager;

pub mod manager;

pub trait Component: 'static + Send + Sync {}

// FIXME: Change this to a more generic trait
pub trait ComponentIndex: Component {
    fn of(_: Option<Self>) -> usize;
}
