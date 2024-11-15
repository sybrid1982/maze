use bevy::prelude::*;
use bevy::render::view::RenderLayers;
use bevy::scene::SceneInstanceReady;

/// Currently [`RenderLayers`] are not applied to children of a scene.
/// This [`SceneInstanceReady`] observer applies the [`RenderLayers`]
/// of a [`SceneRoot`] to all children with a [`Transform`] and without a [`RenderLayers`].
///
/// See [#12461](https://github.com/bevyengine/bevy/issues/12461) for current status.
pub fn apply_render_layers_to_children(
    trigger: Trigger<SceneInstanceReady>,
    mut commands: Commands,
    children: Query<&Children>,
    transforms: Query<&Transform, Without<RenderLayers>>,
    query: Query<(Entity, &RenderLayers)>,
) {
    let Ok((parent, render_layers)) = query.get(trigger.entity()) else {
        return;
    };
    println!("applying to children");
    children.iter_descendants(parent).for_each(|entity| {
        if transforms.contains(entity) {
            commands.entity(entity).insert(render_layers.clone());
        }
    });
}
