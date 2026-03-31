use bevy::prelude::Vec2;

use super::cursor::{MonitorBounds, PlatformCursorCapabilities};

pub fn platform_cursor_capabilities() -> PlatformCursorCapabilities {
    PlatformCursorCapabilities {
        backend_name: "unsupported-platform",
        supports_global_cursor: false,
        supports_native_monitor_bounds: false,
    }
}

pub fn global_cursor_position() -> Option<Vec2> {
    None
}

pub fn monitor_bounds_for_point(_point: Vec2) -> Option<MonitorBounds> {
    None
}
