#![feature(try_blocks_heterogeneous)]
#![feature(try_trait_v2)]
#![feature(try_trait_v2_residual)]

//! Experimental iterator expansions.
//!
//! This implementation is highly unstable, so currently versioning as `0.0.*`.
//!
//! This crate depends some unstable features. Listed below:
//!
//! - [`try_blocks_heterogeneous`](https://doc.rust-lang.org/unstable-book/language-features/try-blocks-heterogeneous.html)
//!   ... type annotated variant of [`try_blocks`](https://doc.rust-lang.org/unstable-book/language-features/try-blocks.html),
//!   used on internal implementation. It also used to verify that implementation based on natural type conversion.
//! - [`try_trait_v2`](https://doc.rust-lang.org/unstable-book/library-features/try-trait-v2.html)
//!   ... usage of [`Try`], [`FromResidual`]
//! - [`try_trait_v2_residual`](https://doc.rust-lang.org/unstable-book/library-features/try-trait-v2-residual.html)
//!   ... usage of [`Residual`]
//!
//! Other crate already provides similar functions:
//!
//! - [`itertools`](https://docs.rs/itertools/0.15.0/itertools/index.html)
//!   ... is most famous and trusted implementation, provides many of functions.
//!   This also has [`Itertools::map_ok`](https://docs.rs/itertools/0.15.0/itertools/trait.Itertools.html#method.map_ok),
//!   but it specialized on where `Item = Result<T, E>`.

use std::ops::{FromResidual, Residual, Try};

pub mod adapters;
use adapters::*;

/// An [`Iterator`] blanket implementation that provides extra methods.
///
/// Referenced for implementation: [`DoubleEndedIterator`], [`Itertools`](https://docs.rs/itertools/0.15.0/itertools/trait.Itertools.html)
pub trait IteratorExt: Iterator {
    /// Transforms each values over `?` bypassed, like `T` of `Ok(T)` or `Some(T)`.
    /// And also allowed to use covariant residual, like `F` of Err(F), where `E: From<F>`.
    ///
    /// Consisely does `f(item?)?`.
    ///
    /// ## Design notes
    ///
    /// *Homogeneous* `try_blocks` locks residual based on first appearance,
    /// but *Heterogeneous* `try_blocks` allows conversion into / from another residual.
    /// This method refers *Heterogeneous* variant, suppose some usages like:
    ///
    /// ```
    /// #![feature(iterator_try_collect)]
    ///
    /// use std::str::FromStr;
    ///
    /// use yaits::IteratorExt;
    ///
    /// let result: Result<Vec<i32>, anyhow::Error> = ["0", "1", "2"]
    ///     .into_iter()
    ///     // Item = &str
    ///     .map(anyhow::Ok)
    ///     // Item = Result<&str, anyhow::Error>
    ///     .map_and_then(str::parse::<i32>)
    ///     // Item = Result<i32, anyhow::Error>
    ///     .try_collect();
    ///
    /// assert_eq!(result.ok(), Some(vec![0, 1, 2]));
    /// ```
    ///
    /// ... Equivalent to:
    ///
    /// ```
    /// use std::str::FromStr;
    ///
    /// use itertools::Itertools;
    ///
    /// let result: Result<Vec<i32>, anyhow::Error> = ["0", "1", "2"]
    ///     .into_iter()
    ///     // Item = &str
    ///     .map(anyhow::Ok)
    ///     // Item = Result<&str, anyhow::Error>
    ///     .map_ok(str::parse::<i32>)
    ///     // Item = Result<Result<i32, std::num::ParseIntError>, anyhow::Error>
    ///     .map(|r| anyhow::Ok(r??))
    ///     // Item = Result<i32, anyhow::Error>
    ///     .try_collect();
    ///
    /// assert_eq!(result.ok(), Some(vec![0, 1, 2]));
    /// ```
    fn map_and_then<B, F>(self, f: F) -> MapAndThen<Self, F>
    // ... for example, let Result :: E -> T -> `Result<T, E>`
    where
        Self: Sized,
        // is: Result F U
        B: Try,
        // is: T -> Result F U
        F: FnMut(<Self::Item as Try>::Output) -> B,
        // is: Result E T
        Self::Item: Try,
        // on [here]: [Result E] * == [Result E] U
        <Self::Item as Try>::Residual: Residual<<B as Try>::Output>,
        // convertible: Result E U <- Result F *
        <<Self::Item as Try>::Residual as Residual<<B as Try>::Output>>::TryType: FromResidual<<B as Try>::Residual>,
        // ... now producible Item == Result E U
    {
        MapAndThen { iter: self, f }
    }
}

impl<I> IteratorExt for I where I: Iterator {}
