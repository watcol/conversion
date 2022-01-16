//! A collection of basic converters.

mod chained;
mod exact;
mod into;
mod iter;
mod map;

pub mod encoding;

pub use chained::ChainedConverter;
pub use exact::ExactConverter;
pub use into::IntoConverter;
pub use iter::{IterConverter, TryIterConverter};
pub use map::{MapConverter, TryMapConverter};
