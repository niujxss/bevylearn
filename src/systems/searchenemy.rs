use bevy::prelude::*;
use crate::comp_data::*;
use bevy::color::palettes::basic::*;
use bevy::window::WindowResized;

#[derive(Component)]
pub struct SpinningRing;

#[derive(Component)]
pub struct RingTimer(Timer);

#[derive(Component)]
pub struct TextTimer {
    pub timer: Timer,
    pub color_index: bool,
}

#[derive(Component)]
pub struct SearchBack;

#[derive(Component)]
pub struct SearchText;

#[derive(Component)]
pub struct SearchNode;

#[derive(Component)]
pub struct SearchButtonAttack;

#[derive(Component)]
pub struct SearchButtonContinue;

#[derive(Component)]
pub struct SearchButtonBack;

pub fn create_search_enemy_ui(mut commands: Commands, asset_server: Res<AssetServer>, windows: Query<&Window>) {

    let window = windows.single().unwrap(); 
    let window_width = window.width();
    let window_height = window.height();

    commands.spawn((
        DespawnOnExit(Appstatus::SearchEnemy),
        Sprite {
            image: asset_server.load("FeiTuHuangYuan_back.png"),
            custom_size: Some(Vec2::new(window_width, window_height)), // 填满窗口
            ..default()
        },
        SearchBack,
        Transform::from_xyz(0.0, 0.0, 0.0), 
    ));

    commands.spawn((
        DespawnOnExit(Appstatus::SearchEnemy),
        Sprite {
            image: asset_server.load("JinDuTiao.png"),
            custom_size: Some(Vec2::new(100.0, 100.0)),   // 设置精灵大小
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 1.0),           // 屏幕坐标 (x,y,z)
        SpinningRing,
        RingTimer(Timer::from_seconds(1.0, TimerMode::Once)),
    ));

    commands.spawn((
        DespawnOnExit(Appstatus::SearchEnemy),
        Node{
                width: percent(100),
                height: percent(30),
                position_type: PositionType::Absolute,
                bottom: px(0),
                flex_direction: FlexDirection::Column, //子元素 垂直排列
                justify_content: JustifyContent::Center, // 子节点水平居中 
                align_items: AlignItems::Center, // 子元素垂直居中
                ..default()

        },
        children![
            (
                Node {
                    width: percent(100),
                    height: percent(20),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    top: px(0),
                    ..default()
                },
                Text::new("正在扫荡该地区！！"),
                TextFont {
                    font: asset_server.load("fonts/STKAITI.TTF"),
                    font_size: 32.0,
                    ..default()
                },
                SearchText,
                TextLayout::new_with_justify(Justify::Center),
                TextColor(Color::srgb(0.9, 0.9, 0.9)), //文本颜色

            ),
            (
                Node {
                    width: percent(100),
                    height: percent(80),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    column_gap: px(20.0),
                    top: px(0),
                    ..default()
                },
                SearchNode,
            )
        ],
    )
    );
}


fn node_button_create(command: &mut Commands, entity: Entity) {
    command.entity(entity).with_children(|parent|{
        parent.spawn(
            (
                Button,
                Node {
                    width: px(200), // 宽 150像素
                    height: px(35), // 高65像素
                    border: UiRect::all(px(5)),  // UiRect 是四个边的矩形，all 是这是四周宽度都是5个像素；
                                                // border 定义边框宽度
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BorderColor::all(Color::WHITE), //边框颜色,白色
                BorderRadius::all(px(8.0)), // 8像素的圆角半径，更现代
                BackgroundColor(Color::BLACK),
                SearchButtonAttack,
                children![
                    (
                        Text::new("干他"),
                        TextFont {
                            font_size: 24.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.9, 0.9, 0.9)), //文本颜色
                    )
                ]
            )
        );

        parent.spawn(
            (
                Button,
                Node {
                    width: px(200), // 宽 150像素
                    height: px(35), // 高65像素
                    border: UiRect::all(px(5)),  // UiRect 是四个边的矩形，all 是这是四周宽度都是5个像素；
                                                // border 定义边框宽度
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BorderColor::all(Color::WHITE), //边框颜色,白色
                BorderRadius::all(px(8.0)), // 8像素的圆角半径，更现代
                BackgroundColor(Color::BLACK),
                SearchButtonContinue,
                children![
                    (
                        Text::new("避其锋芒"),
                        TextFont {
                            font_size: 24.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.9, 0.9, 0.9)), //文本颜色
                    )
                ]
            )
        );

        parent.spawn(
            (
                Button,
                Node {
                    width: px(200), // 宽 150像素
                    height: px(35), // 高65像素
                    border: UiRect::all(px(5)),  // UiRect 是四个边的矩形，all 是这是四周宽度都是5个像素；
                                                // border 定义边框宽度
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BorderColor::all(Color::WHITE), //边框颜色,白色
                BorderRadius::all(px(8.0)), // 8像素的圆角半径，更现代
                BackgroundColor(Color::BLACK),
                SearchButtonBack,
                children![
                    (
                        Text::new("返回大地图"),
                        TextFont {
                            font_size: 24.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.9, 0.9, 0.9)), //文本颜色
                    )
                ]
            )
        );
    });
}

pub fn rotate_ring(
    time: Res<Time>,
    mut ring_query: Query<(&mut Transform, &mut RingTimer), With<SpinningRing>>,
    mut text_query: Query<(Entity, &mut Text, &mut TextColor), With<SearchText>>,
    node_entity_query: Query<Entity, With<SearchNode>>,
    mut command: Commands,
) {
   for (mut transform, mut ring_timer) in ring_query.iter_mut() {
        ring_timer.0.tick(time.delta());


        if ring_timer.0.just_finished() {
            // 扫荡时间

            if let Ok((entity, mut text, mut textcolor)) = text_query.single_mut() {

                text.0 = format!("遭遇怪物！！！");
                textcolor.0 = Color::srgb(1.0, 0.0, 0.0);
                command.entity(entity).insert(TextTimer {
                    timer: Timer::from_seconds(0.2, TimerMode::Repeating),
                    color_index: true,
                });
                
            }

            if let Ok( node_entity) = node_entity_query.single() {
                node_button_create(&mut command, node_entity);
            }
        } else if ring_timer.0.is_finished() {
            continue;
        } else {
            // 每帧增量旋转（顺时针）
            let speed = std::f32::consts::TAU / ring_timer.0.duration().as_secs_f32();
            transform.rotate_z(-speed * time.delta_secs());
        }
    }
}

pub fn sync_back_size(
    mut resize_reader: MessageReader<WindowResized>,
    mut back_size_query: Query<&mut Sprite, With<SearchBack>>
) {
    for e in resize_reader.read() {
        let new_size = Vec2::new(e.width, e.height);
        for mut sprite in back_size_query.iter_mut() {
            sprite.custom_size = Some(new_size);
        }
    }
}

pub fn text_flash(
    time: Res<Time>,
    mut text_query: Query<( &mut TextColor, &mut TextTimer), With<SearchText>>,
) {
    if let Ok((mut text_color, mut timers )) = text_query.single_mut() {
        timers.timer.tick(time.delta());

        if timers.timer.just_finished() {
            if timers.color_index {
                timers.color_index = false;
                text_color.0 = Color::srgb(0.9, 0.9, 0.9);

            } else {
                timers.color_index = true;
                text_color.0 = Color::srgb(1.0, 0.0, 0.0);
            }
        }

    }
}

pub fn check_search_attack_button(
    mut interaction_query: Query<(&Interaction, &mut BorderColor), (Changed<Interaction>, With<SearchButtonAttack>)>,
    mut next_status: ResMut<NextState<Appstatus>>,
    
) {
    if let Ok((inter, mut border_color)) = interaction_query.single_mut() {
        match inter {
            Interaction::None => {
                border_color.set_all(WHITE);
            },
            Interaction::Hovered => {
                border_color.set_all(OLIVE);
            },
            Interaction::Pressed => {
                border_color.set_all(AQUA);
                next_status.set(Appstatus::War);

            }
        }
    }
}

pub fn check_search_continue_button(
    mut interaction_query: Query<(&Interaction, &mut BorderColor), (Changed<Interaction>, With<SearchButtonContinue>)>,
    entity_query: Query<Entity, With<SearchNode>>,
    mut commands: Commands,
    mut ring_query: Query<&mut RingTimer,With<SpinningRing>>,
    mut text_query: Query<(&mut Text, &mut TextColor, Entity), With<SearchText>>
) {
    if let Ok((inter, mut border_color)) = interaction_query.single_mut() {
        match inter {
            Interaction::None => {
                border_color.set_all(WHITE);
            },
            Interaction::Hovered => {
                border_color.set_all(OLIVE);
                
                
            },
            Interaction::Pressed => {
                border_color.set_all(AQUA);
                if let Ok(entity) = entity_query.single()
                {
                    commands.entity(entity).despawn_children();

                    if let Ok(mut ring) = ring_query.single_mut() {
                        ring.0.reset();

                        if let Ok((mut text, mut text_color,entity)) = text_query.single_mut() {
                            text.0 = format!("正在扫荡该地区！！");
                            text_color.0 = Color::srgb(0.9, 0.9, 0.9);
                            commands.entity(entity).remove::<TextTimer>();
                        }
                    }
                }

            }
        }
    }
}

pub fn check_search_back_button(
    mut interaction_query: Query<(&Interaction, &mut BorderColor), (Changed<Interaction>, With<SearchButtonBack>)>,
    mut next_status: ResMut<NextState<Appstatus>>,
) {
    if let Ok((inter, mut border_color)) = interaction_query.single_mut() {
        match inter {
            Interaction::None => {
                border_color.set_all(WHITE);
            },
            Interaction::Hovered => {
                border_color.set_all(OLIVE);
            },
            Interaction::Pressed => {
                border_color.set_all(AQUA);
                next_status.set(Appstatus::WorldMap);

            }
        }
    }
}