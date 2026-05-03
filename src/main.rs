
mod comp_data;
mod systems;

use bevy::prelude::*;
use comp_data::*;
use systems::*;







fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<Appstatus>()
        .add_systems(Startup, setup)
        .add_systems(OnEnter(Appstatus::Game), start_game)
        .add_systems(Update, start_button_systems.run_if(in_state(Appstatus::Menu)))
        .add_systems(Update, stop_button_systems.run_if(in_state(Appstatus::Menu)))
        .add_systems(OnEnter(Appstatus::Vollage), create_home_ui)
        .add_systems(Update,(
            check_zhuangbei_button,
            check_recovery_button,
            check_chuji_button,
            ).run_if(in_state(Appstatus::Vollage)))

        .add_systems(OnEnter(Appstatus::WorldMap), create_wordmap)
        .add_systems(Update, (
                check_back_vollage_button,
                check_word_map_button,
            ).run_if(in_state(Appstatus::WorldMap)))

        .add_systems(OnEnter(Appstatus::SearchEnemy), create_search_enemy_ui)
        .add_systems(Update, (
                rotate_ring,
                sync_back_size,
                text_flash,
                check_search_continue_button,
                check_search_back_button,
                check_search_attack_button,
            ).run_if(in_state(Appstatus::SearchEnemy)))
        .run();

}


fn setup(mut commands : Commands,
asset : Res<AssetServer>) {
    commands.spawn(Camera2d);
    commands.spawn(
        create_menu(&asset)
    );
    let db = CannonDataBase::load().unwrap();
    commands.insert_resource(db);

    let db = SecgunDataBase::load().unwrap();
    commands.insert_resource(db);

    let db = EngineDataBase::load().unwrap();
    commands.insert_resource(db);
}














