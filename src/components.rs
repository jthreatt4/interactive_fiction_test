use bevy::prelude::*;

#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct Actor {
    pub turn_priority: i32,
}


#[derive(Component)]
pub struct Health {
    pub current: i32,
    pub max: i32
}