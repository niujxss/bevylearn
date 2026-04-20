use bevy::prelude::*;
use crate::comp_data::*;
use bevy::color::palettes::basic::*;

pub fn create_menu(asset: &AssetServer) -> impl Bundle {
    let image = asset.load("Menu_image.png");
    (
        Node {
            width: percent(100), // 父节点，没有父节点，所以父节点是窗口
            height: percent(100),
            align_items: AlignItems::Center, // 子节点垂直居中 
            justify_content: JustifyContent::Center, // 子节点水平居中 
            flex_direction: FlexDirection::Column, // 子元素垂直排列
            row_gap: px(20.0), // 垂直间隔
            ..default()
        },
        ImageNode::new(image),
        EnumUi,
        children![ //宏，自动创建子节点
            (
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
                StartGameUi,
                BorderColor::all(Color::WHITE), //边框颜色,白色
                BorderRadius::all(px(8.0)), // 8像素的圆角半径，更现代
                BackgroundColor(Color::BLACK),
                children![
                    (
                        Text::new("开始游戏"),
                        TextFont {
                            font: asset.load("fonts/STKAITI.TTF"),
                            font_size: 33.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.9, 0.9, 0.9)), //文本颜色
                        //TextShadow::default()  //字体阴影
                        
                    )
                ]
            ),
            (
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
                        Text::new("结束游戏"),
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
    )
}


pub fn start_button_systems(
    // Interaction 是记录UI实体与鼠标或触摸之间的交互状态；
    // 他有三个状态，无交互，悬停和按下
    mut commands: Commands,
    mut interaction_query: Query<( //查找同时包含这么多组件的实体，
            &Interaction,  //Interaction会自动添加到button组件的实体上
            &mut BorderColor, //边框颜色
        ), (Changed<Interaction>, With<StartGameUi>) >, // 添加限定条件，Interaction变化时
    menu_entity: Query<Entity, With<EnumUi>>,
    mut next_status: ResMut<NextState<Appstatus>>
) {
    for ( interaction, mut border_color ) in interaction_query.iter_mut() {

        match interaction {
            Interaction::Pressed => {
                border_color.set_all(RED);
                //menu_entity
                if let Ok(menu_en) = menu_entity.single() {
                    commands.entity(menu_en).despawn_children();
                    commands.entity(menu_en).despawn();
                    next_status.set(Appstatus::Game);
                    
                }
            }
            Interaction::Hovered => {
                border_color.set_all(WHITE);
            }
            Interaction::None => {
                border_color.set_all(BLACK);
            }
        }
    }
}

pub fn stop_button_systems(
    // Interaction 是记录UI实体与鼠标或触摸之间的交互状态；
    // 他有三个状态，无交互，悬停和按下
    mut interaction_query: Query<( //查找同时包含这么多组件的实体，
            &Interaction,  //Interaction会自动添加到button组件的实体上
            &mut BorderColor,
        ), (Changed<Interaction>,With<StopGameUi>)>, // 添加限定条件，Interaction变化时
    mut app_exit_events: MessageWriter<AppExit>,
) {
    for ( interaction, mut border_color ) in interaction_query.iter_mut() {

        match interaction {
            Interaction::Pressed => {
                border_color.set_all(RED);
                app_exit_events.write(AppExit::Success);
            }
            Interaction::Hovered => {
                border_color.set_all(WHITE);
            }
            Interaction::None => {
                border_color.set_all(BLACK);
            }
        }
    }
}

pub fn start_game(mut commands: Commands, mut next_status: ResMut<NextState<Appstatus>>) {
    commands.spawn((
        Player {
            health: 0,
            max_health: 0,
            main_gun: MainGun::LEVEL1("碎铁者Ⅰ型".to_string(), "用废旧钢管焊接而成，发射生锈的穿甲弹".to_string(), 0.1, 5),
            second_gun: SecondGun::LEVEL1("啄木鸟机枪".to_string(), "7.62mm同轴机枪，射速每分钟800发，用来驱赶靠近的拾荒者".to_string(), 0.01, 1),
            engine: Engine::LEVEL1("老烟枪".to_string(), "二手柴油机，启动时会冒出浓浓的黑烟，最高时速只有30公里，但胜在能烧各种劣质燃油".to_string(), 0.5),
        } 
    ));

    next_status.set(Appstatus::Vollage);
}
