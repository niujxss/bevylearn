use bevy::{ prelude::*, sprite_render::TilemapChunk};
use rand::SeedableRng;
use std::collections::HashMap;
use bevy::sprite_render::TileData;
use rand_chacha::ChaCha8Rng;
use bevy::sprite_render::TilemapChunkTileData;




// ====================== 组件定义 ===================

/// 世界地图资源
#[derive(Resource)] //Resource是Bevy ECS中的全局单例数据。与组件（附加到实体）不同，资源是独立存在的全局数据。
struct WorldMap {
    chunk_size: UVec2,   //每块的瓦片数量
    tile_size: UVec2,    //每个瓦片的像素大小
    view_distance: i32,  //以块为单位的视野距离
    loaded_chunks: HashMap<IVec2, Entity>, //已加载的块实体
    chunk_data_cache: HashMap<IVec2, Vec<Option<TileData>>>, //缓存已生成的地块数据
}


/// 地图随机器资源
#[derive(Resource, Deref, DerefMut)] // Deref, DerefMut 可以直接调用内部ChaCha8Rng的接口
struct GameRng(ChaCha8Rng);

/// 玩家组件
#[derive(Component)]
struct Player {
    move_speed: f32,
    last_chunk: IVec2, //记录上次所在的块
}

/// 地图块位置组件
#[derive(Component, Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct ChunkPosition {
    x: i32,
    y: i32,
}

#[derive(Component)]
struct UiText;

// ======================== 常量定义 ========================

const CHUNK_TILE_SIZE: u32 = 32;  // 每块32x32个瓦片
const TILE_PIXEL_SIZE: u32 = 80;  // 每个瓦片40像素
const VIEW_DISTANCE: i32 = 2;     // 视野距离2个块
const PLAYER_SPEED: f32 = 150.0;  // 玩家移动速度
const MAP_HASH_VALUE: u64 = 108;  // 随机地图的初始种子

/// 游戏状态
#[derive(States, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
enum GameState {
    #[default]
    Loading,
    Playing,
}

pub fn map_run() {
    App::new()
    .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
    .init_state::<GameState>()
    .add_systems(Startup, setup_windows)
    .add_systems(OnEnter(GameState::Loading), setup_game)
    .add_systems(Update, check_initial_load.run_if(in_state(GameState::Loading)))
    .add_systems(OnEnter(GameState::Playing), spawn_initial_chunks)
    .add_systems(Update, (
            player_movement,
            camera_follow,
            update_chunks,
            update_tileset_image,
        ).run_if(in_state(GameState::Playing)),
    )
    .run();
}

fn setup_windows(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Transform::from_xyz(0.0, 0.0, 1000.0),
    ));
}

fn setup_game(
    mut commands : Commands,
    asset_server: Res<AssetServer>, // 从文件系统异步加载资源
) {
    // 初始化时间地图资源
    commands.insert_resource(  //他的传参是一个 Resource(资源) 全局唯一的单例
        WorldMap {
            chunk_size: UVec2::splat(CHUNK_TILE_SIZE),
            tile_size: UVec2::splat(TILE_PIXEL_SIZE),
            view_distance: VIEW_DISTANCE,
            loaded_chunks: HashMap::new(),
            chunk_data_cache: HashMap::new(),
        }
    );
    //如果insert_resource再次插入相同的数据，那么新数据会替代老数据

    let rng = ChaCha8Rng::seed_from_u64(MAP_HASH_VALUE);
    commands.insert_resource(
        GameRng(rng)
    );

    // 生成玩家实体
    commands.spawn(
(
            Sprite {
                color: Color::srgb(1.0, 0.0, 0.0),
                custom_size: Some(Vec2::splat(20.0)),
                ..default()
            },
            Transform::from_xyz(0.0, 0.0, 1.0),
            Player {
                move_speed: PLAYER_SPEED,
                last_chunk: IVec2::ZERO,
            }
        )
    );



    // 创建UI
    setup_ui(&mut commands, asset_server);
}


fn setup_ui(
    commands: &mut Commands,
    asset_server: Res<AssetServer>
) {
    let chinese_font = asset_server.load("fonts/STKAITI.TTF");

    commands.spawn(
(
            Node {
                width: Val::Percent(100.0), // 宽，父节点100%
                height: Val::Percent(100.0),// 高，父节点100%
                justify_content: JustifyContent::SpaceBetween, // 垂直方向子元素两端对齐
                align_items: AlignItems::Center, // 水平方向居中对齐
                flex_direction: FlexDirection::Column, // 垂直布局
                ..default()
            },
            UiText,
        )
    ).with_children(|parent| {
            parent.spawn(

        (
                    Text::new("动态瓦片地图"),
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
                )

            );


        }

    );
}

fn check_initial_load(mut next_state: ResMut<NextState<GameState>>) {
    // 这里可以添加实际资源加载检查
    // 为了简化，我们假设资源已加载完成
    next_state.set(GameState::Playing);

}

// 初始化视野范围内的瓦片块数据
fn spawn_initial_chunks(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut world_map: ResMut<WorldMap>,
    player_query: Query<&Transform, With<Player>>,
) {
    if let Ok(_) = player_query.single() { //signle确保只有一个数据
        // let player_chunk = world_to_chunk_coords(
        //     player_transform.translation,
        //     world_map.chunk_size,
        //     world_map.tile_size,
        // );

        let view_distance = world_map.view_distance;
        for x in -view_distance..=view_distance { // 从负到正
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

fn world_to_chunk_coords(
    world_position: Vec3,
    chunk_size: UVec2,
    tile_size: UVec2,
) -> IVec2 {
    let chunk_width = chunk_size.x as f32 * tile_size.x as f32;
    let chunk_height = chunk_size.y as f32 * tile_size.y as f32;

    IVec2 { 
        x: (world_position.x / chunk_width).floor() as i32, // floor 是向下取整，无论是正数还是负数
        y: (world_position.y / chunk_height).floor() as i32,
    }
}

// 加载一个瓦片块
fn spawn_chunk(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    chunk_pos: IVec2,
    world_map: &mut WorldMap,
) -> Entity  {
    let chunk_size = world_map.chunk_size;
    let tile_size = world_map.tile_size;

    // 一个块像素大小；
    let x = chunk_size.x as f32 * tile_size.x as f32;
    let y = chunk_size.y as f32 * tile_size.y as f32;

    let world_x = x * chunk_pos.x as f32;
    let world_y = y * chunk_pos.y as f32;

    let tile_data = 
        if let Some(data) = world_map.chunk_data_cache.get(&chunk_pos) {
            data.clone()
        } else {
            generate_chunk_data(chunk_pos, chunk_size)
        };
    
    // 缓存数据
    world_map.chunk_data_cache.insert(chunk_pos,tile_data.clone());

    commands.spawn(
    (
            TilemapChunk { //TilemapChunk 是一个组件，包含瓦片地图块的**元数据**
                           //它告诉渲染器如何渲染这个块
                chunk_size,  // 块的尺寸（瓦片数量）
                tile_display_size: tile_size,  // 每个瓦片的显示大小（像素）
                tileset: asset_server.load("array_texture.png"),  // 瓦片集纹理
                ..default()
            },
            TilemapChunkTileData(tile_data), // 它包含瓦片的具体数据 它包装了一个 Vec<Option<TileData>>
            // 上边两个组件共同描述了一个瓦片块如何进行渲染

            ChunkPosition {
                x: chunk_pos.x,
                y: chunk_pos.y,
            },
            Transform::from_xyz(world_x, world_y, 0.0),
        )
    ).id()
    


}

//生产单个瓦片块信息
fn generate_chunk_data(chunk_pos: IVec2, chunk_size: UVec2) -> Vec<Option<TileData>> { //TileData 主要是瓦片集的索引；就是在瓦片集中是第几个瓦片

    let mut tile_data = Vec::with_capacity( // with_capacity 方法是提前扩充这么大的大小
        chunk_size.element_product() as usize // 整个块的面积大小
    );

    for i in 0..chunk_size.element_product() {
        let x = (i % chunk_size.x) as i32;
        let y = (i / chunk_size.x) as i32;

        //计算全局坐标

        let global_x = chunk_pos.x * chunk_size.x as i32 + x;
        let global_y = chunk_pos.y * chunk_size.y as i32 + y;

        // 使用柏林噪声简化版生成地形  就是一个平滑算法
        let tile_type = (((global_x as f32 * 0.1).sin()  // 平滑算法计算出一个瓦片集中的瓦片编号
            + (global_y as f32 * 0.1).cos()) * 2.0).abs() as u32 % 4;

        tile_data.push(Some(TileData::from_tileset_index(tile_type as u16)));
    }

    tile_data
}





/// 玩家输入处理
fn player_movement(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(&mut Transform, &mut Player)>,
) {
    for (mut transform, player) in player_query.iter_mut() {
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

// 相机跟随
fn camera_follow(
    player_query: Query<&Transform, (With<Player>, Without<Camera>)>,
    mut camera_query: Query<&mut Transform, (With<Camera>, Without<Player>)>,
) {
    if let Ok(player_query) = player_query.single() {
        if let Ok(mut camera_transform) = camera_query.single_mut() {

            let target = player_query.translation;
            let current = camera_transform.translation;
            camera_transform.translation = current.lerp(target, 0.1); //平滑跟随，每次变化10%
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
        
        // 计算玩家所处的块
        let current_chunk = world_to_chunk_coords(
            player_transform.translation,
            world_map.chunk_size,
            world_map.tile_size,
        );
        if current_chunk != player.last_chunk {
            let min_x = current_chunk.x - world_map.view_distance;
            let max_x = current_chunk.x + world_map.view_distance;

            let min_y = current_chunk.y - world_map.view_distance;
            let max_y = current_chunk.y + world_map.view_distance;

            // 收集需要卸载的块
            let mut chunks_to_unload = Vec::new();

            for (entity, chunk_pos) in chunk_query.iter() {
                let pos = IVec2::new(chunk_pos.x, chunk_pos.y);
                if pos.x < min_x || pos.x > max_x ||
                    pos.y < min_y || pos.y > max_y {
                        chunks_to_unload.push((entity, pos));
                    }
            }

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

            player.last_chunk = current_chunk;
        }

    }
}

fn update_tileset_image(
    chunk_query: Query<&TilemapChunk>,
    mut events: MessageReader<AssetEvent<Image>>,
    mut images: ResMut<Assets<Image>>,
) {
    for chunk in chunk_query.iter() {
        for event in events.read() {
            if event.is_loaded_with_dependencies(&chunk.tileset) { //是否是我们需要的瓦片地图加载完了
                if let Some(image) = images.get_mut(&chunk.tileset) {
                    image.reinterpret_stacked_2d_as_array(4); // 纹理集堆叠转换


                }
            }
        }
    }
}