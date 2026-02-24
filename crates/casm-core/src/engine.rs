use anyhow::Result;
use tokio::time::{sleep, Duration};

use crate::{
    config::CasmConfig,
    gaming::{gaming_mode_for, GamingMode},
    gms::GmsCompatibility,
    security::SecurityModel,
    virtualization::{build_hypervisor_plan, HypervisorPlan},
};

#[derive(Debug, Clone)]
pub struct BootReport {
    pub boot_time_ms: u64,
    pub hypervisor: HypervisorPlan,
    pub gms: GmsCompatibility,
    pub gaming: GamingMode,
    pub security: SecurityModel,
}

#[derive(Debug, Clone)]
pub struct CasmEngine {
    pub config: CasmConfig,
}

impl CasmEngine {
    pub fn new(config: CasmConfig) -> Self {
        Self { config }
    }

    pub async fn boot(&self) -> Result<BootReport> {
        let base = match self.config.performance_preset {
            crate::config::PerformancePreset::Eco => 6_500,
            crate::config::PerformancePreset::Balanced => 4_500,
            crate::config::PerformancePreset::Esports => 3_200,
        };

        sleep(Duration::from_millis(50)).await;

        Ok(BootReport {
            boot_time_ms: base,
            hypervisor: build_hypervisor_plan(&self.config),
            gms: GmsCompatibility::default(),
            gaming: gaming_mode_for(self.config.performance_preset),
            security: SecurityModel::default(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{CasmConfig, PerformancePreset};

    #[tokio::test]
    async fn esports_profile_targets_high_fps() {
        let mut cfg = CasmConfig::default();
        cfg.performance_preset = PerformancePreset::Esports;

        let report = CasmEngine::new(cfg).boot().await.unwrap();
        assert_eq!(report.gaming.max_fps_target, 144);
        assert!(report.hypervisor.uses_vtx_or_amdv);
    }
}
