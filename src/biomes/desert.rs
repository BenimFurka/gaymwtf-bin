use gaymwtf_core::Biome;

#[derive(Clone)]
pub struct DesertBiome;

impl Biome for DesertBiome {
    fn get_type_tag(&self) -> &'static str {
        "desert"
    }

    fn is_suitable(&self, height: f64, _moisture: f64, temperature: f64) -> bool {
        height >= 0.45 && temperature >= 0.65
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
