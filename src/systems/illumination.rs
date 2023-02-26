use crate::prelude::*;

#[derive(Component)]
pub struct ProvidesIllumination {
    pub bright_radius: u16,
    pub shadowy_radius: u16,
    pub duration: u32 // in minutes, u32::MAX means infinite
}

// pub fn illumination_system(
//     added: Query<(Entity, &TilePos, &ProvidesIllumination), Added<ProvidesIllumination>>,
//     map: Query<(Entity, &mut TileStorage), With<helpers::map::FogOfWarMapLayer>>,
// ) {
//     for (entity, tile_pos, provides_illumination) in added.iter() {
//         // illuminate all the tiles within the entities' line of sight
//     }
// }