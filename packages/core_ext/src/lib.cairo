pub mod collections;
pub mod snapshots;
pub mod span;
pub mod tuple;
pub use collections::{CollectionSnapForward, CollectionSplit};
pub use snapshots::{
    AsSnapshot, BaseType, EquivalentType, NestedSnapshot, Owned, SingleSnapshot, Snapshot,
    SnapshotOf, ToSnapshot,
};
pub use span::Spannable;
pub use tuple::{TupleSnapForward, TupleSnappable};

