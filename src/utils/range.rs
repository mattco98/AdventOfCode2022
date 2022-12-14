use std::ops::AddAssign;
use std::cmp::PartialOrd;
use std::fmt::Debug;
use num::Signed;

#[derive(Clone, Copy, Debug)]
pub struct Range<T : Clone + Copy + Debug> {
    start: T,
    end_inclusive: T,
    step: T,
}

fn difference_mod<T : Signed + PartialOrd + AddAssign + Copy + Debug>(a: T, b: T, c: T) -> T {
    ((a % c) - (b % c)) % c
}

#[allow(dead_code)]
impl<T : Signed + PartialOrd + AddAssign + Copy + Debug> Range<T> {
    fn new(start: T, end_inclusive: T, step: T) -> Range<T> {
        assert!(step != T::zero(), "Range step must be non-zero");
        Range { start, end_inclusive, step }
    }

    pub fn exclusive(start: T, stop: T) -> Range<T> {
        let step = Self::get_implicit_step_for(start, stop);
        let end_inclusive = if step > T::zero() {
            stop - T::one()
        } else {
            stop + T::one()
        };
        Self::new(start, end_inclusive, step)
    }

    pub fn exclusive_stepped(start: T, stop: T, step: T) -> Range<T> {
        let end_inclusive = if step > T::zero() {
            stop - T::one()
        } else {
            stop + T::one()
        };
        Self::new(start, end_inclusive, step)
    }

    pub fn inclusive(start: T, stop: T) -> Range<T> {
        Self::new(start, stop, Self::get_implicit_step_for(start, stop))
    }

    pub fn inclusive_stepped(start: T, stop: T, step: T) -> Range<T> {
        Self::new(start, stop, step)
    }

    pub fn len(&self) -> usize {
        // TODO: Calculate this
        self.iter().collect::<Vec<_>>().len()
    }

    pub fn iter(&self) -> RangeIterator<T> {
        let last =  if self.step > T::zero() {
            if self.start >= self.end_inclusive {
                self.end_inclusive
            } else {
                self.end_inclusive - difference_mod(self.end_inclusive, self.start, self.step)
            }
        } else if self.step < T::zero() {
            if self.start <= self.end_inclusive {
                self.end_inclusive
            } else {
                self.end_inclusive + difference_mod(self.start, self.end_inclusive, -self.step)
            }
        } else {
            unreachable!();
        };

        RangeIterator::new(self.start, last, self.step)
    }

    fn get_implicit_step_for(start: T, stop: T) -> T {
        if start > stop {
            -T::one()
        } else {
            T::one()
        }
    }
}

#[derive(Clone, Copy)]
pub struct RangeIterator<T : Clone + Copy> {
    step: T,
    final_element: T,
    has_next: bool,
    next: T,
}

impl<T : Signed + PartialOrd + AddAssign + Copy + Debug> RangeIterator<T> {
    fn new(first: T, last: T, step: T) -> Self {
        let has_next = if step > T::zero() { 
            first <= last
        } else {
            first >= last
        };

        RangeIterator {
            step,
            final_element: last,
            has_next,
            next: if has_next { first } else { last },
        }
    }
}

impl<T : Signed + PartialOrd + AddAssign + Copy + Debug> Iterator for RangeIterator<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if !self.has_next {
            None
        } else {
            let value = self.next;
            if value == self.final_element {
                self.has_next = false;
            } else {
                self.next += self.step;
            }

            Some(value)
        }
    }
}

impl<T: Signed + PartialOrd + AddAssign + Copy + Debug> IntoIterator for Range<T> {
    type Item = T;
    type IntoIter = RangeIterator<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

#[cfg(test)]
mod range_tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(Range::inclusive(0, 5).len(), 6);
        assert_eq!(Range::exclusive(0, 5).len(), 5);

        assert_eq!(Range::inclusive_stepped(0, 5, 2).len(), 3);
        assert_eq!(Range::exclusive_stepped(0, 5, 2).len(), 3);
        assert_eq!(Range::inclusive_stepped(0, 4, 2).len(), 3);
        assert_eq!(Range::exclusive_stepped(0, 4, 2).len(), 2);

        assert_eq!(Range::inclusive(0, -5).len(), 6);
        assert_eq!(Range::exclusive(0, -5).len(), 5);

        assert_eq!(Range::inclusive_stepped(0, -5, -2).len(), 3);
        assert_eq!(Range::exclusive_stepped(0, -5, -2).len(), 3);
        assert_eq!(Range::inclusive_stepped(0, -4, -2).len(), 3);
        assert_eq!(Range::exclusive_stepped(0, -4, -2).len(), 2);

        assert_eq!(Range::inclusive_stepped(0, -5, 2).len(), 0);
        assert_eq!(Range::exclusive_stepped(0, -5, 2).len(), 0);
        assert_eq!(Range::inclusive_stepped(0, -4, 2).len(), 0);
        assert_eq!(Range::exclusive_stepped(0, -4, 2).len(), 0);

        assert_eq!(Range::inclusive(0, 0).len(), 1);
        assert_eq!(Range::exclusive(0, 0).len(), 0);
    }
}
