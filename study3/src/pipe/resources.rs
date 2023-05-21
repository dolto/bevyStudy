use bevy::prelude::*;

const PIPSPAWNTIME:f32 = 3.5;
pub const STARTPIPSPEED:f32 = 140.0;

#[derive(Resource)]
pub struct PipSpawnTimer{
    pub timer: Timer
}
impl Default for PipSpawnTimer{
    fn default() -> PipSpawnTimer {
        PipSpawnTimer { timer: Timer::from_seconds(PIPSPAWNTIME, TimerMode::Repeating) }
    }
}

#[derive(Resource)]
pub struct PipMoveSpeed{
    pub x_speed: f32
}
impl Default for PipMoveSpeed {
    fn default() -> PipMoveSpeed {
        PipMoveSpeed { x_speed: STARTPIPSPEED }
    }
}