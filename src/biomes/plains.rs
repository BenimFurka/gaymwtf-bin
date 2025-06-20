use gaymwtf_core::Biome;

#[derive(Clone)]
pub struct PlainsBiome;

impl Biome for PlainsBiome {
    fn get_type_tag(&self) -> &'static str {
        "plains"
    }

    fn is_suitable(&self, height: f64, moisture: f64, temperature: f64) -> bool {
        height >= 0.5 && height <= 1.0 &&
        moisture >= 0.0 && moisture <= 0.5 &&
        temperature >= 0.3 && temperature <= 0.7
    }

    fn get_ground_tile_type(&self) -> &'static str {
        "grass"
    }

    fn get_spawnable_objects(&self) -> Vec<(&'static str, f32)> {
        vec![]
    }

    fn clone_box(&self) -> Box<dyn Biome> {
        Box::new(self.clone())
    }
}
