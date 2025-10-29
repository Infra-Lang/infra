#[allow(dead_code)]
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
#[allow(dead_code)]
pub const NAME: &str = env!("CARGO_PKG_NAME");
#[allow(dead_code)]
pub const DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");

#[allow(dead_code)]
pub fn version_info() -> String {
    format!("{} v{}", NAME, VERSION)
}

#[allow(dead_code)]
pub fn full_version_info() -> String {
    format!("{} v{}\n{}", NAME, VERSION, DESCRIPTION)
}
