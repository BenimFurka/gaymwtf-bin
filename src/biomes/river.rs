use gaymwtf_core::Biome;

#[derive(Clone)]
pub struct RiverBiome;

impl Biome for RiverBiome {
    fn get_type_tag(&self) -> &'static str {
        "river"
    }

    fn is_suitable(&self, height: f64, _moisture: f64, _temperature: f64) -> bool {
        height < 0.425
    }

    fn get_ground_tile_type(&self) -> &'static str {
        "water"
    }

    fn get_spawnable_objects(&self) -> Vec<(&'static str, f32)> {
        vec![]
    }

    fn clone_box(&self) -> Box<dyn Biome> {
        Box::new(self.clone())
    }
}
