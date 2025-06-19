use gaymwtf_core::Biome;

#[derive(Clone)]
pub struct BeachBiome;

impl Biome for BeachBiome {
    fn get_type_tag(&self) -> &'static str {
        "beach"
    }

    fn is_suitable(&self, height: f64, _moisture: f64, temperature: f64) -> bool {
        height >= 0.425 && height < 0.45 && temperature > 0.3 && temperature < 0.8
    }

    fn get_ground_tile_type(&self) -> &'static str {
        "sand"
    }

    fn get_spawnable_entities(&self) -> Vec<(&'static str, f32)> {
        vec![]
    }

    fn clone_box(&self) -> Box<dyn Biome> {
        Box::new(self.clone())
    }
}
