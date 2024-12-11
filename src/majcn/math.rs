pub trait DigitCounter {
    fn count_digits(self) -> usize;
}

impl DigitCounter for u32 {
    fn count_digits(self) -> usize {
        if self < 10 {
            return 1;
        }

        if self < 100 {
            return 2;
        }

        if self < 1000 {
            return 3;
        }

        if self < 10000 {
            return 4;
        }

        if self < 100000 {
            return 5;
        }

        if self < 1000000 {
            return 6;
        }

        if self < 10000000 {
            return 7;
        }

        if self < 100000000 {
            return 8;
        }

        9
    }
}

impl DigitCounter for u64 {
    fn count_digits(self) -> usize {
        if self < 10 {
            return 1;
        }

        if self < 100 {
            return 2;
        }

        if self < 1000 {
            return 3;
        }

        if self < 10000 {
            return 4;
        }

        if self < 100000 {
            return 5;
        }

        if self < 1000000 {
            return 6;
        }

        if self < 10000000 {
            return 7;
        }

        if self < 100000000 {
            return 8;
        }

        if self < 1000000000 {
            return 9;
        }

        if self < 10000000000 {
            return 10;
        }

        if self < 100000000000 {
            return 11;
        }

        if self < 1000000000000 {
            return 12;
        }

        if self < 10000000000000 {
            return 13;
        }

        if self < 100000000000000 {
            return 14;
        }

        if self < 1000000000000000 {
            return 15;
        }

        if self < 10000000000000000 {
            return 16;
        }

        if self < 100000000000000000 {
            return 17;
        }

        if self < 1000000000000000000 {
            return 18;
        }

        19
    }
}
