use bevy::prelude::*;

#[derive(Component)]
pub struct Tank;

#[derive(Component)]
pub struct Enemy {
    pub speed : f32,
}

#[derive(Component)]
pub struct Health {
    pub current : f32,
    pub max : f32,
}

#[derive(Component)]
pub struct CollisionDamage {
    pub amount : f32,
}


#[derive(Component)]
pub struct CollisionEffect {
    pub timer : Timer,
}

#[derive(Component)]
pub struct Bullet {
    pub speed : f32,
    pub damage : f32,
    pub direction : Vec3,
}

#[derive(Component)]
pub struct PlayerBullet;

// 击中效果组件
#[derive(Component)]
pub struct HitEffect {
    pub timer: Timer,
}

impl CollisionEffect {
    pub fn new() -> Self {
        CollisionEffect { timer: Timer::from_seconds(0.2, TimerMode::Once) }
    }
}



impl HitEffect {
    pub fn new() -> Self {
        HitEffect { timer: Timer::from_seconds(0.1, TimerMode::Once) }
    }
}