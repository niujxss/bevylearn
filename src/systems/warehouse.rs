use crate::comp_data::*;
use bevy::color::palettes::basic::*;
use bevy::prelude::*;

// ============ UI 组件标记 ============

#[derive(Component)]
pub struct CangKuButton;

#[derive(Component)]
pub struct WarehouseOverlay;

#[derive(Component)]
pub struct WarehouseCloseBtn;

/// 存入按钮（携带物品类型）
#[derive(Component)]
pub struct DepositButton(pub ItemType);

/// 取出按钮（携带物品类型）
#[derive(Component)]
pub struct WithdrawButton(pub ItemType);

/// 刷新标志
#[derive(Resource, Default)]
pub struct WarehouseNeedsRefresh(pub bool);

// ============ 打开仓库 ============

pub fn check_cangku_button(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut interaction_query: Query<
        (&Interaction, &mut BorderColor),
        (Changed<Interaction>, With<CangKuButton>),
    >,
    existing_overlay: Query<Entity, With<WarehouseOverlay>>,
    backpack: Option<Res<Backpack>>,
    warehouse: Option<Res<Warehouse>>,
) {
    if let Ok((inter, mut border_color)) = interaction_query.single_mut() {
        match inter {
            Interaction::Pressed => {
                border_color.set_all(AQUA);
                if !existing_overlay.is_empty() {
                    return;
                }
                let bp = backpack.map(|r| r.clone()).unwrap_or_default();
                let wh = warehouse.map(|r| r.clone()).unwrap_or_default();
                spawn_warehouse_ui(&mut commands, &asset_server, &bp, &wh);
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

fn spawn_warehouse_ui(
    commands: &mut Commands,
    asset_server: &AssetServer,
    backpack: &Backpack,
    warehouse: &Warehouse,
) {
    let font = asset_server.load("fonts/STKAITI.TTF");
    let bp_count = backpack.used_slots();
    let bp_items = backpack.items.clone();
    let wh_items = warehouse.items.clone();

    commands
        .spawn((
            DespawnOnExit(Appstatus::Vollage),
            WarehouseOverlay,
            Name::new("WarehouseRoot"),
            Node {
                width: percent(100),
                height: percent(100),
                position_type: PositionType::Absolute,
                top: px(0.0),
                left: px(0.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgb(0.05, 0.05, 0.12)),
        ))
        .with_children(|parent| {
            // ===== 标题栏 =====
            parent
                .spawn((
                    Node {
                        width: percent(100),
                        height: percent(8),
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::SpaceBetween,
                        padding: UiRect::horizontal(px(30.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgb(0.1, 0.1, 0.2)),
                ))
                .with_children(|bar| {
                    bar.spawn((
                        Text::new("📦 仓库管理"),
                        TextFont {
                            font: font.clone(),
                            font_size: 28.0,
                            ..default()
                        },
                        TextColor(Color::srgb(0.9, 0.9, 0.9)),
                    ));
                    bar.spawn((
                        Button,
                        WarehouseCloseBtn,
                        Node {
                            width: px(80),
                            height: px(36),
                            border: UiRect::all(px(2)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        BorderColor::all(Color::WHITE),
                        BorderRadius::all(px(6.0)),
                        BackgroundColor(Color::srgb(0.5, 0.1, 0.1)),
                        children![(
                            Text::new("关闭"),
                            TextFont {
                                font: font.clone(),
                                font_size: 22.0,
                                ..default()
                            },
                            TextColor(Color::WHITE),
                        )],
                    ));
                });

            // ===== 内容主体 =====
            parent
                .spawn((Node {
                    width: percent(100),
                    height: percent(92),
                    flex_direction: FlexDirection::Row,
                    ..default()
                },))
                .with_children(|content| {
                    // ===== 左栏：背包 =====
                    content
                        .spawn((
                            Node {
                                width: percent(50),
                                height: percent(100),
                                flex_direction: FlexDirection::Column,
                                align_items: AlignItems::Center,
                                padding: UiRect::all(px(10.0)),
                                ..default()
                            },
                            BackgroundColor(Color::srgb(0.08, 0.08, 0.18)),
                        ))
                        .with_children(|left| {
                            left.spawn((
                                Text::new(format!(
                                    "🎒 背包 ({}/{})",
                                    bp_count, BACKPACK_MAX_SLOTS
                                )),
                                TextFont {
                                    font: font.clone(),
                                    font_size: 22.0,
                                    ..default()
                                },
                                TextColor(Color::srgb(0.3, 0.8, 1.0)),
                            ));
                            for row in bp_items.chunks(3) {
                                left.spawn((Node {
                                    width: percent(100),
                                    height: px(44),
                                    flex_direction: FlexDirection::Row,
                                    justify_content: JustifyContent::Center,
                                    column_gap: px(6.0),
                                    ..default()
                                },))
                                .with_children(|row_node| {
                                    for stack in row {
                                        row_node
                                            .spawn((
                                                Node {
                                                    width: percent(30),
                                                    height: percent(100),
                                                    flex_direction:
                                                        FlexDirection::Row,
                                                    align_items: AlignItems::Center,
                                                    justify_content:
                                                        JustifyContent::Center,
                                                    border: UiRect::all(px(1)),
                                                    column_gap: px(4.0),
                                                    ..default()
                                                },
                                                BorderColor::all(Color::srgb(
                                                    0.3, 0.3, 0.3,
                                                )),
                                                BackgroundColor(Color::srgb(
                                                    0.12, 0.12, 0.22,
                                                )),
                                            ))
                                            .with_children(|cell| {
                                                cell.spawn((
                                                    Text::new(format!(
                                                        "{}×{}",
                                                        stack.item_type.name(),
                                                        stack.quantity
                                                    )),
                                                    TextFont {
                                                        font: font.clone(),
                                                        font_size: 16.0,
                                                        ..default()
                                                    },
                                                    TextColor(Color::srgb(
                                                        0.9, 0.9, 0.9,
                                                    )),
                                                ));
                                                cell.spawn((
                                                    Button,
                                                    DepositButton(stack.item_type),
                                                    Node {
                                                        width: px(52),
                                                        height: px(28),
                                                        border: UiRect::all(px(1)),
                                                        justify_content:
                                                            JustifyContent::Center,
                                                        align_items:
                                                            AlignItems::Center,
                                                        ..default()
                                                    },
                                                    BorderColor::all(Color::WHITE),
                                                    BorderRadius::all(px(4.0)),
                                                    BackgroundColor(Color::srgb(
                                                        0.2, 0.5, 0.2,
                                                    )),
                                                    children![(
                                                        Text::new("存入"),
                                                        TextFont {
                                                            font: font.clone(),
                                                            font_size: 14.0,
                                                            ..default()
                                                        },
                                                        TextColor(Color::WHITE),
                                                    )],
                                                ));
                                            });
                                    }
                                });
                            }
                        });

                    // ===== 右栏：仓库 =====
                    content
                        .spawn((
                            Node {
                                width: percent(50),
                                height: percent(100),
                                flex_direction: FlexDirection::Column,
                                align_items: AlignItems::Center,
                                padding: UiRect::all(px(10.0)),
                                ..default()
                            },
                            BackgroundColor(Color::srgb(0.10, 0.10, 0.18)),
                        ))
                        .with_children(|right| {
                            right.spawn((
                                Text::new("🏪 仓库（无限）"),
                                TextFont {
                                    font: font.clone(),
                                    font_size: 22.0,
                                    ..default()
                                },
                                TextColor(Color::srgb(0.3, 1.0, 0.3)),
                            ));
                            for stack in &wh_items {
                                right
                                    .spawn((
                                        Node {
                                            width: percent(90),
                                            height: px(38),
                                            flex_direction: FlexDirection::Row,
                                            align_items: AlignItems::Center,
                                            justify_content:
                                                JustifyContent::SpaceBetween,
                                            border: UiRect::all(px(1)),
                                            padding: UiRect::horizontal(px(8.0)),
                                            ..default()
                                        },
                                        BorderColor::all(Color::srgb(
                                            0.3, 0.3, 0.3,
                                        )),
                                        BackgroundColor(Color::srgb(
                                            0.12, 0.12, 0.22,
                                        )),
                                    ))
                                    .with_children(|item_row| {
                                        item_row.spawn((
                                            Text::new(format!(
                                                "{} × {}",
                                                stack.item_type.name(),
                                                stack.quantity
                                            )),
                                            TextFont {
                                                font: font.clone(),
                                                font_size: 16.0,
                                                ..default()
                                            },
                                            TextColor(Color::srgb(0.9, 0.9, 0.9)),
                                        ));
                                        item_row.spawn((
                                            Button,
                                            WithdrawButton(stack.item_type),
                                            Node {
                                                width: px(52),
                                                height: px(28),
                                                border: UiRect::all(px(1)),
                                                justify_content:
                                                    JustifyContent::Center,
                                                align_items: AlignItems::Center,
                                                ..default()
                                            },
                                            BorderColor::all(Color::WHITE),
                                            BorderRadius::all(px(4.0)),
                                            BackgroundColor(Color::srgb(
                                                0.5, 0.2, 0.2,
                                            )),
                                            children![(
                                                Text::new("取出"),
                                                TextFont {
                                                    font: font.clone(),
                                                    font_size: 14.0,
                                                    ..default()
                                                },
                                                TextColor(Color::WHITE),
                                            )],
                                        ));
                                    });
                            }
                        });
                });
        });
}

// ============ 关闭仓库 ============

pub fn close_warehouse(
    mut commands: Commands,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<WarehouseCloseBtn>),
    >,
    overlay_query: Query<Entity, With<WarehouseOverlay>>,
) {
    if let Ok((inter, mut bg)) = interaction_query.single_mut() {
        match inter {
            Interaction::Pressed => {
                bg.0 = Color::srgb(0.7, 0.2, 0.2);
                if let Ok(overlay) = overlay_query.single() {
                    commands.entity(overlay).despawn();
                }
            }
            Interaction::Hovered => {
                bg.0 = Color::srgb(0.6, 0.15, 0.15);
            }
            Interaction::None => {
                bg.0 = Color::srgb(0.5, 0.1, 0.1);
            }
        }
    }
}

// ============ 存入操作 ============

pub fn handle_deposit(
    mut backpack: ResMut<Backpack>,
    mut warehouse: ResMut<Warehouse>,
    mut refresh: ResMut<WarehouseNeedsRefresh>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &DepositButton),
        Changed<Interaction>,
    >,
) {
    for (inter, mut bg, btn) in interaction_query.iter_mut() {
        if *inter == Interaction::Pressed {
            bg.0 = Color::srgb(0.3, 0.7, 0.3);
            if let Err(_) = backpack.remove(btn.0, 1) {
                continue;
            }
            warehouse.store(btn.0, 1);
            refresh.0 = true;
        } else if *inter == Interaction::Hovered {
            bg.0 = Color::srgb(0.3, 0.6, 0.3);
        } else {
            bg.0 = Color::srgb(0.2, 0.5, 0.2);
        }
    }
}

// ============ 取出操作 ============

pub fn handle_withdraw(
    mut backpack: ResMut<Backpack>,
    mut warehouse: ResMut<Warehouse>,
    mut refresh: ResMut<WarehouseNeedsRefresh>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &WithdrawButton),
        Changed<Interaction>,
    >,
) {
    for (inter, mut bg, btn) in interaction_query.iter_mut() {
        if *inter == Interaction::Pressed {
            bg.0 = Color::srgb(0.7, 0.3, 0.3);
            if !backpack.has_space_for(btn.0) {
                continue;
            }
            if let Err(_) = warehouse.take(btn.0, 1) {
                continue;
            }
            let _ = backpack.add(btn.0, 1);
            refresh.0 = true;
        } else if *inter == Interaction::Hovered {
            bg.0 = Color::srgb(0.6, 0.3, 0.3);
        } else {
            bg.0 = Color::srgb(0.5, 0.2, 0.2);
        }
    }
}

// ============ 刷新系统 ============

pub fn refresh_warehouse_system(
    mut commands: Commands,
    mut refresh: ResMut<WarehouseNeedsRefresh>,
    backpack: Res<Backpack>,
    warehouse: Res<Warehouse>,
    asset_server: Res<AssetServer>,
    overlay_query: Query<Entity, With<WarehouseOverlay>>,
) {
    if !refresh.0 {
        return;
    }
    refresh.0 = false;

    if overlay_query.is_empty() {
        return;
    }

    // 销毁旧覆盖层
    for entity in overlay_query.iter() {
        commands.entity(entity).despawn();
    }

    // 重建
    spawn_warehouse_ui(&mut commands, &asset_server, &backpack, &warehouse);
}
