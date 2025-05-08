mod boot;

pub mod generic_timer;
#[cfg(not(platform_family = "aarch64-raspi"))]
pub mod psci;

#[cfg(not(platform_family = "aarch64-bsta1000b"))]
pub mod pl011;
