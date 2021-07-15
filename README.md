## `fmt_ext`

![Crates.io](https://img.shields.io/crates/v/fmt_ext)
![docs.rs](https://img.shields.io/docsrs/fmt_ext)

A simple interface for extending external types with custom implementations of [Debug] and
[Display] traits.

The example below shows how implement custom debug formatting for slices that additionally print
their length.

```rust
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

// Look! Now we have just call `debug` method on the target type...
fn main() {
    let numbers = [0, 1, 2, 3];
    println!("{:?}", numbers.debug());

    let strings = vec!["I", "am", "a", "custom", "debug"];
    println!("{:?}", strings.debug());
}
```

All examples are in [this directory].

### Support of `#![no_std]`

`fmt_ext` supports `#![no_std]` mode by default.

### License

`fmt_ext` is distributed under the terms of both the MIT license.

[Debug]: https://doc.rust-lang.org/stable/std/fmt/trait.Display.html

[Display]: https://doc.rust-lang.org/stable/std/fmt/trait.Debug.html

[this directory]: https://github.com/ilyavenner/fmt-ext/tree/master/examples
