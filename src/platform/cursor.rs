use bevy::prelude::{IVec2, Vec2};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MonitorBounds {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}

impl MonitorBounds {
    pub fn centered_window_position(self, window_width: i32, window_height: i32) -> IVec2 {
        let x = self.left + ((self.width() - window_width).max(0) / 2);
        let y = self.top + ((self.height() - window_height).max(0) / 2);
        IVec2::new(x, y)
    }

    pub fn clamp_window_position(
        self,
        position: Vec2,
        window_width: i32,
        window_height: i32,
    ) -> Vec2 {
        Vec2::new(
            position.x.clamp(
                self.left as f32,
                (self.right - window_width).max(self.left) as f32,
            ),
            position.y.clamp(
                self.top as f32,
                (self.bottom - window_height).max(self.top) as f32,
            ),
        )
    }

    pub fn width(self) -> i32 {
        self.right - self.left
    }

    pub fn height(self) -> i32 {
        self.bottom - self.top
    }
}

#[cfg(target_os = "windows")]
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

#[cfg(not(target_os = "windows"))]
pub fn global_cursor_position() -> Option<Vec2> {
    None
}

#[cfg(target_os = "windows")]
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

#[cfg(not(target_os = "windows"))]
pub fn monitor_bounds_for_point(_point: Vec2) -> Option<MonitorBounds> {
    None
}
