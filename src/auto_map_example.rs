use bevy::{
    prelude::*,
    sprite_render::{TileData, TilemapChunk, TilemapChunkTileData},
};
use std::collections::HashMap;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;

// ======================== 组件定义 ========================

/// 地图块位置组件
#[derive(Component, Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct ChunkPosition {
    x: i32,
    y: i32,
}

/// 玩家组件
#[derive(Component)]
struct Player {
    move_speed: f32,
    last_chunk: IVec2,  // 记录上次所在的块
}

/// 地图块管理资源
#[derive(Resource)]
struct WorldMap {
    chunk_size: UVec2,        // 每个块的瓦片数量
    tile_size: UVec2,         // 每个瓦片的像素大小
    view_distance: i32,       // 视野距离（以块为单位）
    loaded_chunks: HashMap<IVec2, Entity>,  // 已加载的块实体
    chunk_data_cache: HashMap<IVec2, Vec<Option<TileData>>>, // 缓存已生成的地块数据
}

/// 游戏状态
#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
enum GameState {
    #[default]
    Loading,
    Playing,
}

/// UI组件
#[derive(Component)]
struct UiText;

/// 随机数生成器资源
#[derive(Resource, Deref, DerefMut)]
struct GameRng(ChaCha8Rng);

// ======================== 常量定义 ========================

const CHUNK_TILE_SIZE: u32 = 32;  // 每块32x32个瓦片
const TILE_PIXEL_SIZE: u32 = 40;  // 每个瓦片40像素
const VIEW_DISTANCE: i32 = 2;     // 视野距离2个块
const PLAYER_SPEED: f32 = 300.0;  // 玩家移动速度

// ======================== 启动系统 ========================

pub fn self_run() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "动态瓦片地图加载 - 重装机兵风格".to_string(),
                    
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .init_state::<GameState>()
        .add_systems(Startup, setup_window)
        .add_systems(OnEnter(GameState::Loading), setup_game)
        .add_systems(Update, check_initial_load.run_if(in_state(GameState::Loading)))
        .add_systems(OnEnter(GameState::Playing), spawn_initial_chunks)
        .add_systems(
            Update,
            (
                player_movement,
                camera_follow,
                update_chunks,
                update_ui,
                debug_chunk_borders,
                update_tileset_image,
            )
                .run_if(in_state(GameState::Playing)),
        )
        .run();
}

/// 设置窗口和相机
fn setup_window(mut commands: Commands) {
    // 创建2D相机
    commands.spawn((
        Camera2d,
        Transform::from_xyz(0.0, 0.0, 1000.0),
    ));
}

/// 设置游戏初始资源
fn setup_game(
    mut commands: Commands,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    asset_server: Res<AssetServer>,
) {
    // 初始化世界地图资源
    commands.insert_resource(WorldMap {
        chunk_size: UVec2::splat(CHUNK_TILE_SIZE),
        tile_size: UVec2::splat(TILE_PIXEL_SIZE),
        view_distance: VIEW_DISTANCE,
        loaded_chunks: HashMap::new(),
        chunk_data_cache: HashMap::new(),
    });
    
    // 初始化随机数生成器
    let mut rng = ChaCha8Rng::seed_from_u64(42);
    commands.insert_resource(GameRng(rng));
    
    // 生成玩家实体
    commands.spawn((
        Sprite {
            color: Color::srgb(1.0, 0.0, 0.0),
            custom_size: Some(Vec2::splat(20.0)),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 1.0),
        Player {
            move_speed: PLAYER_SPEED,
            last_chunk: IVec2::ZERO,
        },
    ));
    
    // 创建UI
    setup_ui(&mut commands,asset_server);
}

/// 创建UI界面
fn setup_ui(commands: &mut Commands,asset_server: Res<AssetServer>) {

    let chinese_font = asset_server.load("fonts/STKAITI.TTF");
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::SpaceBetween,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            UiText,
        ))
        .with_children(|parent| {
            // 标题
            parent.spawn((
                Text::new("动态瓦片地图加载演示"),
                TextFont {
                    font: chinese_font.clone(),
                    font_size: 40.0,
                    ..default()
                },
                TextColor(Color::srgb(1.0, 1.0, 0.0)),
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Px(60.0),
                    ..default()
                },
            ));
            
            // 信息面板
            parent.spawn((
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Px(100.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Start,
                    ..default()
                },
                BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.5)),
            )).with_children(|parent| {
                parent.spawn((
                    Text::new(""),
                    TextFont {
                        font: chinese_font.clone(),
                        font_size: 18.0,
                        ..default()
                    },
                    TextColor(Color::WHITE),
                    Node {
                        width: Val::Percent(100.0),
                        padding: UiRect::all(Val::Px(10.0)),
                        ..default()
                    },
                ));
            });
        });
}

/// 检查初始加载完成
fn check_initial_load(
    mut next_state: ResMut<NextState<GameState>>,
) {
    // 这里可以添加实际资源加载检查
    // 为了简化，我们假设资源已加载完成
    next_state.set(GameState::Playing);
}

/// 生成初始地图块
fn spawn_initial_chunks(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut world_map: ResMut<WorldMap>,
    player_query: Query<&Transform, With<Player>>,
) {
    if let Ok(player_transform) = player_query.single() {
        let player_chunk = world_to_chunk_coords(
            player_transform.translation,
            world_map.chunk_size,
            world_map.tile_size,
        );
        
        // 初始加载玩家周围的块
        let view_distance = world_map.view_distance;
        
        for x in -view_distance..=view_distance {
            for y in -view_distance..=view_distance {
                let chunk_pos = IVec2::new(x, y);
                if !world_map.loaded_chunks.contains_key(&chunk_pos) {
                    let entity = spawn_chunk(
                        &mut commands,
                        &asset_server,
                        chunk_pos,
                        &mut world_map,
                    );
                    world_map.loaded_chunks.insert(chunk_pos, entity);
                }
            }
        }
    }
}

/// 生成单个地图块
fn spawn_chunk(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    chunk_pos: IVec2,
    world_map: &mut WorldMap,
) -> Entity {
    let chunk_size = world_map.chunk_size;
    let tile_size = world_map.tile_size;
    
    // 计算块的世界位置
    let world_x = chunk_pos.x as f32 * (chunk_size.x as f32 * tile_size.x as f32);
    let world_y = chunk_pos.y as f32 * (chunk_size.y as f32 * tile_size.y as f32);
    
    // 生成或获取缓存的瓦片数据
    let tile_data = if let Some(data) = world_map.chunk_data_cache.get(&chunk_pos) {
        data.clone()
    } else {
        generate_chunk_data(chunk_pos, chunk_size)
    };
    
    // 缓存数据
    world_map.chunk_data_cache.insert(chunk_pos, tile_data.clone());
    
    // 创建地图块实体
    commands.spawn((
        TilemapChunk {
            chunk_size,
            tile_display_size: tile_size,
            tileset: asset_server.load("array_texture.png"),
            ..default()
        },
        TilemapChunkTileData(tile_data),
        ChunkPosition {
            x: chunk_pos.x,
            y: chunk_pos.y,
        },
        Transform::from_xyz(world_x, world_y, 0.0),
    )).id()
}

/// 生成块数据
fn generate_chunk_data(chunk_pos: IVec2, chunk_size: UVec2) -> Vec<Option<TileData>> {
    let mut tile_data = Vec::with_capacity(chunk_size.element_product() as usize);
    
    for i in 0..chunk_size.element_product() {
        let x = (i % chunk_size.x) as i32;
        let y = (i / chunk_size.x) as i32;
        
        // 计算全局坐标
        let global_x = chunk_pos.x * chunk_size.x as i32 + x;
        let global_y = chunk_pos.y * chunk_size.y as i32 + y;
        
        // 使用柏林噪声简化版生成地形
        let tile_type = (((global_x as f32 * 0.1).sin() 
            + (global_y as f32 * 0.1).cos()) * 2.0).abs() as u32 % 4;
        
        tile_data.push(Some(TileData::from_tileset_index(tile_type as u16)));
    }
    
    tile_data
}

// ======================== 核心逻辑系统 ========================

/// 玩家输入处理
fn player_movement(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(&mut Transform, &mut Player)>,
) {
    for (mut transform, mut player) in player_query.iter_mut() {
        let mut direction = Vec2::ZERO;
        
        if keyboard_input.pressed(KeyCode::KeyW) {
            direction.y += 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
            direction.y -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyA) {
            direction.x -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            direction.x += 1.0;
        }
        
        if direction.length_squared() > 0.0 {
            direction = direction.normalize();
            transform.translation.x += direction.x * player.move_speed * time.delta_secs();
            transform.translation.y += direction.y * player.move_speed * time.delta_secs();
        }
    }
}

/// 相机跟随玩家
fn camera_follow(
    player_query: Query<&Transform, (With<Player>, Without<Camera>)>,
    mut camera_query: Query<&mut Transform, (With<Camera>, Without<Player>)>,
) {
    if let Ok(player_transform) = player_query.single() {
        if let Ok(mut camera_transform) = camera_query.single_mut() {
            // 平滑跟随
            let target = player_transform.translation;
            let current = camera_transform.translation;
            camera_transform.translation = current.lerp(target, 0.1);
        }
    }
}

/// 动态加载/卸载地图块
fn update_chunks(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut player_query: Query<(&Transform, &mut Player)>,
    chunk_query: Query<(Entity, &ChunkPosition)>,
    mut world_map: ResMut<WorldMap>,
) {
    if let Ok((player_transform, mut player)) = player_query.single_mut() {
        // 计算玩家当前所在的块
        let current_chunk = world_to_chunk_coords(
            player_transform.translation,
            world_map.chunk_size,
            world_map.tile_size,
        );
        
        // 如果玩家移动到了新块
        if current_chunk != player.last_chunk {
            // 计算需要加载的块范围
            let min_x = current_chunk.x - world_map.view_distance;
            let max_x = current_chunk.x + world_map.view_distance;
            let min_y = current_chunk.y - world_map.view_distance;
            let max_y = current_chunk.y + world_map.view_distance;
            
            // 收集需要卸载的块
            let mut chunks_to_unload = Vec::new();
            
            for (entity, chunk_pos) in chunk_query.iter() {
                let pos = IVec2::new(chunk_pos.x, chunk_pos.y);
                if pos.x < min_x || pos.x > max_x || pos.y < min_y || pos.y > max_y {
                    chunks_to_unload.push((entity, pos));
                }
            }
            
            // 卸载远处的块
            for (entity, pos) in chunks_to_unload {
                commands.entity(entity).despawn();
                world_map.loaded_chunks.remove(&pos);
            }
            
            // 加载新块
            for x in min_x..=max_x {
                for y in min_y..=max_y {
                    let chunk_pos = IVec2::new(x, y);
                    
                    if !world_map.loaded_chunks.contains_key(&chunk_pos) {
                        let entity = spawn_chunk(&mut commands, &asset_server, chunk_pos, &mut world_map);
                        world_map.loaded_chunks.insert(chunk_pos, entity);
                    }
                }
            }
            
            // 更新玩家记录
            player.last_chunk = current_chunk;
        }
    }
}

/// 更新UI显示
fn update_ui(
    player_query: Query<(&Transform, &Player)>,
    mut text_query: Query<&mut Text, With<UiText>>,
    world_map: Res<WorldMap>,
) {
    if let Ok((player_transform, player)) = player_query.single() {
        if let Ok(mut text) = text_query.single_mut() {
            let current_chunk = world_to_chunk_coords(
                player_transform.translation,
                world_map.chunk_size,
                world_map.tile_size,
            );
            
            **text = format!(
                "玩家位置: ({:.1}, {:.1})\n\
                 当前块: ({}, {})\n\
                 已加载块: {}\n\
                 视野距离: {} 块\n\
                 移动: WASD\n\
                 块大小: {}x{} 瓦片",
                player_transform.translation.x,
                player_transform.translation.y,
                current_chunk.x,
                current_chunk.y,
                world_map.loaded_chunks.len(),
                world_map.view_distance,
                world_map.chunk_size.x,
                world_map.chunk_size.y
            );
        }
    }
}

/// 瓦片图集纹理转换系统
fn update_tileset_image(
    chunk_query: Query<&TilemapChunk>,
    mut events: MessageReader<AssetEvent<Image>>,
    mut images: ResMut<Assets<Image>>,
) {
    for chunk in chunk_query.iter() {
        for event in events.read() {
            if event.is_loaded_with_dependencies(&chunk.tileset) {
                if let Some(image) = images.get_mut(&chunk.tileset) {
                    image.reinterpret_stacked_2d_as_array(4);
                }
            }
        }
    }
}

/// 调试块边界显示
fn debug_chunk_borders(
    world_map: Res<WorldMap>,
    mut gizmos: Gizmos,
    player_query: Query<&Transform, With<Player>>,
) {
    if let Ok(player_transform) = player_query.single() {
        let current_chunk = world_to_chunk_coords(
            player_transform.translation,
            world_map.chunk_size,
            world_map.tile_size,
        );
        
        // 计算当前块的像素大小
        let chunk_pixel_width = world_map.chunk_size.x as f32 * world_map.tile_size.x as f32;
        let chunk_pixel_height = world_map.chunk_size.y as f32 * world_map.tile_size.y as f32;
        
        // 绘制当前块边界
        let chunk_center = Vec3::new(
            (current_chunk.x as f32 + 0.5) * chunk_pixel_width,
            (current_chunk.y as f32 + 0.5) * chunk_pixel_height,
            0.0,
        );
        
        gizmos.rect_2d(
            chunk_center.truncate(),
            Vec2::new(chunk_pixel_width, chunk_pixel_height),
            Color::srgb(1.0, 0.0, 0.0),
        );
    }
}

// ======================== 工具函数 ========================

/// 世界坐标转块坐标
fn world_to_chunk_coords(world_position: Vec3, chunk_size: UVec2, tile_size: UVec2) -> IVec2 {
    let chunk_width = chunk_size.x as f32 * tile_size.x as f32;
    let chunk_height = chunk_size.y as f32 * tile_size.y as f32;
    
    IVec2::new(
        (world_position.x / chunk_width).floor() as i32,
        (world_position.y / chunk_height).floor() as i32,
    )
}