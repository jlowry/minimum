#[allow(unused_imports)]
#[macro_use]
extern crate log;

#[allow(unused_imports)]
#[macro_use]
extern crate imgui;

//TODO: This is for selection, get rid of this when possible
extern crate nalgebra as na;
extern crate nalgebra_glm as glm;

mod select;
pub use select::EditorSelectRegistryBuilder;
pub use select::EditorSelectRegistry;
pub use select::EditorSelectable;
pub use select::EditorSelectableTransformed;

mod inspect;
pub use inspect::EditorInspectRegistryBuilder;
pub use inspect::EditorInspectRegistry;

pub mod components;
pub mod resources;
pub mod systems;
