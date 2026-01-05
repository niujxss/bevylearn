use bevy::prelude::*;

mod components;
mod systems;
mod village;


use systems::*;
use village::*;


#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum GameState {
    #[default]
    MainMenu,
    Village,
    Dialogue,
    Shop,
    WorldMap,
    Battle,
}




fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<GameState>()
        .init_resource::<VillageManager>()
        .add_systems(Startup, setup)
        // .add_systems(Update, (
        //     move_tank,
        //     move_enemies,
        //     check_collisions,
        //     check_health,
        //     collision_effect_system,
        //     shoot_bullet,
        //     move_bullets,
        //     bullets_out_of_screen,
        //     hit_effect_system,
        //     bullet_hit_enemy,
        //     update_ui_test,
        

        // ) )
        .add_systems(Update, (
            move_tank.run_if(in_state(GameState::Village)),
            interact_with_npc.run_if(in_state(GameState::Village)),
            interact_with_building.run_if(in_state(GameState::Village)),
            move_tank,
        ))
        .run();
}


