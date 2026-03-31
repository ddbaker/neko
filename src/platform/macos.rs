use bevy::prelude::Vec2;
use core_graphics::event::CGEvent;
use core_graphics::event_source::{CGEventSource, CGEventSourceStateID};

use super::cursor::{MonitorBounds, PlatformCursorCapabilities};

pub fn platform_cursor_capabilities() -> PlatformCursorCapabilities {
    PlatformCursorCapabilities {
        backend_name: "macos-core-graphics",
        supports_global_cursor: true,
        supports_native_monitor_bounds: false,
    }
}

pub fn global_cursor_position() -> Option<Vec2> {
    let source = CGEventSource::new(CGEventSourceStateID::CombinedSessionState).ok()?;
    let event = CGEvent::new(source).ok()?;
    let location = event.location();

    Some(Vec2::new(location.x as f32, location.y as f32))
}

pub fn monitor_bounds_for_point(_point: Vec2) -> Option<MonitorBounds> {
    None
}
