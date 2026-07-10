use bevy::prelude::*;
use serde::Deserialize;
use std::{collections::BTreeMap, fs, path::Path};

#[derive(Debug, Clone, Deserialize)]
pub struct AtlasFile {
    pub image: String,
    pub columns: u32,
    pub rows: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AtlasFiles {
    pub units: AtlasFile,
    pub world: AtlasFile,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UnitFamily {
    pub id: String,
    pub row: usize,
    pub idle: [usize; 2],
    pub r#move: [usize; 2],
    pub attack: [usize; 2],
    pub hit: usize,
    pub disabled: usize,
    pub fps: u8,
}

impl UnitFamily {
    pub fn atlas_index(&self, column: usize) -> usize {
        self.row * 8 + column
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct StructureFamily {
    pub id: String,
    pub idle: usize,
    pub active: usize,
}

#[derive(Debug, Clone, Deserialize, Resource)]
pub struct FirstContactAtlasManifest {
    pub contract_version: String,
    pub art_direction: String,
    pub cell_size: u32,
    pub atlases: AtlasFiles,
    pub unit_families: Vec<UnitFamily>,
    pub terrain_frames: BTreeMap<String, usize>,
    pub detail_frames: BTreeMap<String, usize>,
    pub structure_families: Vec<StructureFamily>,
    pub landmark_frames: BTreeMap<String, [usize; 2]>,
    pub effect_frames: BTreeMap<String, usize>,
}

impl FirstContactAtlasManifest {
    pub fn validate(&self) -> Result<(), String> {
        if self.contract_version != "trnm_first_contact_atlas_v1" {
            return Err(format!(
                "unsupported First Contact atlas contract: {}",
                self.contract_version
            ));
        }
        if self.art_direction != "top_down_pixel_rts" {
            return Err("First Contact live screen must use top_down_pixel_rts".into());
        }
        if self.cell_size != 128
            || self.atlases.units.columns != 8
            || self.atlases.units.rows != 6
            || self.atlases.world.columns != 8
            || self.atlases.world.rows != 6
        {
            return Err("First Contact atlas must be a normalized 8x6 128px grid".into());
        }
        if self.unit_families.len() != 6 || self.structure_families.len() < 5 {
            return Err("First Contact atlas requires six unit and five structure families".into());
        }
        if self.unit_families.iter().any(|family| {
            family.fps < 4 || family.idle.len() + family.r#move.len() + family.attack.len() + 2 < 8
        }) {
            return Err("every unit family requires a complete 8-frame base animation set".into());
        }
        for required in [
            "moss_basalt_west",
            "moss_basalt_east",
            "moss_basalt_north",
            "moss_basalt_south",
            "moss_basalt_nw",
            "moss_basalt_ne",
            "moss_basalt_sw",
            "moss_basalt_se",
        ] {
            if !self.terrain_frames.contains_key(required) {
                return Err(format!("missing terrain transition frame: {required}"));
            }
        }
        for required in [
            "contact_shadow",
            "selection_ring",
            "hit_a",
            "hit_b",
            "muzzle_a",
            "muzzle_b",
        ] {
            if !self.effect_frames.contains_key(required) {
                return Err(format!("missing gameplay feedback frame: {required}"));
            }
        }
        Ok(())
    }

    pub fn unit(&self, id: &str) -> Option<&UnitFamily> {
        self.unit_families.iter().find(|family| family.id == id)
    }

    pub fn structure(&self, id: &str) -> Option<&StructureFamily> {
        self.structure_families
            .iter()
            .find(|family| family.id == id)
    }
}

#[derive(Resource, Clone)]
pub struct FirstContactAtlasHandles {
    pub units_image: Handle<Image>,
    pub units_layout: Handle<TextureAtlasLayout>,
    pub world_image: Handle<Image>,
    pub world_layout: Handle<TextureAtlasLayout>,
}

pub fn load_first_contact_atlas(path: &Path) -> Result<FirstContactAtlasManifest, String> {
    let source = fs::read_to_string(path).map_err(|error| {
        format!(
            "failed to read First Contact atlas manifest {}: {error}",
            path.display()
        )
    })?;
    let manifest: FirstContactAtlasManifest = serde_yaml::from_str(&source).map_err(|error| {
        format!(
            "failed to parse First Contact atlas manifest {}: {error}",
            path.display()
        )
    })?;
    manifest.validate()?;
    Ok(manifest)
}

pub fn register_first_contact_atlases(
    asset_server: &AssetServer,
    layouts: &mut Assets<TextureAtlasLayout>,
    manifest: &FirstContactAtlasManifest,
) -> FirstContactAtlasHandles {
    let units_image = asset_server.load(manifest.atlases.units.image.clone());
    let world_image = asset_server.load(manifest.atlases.world.image.clone());
    let cell = UVec2::splat(manifest.cell_size);
    let units_layout = layouts.add(TextureAtlasLayout::from_grid(
        cell,
        manifest.atlases.units.columns,
        manifest.atlases.units.rows,
        None,
        None,
    ));
    let world_layout = layouts.add(TextureAtlasLayout::from_grid(
        cell,
        manifest.atlases.world.columns,
        manifest.atlases.world.rows,
        None,
        None,
    ));
    FirstContactAtlasHandles {
        units_image,
        units_layout,
        world_image,
        world_layout,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn authored_atlas_manifest_has_six_units_five_structures_and_feedback() {
        let path =
            Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../assets/first_contact/atlas.yaml");
        let atlas = load_first_contact_atlas(&path).expect("authored atlas loads");
        assert_eq!(atlas.unit_families.len(), 6);
        assert_eq!(atlas.structure_families.len(), 5);
        assert!(atlas.effect_frames.contains_key("selection_ring"));
        assert!(atlas.terrain_frames.contains_key("moss_basalt_ne"));
    }
}
