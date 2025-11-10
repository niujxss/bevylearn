use bevy::prelude::*;

#[derive(Component)]
struct Tank;

#[derive(Component)]
struct Enemy {
    speed : f32,
}

#[derive(Component)]
struct Health {
    current : f32,
    max : f32,
}

#[derive(Component)]
struct CollisionDamage {
    amount : f32,
}


#[derive(Component)]
struct CollisionEffect {
    timer : Timer,
}

impl CollisionEffect {
    fn new() -> Self {
        CollisionEffect { timer: Timer::from_seconds(0.2, TimerMode::Once) }
    }
}


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (move_tank, move_enemies, check_collisions, check_health, collision_effect_system) )
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
        Health { current : 100. ,max : 100.0 },
        CollisionDamage { amount : 25.0 },
        )
    );

    for i in 0..3 {
        commands.spawn(
        (
            Sprite {
                color:Color::srgb(0.0, 0.0, 1.0),
                custom_size : Some(Vec2::new(30.0, 20.0)),
                ..default()
            },
            Enemy { speed: 2.0 },
            Health { current : 100. ,max : 100.0 },
            CollisionDamage { amount:10.0 },

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

//敌人移动
fn move_enemies( query_tank : Query<&Transform,(With<Tank> ,Without<Enemy> )> , mut query_enemy : Query<(&mut Transform ,&Enemy), With<Enemy> > , time : Res<Time> ) {
    if let Ok(tank_transform) = query_tank.single() {
        for (mut enemy_transform,enemy) in query_enemy.iter_mut() {
            let direction = (tank_transform.translation - enemy_transform.translation).normalize_or_zero(); //这里进行向量标准化，相当于只获取一个方向向量

            enemy_transform.translation += direction * enemy.speed * time.delta_secs(); // 这里乘以时间参数，根据不同帧率来设置速度，确保不同帧率下速度一致

            
        }
    }

}

//碰撞检测
fn check_collisions(mut tank_query : Query<(&Transform , &CollisionDamage , &mut Health ) , With<Tank>>,
                    mut enemy_query : Query<(&Transform , Entity , &CollisionDamage , &mut Health) , (With<Enemy> , Without<Tank>)> ,
                    mut commands : Commands ) {  //Entity 这里获取实体

    if let Ok((tank_transform , tank_damage ,mut tank_health)) = tank_query.single_mut() {
        
        for (enemy_transform, enemy_entry,enemy_damage, mut enemy_health ) in enemy_query.iter_mut() {
            let distance = tank_transform.translation.distance(enemy_transform.translation);
            if distance < 45.0 {
                println!("战车与敌人发生碰撞!");

                tank_health.current -= enemy_damage.amount;
                enemy_health.current -= tank_damage.amount;

                println!("战车状态：{}/{}",tank_health.current,tank_health.max);
                println!("怪物状态: {}/{}",enemy_health.current,enemy_health.max);

                commands.entity(enemy_entry).insert(CollisionEffect::new());

                
            }
        }
    }
}

//生命值检测
fn check_health(query : Query<(Entity , &Health )> , mut commands : Commands ) {
    for (entity,health) in query.iter() {
        if health.current <= 0.0 {
            println!("实体被摧毁!!");
            commands.entity(entity).despawn();
        }
    }
}


//碰撞效果

fn collision_effect_system(mut commands : Commands , 
                           mut query : Query<(Entity, &mut CollisionEffect, &mut Sprite)> ,
                           time : Res<Time> ) {
    for (entity , mut effect, mut sprite) in query.iter_mut() {
        effect.timer.tick(time.delta()); // 更新定时器数据

        if effect.timer.is_finished() {
            sprite.color = Color::srgb(0.0, 0.0, 1.0);
            commands.entity(entity).remove::<CollisionEffect>();
        } else {
            sprite.color = Color::srgb(1.0, 1.0, 1.0);
        }
    }
}
