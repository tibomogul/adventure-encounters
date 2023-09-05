pub mod helpers;
pub mod systems;

mod prelude {
    pub use bevy::prelude::*;
    pub use bevy_ecs_tilemap::prelude::*;
    pub use bracket_geometry::*;
    pub use bracket_pathfinding::prelude::*;
    pub use bracket_random::prelude::*;
    pub use crate::helpers::*;
    pub use crate::systems::*;
}

use prelude::{
    *,
    map::ObjectsMapLayer,
    map_builder::{MapBuilder, themes::MapTheme, MapArchitect, rooms::RoomsArchitect, custom::CustomFileBuilder},
    illumination::{ProvidesIllumination, illumination_system},
    tiles::TileType, field_of_view::FieldOfView,
};

fn startup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    #[cfg(all(not(feature = "atlas"), feature = "render"))] array_texture_loader: Res<
        ArrayTextureLoader,
    >,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    commands.spawn(Camera2dBundle::default());

    let mut rng = RandomNumberGenerator::new();

    // Choose how to build the map
    // This is random
    // let map_builder = MapBuilder::new_random(80, 50, &mut rng);
    // These three lines are for specific arhictect and theme
    // let architect: Box<dyn MapArchitect> = Box::new(RoomsArchitect {});
    // let theme = MapTheme::DungeonTheme;
    // let map_builder = MapBuilder::new(architect, theme, 80, 50, &mut rng);
    // This loads a map
    let map_builder = CustomFileBuilder::create_map_builder("campfire".to_string());

    let texture_handle: Handle<Image> = asset_server.load("ground.png");

    let map_size = TilemapSize { x: map_builder.map.dimensions.x as u32, y: map_builder.map.dimensions.y as u32 };

    // Create a tilemap entity a little early.
    // We want this entity early because we need to tell each tile which tilemap entity
    // it is associated with. This is done with the TilemapId component on each tile.
    // Eventually, we will insert the `TilemapBundle` bundle on the entity, which
    // will contain various necessary components, such as `TileStorage`.
    let tilemap_entity = commands.spawn_empty().id();

    // To begin creating the map we will need a `TileStorage` component.
    // This component is a grid of tile entities and is used to help keep track of individual
    // tiles in the world. If you have multiple layers of tiles you would have a tilemap entity
    // per layer, each with their own `TileStorage` component.
    let mut tile_storage = TileStorage::empty(map_size);

    let tile_size = TilemapTileSize { x: 32.0, y: 32.0 };
    let grid_size = tile_size.into();
    let map_type = TilemapType::default();

    // Spawn the elements of the tilemap.
    fill_tilemap(
        TileTextureIndex(map_builder.theme.tile_to_render(TileType::ThemeFloor).to_texture_index() as u32),
        map_size,
        TilemapId(tilemap_entity),
        &mut commands,
        &mut tile_storage,
    );

    commands.entity(tilemap_entity).insert(TilemapBundle {
        grid_size,
        map_type,
        size: map_size,
        storage: tile_storage,
        texture: TilemapTexture::Single(texture_handle.clone()),
        tile_size,
        transform: get_tilemap_center_transform(&map_size, &grid_size, &map_type, 0.0),
        ..Default::default()
    });

    // Layer 2
    let tilemap_entity = commands.spawn_empty().id();
    let mut tile_storage = TileStorage::empty(map_size);

    for x in 0..map_builder.map.dimensions.x {
        for y in 0..map_builder.map.dimensions.y {
            let idx = map_builder.map.map_idx(x, y);
            let tile = &map_builder.map.tiles[idx];
            let tile_pos = map_builder.map.to_bevy_ecs_tilemap(x, y);
            // let tile_pos = TilePos { x: x as u32, y: y as u32 };
            let tile_entity = commands
                .spawn(TileBundle {
                    position: tile_pos,
                    tilemap_id: TilemapId(tilemap_entity),
                    texture_index: TileTextureIndex(tile.tile_type.to_texture_index() as u32),
                    ..Default::default()
                })
                .id();
            tile_storage.set(&tile_pos, tile_entity);
        }
    }

    commands.entity(tilemap_entity).insert(TilemapBundle {
        grid_size,
        map_type,
        size: map_size,
        storage: tile_storage,
        texture: TilemapTexture::Single(texture_handle.clone()),
        tile_size,
        transform: get_tilemap_center_transform(&map_size, &grid_size, &map_type, 1.0),
        ..Default::default()
    }).insert(ObjectsMapLayer);

    // Layer fog of war
    let tilemap_entity = commands.spawn_empty().id();
    let mut tile_storage = TileStorage::empty(map_size);

    // Spawn the elements of the tilemap. Using 255, the black tile
    // visible false shows the underlying map
    // white and fully opaque will show black
    // white and 95% opacity shows dim
    // black and 95% opacity shows dim much like above
    // gray and 95% opacity shows dim much like above
    for x in 0..map_size.x {
        for y in 0..map_size.y {
            let tile_pos = TilePos { x, y };
            let tile_entity = commands
                .spawn(TileBundle {
                    position: tile_pos,
                    tilemap_id: TilemapId(tilemap_entity),
                    texture_index: TileTextureIndex(255),
                    color: TileColor(Color::rgba(0.0, 0.0, 0.0, 1.0)),
                    visible: TileVisible(true),
                    ..Default::default()
                })
                .id();
            tile_storage.set(&tile_pos, tile_entity);
        }
    }
    commands.entity(tilemap_entity).insert(TilemapBundle {
        grid_size,
        map_type,
        size: map_size,
        storage: tile_storage,
        texture: TilemapTexture::Single(texture_handle.clone()),
        tile_size,
        transform: get_tilemap_center_transform(&map_size, &grid_size, &map_type, 3.0),
        ..Default::default()
    }).insert(helpers::map::FogOfWarMapLayer);

    // Load player sprite
    let texture_handle: Handle<Image> = asset_server.load("monsters.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(32.0, 32.0), 16, 16, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    let mut transform = get_tilemap_center_transform(&map_size, &grid_size, &map_type, 2.0);
    let player_start = map_builder.player_start;
    let in_b_e_t = map_builder.map.to_bevy_ecs_tilemap(player_start.x, player_start.y);
    transform.translation.x += (in_b_e_t.x * 32) as f32;
    transform.translation.y += (in_b_e_t.y * 32) as f32;
    commands.spawn((
        SpriteSheetBundle {
            sprite: TextureAtlasSprite { index: 38, ..Default::default() },
            texture_atlas: texture_atlas_handle,
            transform,
            ..default()
        },
    ))
    // .insert(ProvidesIllumination::new(30, 60, None))
    .insert(FieldOfView::new(60, Some(0), Some(0)))
    .insert(map::MapPoint::new(player_start))
    .insert(Player);

    let texture_handle: Handle<Image> = asset_server.load("ground.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(32.0, 32.0), 16, 16, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    for (point, c) in map_builder.entity_spawns.clone() {
        let mut transform = get_tilemap_center_transform(&map_size, &grid_size, &map_type, 3.0);
        let in_b_e_t = map_builder.map.to_bevy_ecs_tilemap(point.x, point.y);
        transform.translation.x += (in_b_e_t.x * 32) as f32;
        transform.translation.y += (in_b_e_t.y * 32) as f32;
        commands.spawn((
            SpriteSheetBundle {
                sprite: TextureAtlasSprite { index: 135, ..Default::default() },
                texture_atlas: texture_atlas_handle.clone(),
                transform,
                ..default()
            },
        ))
        .insert(ProvidesIllumination::new(30, 60, None))
        .insert(map::MapPoint::new(point));    
    }

    commands.insert_resource(map_builder);

    // Add atlas to array texture loader so it's preprocessed before we need to use it.
    // Only used when the atlas feature is off and we are using array textures.
    #[cfg(all(not(feature = "atlas"), feature = "render"))]
    {
        array_texture_loader.add(TilemapArrayTexture {
            texture: TilemapTexture::Single(asset_server.load("tiles.png")),
            tile_size,
            ..Default::default()
        });
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: String::from("Adventure Encounters"),
                ..Default::default()
            }),
            ..default()
        }).set(ImagePlugin::default_nearest()))
        .add_plugin(TilemapPlugin)
        .add_startup_system(startup)
        .add_system(helpers::camera::movement)
        .add_system(systems::illumination::illumination_system)
        .add_system(systems::field_of_view::field_of_view_system.after(illumination_system))
        .add_system(systems::player_render_system.after(systems::field_of_view::field_of_view_system))
        .run();
}