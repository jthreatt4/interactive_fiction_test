use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_tiles::{commands::TileCommandExt, coords::CoordIterator, tiles_2d::*, TilesPlugin};
use leafwing_input_manager::prelude::*;

use graphics::CharsetAsset;

mod components;
mod resources;

// mod assets;
mod board;
mod globals;
mod graphics;
mod states;
mod vectors;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: (globals::WINDOW_WIDTH, globals::WINDOW_HEIGHT).into(),
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
        )
        .init_state::<states::MainState>()
        .add_plugins((
            // assets::AssetPlugin,
            board::BoardPlugin,
            graphics::GraphicsPlugin,
            TilesPlugin,
        ))
        .add_loading_state(
            LoadingState::new(states::MainState::LoadAssets)
                .continue_to_state(states::MainState::Game)
                .load_collection::<graphics::CharsetAsset>(),
        )
        // .add_systems(Startup, setup)
        .add_systems(OnEnter(states::MainState::Game), spawn)
        .add_systems(
            PostUpdate,
            sync_tile_transform.run_if(in_state(states::MainState::Game)),
        )
        .run();
}

#[derive(Component)]
struct Block;

#[derive(Component)]
struct GameLayer;

fn spawn(mut commands: Commands, charset_asset: Res<CharsetAsset>) {
    let glyph_color = Color::rgba(0.105, 0.470, 0.215, 1.0);
    // let cell_color = Color::rgba(0.352, 0.682, 0.380, 1.0);

    let sprite_bundle = SpriteBundle {
        sprite: Sprite {
            color: glyph_color,
            custom_size: Some(Vec2::splat(6.0)),
            ..Default::default()
        },
        // visibility: Visibility::Hidden,
        ..Default::default()
    };

    let mut map = commands.spawn_map(16, GameLayer);

    // let logo = r#"
    // eeeee  eeee e    e e    e       eeee8 eeeee e     eeee  eeeee
    // 8   8  8    8    8 8    8         8     8   8     8     8
    // 8eee8e 8ee  88  e8 8eeee8         8     e   8     8ee   8eeee
    // 8    8 8     8  8    88           8     8   8     8         8
    // 88eee8 88ee  8ee8    88   eeeee   e   88eee 88eee 88ee  8ee88 "#;

    // let logo = logo.split('\n').enumerate().flat_map(|(y, line)| {
    //     line.bytes().enumerate().filter_map(move |(x, byte)| {
    //         if byte == 56 || byte == 101 {
    //             Some([x as i32, 6-y as i32])
    //         } else {
    //             None
    //         }
    //     })
    // });

    // // spawn a 10 * 10 room
    // map.spawn_tile_batch(logo.collect::<Vec<[i32; 2]>>(), move |_| {
    //     (Block, sprite_bundle.clone())
    // });

    // spawn a 10 * 10 room
    map.spawn_tile_batch(
        CoordIterator::new([-5, 5], [5, 5])
            .chain(CoordIterator::new([-5, -5], [5, -5]))
            .chain(CoordIterator::new([5, -4], [5, 4]))
            .chain(CoordIterator::new([-5, -4], [-5, 4])),
        move |_| (Block, sprite_bundle.clone()),
    );

    // spawn the player character
    map.spawn_tile(
        IVec2::ZERO,
        (
            components::Actor { turn_priority: 1 },
            SpriteSheetBundle {
                sprite: Sprite {
                    // custom_size: Some(Vec2::splat(1.0)),
                    ..Default::default()
                },
                atlas: TextureAtlas {
                    layout: charset_asset.atlas.clone(),
                    index: '@' as usize,
                },
                texture: charset_asset.texture.clone(),
                ..Default::default()
            },
        ),
    );
}

fn move_character() {}

fn sync_tile_transform(mut tiles: Query<(&TileCoord, &mut Transform), Changed<TileCoord>>) {
    for (tile_c, mut transform) in tiles.iter_mut() {
        transform.translation.x = tile_c[0] as f32 * 16.0;
        transform.translation.y = tile_c[1] as f32 * 16.0;
    }
}
