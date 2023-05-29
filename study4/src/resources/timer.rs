use bevy::prelude::*;

const MINOMOVETIME: f32 = 0.7;

#[derive(Resource)]
pub struct MinoMovementTimer{
    pub timer: Timer
}
impl Default for MinoMovementTimer {
    fn default() -> MinoMovementTimer {
        MinoMovementTimer { timer: Timer::from_seconds(MINOMOVETIME, TimerMode::Repeating) }
    }
}