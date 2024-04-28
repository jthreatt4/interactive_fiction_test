use bevy::prelude::*;

mod components;
mod resources;

mod assets;
mod board;
mod camera;
mod globals;
mod graphics;
mod states;
mod vectors;

// mod prelude {
//     pub use bevy::prelude::*;
//     pub use bevy::window::PrimaryWindow;

//     pub use crate::components::*;
//     pub use crate::resources::*;
// }

// use prelude::*;

// fn setup(
//     mut commands: Commands,
//     asset_server: Res<AssetServer>,
//     mut atlases: ResMut<Assets<TextureAtlasLayout>>,
// ) {
//     // setup the sprite sheet
//     let texture_handle: Handle<Image> = asset_server.load("terminal8x8_transparent.png");
//     let layout =  TextureAtlasLayout::from_grid(Vec2::new(8.0, 8.0), 16, 16, None, None);
//     let layout_handle = atlases.add(layout);
//     // add sprite atlas as resource
//     commands.insert_resource(CharsetAsset { atlas: layout_handle.clone(), texture: texture_handle.clone() });

//     // Add a 2D camera
//     let mut cam = Camera2dBundle::default();
//     cam.transform.scale = Vec3::new(0.5, 0.5, 1.0);
//     commands.spawn((MainCamera, cam));
// }

fn main() {
    App::new()
    .add_plugins(
        DefaultPlugins.set(
            WindowPlugin {
                primary_window: Some(Window {
                    resolution: (
                        globals::WINDOW_WIDTH,
                        globals::WINDOW_HEIGHT
                    ).into(),
                    ..Default::default()
                }),
                ..Default::default()
            }
        ).set(
            ImagePlugin::default_nearest()
        )
    )
    .init_state::<states::MainState>()
    .add_plugin(assets::AssetPlugin)
    .add_plugin(board::BoardPlugin)
    .add_plugin(graphics::GraphicsPlugin)
    // .add_systems(Startup, setup)
    .add_startup_system(camera::setup)
    .run()
}
