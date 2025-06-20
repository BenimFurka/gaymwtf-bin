use gaymwtf_core::Biome;

#[derive(Clone)]
pub struct BeachBiome;

impl Biome for BeachBiome {
    fn get_type_tag(&self) -> &'static str {
        "beach"
    }

    fn is_suitable(&self, height: f64, moisture: f64, temperature: f64) -> bool {
        height >= 0.48 && height <= 0.5 &&
        moisture >= 0.0 && moisture <= 1.0 &&
        temperature >= 0.0 && temperature <= 1.0
    }

    fn get_ground_tile_type(&self) -> &'static str {
        "sand"
    }

    fn get_spawnable_objects(&self) -> Vec<(&'static str, f32)> {
        vec![]
    }

    fn clone_box(&self) -> Box<dyn Biome> {
        Box::new(self.clone())
    }
}
