use gaymwtf_core::Biome;

#[derive(Clone)]
pub struct DesertBiome;

impl Biome for DesertBiome {
    fn get_type_tag(&self) -> &'static str {
        "desert"
    }

    fn is_suitable(&self, height: f64, moisture: f64, temperature: f64) -> bool {
        height >= 0.5 && height <= 1.0 &&
        moisture >= 0.0 && moisture <= 0.3 &&
        temperature >= 0.7 && temperature <= 1.0
    }

    fn get_ground_tile_type(&self) -> &'static str {
        "sand"
    }

    fn get_spawnable_objects(&self) -> Vec<(&'static str, f32)> {
        vec![("cactus", 0.06)]
    }

    fn clone_box(&self) -> Box<dyn Biome> {
        Box::new(self.clone())
    }
}
