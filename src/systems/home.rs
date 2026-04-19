use bevy::prelude::*;
use crate::comp_data::*;
use bevy::color::palettes::basic::*;

#[derive(Component)]
struct VollageUi;

#[derive(Component)]
struct MiscUi;

#[derive(Component)]
struct ChuJiButton;

#[derive(Component)]
struct UpdateButton;

#[derive(Component)]
pub struct ZhuangBeiButton;

#[derive(Component)]
struct BeiBaoButton;

#[derive(Component)]
pub struct VollageMessage;

pub fn create_home_ui(mut commands: Commands,asset: Res<AssetServer>) {

    let image: Handle<Image> = asset.load("vollage.png");
    let tank_image = asset.load("tank.png");
    commands.spawn((
       Player {
        health: 39,
        max_health: 39,
        main_gun: MainGun::LEVEL1("碎铁者Ⅰ型".to_string(), "用废旧钢管焊接而成，发射生锈的穿甲弹".to_string(), 0.1, 5),
        second_gun: SecondGun::LEVEL1("啄木鸟机枪".to_string(), "7.62mm同轴机枪，射速每分钟800发，用来驱赶靠近的拾荒者".to_string(), 0.01, 1),
        engine: Engine::LEVEL1("老烟枪".to_string(), "二手柴油机，启动时会冒出浓浓的黑烟，最高时速只有30公里，但胜在能烧各种劣质燃油".to_string(), 0.5),
       } 
    ));
    commands.spawn((
        VollageUi,
        Node {
            width: percent(100), // 父节点，没有父节点，所以父节点是窗口
            height: percent(60),
            position_type: PositionType::Absolute, // 绝对定位
            top: px(0),
            align_items: AlignItems::FlexEnd, // 子节点垂直贴底 
            justify_content: JustifyContent::Center, // 子节点水平居中 
            flex_direction: FlexDirection::Row, // 子元素水平排列
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
            )
        ],
    ));
    commands.spawn(
    (
        MiscUi,
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
    player: Query<&Player>,
    interaction_query: Query<(&Interaction), (Changed<Interaction>, With<ZhuangBeiButton>)>,
    mut text: Query<&mut Text, With<VollageMessage>>
) {
    for inter in interaction_query.iter() {
        match inter {
            Interaction::Pressed => {
                if let Ok(player) =  player.single() {
                    let name = player.main_gun.get_name();
                    let second_name = player.second_gun.get_name();
                    let eng_name = player.engine.get_name();

                    let main_message = player.main_gun.get_message();
                    let second_message = player.second_gun.get_message();
                    let eng_message = player.engine.get_message();

                    let d_main = player.main_gun.get_zhongliang();
                    let d_second = player.second_gun.get_zhongliang();
                    let d_eng = player.engine.get_zhongliang();

                    let sh_main = player.main_gun.get_shanghai();
                    let sh_second = player.second_gun.get_shanghai();

                    if let Ok(mut text) = text.single_mut() {
                        text.0 = format!("装甲片：{}/{}\n\n主炮：{}\n\t信息：{}\n\t重量：{}T\t\t伤害：{}\n副炮：{}\n\t信息：{}\n\t重量：{}T\t\t伤害：{}\n引擎：{}\n\t信息：{}\n\t重量：{}T", 
                                player.health, player.max_health,
                                name,main_message, d_main, sh_main,
                                second_name, second_message, d_second, sh_second,
                                eng_name, eng_message, d_eng);
                    }
                    
                }
                
            },
            _ => {

            }
        }
    }
}