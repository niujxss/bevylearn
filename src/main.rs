use bevy::prelude::*;

mod components;
mod systems;


use systems::*;




fn main() {
    App::new()
        .add_plugins(DefaultPlugins) //设置默认的资源等
        .add_systems(Startup, (setup,setup_ui)) // 初始化时执行的系统
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
            update_ui_test,
            // 以上相当于一直循环执行的系统

        ) )
        .run();
}


