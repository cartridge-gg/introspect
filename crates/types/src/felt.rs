use starknet_types_core::felt::Felt;

use crate::DecodeError;

pub trait FeltSource {
    fn next(&mut self) -> Result<Felt, DecodeError>;
    fn position(&self) -> usize;
}

pub struct SliceFeltSource<'a> {
    felts: &'a [Felt],
    pos: usize,
}

pub struct VecFeltSource {
    felts: Vec<Felt>,
    pos: usize,
}

pub struct FeltIterator<I> {
    iter: I,
    pos: usize,
}

impl<S: FeltSource + ?Sized> FeltSource for &mut S {
    #[inline]
    fn next(&mut self) -> Result<Felt, DecodeError> {
        (**self).next()
    }

    #[inline]
    fn position(&self) -> usize {
        (**self).position()
    }
}

pub trait IntoFeltSource {
    type Source: FeltSource;
    fn into_source(self) -> Self::Source;
}

impl IntoFeltSource for Vec<Felt> {
    type Source = VecFeltSource;
    fn into_source(self) -> Self::Source {
        VecFeltSource::new(self)
    }
}

impl<'a> IntoFeltSource for &'a Vec<Felt> {
    type Source = SliceFeltSource<'a>;
    fn into_source(self) -> Self::Source {
        SliceFeltSource::new(self)
    }
}

impl<'a> IntoFeltSource for &'a [Felt] {
    type Source = SliceFeltSource<'a>;
    fn into_source(self) -> Self::Source {
        SliceFeltSource::new(self)
    }
}

impl<'a, S: FeltSource + ?Sized> IntoFeltSource for &'a mut S {
    type Source = &'a mut S;
    #[inline]
    fn into_source(self) -> Self::Source {
        self
    }
}

impl VecFeltSource {
    pub fn new(felts: Vec<Felt>) -> Self {
        Self { felts, pos: 0 }
    }
}

impl FeltSource for VecFeltSource {
    fn next(&mut self) -> Result<Felt, DecodeError> {
        let f = *self.felts.get(self.pos).ok_or(DecodeError::Eof)?;
        self.pos += 1;
        Ok(f)
    }

    fn position(&self) -> usize {
        self.pos
    }
}

impl<'a> SliceFeltSource<'a> {
    #[inline]
    pub fn new(felts: &'a [Felt]) -> Self {
        Self { felts, pos: 0 }
    }

    #[inline]
    pub fn remaining(&self) -> usize {
        self.felts.len().saturating_sub(self.pos)
    }
}

impl<'a> FeltSource for SliceFeltSource<'a> {
    #[inline]
    fn next(&mut self) -> Result<Felt, DecodeError> {
        let f = *self.felts.get(self.pos).ok_or(DecodeError::Eof)?;
        self.pos += 1;
        Ok(f)
    }

    #[inline]
    fn position(&self) -> usize {
        self.pos
    }
}

impl<I> FeltIterator<I> {
    #[inline]
    pub fn new(iter: I) -> Self {
        Self { iter, pos: 0 }
    }
}

impl<I: Iterator<Item = Felt>> FeltSource for FeltIterator<I> {
    #[inline]
    fn next(&mut self) -> Result<Felt, DecodeError> {
        match self.iter.next() {
            Some(f) => {
                self.pos += 1;
                Ok(f)
            }
            None => Err(DecodeError::Eof),
        }
    }

    #[inline]
    fn position(&self) -> usize {
        self.pos
    }
}
