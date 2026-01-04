use bevy::prelude::*;

#[derive(Component)]
pub struct Tank;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Human;

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

impl CollisionEffect {
    pub fn new() -> Self {
        CollisionEffect { timer: Timer::from_seconds(0.2, TimerMode::Once) }
    }
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

impl HitEffect {
    pub fn new() -> Self {
        HitEffect { timer: Timer::from_seconds(0.1, TimerMode::Once) }
    }
}


#[derive(Component)]
pub struct HealthText; //生命值文本


#[derive(Component)]
pub struct MoneyText; //金币文本


#[derive(Component)]
pub struct PlayerHUD; //HUD 标记
