use std::{fmt, marker::PhantomData};

use fmt_ext::{debug::*, DebugExt};

// Create a type that will implement custom debug...
struct SliceWithLenDebug<T>(PhantomData<T>);

// Implement custom debug...
impl<T> CustomDebug for SliceWithLenDebug<T>
where
    T: fmt::Debug,
{
    type Target = [T];

    fn fmt_target(target: &Self::Target, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Slice {{ len: {}, items: {:?} }}", target.len(), target)
    }
}

// Attach custom debug implementation to the target type...
impl<T> AttachDebug<SliceWithLenDebug<T>> for [T] {}

// Look! We have just call `debug` method on the target type...
fn main() {
    let numbers = [0, 1, 2, 3];
    println!("{:?}", numbers.debug());

    let strings = vec!["I", "am", "a", "custom", "debug"];
    println!("{:?}", strings.debug());
}
