use bevy::prelude::*;

use super::{CharsetAsset, TILE_SIZE};

const ATLAS_PATH: &str = "terminal8x8_transparent.png";

pub fn load_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut atlases: ResMut<Assets<TextureAtlasLayout>>,
) {
    // Setup the sprite sheet
    let texture_handle: Handle<Image> = asset_server.load("terminal8x8_transparent.png");
    let layout = TextureAtlasLayout::from_grid(Vec2::splat(TILE_SIZE), 16, 16, None, None);
    let layout_handle = atlases.add(layout);
    // add sprite atlas as resource
    commands.insert_resource(CharsetAsset {
        atlas: layout_handle.clone(),
        texture: texture_handle.clone()
    });
}