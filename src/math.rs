use std::ops::{Add, Div, Mul, Range, Sub};

pub fn map_range<T>(value: T, from: Range<T>, to: Range<T>) -> T
where
    T: Copy + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T>,
{
    to.start + (value - from.start) * (to.end - to.start) / (from.end - from.start)
}
