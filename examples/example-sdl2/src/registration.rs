use minimum::resources::*;
use minimum::components::*;

use minimum::editor::EditorSelectRegistry;
use minimum::editor::EditorSelectRegistryBuilder;
use minimum::editor::EditorInspectRegistry;
use minimum::editor::EditorInspectRegistryBuilder;

use minimum::ComponentRegistry;

use minimum_skulpin::components::*;
use minimum_nphysics2d::components::*;

use atelier_assets::loader::rpc_loader::RpcLoader;

/// Create the asset manager that has all the required types registered
pub fn create_asset_manager(loader: RpcLoader) -> AssetResource {
    let mut asset_manager = AssetResource::new(loader);
    asset_manager.add_storage::<minimum::pipeline::PrefabAsset>();
    asset_manager
}

pub fn create_component_registry() -> ComponentRegistry {
    minimum::ComponentRegistryBuilder::new()
        .auto_register_components()
        .add_spawn_mapping_into::<DrawSkiaCircleComponentDef, DrawSkiaCircleComponent>()
        .add_spawn_mapping_into::<DrawSkiaBoxComponentDef, DrawSkiaBoxComponent>()
        .add_spawn_mapping::<RigidBodyBallComponentDef, RigidBodyComponent>()
        .add_spawn_mapping::<RigidBodyBoxComponentDef, RigidBodyComponent>()
        .add_spawn_mapping_into::<TransformComponentDef, TransformComponent>()
        .build()
}

pub fn create_editor_selection_registry() -> EditorSelectRegistry {
    EditorSelectRegistryBuilder::new()
        .register::<DrawSkiaBoxComponent>()
        .register::<DrawSkiaCircleComponent>()
        .register_transformed::<RigidBodyBoxComponentDef, RigidBodyComponent>()
        .register_transformed::<RigidBodyBallComponentDef, RigidBodyComponent>()
        .build()
}

pub fn create_editor_inspector_registry() -> EditorInspectRegistry {
    EditorInspectRegistryBuilder::default()
        .register::<DrawSkiaCircleComponentDef>()
        .register::<DrawSkiaBoxComponentDef>()
        .register::<TransformComponentDef>()
        .register::<RigidBodyBallComponentDef>()
        .register::<RigidBodyBoxComponentDef>()
        .build()
}
