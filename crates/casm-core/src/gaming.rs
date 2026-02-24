use crate::config::PerformancePreset;

#[derive(Debug, Clone)]
pub struct GamingMode {
    pub max_fps_target: u16,
    pub anti_stutter: bool,
    pub latency_optimization: bool,
    pub macro_engine: bool,
    pub keymapping_profiles: Vec<&'static str>,
}

pub fn gaming_mode_for(preset: PerformancePreset) -> GamingMode {
    let max_fps_target = match preset {
        PerformancePreset::Eco => 60,
        PerformancePreset::Balanced => 90,
        PerformancePreset::Esports => 144,
    };

    GamingMode {
        max_fps_target,
        anti_stutter: true,
        latency_optimization: true,
        macro_engine: true,
        keymapping_profiles: vec!["Free Fire", "PUBG Mobile", "Call of Duty Mobile"],
    }
}
