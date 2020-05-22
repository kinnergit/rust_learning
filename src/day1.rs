use std::ops::Add;

pub fn sum<T: Add<Output=T>>(a: T, b: T) -> T
{
    a + b
}

