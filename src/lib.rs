mod application;
mod codec;
pub mod error;

// Common exports
// Example applications
#[cfg(feature = "motor-app")]
pub use application::motor::{KeyValueStoreApp, KeyValueStoreDriver};
pub use application::Application;
pub use error::Error;