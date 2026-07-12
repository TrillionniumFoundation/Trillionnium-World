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
    pub identities: AtlasFile,
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
    #[serde(default = "white_tint")]
    pub tint: [f32; 3],
    #[serde(default = "identity_scale")]
    pub silhouette_scale: [f32; 2],
    #[serde(default)]
    pub silhouette_rotation_degrees: f32,
    #[serde(default)]
    pub identity_frame: Option<usize>,
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
    #[serde(default = "white_tint")]
    pub tint: [f32; 3],
    #[serde(default = "identity_scale")]
    pub silhouette_scale: [f32; 2],
    #[serde(default)]
    pub silhouette_rotation_degrees: f32,
    #[serde(default)]
    pub identity_frame: Option<usize>,
}

fn white_tint() -> [f32; 3] {
    [1.0, 1.0, 1.0]
}

fn identity_scale() -> [f32; 2] {
    [1.0, 1.0]
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
            || self.atlases.identities.columns != 8
            || self.atlases.identities.rows != 3
        {
            return Err("First Contact atlases require normalized 128px grids".into());
        }
        if self.unit_families.len() < 18 || self.structure_families.len() < 15 {
            return Err(
                "First Contact atlas requires base and unique roster visual families".into(),
            );
        }
        if self.unit_families.iter().any(|family| {
            family.fps < 4 || family.idle.len() + family.r#move.len() + family.attack.len() + 2 < 8
        }) {
            return Err("every unit family requires a complete 8-frame base animation set".into());
        }
        let unit_identities = self
            .unit_families
            .iter()
            .filter(|family| {
                family.id.starts_with("mirror_")
                    || family.id.starts_with("ash_")
                    || matches!(family.id.as_str(), "relay_engineer_variant" | "field_medic")
            })
            .map(|family| {
                (
                    (family.silhouette_scale[0] * 100.0).round() as i16,
                    (family.silhouette_scale[1] * 100.0).round() as i16,
                    (family.silhouette_rotation_degrees * 10.0).round() as i16,
                )
            })
            .collect::<std::collections::BTreeSet<_>>();
        if unit_identities.len() < 12 {
            return Err("the twelve roster identities require distinct runtime silhouettes".into());
        }
        let bitmap_unit_frames = self
            .unit_families
            .iter()
            .filter_map(|family| family.identity_frame)
            .collect::<std::collections::BTreeSet<_>>();
        let bitmap_structure_frames = self
            .structure_families
            .iter()
            .filter_map(|family| family.identity_frame)
            .collect::<std::collections::BTreeSet<_>>();
        if bitmap_unit_frames.len() < 12 || bitmap_structure_frames.len() < 10 {
            return Err("twelve units and ten structures require independent bitmap frames".into());
        }
        let structure_identities = self
            .structure_families
            .iter()
            .skip(5)
            .map(|family| {
                (
                    (family.silhouette_scale[0] * 100.0).round() as i16,
                    (family.silhouette_scale[1] * 100.0).round() as i16,
                    (family.silhouette_rotation_degrees * 10.0).round() as i16,
                )
            })
            .collect::<std::collections::BTreeSet<_>>();
        if structure_identities.len() < 10 {
            return Err("the ten structure identities require distinct runtime silhouettes".into());
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
    pub identities_image: Handle<Image>,
    pub identities_layout: Handle<TextureAtlasLayout>,
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
    let identities_image = asset_server.load(manifest.atlases.identities.image.clone());
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
    let identities_layout = layouts.add(TextureAtlasLayout::from_grid(
        cell,
        manifest.atlases.identities.columns,
        manifest.atlases.identities.rows,
        None,
        None,
    ));
    FirstContactAtlasHandles {
        units_image,
        units_layout,
        world_image,
        world_layout,
        identities_image,
        identities_layout,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn authored_atlas_manifest_has_unique_roster_visuals_and_feedback() {
        let path =
            Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../assets/first_contact/atlas.yaml");
        let atlas = load_first_contact_atlas(&path).expect("authored atlas loads");
        assert_eq!(atlas.unit_families.len(), 18);
        assert_eq!(atlas.structure_families.len(), 15);
        assert_ne!(
            atlas.unit("mirror_wayfinder").unwrap().identity_frame,
            atlas.unit("ash_runner").unwrap().identity_frame
        );
        assert_ne!(
            atlas.structure("command_post").unwrap().identity_frame,
            atlas.structure("ash_beacon").unwrap().identity_frame
        );
        assert!(atlas.effect_frames.contains_key("selection_ring"));
        assert!(atlas.terrain_frames.contains_key("moss_basalt_ne"));
    }
}
