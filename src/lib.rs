/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 */

#![cfg_attr(feature = "fatal-warnings", deny(warnings))]
#![deny(clippy::correctness)]
#![warn(clippy::pedantic)]
#![allow(clippy::match_bool)]
#![allow(clippy::if_not_else)]
#![allow(clippy::stutter)]
#![allow(clippy::similar_names)]
#![allow(clippy::use_self)]
#![allow(clippy::single_match_else)]
// Note: If you change this remember to update `README.md`.  To do so run `./tools/update-readme.sh`.
//! # Rust Persistent Data Structures
//!
//! Rust Persistent Data Structures provides [fully persistent data structures](https://en.wikipedia.org/wiki/Persistent_data_structure)
//! with structural sharing.
//!
//! ## Setup
//!
//! To use rpds add the following to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! rpds = "<version>"
//! ```
//!
//! ## Data structures
//!
//! This crate offers the following data structures:
//!
//!   1. [`List`](#list)
//!   2. [`Vector`](#vector)
//!   3. [`Stack`](#stack)
//!   4. [`Queue`](#queue)
//!   5. [`HashTrieMap`](#hashtriemap)
//!   6. [`HashTrieSet`](#hashtrieset)
//!   7. [`RedBlackTreeMap`](#redblacktreemap)
//!   8. [`RedBlackTreeSet`](#redblacktreeset)
//!
//! ### `List`
//! [![List documentation](https://img.shields.io/badge/doc-List-303070.svg)](./list/struct.List.html)
//!
//! Your classic functional list.
//!
//! #### Example
//!
//! ```rust
//! use rpds::List;
//!
//! let list = List::new().push_front("list");
//!
//! assert_eq!(list.first(), Some(&"list"));
//!
//! let a_list = list.push_front("a");
//!
//! assert_eq!(a_list.first(), Some(&"a"));
//!
//! let list_dropped = a_list.drop_first().unwrap();
//!
//! assert_eq!(list_dropped, list);
//! ```
//!
//! ### `Vector`
//! [![`Vector` documentation](https://img.shields.io/badge/doc-Vector-303070.svg)](./vector/struct.Vector.html)
//!
//! A sequence that can be indexed.  The implementation is described in
//! [Understanding Persistent Vector Part 1](http://hypirion.com/musings/understanding-persistent-vector-pt-1)
//! and [Understanding Persistent Vector Part 2](http://hypirion.com/musings/understanding-persistent-vector-pt-2).
//!
//! #### Example
//!
//! ```rust
//! use rpds::Vector;
//!
//! let vector = Vector::new()
//!     .push_back("I’m")
//!     .push_back("a")
//!     .push_back("vector");
//!
//! assert_eq!(vector[1], "a");
//!
//! let screaming_vector = vector
//!     .drop_last().unwrap()
//!     .push_back("VECTOR!!!");
//!
//! assert_eq!(screaming_vector[2], "VECTOR!!!");
//! ```
//!
//! ### `Stack`
//! [![`Stack` documentation](https://img.shields.io/badge/doc-Stack-303070.svg)](./stack/struct.Stack.html)
//!
//! A LIFO (last in, first out) data structure.  This is just a [`List`](#list) in disguise.
//!
//! #### Example
//!
//! ```rust
//! use rpds::Stack;
//!
//! let stack = Stack::new().push("stack");
//!
//! assert_eq!(stack.peek(), Some(&"stack"));
//!
//! let a_stack = stack.push("a");
//!
//! assert_eq!(a_stack.peek(), Some(&"a"));
//!
//! let stack_popped = a_stack.pop().unwrap();
//!
//! assert_eq!(stack_popped, stack);
//! ```
//!
//! ### `Queue`
//! [![`Queue` documentation](https://img.shields.io/badge/doc-Queue-303070.svg)](./queue/struct.Queue.html)
//!
//! A FIFO (first in, first out) data structure.
//!
//! #### Example
//!
//! ```rust
//! use rpds::Queue;
//!
//! let queue = Queue::new()
//!     .enqueue("um")
//!     .enqueue("dois")
//!     .enqueue("tres");
//!
//! assert_eq!(queue.peek(), Some(&"um"));
//!
//! let queue_dequeued = queue.dequeue().unwrap();
//!
//! assert_eq!(queue_dequeued.peek(), Some(&"dois"));
//! ```
//!
//! ### `HashTrieMap`
//! [![`HashTrieMap` documentation](https://img.shields.io/badge/doc-HashTrieMap-303070.svg)](./map/hash_trie_map/struct.HashTrieMap.html)
//!
//! A map implemented with a [hash array mapped trie](https://en.wikipedia.org/wiki/Hash_array_mapped_trie).
//! See [Ideal Hash Trees](https://infoscience.epfl.ch/record/64398/files/idealhashtrees.pdf) for
//! details.
//!
//! #### Example
//!
//! ```rust
//! use rpds::HashTrieMap;
//!
//! let map_en = HashTrieMap::new()
//!     .insert(0, "zero")
//!     .insert(1, "one");
//!
//! assert_eq!(map_en.get(&1), Some(&"one"));
//!
//! let map_pt = map_en
//!     .insert(1, "um")
//!     .insert(2, "dois");
//!
//! assert_eq!(map_pt.get(&2), Some(&"dois"));
//!
//! let map_pt_binary = map_pt.remove(&2);
//!
//! assert_eq!(map_pt_binary.get(&2), None);
//! ```
//!
//! ### `HashTrieSet`
//! [![`HashTrieSet` documentation](https://img.shields.io/badge/doc-HashTrieSet-303070.svg)](./set/hash_trie_set/struct.HashTrieSet.html)
//!
//! A set implemented with a [`HashTrieMap`](#hashtriemap).
//!
//! #### Example
//!
//! ```rust
//! use rpds::HashTrieSet;
//!
//! let set = HashTrieSet::new()
//!     .insert("zero")
//!     .insert("one");
//!
//! assert!(set.contains(&"one"));
//!
//! let set_extended = set.insert("two");
//!
//! assert!(set_extended.contains(&"two"));
//!
//! let set_positive = set_extended.remove(&"zero");
//!
//! assert!(!set_positive.contains(&"zero"));
//! ```
//!
//! ### `RedBlackTreeMap`
//! [![`RedBlackTreeMap` documentation](https://img.shields.io/badge/doc-RedBlackTreeMap-303070.svg)](./map/red_black_tree_map/struct.RedBlackTreeMap.html)
//!
//! A map implemented with a [red-black tree](https://en.wikipedia.org/wiki/Red-Black_tree).
//!
//! #### Example
//!
//! ```rust
//! use rpds::RedBlackTreeMap;
//!
//! let map_en = RedBlackTreeMap::new()
//!     .insert(0, "zero")
//!     .insert(1, "one");
//!
//! assert_eq!(map_en.get(&1), Some(&"one"));
//!
//! let map_pt = map_en
//!     .insert(1, "um")
//!     .insert(2, "dois");
//!
//! assert_eq!(map_pt.get(&2), Some(&"dois"));
//!
//! let map_pt_binary = map_pt.remove(&2);
//!
//! assert_eq!(map_pt_binary.get(&2), None);
//!
//! assert_eq!(map_pt_binary.first(), Some((&0, &"zero")));
//! ```
//!
//! ### `RedBlackTreeSet`
//! [![`RedBlackTreeSet` documentation](https://img.shields.io/badge/doc-RedBlackTreeSet-303070.svg)](./set/red_black_tree_set/struct.RedBlackTreeSet.html)
//!
//! A set implemented with a [`RedBlackTreeMap`](#redblacktreemap).
//!
//! #### Example
//!
//! ```rust
//! use rpds::RedBlackTreeSet;
//!
//! let set = RedBlackTreeSet::new()
//!     .insert("zero")
//!     .insert("one");
//!
//! assert!(set.contains(&"one"));
//!
//! let set_extended = set.insert("two");
//!
//! assert!(set_extended.contains(&"two"));
//!
//! let set_positive = set_extended.remove(&"zero");
//!
//! assert!(!set_positive.contains(&"zero"));
//!
//! assert_eq!(set_positive.first(), Some(&"one"));
//! ```
//!
//! ## Other features
//!
//! ### Mutable methods
//!
//! When you change a data structure you often do not need its previous versions.  For those cases
//! rpds offers you mutable methods which are generally faster:
//!
//! ```rust
//! use rpds::HashTrieSet;
//!
//! let mut set = HashTrieSet::new();
//!
//! set.insert_mut("zero");
//! set.insert_mut("one");
//!
//! let set_0_1 = set.clone();
//! let set_0_1_2 = set.insert("two");
//! ```
//!
//! ### Initialization macros
//!
//! There are convenient initialization macros for all data structures:
//!
//! ```rust
//! use rpds::*;
//!
//! let vector = vector![3, 1, 4, 1, 5];
//! let map = ht_map!["orange" => "orange", "banana" => "yellow"];
//! ```
//!
//! Check the documentation for initialization macros of other data structures.
//!
//! ### Thread safety
//!
//! All data structures in this crate can be shared between threads, but that is an opt-in ability.
//! This is because there is a performance cost to make data structures thread safe.  That cost
//! is worth avoiding when you are not actually sharing them between threads.
//!
//! Of course if you try to share a rpds data structure across different threads you can count on
//! the rust compiler to ensure that it is safe to do so.  If you are using the version of the data
//! structure that is not thread safe you will get a compile-time error.
//!
//! To create a thread-safe version of any data structure use `new_sync()`:
//!
//! ```rust
//! # /*DROP_LINE_IN_README*/ use rpds::Vector;
//! # /*DROP_LINE_IN_README*/
//! let vec = Vector::new_sync()
//!     .push_back(42);
//! ```
//!
//! Or use the `_sync` variant of the initialization macro:
//!
//! ```rust
//! # /*DROP_LINE_IN_README*/ use rpds::vector_sync;
//! # /*DROP_LINE_IN_README*/
//! let vec = vector_sync!(42);
//! ```
//!
//! #### Further details
//!
//! Internally the data structures in this crate maintain a lot of reference-counting pointers.
//! These pointers are used both for links between the internal nodes of the data structure as well
//! as for the values it stores.
//!
//! There are two implementations of reference-counting pointers in the standard library:
//! [`Rc`](https://doc.rust-lang.org/std/rc/struct.Rc.html) and
//! [`Arc`](https://doc.rust-lang.org/std/sync/struct.Arc.html).  They behave the same way, but
//! `Arc` allows you to share the data it points to across multiple threads.  The downside is that
//! it is significantly slower to clone and drop than `Rc`, and persistent data structures do a
//! lot of those operations. In some microbenchmarks with rpds data structure we can see that
//! using `Rc` instead of  `Arc` can make some operations twice as fast!  You can see this for
//! yourself by running `cargo bench`.
//!
//! To implement this we parameterize the type of reference-counting pointer (`Rc` or `Arc`) as a
//! type argument of the data structure.  We use the [archery](https://github.com/orium/archery/)
//! crate to do this in a convenient way.
//!
//! The pointer type can be parameterized like this:
//!
//! ```rust
//! # /*DROP_LINE_IN_README*/ use rpds::Vector;
//! # /*DROP_LINE_IN_README*/
//! let vec: Vector<u32, archery::ArcK> = Vector::new_with_ptr_kind();
//! //                              ↖
//! //                                This will use `Arc` pointers.
//! //                                Change it to `archery::RcK` to use a `Rc` pointer.
//! ```
//!
//! ### Serialization
//!
//! We support serialization through [serde](https://crates.io/crates/serde).  To use it
//! enable the `serde` feature.  To do so change the rpds dependency in your `Cargo.toml` to
//!
//! ```toml
//! [dependencies]
//! rpds = { version = "<version>", features = ["serde"] }
//! ```

mod utils;
#[macro_use]
pub mod list;
pub mod map;
pub mod queue;
pub mod set;
pub mod stack;
pub mod vector;

pub use crate::list::List;
pub use crate::list::ListSync;
pub use crate::map::hash_trie_map::HashTrieMap;
pub use crate::map::hash_trie_map::HashTrieMapSync;
pub use crate::map::red_black_tree_map::RedBlackTreeMap;
pub use crate::map::red_black_tree_map::RedBlackTreeMapSync;
pub use crate::queue::Queue;
pub use crate::queue::QueueSync;
pub use crate::set::hash_trie_set::HashTrieSet;
pub use crate::set::hash_trie_set::HashTrieSetSync;
pub use crate::set::red_black_tree_set::RedBlackTreeSet;
pub use crate::set::red_black_tree_set::RedBlackTreeSetSync;
pub use crate::stack::Stack;
pub use crate::stack::StackSync;
pub use crate::vector::Vector;
pub use crate::vector::VectorSync;
