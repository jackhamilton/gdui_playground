use godot::prelude::*;

struct RustExtension;

pub mod gdrust_trinkets;
pub mod test_control;

#[gdextension]
unsafe impl ExtensionLibrary for RustExtension {}
