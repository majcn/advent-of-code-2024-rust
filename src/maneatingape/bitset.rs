// source: https://github.com/maneatingape/advent-of-code-rust/blob/109bf05f8cb5026d97af42b42ea3985afe600dfb/src/util/bitset.rs

//! Add `biterator` method that treats an integer as a set, iterating over each element where
//! the respective bit is set. For example `1101` would return 0, 2 and 3.
use crate::maneatingape::integer::*;

pub trait BitOps<T> {
    fn biterator(self) -> Bitset<T>;
}

impl<T> BitOps<T> for T
where
    T: Integer<T>,
{
    fn biterator(self) -> Bitset<T> {
        Bitset { t: self }
    }
}

pub struct Bitset<T> {
    t: T,
}

impl<T> Iterator for Bitset<T>
where
    T: Integer<T>,
    T: TryInto<usize>,
{
    type Item = usize;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.t == T::ZERO {
            return None;
        }

        let tz = self.t.trailing_zeros();
        self.t = self.t ^ (T::ONE << tz);

        tz.try_into().ok()
    }
}
