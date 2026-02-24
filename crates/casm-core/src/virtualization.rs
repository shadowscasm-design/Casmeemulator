use crate::config::CasmConfig;

#[derive(Debug, Clone)]
pub struct HypervisorPlan {
    pub uses_vtx_or_amdv: bool,
    pub nested_paging: bool,
    pub huge_pages: bool,
    pub arm_translation: &'static str,
}

pub fn build_hypervisor_plan(config: &CasmConfig) -> HypervisorPlan {
    let huge_pages = config.ram_mb >= 4096;
    HypervisorPlan {
        uses_vtx_or_amdv: true,
        nested_paging: true,
        huge_pages,
        arm_translation: "hybrid JIT + cached block translation",
    }
}
