use bevy::prelude::*;


pub fn self_run() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, button_systems)
        .run();
}

fn setup(mut commands : Commands,
asset : Res<AssetServer>) {
    commands.spawn(Camera2d);
    commands.spawn(
        self_button(&asset)
    );
}

fn self_button(asset: &AssetServer) -> impl Bundle {
    (
        Node {
            width: percent(100), // 父节点，没有父节点，所以父节点是窗口
            height: percent(100),
            align_items: AlignItems::Center, // 子节点垂直居中 
            justify_content: JustifyContent::Center, // 子节点水平居中 
            ..default()
        },
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
                BorderColor::all(Color::WHITE), //边框颜色,白色
                //BorderRadius::MAX, // 最大圆角，形成一个圆角矩形，最大圆角是胶囊圆角
                BorderRadius::all(px(8.0)), // 8像素的圆角半径，更现代
                children![
                    (
                        Text::new("开始"),
                        TextFont {
                            font: asset.load("fonts/STKAITI.TTF"),
                            font_size: 33.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.9, 0.9, 0.9)), //文本颜色
                        TextShadow::default()
                    )
                ]

            )
        ],
    )
}


fn button_systems() {
    
}