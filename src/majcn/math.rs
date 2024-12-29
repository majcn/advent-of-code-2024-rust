pub trait SumOfNaturalNumbers<T> {
    fn sum_of_natural_numbers(self) -> T;
}

impl SumOfNaturalNumbers<usize> for usize {
    #[inline]
    fn sum_of_natural_numbers(self) -> usize {
        self * (self + 1) / 2
    }
}

pub trait DigitCounter {
    fn count_digits(self) -> usize;
}

impl DigitCounter for u32 {
    #[inline]
    fn count_digits(self) -> usize {
        match self {
            0..10 => 1,
            10..100 => 2,
            100..1000 => 3,
            1000..10000 => 4,
            10000..100000 => 5,
            100000..1000000 => 6,
            1000000..10000000 => 7,
            10000000..100000000 => 8,
            _ => 9,
        }
    }
}

impl DigitCounter for u64 {
    #[inline]
    fn count_digits(self) -> usize {
        match self {
            0..10 => 1,
            10..100 => 2,
            100..1000 => 3,
            1000..10000 => 4,
            10000..100000 => 5,
            100000..1000000 => 6,
            1000000..10000000 => 7,
            10000000..100000000 => 8,
            100000000..1000000000 => 9,
            1000000000..10000000000 => 10,
            10000000000..100000000000 => 11,
            100000000000..1000000000000 => 12,
            1000000000000..10000000000000 => 13,
            10000000000000..100000000000000 => 14,
            100000000000000..1000000000000000 => 15,
            1000000000000000..10000000000000000 => 16,
            10000000000000000..100000000000000000 => 17,
            100000000000000000..1000000000000000000 => 18,
            _ => 19,
        }
    }
}