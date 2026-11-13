//! Collection of Iterator adapters this crate provides.

use std::fmt;
use std::iter::FusedIterator;
use std::ops::{FromResidual, Residual, Try};

#[cfg(doc)]
use crate::IteratorExt;

/// An adapter returned on [`IteratorExt::map_and_then`]
///
/// Referenced for implementation: [`std::iter::Map`]
#[derive(Clone)]
pub struct MapAndThen<I, F> {
    pub(crate) iter: I,
    pub(crate) f: F,
}

impl<I, F> fmt::Debug for MapAndThen<I, F>
where
    I: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MapAndThen").field("iter", &self.iter).finish()
    }
}

impl<B, I, F> Iterator for MapAndThen<I, F>
where
    B: Try,
    I: Iterator,
    F: FnMut(<<I as Iterator>::Item as Try>::Output) -> B,
    <I as Iterator>::Item: Try,
    <<I as Iterator>::Item as Try>::Residual: Residual<<B as Try>::Output>,
    <<<I as Iterator>::Item as Try>::Residual as Residual<<B as Try>::Output>>::TryType: FromResidual<<B as Try>::Residual>,
{
    type Item = <<<I as Iterator>::Item as Try>::Residual as Residual<<B as Try>::Output>>::TryType;

    fn next(&mut self) -> Option<Self::Item> {
        let item = self.iter.next()?;
        Some(try bikeshed Self::Item { (self.f)(item?)? })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<B, I, F> DoubleEndedIterator for MapAndThen<I, F>
where
    B: Try,
    I: DoubleEndedIterator,
    F: FnMut(<<I as Iterator>::Item as Try>::Output) -> B,
    <I as Iterator>::Item: Try,
    <<I as Iterator>::Item as Try>::Residual: Residual<<B as Try>::Output>,
    <<<I as Iterator>::Item as Try>::Residual as Residual<<B as Try>::Output>>::TryType: FromResidual<<B as Try>::Residual>,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        let item = self.iter.next_back()?;
        Some(try bikeshed Self::Item { (self.f)(item?)? })
    }
}

impl<B, I, F> ExactSizeIterator for MapAndThen<I, F>
where
    B: Try,
    I: ExactSizeIterator,
    F: FnMut(<<I as Iterator>::Item as Try>::Output) -> B,
    <I as Iterator>::Item: Try,
    <<I as Iterator>::Item as Try>::Residual: Residual<<B as Try>::Output>,
    <<<I as Iterator>::Item as Try>::Residual as Residual<<B as Try>::Output>>::TryType: FromResidual<<B as Try>::Residual>,
{
    fn len(&self) -> usize {
        self.iter.len()
    }
}

impl<B, I, F> FusedIterator for MapAndThen<I, F>
where
    B: Try,
    I: FusedIterator,
    F: FnMut(<<I as Iterator>::Item as Try>::Output) -> B,
    <I as Iterator>::Item: Try,
    <<I as Iterator>::Item as Try>::Residual: Residual<<B as Try>::Output>,
    <<<I as Iterator>::Item as Try>::Residual as Residual<<B as Try>::Output>>::TryType: FromResidual<<B as Try>::Residual>,
{
}
