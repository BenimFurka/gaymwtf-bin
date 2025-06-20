use gaymwtf_core::Biome;

#[derive(Clone)]
pub struct ForestBiome;

impl Biome for ForestBiome {
    fn get_type_tag(&self) -> &'static str {
        "forest"
    }

    fn is_suitable(&self, height: f64, moisture: f64, temperature: f64) -> bool {
        height >= 0.45 && temperature >= 0.45 && temperature <= 0.65 && moisture > 0.6
    }

    fn get_ground_tile_type(&self) -> &'static str {
        "grass"
    }

    fn get_spawnable_objects(&self) -> Vec<(&'static str, f32)> {
        vec![("tree", 0.1)]
    }

    fn clone_box(&self) -> Box<dyn Biome> {
        Box::new(self.clone())
    }
}
