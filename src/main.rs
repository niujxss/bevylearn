
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
        .add_systems(Update, load_button_systems.run_if(in_state(Appstatus::Menu)))
        .add_systems(OnEnter(Appstatus::Vollage), create_home_ui)
        .add_systems(Update,(
            check_zhuangbei_button,
            check_recovery_button,
            check_update_button,
            check_forge_button,
            check_craft_button,
            check_chuji_button,
            check_beibao_button,
            check_save_button,
            check_cangku_button,
            close_warehouse,
            handle_deposit,
            handle_withdraw,
            refresh_warehouse_system,
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

        .add_systems(OnEnter(Appstatus::War), create_battle_ui)
        .add_systems(Update, (
                update_battle_display,
                check_fire_button,
                check_secfire_button,
                check_item_button,
                check_use_item_buttons,
                check_retreat_button,
                check_battle_result,
                check_result_button,
            ).run_if(in_state(Appstatus::War)))

        .add_systems(OnEnter(Appstatus::Upgrade), create_upgrade_ui)
        .add_systems(Update, (
                update_upgrade_display,
                check_upgrade_button,
                check_upgrade_back_button,
            ).run_if(in_state(Appstatus::Upgrade)))

        .add_systems(OnEnter(Appstatus::Forge), create_forge_ui)
        .add_systems(Update, (
                refresh_forge_recipes,
                update_forge_display,
                check_forge_recipe_buttons,
                check_forge_do_button,
                check_forge_back_button,
            ).run_if(in_state(Appstatus::Forge)))

        .add_systems(OnEnter(Appstatus::Craft), create_craft_ui)
        .add_systems(Update, (
                refresh_craft_recipes,
                update_craft_display,
                check_craft_recipe_buttons,
                check_craft_do_button,
                check_craft_back_button,
            ).run_if(in_state(Appstatus::Craft)))

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

    let db = EnemyDataBase::load().unwrap();
    commands.insert_resource(db);

    let db = UpgradeDataBase::load().unwrap();
    commands.insert_resource(db);

    let db = ForgeDataBase::load().unwrap();
    commands.insert_resource(db);

    let db = ItemDataBase::load().unwrap();
    commands.insert_resource(db);
}
