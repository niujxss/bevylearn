use bevy::prelude::*;

#[derive(Component)]
struct Tank;

#[derive(Component)]
struct Enemy {
    speed : f32,
}

#[derive(Component)]
struct Health {
    value : f32,
}


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (move_tank, move_enemies, check_collisions) )
        .run();
}

fn setup(mut commands: Commands) {    //Commands 用于创建或修改实体,Commands 是一个”命令缓冲区“ ，可以安全的修改游戏世界
    commands.spawn(Camera2d);  //创建2D相机实体，没有相机看不到东西
                                       //spawn() 方法用来创建一个实体，并向实体中添加组件

    commands.spawn((    //创建一个实体，这里添加了三个组件
        Sprite {             //渲染组件，外观
            color: Color::srgb(1.0, 0.0, 0.0), //颜色
            custom_size: Some(Vec2::new(50.0, 30.0)),    //宽50，高30 像素
            ..default() //其他属性默认
        },
        Transform::from_xyz(0.0, 0.0, 0.0), //变形组件
        Tank,
        Health { value : 100. },
    ));

    for i in 0..3 {
        commands.spawn(
        (
            Sprite {
                color:Color::srgb(0.0, 0.0, 1.0),
                custom_size : Some(Vec2::new(30.0, 20.0)),
                ..default()
            },
            Enemy { speed: 2.0 },
            Health { value: 30.0},

            Transform::from_xyz((i as f32 * 100.0) - 100.0, 100.0, 0.0),
        )
        );
    }


}


// 这里 Query 用来查找需要的组件，bevy会自动根据添加查找并填充好；
// Query 参数1，需要 Transform组件， 参数2： With增加过滤，在Transform组件基础上，同时有Tank组件的实体
fn move_tank(keyboard_input : Res< ButtonInput<KeyCode> >, mut query : Query< &mut Transform, With<Tank> >  ) {
    
    if let Ok(mut transform) = query.single_mut() { //这里single_mut 方法确保只有一个实体，如果有多个或没有实体，则会进入else
        
        let mut direction  = Vec3::ZERO;
        let speed = 5.0;
        if keyboard_input.pressed(KeyCode::KeyW) {
            direction.y += speed;
        }

        if keyboard_input.pressed(KeyCode::KeyS) {
            direction.y -= speed;
        }

        if keyboard_input.pressed(KeyCode::KeyA) {
            direction.x -= speed;
        }

        if keyboard_input.pressed(KeyCode::KeyD) {
            direction.x += speed;
        }

        if direction != Vec3::ZERO {
            direction = direction.normalize(); //标准化，确保同时两个方向时，速度还是speed；
            transform.translation += direction;
        }
    }
}


fn move_enemies(mut query : Query<&mut Transform , With<Enemy> > ) {
    for mut transform in &mut query {
        transform.translation.x += 0.5;
        if transform.translation.x > 400.0 || transform.translation.x < -400.0 {
            transform.translation.x = transform.translation.x.clamp(-400.0, 400.0);
            transform.translation.x -= 20.0;
        }
    }
}


fn check_collisions(tank_query : Query<&Transform, With<Tank>>,
                    enemy_query : Query<(&Transform,Entity), (With<Enemy>,Without<Tank>)> , mut commands : Commands ) {  //Entity 这里获取实体

    if let Ok(tank_transform) = tank_query.single() {
        for (enemy_transform, enemy_entry) in &enemy_query {
            let distance = tank_transform.translation.distance(enemy_transform.translation);
            if distance < 50.0 {
                println!("战车与敌人发生碰撞!");

                commands.entity(enemy_entry).despawn();
            }
        }
    }
}
