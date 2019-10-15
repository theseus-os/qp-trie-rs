#![no_std]

#[macro_use]
extern crate debug_unreachable;
extern crate unreachable;

#[cfg(feature = "serde")]
#[macro_use]
extern crate serde;

#[macro_use]
#[cfg(test)]
extern crate quickcheck;

#[macro_use] extern crate alloc;

#[cfg(feature = "serde")]
mod serialization;

mod entry;
mod iter;
mod node;
mod sparse;
mod subtrie;
mod trie;
mod util;

pub mod wrapper;

pub use entry::{Entry, OccupiedEntry, VacantEntry};
pub use iter::{Iter, IterMut, IntoIter};
pub use trie::{Trie, Break};
pub use subtrie::SubTrie;
