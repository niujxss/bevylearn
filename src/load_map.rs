use bevy::{prelude::*, sprite_render::TileData};
use bevy::sprite_render::{TilemapChunk, TilemapChunkTileData};

pub fn self_run() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (set_up,).chain()) //加入chain方法，表示链式执行；有先后顺序
        .add_systems(Update, update_tileset_image)
        .run();
}

fn set_up(
    mut commands: Commands,
    assets: Res<AssetServer>
) {
    let chunk_size = UVec2::splat(32); //32 * 32 个瓦片
    let tile_display_size = UVec2::splat(40); // 每个瓦片显示的尺寸是 40 * 40 像素

    // 创建预设的瓦片数据
    let tile_data = (0..chunk_size.element_product()).map( |index| { //计算出总的瓦片树
                let x = index % chunk_size.x;  // 第几列
                let y = index / chunk_size.x;  // 第几行

                match  (x + y) % 4 {
                    0 => {
                        Some(TileData::from_tileset_index(0))
                    },
                    1 => {
                        Some(TileData::from_tileset_index(1))
                    },
                    2 => {
                        Some(TileData::from_tileset_index(2))
                    },
                    3 => {
                        Some(TileData::from_tileset_index(3))
                    },
                    _ => unreachable!(),
                }
            }
        ).collect();
    
    commands.spawn((
       TilemapChunk {   //定义瓦片地图的基本属性
            chunk_size,
            tile_display_size,
            tileset: assets.load("array_texture.png"),
            ..default()
       },
       TilemapChunkTileData(tile_data),
       UpdateTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),

    ));

    commands.spawn(Camera2d);
}


fn update_tileset_image(
    chunk_query: Single<&TilemapChunk>, // 使用Single 进行查询；确保只有一个实体，否则会报错； 场景理论上只有一个地图块 &只读访问
    mut events: MessageReader<AssetEvent<Image>>, //事件读取器  监控图像加载事件
    mut images: ResMut<Assets<Image>>, //可变的图像资源
) {
    for event in events.read() {
        if event.is_loaded_with_dependencies(chunk_query.tileset.id()) { //chunk.tileset 瓦片地图纹理权柄
            let image = images.get_mut(chunk_query.tileset.id()).unwrap();
            image.reinterpret_stacked_2d_as_array(4); // 图片中有四个纹理
        }
    }
}


#[derive(Component, Deref, DerefMut)]
struct UpdateTimer(Timer);