use bevy::prelude::*;

fn main() {
    App::new()
            .add_plugins(DefaultPlugins)
            .add_plugins(HelloPlugin)
            .run();
}

fn helloworld() {
    println!("hello world!!");
}

#[derive(Component)]
struct Persion;


#[derive(Component)]
struct Name(String);


fn add_people(mut commands : Commands) {
    commands.spawn((Persion, Name("张三".to_string())));
    commands.spawn((Persion, Name("李四".to_string())));
    commands.spawn((Persion, Name("王麻子".to_string())));
}

fn greet_people(time : Res<Time>, mut timer : ResMut<GreetTimer>, query: Query<&Name , With<Persion> > ) {
    
    if timer.0.tick(time.delta()).just_finished() {
        for name in &query {
            println!("hello {}!",name.0);
        }
    }

}


pub struct HelloPlugin ;


impl Plugin for HelloPlugin {
    fn build(&self, app : &mut App) {
        app.insert_resource(GreetTimer(Timer::from_seconds(2.0, TimerMode::Repeating)));
        app.add_systems(Startup, add_people);
        app.add_systems(Update,  greet_people );
    }
}

#[derive(Resource)]
struct GreetTimer(Timer);
