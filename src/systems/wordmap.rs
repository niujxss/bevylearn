use bevy::prelude::*;
use crate::comp_data::*;
use bevy::color::palettes::basic::*;


#[derive(Component)]
pub struct WorldMapButton;

#[derive(Component)]
pub struct BackVollageButton;

pub fn create_wordmap(mut commands: Commands, asset: Res<AssetServer>) {
    let image:Handle<Image> = asset.load("HuangYuanFeiTu.png");
    commands.spawn((
            DespawnOnExit(Appstatus::WorldMap),
            WorldMapButton,
            Node {
                width: percent(100),
                height: percent(90),
                position_type: PositionType::Absolute,
                top: px(0),
                ..default()
            },
            ImageNode::new(image),
            Button,
            Text::new("荒芜废土"),
            TextFont {
                font: asset.load("fonts/STKAITI.TTF"),
                font_size: 33.0,
                ..default()
            },
        )
    );


    commands.spawn(
        (
            DespawnOnExit(Appstatus::WorldMap),
            Node {
                width: percent(100),
                height: percent(10),
                position_type: PositionType::Absolute,
                bottom: px(0),
                flex_direction: FlexDirection::Row, // 水平排列
                justify_content: JustifyContent::Center, // 子节点水平居中 
                align_items: AlignItems::Center, // 子节点垂直贴底 
                column_gap: px(20.0),
                ..default()
            },
            children![
            (
                Button,
                BackVollageButton,
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
                        Text::new("返回基地"),
                        TextFont {
                            font: asset.load("fonts/STKAITI.TTF"),
                            font_size: 16.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.9, 0.9, 0.9)), //文本颜色
                        //TextShadow::default()  //字体阴影
                        
                    )
                ]
            )
            ]
        )
    );
}


pub fn check_back_vollage_button(
    mut interatcion_query: Query<(&Interaction, &mut BorderColor), (Changed<Interaction>,With<BackVollageButton>)>,
    mut next_status: ResMut<NextState<Appstatus>>,
) {
    if let Ok((inter, mut border_color)) = interatcion_query.single_mut() {
        match inter {
            Interaction::None => {

            },
            Interaction::Hovered => {
                border_color.set_all(OLIVE);
            },
            Interaction::Pressed => {
                next_status.set(Appstatus::Vollage);
            }
        }
    }
}