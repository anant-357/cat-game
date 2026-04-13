mod cat;

pub use cat::{Cat, CatLocomotion, CatPlugin};

// Exported for the setup_camera ordering constraint in main.rs
pub use cat::setup_cat;
