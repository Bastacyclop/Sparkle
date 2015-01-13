pub use self::mapper::Mapper;

pub mod mapper;

pub trait Component: 'static {}

// FIXME: Change this to a more generic trait
pub trait ComponentIndex: Component {
    fn of(_: Option<Self>) -> usize;
}
