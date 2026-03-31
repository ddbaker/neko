use bevy::prelude::Vec2;
use x11rb::connection::Connection;
use x11rb::protocol::xproto::ConnectionExt as _;

use super::cursor::{MonitorBounds, PlatformCursorCapabilities};

pub fn platform_cursor_capabilities() -> PlatformCursorCapabilities {
    PlatformCursorCapabilities {
        backend_name: "linux-x11-best-effort",
        supports_global_cursor: true,
        supports_native_monitor_bounds: false,
    }
}

pub fn global_cursor_position() -> Option<Vec2> {
    let (connection, screen_num) = x11rb::connect(None).ok()?;
    let screen = &connection.setup().roots[screen_num];
    let reply = connection.query_pointer(screen.root).ok()?.reply().ok()?;

    Some(Vec2::new(reply.root_x as f32, reply.root_y as f32))
}

pub fn monitor_bounds_for_point(_point: Vec2) -> Option<MonitorBounds> {
    None
}
