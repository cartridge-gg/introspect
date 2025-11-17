pub mod database {
    pub mod deserialize;
    pub mod events;
    pub use events::*;
}
pub mod event;
pub mod types;
pub use event::EventTrait;
