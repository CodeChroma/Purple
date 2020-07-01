use amethyst::{
    assets::{Handle, Prefab},
    core::{
        math::{Vector2, Vector3},
        Named, Transform,
    },
    ecs::{Entities, Entity, LazyUpdate, ReadExpect},
    renderer::{sprite::SpriteSheetHandle, transparent::Transparent, SpriteRender},
};

pub fn load_fireball(
    world: &mut World,
    prefab: Handle<Prefab<AnimationPrefabData>>,
    ctx: &Context,
) {
    let mut transform = Transform::default();
}
