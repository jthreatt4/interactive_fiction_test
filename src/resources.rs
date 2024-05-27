use bevy::prelude::*;

/*
Example turn queue
1. Say you have three events in your queue: Player [0], Enemy [0], and Turn [100]. The initiative is set in that order.
2. Player goes first, spends 120 time on their action, and the new queue order is Enemy [0], Turn [100], Player [120].
3. The first event in the queue is always the next one to take place, so Enemy goes next, and decides to perform an action that requires 50 time. The new queue order is Enemy [50], Turn [100], Player [120].
4. So now the enemy gets to act again because they are still at the front of the queue. This time they do something that requires 100 time. The new queue is now Turn [100], Player [120], Enemy [150].
5. At the front of the queue is… the Turn counter itself! So it handles any absolute turn updates, i.e. things that should happen “once per turn.” Then because each turn is set to be 100 time, the new queue is Player [120], Enemy [150], Turn [200].
6. So the player acts next, and so on… As you can see, the turn itself is an event/actor, just like the others. You can even add other types of events into the queue if you like, for example as of Beta 8 Cogmind has autonomous weapons that take their own actions independent of the turn counter or even their owner.
*/
// might need binary search for speed if sorting with the turn queue
#[derive(Resource)]
pub struct TurnQueue;

#[derive(Resource)]
pub struct GameLog {
    pub entries: Vec<String>
}

impl GameLog {
    pub fn new() -> Self {
        let mut log: Vec<String> = Vec::with_capacity(4);
        log.push("Log...\n".to_string());
        log.push("Use the arrow keys to move.\n".to_string());
        log.push("Bump into the enemies to attack them.\n".to_string());
        log.push("Find the amulet to win the game.\n".to_string());

        let entries_log = GameLog{
            entries: log
        };
        entries_log
    }

    pub fn add_entry(&mut self, message:String) {
        self.entries.push(message);
        self.entries.remove(0);
    }
}