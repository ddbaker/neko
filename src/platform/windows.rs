use bevy::prelude::Vec2;

use super::cursor::{MonitorBounds, PlatformCursorCapabilities};

pub fn platform_cursor_capabilities() -> PlatformCursorCapabilities {
    PlatformCursorCapabilities {
        backend_name: "windows-win32",
        supports_global_cursor: true,
        supports_native_monitor_bounds: true,
    }
}

pub fn global_cursor_position() -> Option<Vec2> {
    use windows_sys::Win32::Foundation::POINT;
    use windows_sys::Win32::UI::WindowsAndMessaging::GetCursorPos;

    unsafe {
        let mut point = POINT { x: 0, y: 0 };
        if GetCursorPos(&mut point) == 0 {
            return None;
        }

        Some(Vec2::new(point.x as f32, point.y as f32))
    }
}

pub fn monitor_bounds_for_point(point: Vec2) -> Option<MonitorBounds> {
    use std::mem::size_of;

    use windows_sys::Win32::Foundation::{POINT, RECT};
    use windows_sys::Win32::Graphics::Gdi::{
        GetMonitorInfoW, MONITOR_DEFAULTTONEAREST, MONITORINFO, MonitorFromPoint,
    };

    unsafe {
        let monitor = MonitorFromPoint(
            POINT {
                x: point.x.round() as i32,
                y: point.y.round() as i32,
            },
            MONITOR_DEFAULTTONEAREST,
        );

        if monitor.is_null() {
            return None;
        }

        let mut info = MONITORINFO {
            cbSize: size_of::<MONITORINFO>() as u32,
            rcMonitor: RECT {
                left: 0,
                top: 0,
                right: 0,
                bottom: 0,
            },
            rcWork: RECT {
                left: 0,
                top: 0,
                right: 0,
                bottom: 0,
            },
            dwFlags: 0,
        };

        if GetMonitorInfoW(monitor, &mut info as *mut MONITORINFO) == 0 {
            return None;
        }

        Some(MonitorBounds {
            left: info.rcMonitor.left,
            top: info.rcMonitor.top,
            right: info.rcMonitor.right,
            bottom: info.rcMonitor.bottom,
        })
    }
}
