use std::fmt;

use fmt_ext::{display::*, DisplayExt};

// Oops, this type does not implement `Display`...
struct ExternCrateType(i32);

// Create a type that will implement custom display...
struct ExternCrateTypeDisplay;

// Implement custom display...
impl CustomDisplay for ExternCrateTypeDisplay {
    type Target = ExternCrateType;

    fn fmt_target(target: &Self::Target, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Extern type has a value - {}!", target.0)
    }
}

// Attach custom display implementation to the target type...
impl AttachDisplay<ExternCrateTypeDisplay> for ExternCrateType {}

// Look! We have just call `display` method on the target type...
fn main() {
    let foo = ExternCrateType(42);
    println!("{}", foo.display());
}
