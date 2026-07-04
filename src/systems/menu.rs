use super::warehouse::WarehouseNeedsRefresh;
use crate::comp_data::*;
use bevy::color::palettes::basic::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct LoadGameUi;

pub fn create_menu(asset: &AssetServer) -> impl Bundle {
    let image = asset.load("Menu_image.png");
    (
        Node {
            width: percent(100),
            height: percent(100),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Column,
            row_gap: px(20.0),
            ..default()
        },
        ImageNode::new(image),
        EnumUi,
        children![
            (
                Button,
                Node {
                    width: px(150),
                    height: px(65),
                    border: UiRect::all(px(5)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                StartGameUi,
                BorderColor::all(Color::WHITE),
                BorderRadius::all(px(8.0)),
                BackgroundColor(Color::BLACK),
                children![(
                    Text::new("开始游戏"),
                    TextFont { font: asset.load("fonts/STKAITI.TTF"), font_size: 33.0, ..default() },
                    TextColor(Color::srgb(0.9, 0.9, 0.9)),
                )],
            ),
            (
                Button,
                Node {
                    width: px(150),
                    height: px(65),
                    border: UiRect::all(px(5)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                LoadGameUi,
                BorderColor::all(Color::WHITE),
                BorderRadius::all(px(8.0)),
                BackgroundColor(Color::BLACK),
                children![(
                    Text::new("读取存档"),
                    TextFont { font: asset.load("fonts/STKAITI.TTF"), font_size: 33.0, ..default() },
                    TextColor(Color::srgb(0.8, 0.9, 0.6)),
                )],
            ),
            (
                Button,
                Node {
                    width: px(150),
                    height: px(65),
                    border: UiRect::all(px(5)),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                StopGameUi,
                BorderColor::all(Color::WHITE),
                BorderRadius::all(px(8.0)),
                BackgroundColor(Color::BLACK),
                children![(
                    Text::new("结束游戏"),
                    TextFont { font: asset.load("fonts/STKAITI.TTF"), font_size: 33.0, ..default() },
                    TextColor(Color::srgb(0.9, 0.9, 0.9)),
                )],
            ),
        ],
    )
}

pub fn load_button_systems(
    mut commands: Commands,
    mut interaction_query: Query<
        (&Interaction, &mut BorderColor),
        (Changed<Interaction>, With<LoadGameUi>),
    >,
    menu_entity: Query<Entity, With<EnumUi>>,
    mut next_status: ResMut<NextState<Appstatus>>,
    db: Res<CannonDataBase>,
    sec_db: Res<SecgunDataBase>,
    eng_db: Res<EngineDataBase>,
) {
    for (interaction, mut border_color) in interaction_query.iter_mut() {
        match interaction {
            Interaction::Pressed => {
                border_color.set_all(RED);
                if let Some(save) = load_save() {
                    // 清除菜单
                    if let Ok(menu_en) = menu_entity.single() {
                        commands.entity(menu_en).despawn_children();
                        commands.entity(menu_en).despawn();
                    }

                    // 初始化资源
                    commands.insert_resource(Backpack::new());
                    commands.insert_resource(Warehouse::new());
                    commands.insert_resource(WarehouseNeedsRefresh(false));
                    commands.insert_resource(PlayerLevel { level: save.player_level, exp: save.player_exp });

                    // 填充背包
                    let mut bp = Backpack::new();
                    for s in &save.backpack {
                        let _ = bp.add(s.item_type, s.quantity);
                    }
                    commands.insert_resource(bp);

                    // 填充仓库
                    let mut wh = Warehouse::new();
                    for s in &save.warehouse {
                        wh.store(s.item_type, s.quantity);
                    }
                    commands.insert_resource(wh);

                    // 创建玩家实体（恢复装备和血量）
                    let cannon_cfg = db.get(save.cannon_type).unwrap();
                    let secgun_cfg = sec_db.get(save.secgun_type).unwrap();
                    let engine_cfg = eng_db.get(save.engine_type).unwrap();

                    commands.spawn((
                        Player { health: save.player_health, max_health: save.player_max_health },
                        Cannon::new(save.cannon_type, cannon_cfg),
                        SecGun::new(save.secgun_type, secgun_cfg),
                        Engine::new(save.engine_type, engine_cfg),
                    ));

                    next_status.set(Appstatus::Vollage);
                } else {
                    // 没有存档可用，但按钮还在菜单，让系统继续运行
                    // 实际上 log 没法在这里用，暂时忽略
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

pub fn start_button_systems(
    mut commands: Commands,
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BorderColor,
        ),
        (Changed<Interaction>, With<StartGameUi>),
    >,
    menu_entity: Query<Entity, With<EnumUi>>,
    mut next_status: ResMut<NextState<Appstatus>>,
) {
    for (interaction, mut border_color) in interaction_query.iter_mut() {
        match interaction {
            Interaction::Pressed => {
                border_color.set_all(RED);
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
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BorderColor,
        ),
        (Changed<Interaction>, With<StopGameUi>),
    >,
    mut app_exit_events: MessageWriter<AppExit>,
) {
    for (interaction, mut border_color) in interaction_query.iter_mut() {
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

pub fn start_game(mut commands: Commands, mut next_status: ResMut<NextState<Appstatus>>,
    db: Res<CannonDataBase>, sec_db: Res<SecgunDataBase>, eng_db: Res<EngineDataBase>)
{
    commands.insert_resource(Backpack::new());
    commands.insert_resource(Warehouse::new());
    commands.insert_resource(WarehouseNeedsRefresh(false));
    commands.insert_resource(PlayerLevel::new());

    let canno_config = db.get(CannonType::CannonLevel1).unwrap();
    let secgun_config = sec_db.get(SecgunType::SecGunLevel1).unwrap();
    let engine_config = eng_db.get(EngineType::EngineLevel1).unwrap();
    commands.spawn((
        Player { health: 0, max_health: 0 },
        Cannon::new(CannonType::CannonLevel1, canno_config),
        SecGun::new(SecgunType::SecGunLevel1, secgun_config),
        Engine::new(EngineType::EngineLevel1, engine_config),
    ));

    next_status.set(Appstatus::Vollage);
}
