use bevy::prelude::*;

mod components;
mod systems;


use systems::*;




fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (
            move_tank,
            move_enemies,
            check_collisions,
            check_health,
            collision_effect_system,
            shoot_bullet,
            move_bullets,
            bullets_out_of_screen,
            hit_effect_system,
            bullet_hit_enemy,

        ) )
        .run();
}


