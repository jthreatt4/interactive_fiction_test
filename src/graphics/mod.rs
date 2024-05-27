use bevy::{prelude::*, utils::HashMap};
use bevy_asset_loader::prelude::*;

pub const TILE_SIZE: f32 = 32.;

mod assets;
mod camera;
mod tiles;

#[derive(AssetCollection, Resource)]
pub struct CharsetAsset {
    // if the sheet would have padding, you could set that with `padding_x` and `padding_y`
    // if there would be space between the top left corner of the sheet and the first sprite,
    // you could configure that with `offset_x` and `offset_y`
    // A texture atlas layout does not have a path as no asset file will be loaded for the layout
    #[asset(texture_atlas_layout(tile_size_x = 8., tile_size_y = 8., columns = 16, rows = 26))]
    pub atlas: Handle<TextureAtlasLayout>,
    #[asset(image(sampler = nearest))]
    #[asset(path = "terminal8x8_transparent.png")]
    pub texture: Handle<Image>,
}

// example resource of sprite assets
// #[derive(Resource)]
// pub struct SpriteSheetMaps {
// character_atlas: Handle<TextureAtlas>,
// icon_atlas: Handle<TextureAtlas>,
// pub characters: HashMap<Charater, usize>,
// pub weapons: HashMap<Weapon, usize>,
// pub icons: HashMap<Icon, usize>
// }

#[derive(Component)]
pub struct MainCamera;

pub struct GraphicsPlugin;

impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, camera::setup);
    }
}
