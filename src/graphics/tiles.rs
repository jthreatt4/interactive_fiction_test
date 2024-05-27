// use bevy::prelude::*;

// use crate::board::components::{Position, Tile};
// use super::{GraphicAssets, TILE_SIZE};

// pub fn spawn_tile_renderer(
//     mut commands: Commands,
//     query: Query<(Entity, &Position), Added<Tile>>,
//     assets: Res<GraphicAssets>
// ) {
//     for (entity, position) in query.iter()  {
//         let layout  = TextureAtlasLayout::from_grid(Vec2::splat(TILE_SIZE), 16, 16, None, None);
//         let v = Vec3::new(
//             TILE_SIZE * position.v.x as f32,
//             TILE_SIZE * position.v.y as f32,
//             0,
//         );
//         commands.entity(entity)
//         .insert(SpriteSheetBundle {
//             sprite,
//             texture: assets.texture.clone(),
//             transform: Transform::from_translation(v),
//             ..Default::default()
//         });
//     }
// }