pub mod fifo;
pub mod freq;
pub mod lru;
pub mod rand;
pub mod none;

pub use fifo::Fifo;
pub use lru::LRU;
pub use freq::Freq;
pub use rand::Rand;
pub use none::None;