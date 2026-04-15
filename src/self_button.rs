
use bevy::color::palettes::basic::*;
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
                BackgroundColor(Color::BLACK),
                children![
                    (
                        Text::new("开始"),
                        TextFont {
                            font: asset.load("fonts/STKAITI.TTF"),
                            font_size: 33.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.9, 0.9, 0.9)), //文本颜色
                        //TextShadow::default()  //字体阴影
                        
                    )
                ]

            )
        ],
    )
}

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);
fn button_systems(
    // Interaction 是记录UI实体与鼠标或触摸之间的交互状态；
    // 他有三个状态，无交互，悬停和按下
    mut interaction_query: Query<( //查找同时包含这么多组件的实体，
            &Interaction,  //Interaction会自动添加到button组件的实体上
            &mut BackgroundColor, 
            &mut BorderColor,
            &mut Button,
            &Children,
        ), Changed<Interaction>>, // 添加限定条件，Interaction变化时
    mut text_query: Query<&mut Text>,
) {
    for ( interaction, mut background_color, mut border_color, mut button, children ) in interaction_query.iter_mut() {
        let mut text = text_query.get_mut(children[0]).unwrap();
        // 这里 text_query.get_mut(entity) 是用来获取实体中的Text组件；
        // 正常情况下 text_query是获取全局的Text组件
        match interaction {
            Interaction::Pressed => {
                text.0 = "按下".to_string(); // text是一个Mut<'_,Text>类型，需要解引用才能获得Text
                background_color.0 = PRESSED_BUTTON.into();
                
                border_color.set_all(RED);
            }
            Interaction::Hovered => {
                text.0 = "停留".to_string(); // text是一个Mut<'_,Text>类型，需要解引用才能获得Text
                background_color.0 = HOVERED_BUTTON.into();
                
                border_color.set_all(WHITE);
            }
            Interaction::None => {
                text.0 = "开始".to_string(); // text是一个Mut<'_,Text>类型，需要解引用才能获得Text
                background_color.0 = NORMAL_BUTTON.into();
                
                border_color.set_all(BLACK);
            }
        }
    }
}