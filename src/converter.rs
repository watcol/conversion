//! A collection of basic converters.

mod into;
mod iter;
mod map;

pub mod encoding;

pub use into::IntoConverter;
pub use iter::{IterConverter, TryIterConverter};
pub use map::{MapConverter, TryMapConverter};
