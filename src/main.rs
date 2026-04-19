
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
        .add_systems(Update, start_button_systems.run_if(in_state(Appstatus::Menu)))
        .add_systems(Update, stop_button_systems.run_if(in_state(Appstatus::Menu)))
        .add_systems(OnEnter(Appstatus::Vollage), create_home_ui)
        .add_systems(Update, 
            check_zhuangbei_button.run_if(in_state(Appstatus::Vollage)))
        .run();

}


fn setup(mut commands : Commands,
asset : Res<AssetServer>) {
    commands.spawn(Camera2d);
    commands.spawn(
        create_menu(&asset)
    );
}














