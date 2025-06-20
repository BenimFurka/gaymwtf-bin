use gaymwtf_core::Biome;

#[derive(Clone)]
pub struct SnowForestBiome;

impl Biome for SnowForestBiome {
    fn get_type_tag(&self) -> &'static str {
        "snow_forest"
    }

    fn is_suitable(&self, height: f64, moisture: f64, temperature: f64) -> bool {
        height >= 0.5 && height <= 1.0 &&
        moisture >= 0.5 && moisture <= 1.0 &&
        temperature >= 0.0 && temperature <= 0.3
    }

    fn get_ground_tile_type(&self) -> &'static str {
        "snowgrass"
    }

    fn get_spawnable_objects(&self) -> Vec<(&'static str, f32)> {
        vec![("snow_tree", 0.1)]
    }

    fn clone_box(&self) -> Box<dyn Biome> {
        Box::new(self.clone())
    }
}
