#![allow(dead_code)]

use std::ops::{BitAnd, BitOr, BitOrAssign, Not, Shl, Shr};

pub trait Bitwise:
    BitAnd<Output = Self>
    + BitOr<Output = Self>
    + BitOrAssign
    + Not<Output = Self>
    + Eq
    + Shr<usize, Output = Self>
    + Shl<usize, Output = Self>
    + Copy
    + Clone
{
    fn zero() -> Self;
}

macro_rules! impl_bitwise_for {
    ($type:ty) => {
        impl Bitwise for $type {
            #[inline]
            fn zero() -> Self {
                0
            }
        }
    };
}

impl_bitwise_for!(u8);
impl_bitwise_for!(u16);
impl_bitwise_for!(u32);

pub struct Field<T: Bitwise> {
    mask: T,
    shift: usize,
}

impl<T: Bitwise> Field<T> {
    #[inline]
    pub fn new(mask: T, shift: usize) -> Self {
        Self { mask, shift }
    }

    #[inline]
    pub fn read(self, val: T) -> T {
        (val & (self.mask << self.shift)) >> self.shift
    }

    #[allow(clippy::wrong_self_convention)]
    #[inline]
    pub fn is_set(self, val: T) -> bool {
        (val & (self.mask << self.shift)) != T::zero()
    }

    pub fn read_as_enum<E: TryFromValue<T, EnumType = E>>(self, val: T) -> Option<E> {
        E::try_from_value(self.read(val))
    }
}

pub trait TryFromValue<V> {
    type EnumType;

    fn try_from_value(value: V) -> Option<Self::EnumType>;
}

#[derive(Copy, Clone)]
pub struct FieldValue<T: Bitwise> {
    mask: T,
    value: T,
}

impl<T: Bitwise> FieldValue<T> {
    #[inline]
    pub fn mask(&self) -> T {
        self.mask as T
    }

    pub fn get(&self) -> T {
        self.value
    }

    #[inline]
    pub fn read(&self, field: Field<T>) -> T {
        field.read(self.value)
    }

    #[inline]
    pub fn modify(self, val: T) -> T {
        (val & !self.mask) | self.value
    }

    #[inline]
    pub fn matches(&self, val: T) -> bool {
        val & self.mask == self.value
    }
}

impl<T: Bitwise> std::ops::Add for FieldValue<T> {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self {
        FieldValue {
            mask: self.mask | rhs.mask,
            value: self.value | rhs.value,
        }
    }
}

pub struct Register<T: Bitwise> {
    pub value: T,
}

impl<T: Bitwise> Register<T> {
    pub fn new(value: T) -> Self {
        Self { value }
    }

    #[inline]
    pub fn get(&self) -> T {
        self.value
    }

    #[inline]
    pub fn set(&mut self, value: T) {
        self.value = value
    }

    #[inline]
    pub fn read(&self, field: Field<T>) -> T {
        field.read(self.get())
    }

    #[inline]
    pub fn read_as_enum<E: TryFromValue<T, EnumType = E>>(self, field: Field<T>) -> Option<E> {
        field.read_as_enum(self.get())
    }

    #[inline]
    pub fn is_set(&self, field: Field<T>) -> bool {
        field.is_set(self.get())
    }

    #[inline]
    pub fn write(&mut self, field: FieldValue<T>) {
        self.set(field.get());
    }

    #[inline]
    pub fn modify(&mut self, field: FieldValue<T>) {
        self.set(field.modify(self.get()));
    }

    #[inline]
    pub fn matches(&self, field: FieldValue<T>) -> bool {
        field.matches(self.get())
    }
}
