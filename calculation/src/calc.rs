use std::ops::Add;

pub fn calc<T: Add<Output=T>>(a: T, b: T) -> T {
    a + b
}