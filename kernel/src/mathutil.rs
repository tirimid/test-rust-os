use core::fmt::Display;

pub fn clamp<T: Copy + PartialOrd + Display>(val: T, min: T, max: T) -> T {
    if min > max {
        panic!("min ({min}) is greater than max ({max})");
    }
    
    if val < min {
        min
    } else if val > max {
        max
    } else {
        val
    }
}
