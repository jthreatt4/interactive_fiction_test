use bevy::prelude::*;

// this isn't how we're currently representing the world
use crate::board::{
    CurrentBoard,
    components::Position
};

// this ins't the 2d vector library that we're using
use crate::vectors::Vector2Int;

use super::Action;

// example action
pub struct WalkAction(pub Entity, pub Vector2Int);
impl Action for WalkAction {
    fn execute(&self, world: &mut World) -> bool {
        let Some(board) = world.get_resource::<CurrentBoard>() else { return false };
        if !board.tiles.contains_key(&self.1) { return false };

        let Some(mut position) = world.get_mut::<Position>(self.0) else { return false };
        position.v = self.1;
        true
    }
}