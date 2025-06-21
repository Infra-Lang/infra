pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const NAME: &str = env!("CARGO_PKG_NAME");
pub const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");

pub fn version_info() -> String {
    format!("{} v{}", NAME, VERSION)
}

pub fn full_version_info() -> String {
    format!("{} v{}\n{}", NAME, VERSION, DESCRIPTION)
}
