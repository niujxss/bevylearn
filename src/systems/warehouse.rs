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

#[derive(Component)]
pub struct DepositButton(pub ItemType);

#[derive(Component)]
pub struct WithdrawButton(pub ItemType);

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
            Interaction::Hovered => { border_color.set_all(OLIVE); }
            Interaction::None => { border_color.set_all(WHITE); }
        }
    }
}

// ============ 构建仓库UI ============

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
    let cap_pct = (bp_count as f32 / BACKPACK_MAX_SLOTS as f32).min(1.0);

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
                ..default()
            },
            BackgroundColor(Color::srgb(0.04, 0.04, 0.10)),
        ))
        .with_children(|parent| {
            // ===== 标题栏 =====
            parent
                .spawn((
                    Node {
                        width: percent(100),
                        height: px(56),
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::SpaceBetween,
                        padding: UiRect::horizontal(px(24.0)),
                        border: UiRect::bottom(px(2)),
                        ..default()
                    },
                    BorderColor::all(Color::srgb(0.25, 0.15, 0.05)),
                    BackgroundColor(Color::srgb(0.08, 0.06, 0.14)),
                ))
                .with_children(|bar| {
                    bar.spawn((
                        Text::new("📦  仓库管理"),
                        TextFont { font: font.clone(), font_size: 24.0, ..default() },
                        TextColor(Color::srgb(1.0, 0.85, 0.5)),
                    ));
                    // 关闭按钮
                    bar.spawn((
                        Button,
                        WarehouseCloseBtn,
                        Node {
                            width: px(72),
                            height: px(32),
                            border: UiRect::all(px(1)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        BorderColor::all(Color::srgb(0.6, 0.2, 0.2)),
                        BorderRadius::all(px(4.0)),
                        BackgroundColor(Color::srgb(0.35, 0.10, 0.10)),
                        children![(
                            Text::new("✕ 关闭"),
                            TextFont { font: font.clone(), font_size: 18.0, ..default() },
                            TextColor(Color::srgb(1.0, 0.6, 0.6)),
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
                                padding: UiRect::all(px(12.0)),
                                ..default()
                            },
                            BackgroundColor(Color::srgb(0.06, 0.06, 0.14)),
                        ))
                        .with_children(|left| {
                            // --- 标题行：图标 + 文字 + 容量数字 ---
                            left.spawn((
                                Node {
                                    width: percent(100),
                                    height: px(36),
                                    flex_direction: FlexDirection::Row,
                                    align_items: AlignItems::Center,
                                    justify_content: JustifyContent::SpaceBetween,
                                    padding: UiRect::horizontal(px(4.0)),
                                    ..default()
                                },
                            ))
                            .with_children(|hdr| {
                                hdr.spawn((
                                    Text::new(format!(
                                        "🎒  背包  {}/{}", bp_count, BACKPACK_MAX_SLOTS
                                    )),
                                    TextFont { font: font.clone(), font_size: 20.0, ..default() },
                                    TextColor(Color::srgb(0.4, 0.85, 1.0)),
                                ));
                                let cap_color = if cap_pct > 0.9 {
                                    Color::srgb(1.0, 0.3, 0.3)
                                } else if cap_pct > 0.6 {
                                    Color::srgb(1.0, 0.8, 0.2)
                                } else {
                                    Color::srgb(0.3, 1.0, 0.4)
                                };
                                hdr.spawn((
                                    Text::new(format!("{:.0}%", cap_pct * 100.0)),
                                    TextFont { font: font.clone(), font_size: 16.0, ..default() },
                                    TextColor(cap_color),
                                ));
                            });

                            // --- 容量进度条 ---
                            left.spawn((
                                Node {
                                    width: percent(100),
                                    height: px(8),
                                    border: UiRect::all(px(1)),
                                    ..default()
                                },
                                BorderColor::all(Color::srgb(0.25, 0.25, 0.35)),
                                BackgroundColor(Color::srgb(0.10, 0.10, 0.18)),
                            ))
                            .with_children(|bar| {
                                let fill_color = if cap_pct > 0.9 {
                                    Color::srgb(0.9, 0.2, 0.2)
                                } else if cap_pct > 0.6 {
                                    Color::srgb(0.9, 0.7, 0.1)
                                } else {
                                    Color::srgb(0.2, 0.7, 0.3)
                                };
                                bar.spawn((
                                    Node {
                                        width: percent(cap_pct * 100.0),
                                        height: percent(100),
                                        ..default()
                                    },
                                    BackgroundColor(fill_color),
                                ));
                            });

                            // --- 分隔线 ---
                            left.spawn((
                                Node {
                                    width: percent(100),
                                    height: px(1),
                                    margin: UiRect::vertical(px(6.0)),
                                    ..default()
                                },
                                BackgroundColor(Color::srgb(0.15, 0.15, 0.25)),
                            ));

                            // --- 背包网格 ---
                            if bp_items.is_empty() {
                                left.spawn((
                                    Node {
                                        width: percent(100),
                                        height: px(60),
                                        flex_direction: FlexDirection::Column,
                                        align_items: AlignItems::Center,
                                        justify_content: JustifyContent::Center,
                                        ..default()
                                    },
                                    children![(
                                        Text::new("(空)"),
                                        TextFont { font: font.clone(), font_size: 18.0, ..default() },
                                        TextColor(Color::srgb(0.4, 0.4, 0.5)),
                                    )],
                                ));
                            } else {
                                for row in bp_items.chunks(3) {
                                    left.spawn((
                                        Node {
                                            width: percent(100),
                                            height: px(54),
                                            flex_direction: FlexDirection::Row,
                                            justify_content: JustifyContent::Center,
                                            column_gap: px(8.0),
                                            ..default()
                                        },
                                    ))
                                    .with_children(|row_node| {
                                        for stack in row {
                                            let rarity = stack.item_type.rarity_color();
                                            row_node
                                                .spawn((
                                                    Node {
                                                        width: percent(30),
                                                        height: percent(100),
                                                        flex_direction: FlexDirection::Column,
                                                        align_items: AlignItems::Center,
                                                        justify_content: JustifyContent::Center,
                                                        border: UiRect::all(px(1)),
                                                        row_gap: px(2.0),
                                                        ..default()
                                                    },
                                                    BorderColor::all(Color::srgb(
                                                        0.25, 0.25, 0.35,
                                                    )),
                                                    BorderRadius::all(px(6.0)),
                                                    BackgroundColor(Color::srgb(
                                                        0.09, 0.09, 0.18,
                                                    )),
                                                ))
                                                .with_children(|cell| {
                                                    // 物品名 + 品质色
                                                    cell.spawn((
                                                        Text::new(format!(
                                                            "{} {}",
                                                            stack.item_type.icon(),
                                                            stack.item_type.name()
                                                        )),
                                                        TextFont {
                                                            font: font.clone(),
                                                            font_size: 14.0,
                                                            ..default()
                                                        },
                                                        TextColor(rarity),
                                                    ));
                                                    // 数量 + 存入按钮行
                                                    cell.spawn((
                                                        Node {
                                                            width: percent(100),
                                                            flex_direction: FlexDirection::Row,
                                                            align_items: AlignItems::Center,
                                                            justify_content:
                                                                JustifyContent::Center,
                                                            column_gap: px(4.0),
                                                            ..default()
                                                        },
                                                    ))
                                                    .with_children(|info| {
                                                        info.spawn((
                                                            Text::new(format!(
                                                                "×{}", stack.quantity
                                                            )),
                                                            TextFont {
                                                                font: font.clone(),
                                                                font_size: 15.0,
                                                                ..default()
                                                            },
                                                            TextColor(Color::srgb(
                                                                0.8, 0.8, 0.9,
                                                            )),
                                                        ));
                                                        info.spawn((
                                                            Button,
                                                            DepositButton(stack.item_type),
                                                            Node {
                                                                width: px(46),
                                                                height: px(22),
                                                                border: UiRect::all(px(1)),
                                                                justify_content:
                                                                    JustifyContent::Center,
                                                                align_items: AlignItems::Center,
                                                                ..default()
                                                            },
                                                            BorderColor::all(Color::srgb(
                                                                0.3, 0.6, 0.3,
                                                            )),
                                                            BorderRadius::all(px(4.0)),
                                                            BackgroundColor(Color::srgb(
                                                                0.15, 0.40, 0.15,
                                                            )),
                                                            children![(
                                                                Text::new("存入"),
                                                                TextFont {
                                                                    font: font.clone(),
                                                                    font_size: 13.0,
                                                                    ..default()
                                                                },
                                                                TextColor(Color::srgb(
                                                                    0.7, 1.0, 0.7,
                                                                )),
                                                            )],
                                                        ));
                                                    });
                                                });
                                        }
                                    });
                                }
                            }
                        });

                    // ===== 右栏：仓库 =====
                    let sep_color = Color::srgb(0.25, 0.15, 0.05);
                    content
                        .spawn((
                            Node {
                                width: percent(50),
                                height: percent(100),
                                flex_direction: FlexDirection::Column,
                                align_items: AlignItems::Center,
                                padding: UiRect::all(px(12.0)),
                                border: UiRect::left(px(2)),
                                ..default()
                            },
                            BorderColor::all(sep_color),
                            BackgroundColor(Color::srgb(0.07, 0.07, 0.14)),
                        ))
                        .with_children(|right| {
                            // 标题
                            right.spawn((
                                Node {
                                    width: percent(100),
                                    height: px(36),
                                    flex_direction: FlexDirection::Row,
                                    align_items: AlignItems::Center,
                                    padding: UiRect::horizontal(px(4.0)),
                                    ..default()
                                },
                                children![(
                                    Text::new("🏪  仓库（无限）"),
                                    TextFont {
                                        font: font.clone(),
                                        font_size: 20.0,
                                        ..default()
                                    },
                                    TextColor(Color::srgb(0.4, 1.0, 0.4)),
                                )],
                            ));

                            // 分隔线
                            right.spawn((
                                Node {
                                    width: percent(100),
                                    height: px(1),
                                    margin: UiRect::vertical(px(6.0)),
                                    ..default()
                                },
                                BackgroundColor(Color::srgb(0.15, 0.15, 0.25)),
                            ));

                            // 仓库物品列表
                            if wh_items.is_empty() {
                                right.spawn((
                                    Node {
                                        width: percent(100),
                                        height: px(60),
                                        flex_direction: FlexDirection::Column,
                                        align_items: AlignItems::Center,
                                        justify_content: JustifyContent::Center,
                                        ..default()
                                    },
                                    children![(
                                        Text::new("(空)"),
                                        TextFont {
                                            font: font.clone(),
                                            font_size: 18.0,
                                            ..default()
                                        },
                                        TextColor(Color::srgb(0.4, 0.4, 0.5)),
                                    )],
                                ));
                            } else {
                                for stack in &wh_items {
                                    let rarity = stack.item_type.rarity_color();
                                    right
                                        .spawn((
                                            Node {
                                                width: percent(95),
                                                height: px(38),
                                                flex_direction: FlexDirection::Row,
                                                align_items: AlignItems::Center,
                                                justify_content:
                                                    JustifyContent::SpaceBetween,
                                                border: UiRect::all(px(1)),
                                                padding: UiRect::horizontal(px(10.0)),
                                                ..default()
                                            },
                                            BorderColor::all(Color::srgb(
                                                0.22, 0.22, 0.32,
                                            )),
                                            BorderRadius::all(px(6.0)),
                                            BackgroundColor(Color::srgb(
                                                0.09, 0.09, 0.18,
                                            )),
                                        ))
                                        .with_children(|item_row| {
                                            item_row.spawn((
                                                Text::new(format!(
                                                    "{} {} × {}",
                                                    stack.item_type.icon(),
                                                    stack.item_type.name(),
                                                    stack.quantity
                                                )),
                                                TextFont {
                                                    font: font.clone(),
                                                    font_size: 15.0,
                                                    ..default()
                                                },
                                                TextColor(rarity),
                                            ));
                                            item_row.spawn((
                                                Button,
                                                WithdrawButton(stack.item_type),
                                                Node {
                                                    width: px(46),
                                                    height: px(24),
                                                    border: UiRect::all(px(1)),
                                                    justify_content:
                                                        JustifyContent::Center,
                                                    align_items: AlignItems::Center,
                                                    ..default()
                                                },
                                                BorderColor::all(Color::srgb(
                                                    0.6, 0.3, 0.3,
                                                )),
                                                BorderRadius::all(px(4.0)),
                                                BackgroundColor(Color::srgb(
                                                    0.40, 0.15, 0.15,
                                                )),
                                                children![(
                                                    Text::new("取出"),
                                                    TextFont {
                                                        font: font.clone(),
                                                        font_size: 13.0,
                                                        ..default()
                                                    },
                                                    TextColor(Color::srgb(
                                                        1.0, 0.7, 0.7,
                                                    )),
                                                )],
                                            ));
                                        });
                                }
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
                bg.0 = Color::srgb(0.5, 0.2, 0.2);
                if let Ok(overlay) = overlay_query.single() {
                    commands.entity(overlay).despawn();
                }
            }
            Interaction::Hovered => bg.0 = Color::srgb(0.45, 0.15, 0.15),
            Interaction::None => bg.0 = Color::srgb(0.35, 0.10, 0.10),
        }
    }
}

// ============ 存入 ============

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
            bg.0 = Color::srgb(0.25, 0.55, 0.25);
            if backpack.remove(btn.0, 1).is_err() {
                continue;
            }
            warehouse.store(btn.0, 1);
            refresh.0 = true;
        } else if *inter == Interaction::Hovered {
            bg.0 = Color::srgb(0.2, 0.5, 0.2);
        } else {
            bg.0 = Color::srgb(0.15, 0.40, 0.15);
        }
    }
}

// ============ 取出 ============

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
            bg.0 = Color::srgb(0.55, 0.25, 0.25);
            if !backpack.has_space_for(btn.0) {
                continue;
            }
            if warehouse.take(btn.0, 1).is_err() {
                continue;
            }
            let _ = backpack.add(btn.0, 1);
            refresh.0 = true;
        } else if *inter == Interaction::Hovered {
            bg.0 = Color::srgb(0.5, 0.2, 0.2);
        } else {
            bg.0 = Color::srgb(0.40, 0.15, 0.15);
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
    for entity in overlay_query.iter() {
        commands.entity(entity).despawn();
    }
    spawn_warehouse_ui(&mut commands, &asset_server, &backpack, &warehouse);
}
