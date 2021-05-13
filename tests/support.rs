use std::fmt::{Debug, Display};

use eio::{FromBytes, ToBytes};

pub trait Num<const N: usize>: Debug + Display + PartialEq + FromBytes<N> + ToBytes<N> {}

impl<T, const N: usize> Num<N> for T where T: Debug + Display + PartialEq + FromBytes<N> + ToBytes<N>
{}

pub trait Abs {
    fn abs(self) -> Self;
}

pub trait Epsilon {
    fn epsilon() -> Self;
}

impl Abs for f32 {
    fn abs(self) -> Self {
        self.abs()
    }
}

impl Abs for f64 {
    fn abs(self) -> Self {
        self.abs()
    }
}

impl Epsilon for f32 {
    fn epsilon() -> Self {
        Self::EPSILON
    }
}

impl Epsilon for f64 {
    fn epsilon() -> Self {
        Self::EPSILON
    }
}
