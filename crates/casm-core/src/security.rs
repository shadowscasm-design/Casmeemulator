#[derive(Debug, Clone)]
pub struct SecurityModel {
    pub sandbox_mode: bool,
    pub optional_hwid_randomization: bool,
    pub secure_boot_chain: bool,
    pub telemetry_minimized: bool,
}

impl Default for SecurityModel {
    fn default() -> Self {
        Self {
            sandbox_mode: true,
            optional_hwid_randomization: true,
            secure_boot_chain: true,
            telemetry_minimized: true,
        }
    }
}
