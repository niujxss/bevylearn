use bevy::prelude::*;
use crate::comp_data::*;
use bevy::color::palettes::basic::*;

#[derive(Component)]
struct VollageUi;

#[derive(Component)]
struct MiscUi;

#[derive(Component)]
pub struct ChuJiButton;

#[derive(Component)]
struct UpdateButton;

#[derive(Component)]
pub struct RecoveryButton;

#[derive(Component)]
pub struct ZhuangBeiButton;

#[derive(Component)]
pub struct BeiBaoButton;

#[derive(Component)]
pub struct VollageMessage;


fn update_player_maxhealth(player: &mut Player, cannon: &Cannon, secgun: &SecGun, engine: &Engine) {
    let mut cannone_weight = 0.;
    let mut secgun_weight = 0.;

    if cannon.available {
        cannone_weight = cannon.weight;
    }

    if secgun.available {
        secgun_weight = secgun.weight;
    }

    let zhongliang = engine.weight - cannone_weight - secgun_weight;
    let health = (zhongliang * 100.0) as i32;
    player.max_health = health; 
}

pub fn create_home_ui(mut commands: Commands, asset: Res<AssetServer>, mut player: Query<(&mut Player, &Cannon, &SecGun, &Engine)>) {

    let image: Handle<Image> = asset.load("vollage.png");
    let tank_image = asset.load("tank.png");

    if let Ok((mut player,cannon, secgun, engine)) = player.single_mut() {
        update_player_maxhealth(&mut player, cannon, secgun, engine);
    }
    
    commands.spawn((
        VollageUi,
        DespawnOnExit(Appstatus::Vollage), // 离开该状态时销毁该实体
        Node {
            width: percent(100), // 父节点，没有父节点，所以父节点是窗口
            height: percent(60),
            position_type: PositionType::Absolute, // 绝对定位
            top: px(0),
            align_items: AlignItems::FlexEnd, // 子节点垂直贴底 （垂直方向上底端对齐）
            justify_content: JustifyContent::Center, // 子节点水平居中 
            flex_direction: FlexDirection::Row, // 子元素水平排列， 从左到右排列， 还可以选择 column ,垂直排列，从上到下
            row_gap: px(20.0), // 垂直间隔
            column_gap: px(20.0), // 水平间隔
            padding: UiRect::all(px(20.0)), // 内边距20
            ..default()
        },
        Text::new("幸存者基地"),
        TextFont {
            font: asset.load("fonts/STKAITI.TTF"),
            font_size: 33.0,
            ..default()
        },
        TextColor(Color::srgb(0.9, 0.9, 0.9)), //文本颜色
        ImageNode::new(image),
        children![
            (
                // 出击
                Button,
                ChuJiButton,
                Node {
                    width: px(150), // 宽 150像素
                    height: px(65), // 高65像素
                    border: UiRect::all(px(5)),  // UiRect 是四个边的矩形，all 是这是四周宽度都是5个像素；
                                                 // border 定义边框宽度
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                StopGameUi,
                BorderColor::all(Color::WHITE), //边框颜色,白色
                BorderRadius::all(px(8.0)), // 8像素的圆角半径，更现代
                BackgroundColor(Color::BLACK),
                children![
                    (
                        Text::new("出击"),
                        TextFont {
                            font: asset.load("fonts/STKAITI.TTF"),
                            font_size: 33.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.9, 0.9, 0.9)), //文本颜色
                    )
                ]
            ),
            (
                // 升级
                UpdateButton,
                Button,
                Node {
                    width: px(150), // 宽 150像素
                    height: px(65), // 高65像素
                    border: UiRect::all(px(5)),  // UiRect 是四个边的矩形，all 是这是四周宽度都是5个像素；
                                                 // border 定义边框宽度
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                StopGameUi,
                BorderColor::all(Color::WHITE), //边框颜色,白色
                BorderRadius::all(px(8.0)), // 8像素的圆角半径，更现代
                BackgroundColor(Color::BLACK),
                children![
                    (
                        Text::new("升级"),
                        TextFont {
                            font: asset.load("fonts/STKAITI.TTF"),
                            font_size: 33.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.9, 0.9, 0.9)), //文本颜色
                    )
                ]
            ),
            (
                // 补充
                UpdateButton,
                Button,
                Node {
                    width: px(150), // 宽 150像素
                    height: px(65), // 高65像素
                    border: UiRect::all(px(5)),  // UiRect 是四个边的矩形，all 是这是四周宽度都是5个像素；
                                                 // border 定义边框宽度
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                StopGameUi,
                RecoveryButton,
                BorderColor::all(Color::WHITE), //边框颜色,白色
                BorderRadius::all(px(8.0)), // 8像素的圆角半径，更现代
                BackgroundColor(Color::BLACK),
                children![
                    (
                        Text::new("充能"),
                        TextFont {
                            font: asset.load("fonts/STKAITI.TTF"),
                            font_size: 33.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.9, 0.9, 0.9)), //文本颜色
                    )
                ]
            ),
        ],
    ));
    commands.spawn(
    (
        MiscUi,
        DespawnOnExit(Appstatus::Vollage),
        Node {
            width: percent(100), // 父节点，没有父节点，所以父节点是窗口
            height: percent(40),
            position_type: PositionType::Absolute, // 绝对定位
            bottom: px(0),
            align_items: AlignItems::FlexEnd, // 子节点垂直贴底 
            //justify_content: JustifyContent::Center, // 子节点水平居中 
            //flex_direction: FlexDirection::Row, // 子元素水平排列
            // row_gap: px(20.0), // 垂直间隔
            // column_gap: px(20.0), // 水平间隔
            //padding: UiRect::all(px(20.0)), // 内边距20
            ..default()
        },
        children![
            (
                // 战车图片
                Node {
                    width: percent(30),
                    height: percent(100),
                    left: px(0),
                    ..default()
                },
                ImageNode::new(tank_image),

            ),
            (
                // 信息按钮
                Node {
                    width: percent(20),
                    height: percent(100),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    flex_direction: FlexDirection::Column,
                    row_gap: px(20.0),
                    ..default()
                },
                BackgroundColor(GRAY.into()),
                children![
                    (
                        // 装备信息
                        Button,
                        ZhuangBeiButton,
                        Node {
                            width: px(100), // 宽 150像素
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
                        children![
                            (
                                Text::new("装备"),
                                TextFont {
                                    font: asset.load("fonts/STKAITI.TTF"),
                                    font_size: 16.0,
                                    ..default()
                                },
                                TextColor(Color::srgb(0.9, 0.9, 0.9)), //文本颜色
                                //TextShadow::default()  //字体阴影
                                
                            )
                        ]
                    ),
                    (
                        // 背包信息
                        BeiBaoButton,
                        Button,
                        Node {
                            width: px(100), // 宽 150像素
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
                        children![
                            (
                                Text::new("背包"),
                                TextFont {
                                    font: asset.load("fonts/STKAITI.TTF"),
                                    font_size: 16.0,
                                    ..default()
                                },
                                TextColor(Color::srgb(0.9, 0.9, 0.9)), //文本颜色
                                //TextShadow::default()  //字体阴影
                                
                            )
                        ]
                    ),
                ],
            ),
            (
                // 文本框
                Node {
                    width: percent(50),
                    height: percent(100),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(SILVER.into()),
                children![
                (
                    VollageMessage,
                    Text::new("信息显示窗口"),
                    TextFont {
                        font: asset.load("fonts/STKAITI.TTF"),
                        font_size: 20.0,
                        
                        ..default()
                    },
                    TextColor(Color::srgb(0.9, 0.2, 0.2)), //文本颜色
                    //TextShadow::default()  //字体阴影
                    
                )
            ]
            ),
        ],
    ));
}

pub fn check_zhuangbei_button(
    player: Query<(&Player, &Cannon, &SecGun, &Engine)>,
    mut interaction_query: Query<(&Interaction, &mut BorderColor), (Changed<Interaction>, With<ZhuangBeiButton>)>,
    mut text: Query<&mut Text, With<VollageMessage>>
) {
    for (inter,mut border_color) in interaction_query.iter_mut() {
        match inter {
            Interaction::Pressed => {
                border_color.set_all(AQUA);
                if let Ok((player,cannon, secgun, engine)) =  player.single() {
                    
                    let name = &cannon.name;
                    let second_name = &secgun.name;
                    let eng_name = &engine.name;

                    let main_message = &cannon.description;
                    let second_message = &secgun.description;
                    let eng_message = &engine.description;

                    let d_main = cannon.weight;
                    let d_second = secgun.weight;
                    let d_eng = engine.weight;

                    let sh_main = cannon.damage;
                    let sh_second = secgun.damage;

                    if let Ok(mut text) = text.single_mut() {
                        text.0 = format!("装甲片：{}/{}\n\n主炮：{}\n\t信息：{}\n\t重量：{}T\t\t伤害：{}\n副炮：{}\n\t信息：{}\n\t重量：{}T\t\t伤害：{}\n引擎：{}\n\t信息：{}\n\t重量：{}T", 
                                player.health, player.max_health,
                                name,main_message, d_main, sh_main,
                                second_name, second_message, d_second, sh_second,
                                eng_name, eng_message, d_eng);
                    }
                    
                }
                
            }
            Interaction::Hovered => {
                border_color.set_all(OLIVE);
            }
            Interaction::None => {
                border_color.set_all(WHITE);
            }
        }
    }
}


pub fn check_recovery_button(
    mut player: Query<(&mut Player, &mut Cannon)>,
    mut interaction_query: Query<(&Interaction, &mut BorderColor), (Changed<Interaction>, With<RecoveryButton>)>,
    mut text: Query<&mut Text, With<VollageMessage>>
) {
    if let Ok((inter,mut border_color)) =  interaction_query.single_mut() {
        match inter {
            Interaction::Pressed => {
                border_color.set_all(AQUA);
                if let Ok(mut text) = text.single_mut() {
                    if let Ok((mut player, mut cannon)) = player.single_mut() {

                        if player.max_health == player.health {
                            text.0 = format!("二傻子，装甲片满的，补充个锤子！！\n");
                        } else {
                            if player.max_health > 0 {
                                player.health = player.max_health;

                                text.0 = format!("战车装甲片已更新！！\n");
                            } else {
                                text.0 = format!("无法更新装甲片，载重异常！！\n");
                            }
                        }

                        if cannon.current_ammo != cannon.max_ammo {
                            cannon.current_ammo = cannon.max_ammo;
                            text.0 += format!("炮弹已更新！！，补充炮弹{}颗",(cannon.max_ammo - cannon.current_ammo)).as_str();

                        } else {
                            text.0 += &format!("炮弹不需要补充！！");
                        }
                    }
                }

                
            }
            Interaction::Hovered => {
                border_color.set_all(OLIVE);
            }
            Interaction::None => {
                border_color.set_all(WHITE);
            }
        }
    }
}

pub fn check_beibao_button(
    mut interaction_query: Query<(&Interaction, &mut BorderColor), (Changed<Interaction>, With<BeiBaoButton>)>,
    backpack: Option<Res<Backpack>>,
    mut text: Query<&mut Text, With<VollageMessage>>,
) {
    if let Ok((inter, mut border_color)) = interaction_query.single_mut() {
        match inter {
            Interaction::Pressed => {
                border_color.set_all(AQUA);
                if let Ok(mut t) = text.single_mut() {
                    t.0 = match backpack {
                        Some(ref bp) => {
                            let s = bp.summary();
                            if s.is_empty() {
                                "背包空空如也……".to_string()
                            } else {
                                format!("📦 背包内容：\n{}", s)
                            }
                        }
                        None => "背包空空如也……".to_string(),
                    };
                }
            }
            Interaction::Hovered => {
                border_color.set_all(OLIVE);
            }
            Interaction::None => {
                border_color.set_all(WHITE);
            }
        }
    }
}

pub fn check_chuji_button(
    mut interaction_query: Query<(&Interaction, &mut BorderColor), (Changed<Interaction>, With<ChuJiButton>)>,
    mut next_status: ResMut<NextState<Appstatus>>,
) {
    if let Ok((interaction, mut border_color)) = interaction_query.single_mut() {
        match interaction {
            Interaction::Pressed => {
                border_color.set_all(AQUA);
                next_status.set(Appstatus::WorldMap);
            }
            Interaction::Hovered => {
                border_color.set_all(OLIVE);
            }
            Interaction::None => {
                border_color.set_all(WHITE);
            }
        }
    }
}