struct LevelConfig {
    xp_level_0: usize,
    max_xp_message: u8,
    difficulty: f32,
}

pub struct LevelData {
    xp: usize,
    level: usize   
}

impl LevelData {
    pub fn add_xp(&self, xp: usize) {
        
    }
}