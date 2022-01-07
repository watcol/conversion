//! A collection of basic converters.

mod into;
mod map;

pub mod encoding;

pub use into::IntoConverter;
pub use map::{MapConverter, TryMapConverter};
