use std::ops::Range;
use legion::storage::ComponentStorage;
use legion::storage::ComponentTypeId;
use legion::storage::Component;
use legion::index::ComponentIndex;

mod physics;
pub use physics::RigidBodyComponent;
pub use physics::RigidBodyBoxComponentDef;
pub use physics::RigidBodyBallComponentDef;

mod draw;
pub use draw::DrawSkiaCircleComponent;
pub use draw::DrawSkiaCircleComponentDef;
pub use draw::DrawSkiaBoxComponent;
pub use draw::DrawSkiaBoxComponentDef;
pub use draw::PaintDef;
pub use draw::Paint;
