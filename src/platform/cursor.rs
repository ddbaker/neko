use bevy::prelude::{Entity, IVec2, Vec2};
use bevy::window::Monitor;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MonitorBounds {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DesktopMonitor {
    pub bounds: MonitorBounds,
    pub is_primary: bool,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct DesktopMonitorLayout {
    monitors: Vec<DesktopMonitor>,
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

    pub fn contains_point(self, point: Vec2) -> bool {
        point.x >= self.left as f32
            && point.x < self.right as f32
            && point.y >= self.top as f32
            && point.y < self.bottom as f32
    }

    pub fn squared_distance_to_point(self, point: Vec2) -> f32 {
        let dx = if point.x < self.left as f32 {
            self.left as f32 - point.x
        } else if point.x >= self.right as f32 {
            point.x - self.right as f32
        } else {
            0.0
        };

        let dy = if point.y < self.top as f32 {
            self.top as f32 - point.y
        } else if point.y >= self.bottom as f32 {
            point.y - self.bottom as f32
        } else {
            0.0
        };

        dx * dx + dy * dy
    }
}

impl DesktopMonitorLayout {
    pub fn new<I>(monitors: I) -> Self
    where
        I: IntoIterator<Item = DesktopMonitor>,
    {
        let mut monitors: Vec<_> = monitors.into_iter().collect();
        monitors.sort_by_key(|monitor| {
            (
                !monitor.is_primary,
                monitor.bounds.left,
                monitor.bounds.top,
                monitor.bounds.right,
                monitor.bounds.bottom,
            )
        });
        Self { monitors }
    }

    pub fn is_empty(&self) -> bool {
        self.monitors.is_empty()
    }

    pub fn primary_monitor(&self) -> Option<DesktopMonitor> {
        self.monitors
            .iter()
            .find(|monitor| monitor.is_primary)
            .copied()
    }

    pub fn active_monitor(&self, point: Vec2) -> Option<DesktopMonitor> {
        self.monitor_containing_point(point)
            .or_else(|| self.nearest_monitor(point))
    }

    pub fn initial_window_position(
        &self,
        cursor_position: Option<Vec2>,
        window_width: i32,
        window_height: i32,
    ) -> Option<IVec2> {
        cursor_position
            .and_then(|point| self.active_monitor(point))
            .or_else(|| self.primary_monitor())
            .or_else(|| self.monitors.first().copied())
            .map(|monitor| {
                monitor
                    .bounds
                    .centered_window_position(window_width, window_height)
            })
    }

    pub fn clamp_window_position_for_cursor(
        &self,
        cursor_position: Vec2,
        position: Vec2,
        window_width: i32,
        window_height: i32,
    ) -> Option<Vec2> {
        self.active_monitor(cursor_position).map(|monitor| {
            monitor
                .bounds
                .clamp_window_position(position, window_width, window_height)
        })
    }

    fn monitor_containing_point(&self, point: Vec2) -> Option<DesktopMonitor> {
        self.monitors
            .iter()
            .find(|monitor| monitor.bounds.contains_point(point))
            .copied()
    }

    fn nearest_monitor(&self, point: Vec2) -> Option<DesktopMonitor> {
        self.monitors.iter().copied().min_by(|left, right| {
            left.bounds
                .squared_distance_to_point(point)
                .total_cmp(&right.bounds.squared_distance_to_point(point))
        })
    }
}

pub fn monitor_bounds_from_bevy(monitor: &Monitor) -> MonitorBounds {
    let size = monitor.physical_size();

    MonitorBounds {
        left: monitor.physical_position.x,
        top: monitor.physical_position.y,
        right: monitor.physical_position.x + size.x as i32,
        bottom: monitor.physical_position.y + size.y as i32,
    }
}

pub fn desktop_monitor_layout_from_bevy<'a, I>(
    monitors: I,
    primary_monitor: Option<Entity>,
) -> DesktopMonitorLayout
where
    I: IntoIterator<Item = (Entity, &'a Monitor)>,
{
    DesktopMonitorLayout::new(
        monitors
            .into_iter()
            .map(|(entity, monitor)| DesktopMonitor {
                bounds: monitor_bounds_from_bevy(monitor),
                is_primary: Some(entity) == primary_monitor,
            }),
    )
}

#[cfg(target_os = "windows")]
pub fn global_cursor_position() -> Option<Vec2> {
    use windows_sys::Win32::Foundation::POINT;
    use windows_sys::Win32::UI::WindowsAndMessaging::GetCursorPos;

    unsafe {
        // Bevy's window-local cursor APIs only report positions while the pointer is inside
        // the pet window. The desktop-pet chase behavior needs global desktop coordinates.
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
        // Keep the app on the nearest monitor instead of allowing accidental roaming
        // across displays, which matches the conservative Go reference behavior.
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

#[cfg(test)]
mod tests {
    use bevy::prelude::{IVec2, Vec2};

    use super::{DesktopMonitor, DesktopMonitorLayout, MonitorBounds};

    fn monitor(bounds: MonitorBounds, is_primary: bool) -> DesktopMonitor {
        DesktopMonitor { bounds, is_primary }
    }

    #[test]
    fn startup_position_prefers_monitor_containing_cursor() {
        let layout = DesktopMonitorLayout::new([
            monitor(
                MonitorBounds {
                    left: 0,
                    top: 0,
                    right: 100,
                    bottom: 100,
                },
                true,
            ),
            monitor(
                MonitorBounds {
                    left: 100,
                    top: 0,
                    right: 200,
                    bottom: 100,
                },
                false,
            ),
        ]);

        let position = layout.initial_window_position(Some(Vec2::new(150.0, 50.0)), 20, 20);
        assert_eq!(position, Some(IVec2::new(140, 40)));
    }

    #[test]
    fn startup_position_falls_back_to_primary_monitor_without_cursor() {
        let layout = DesktopMonitorLayout::new([
            monitor(
                MonitorBounds {
                    left: 300,
                    top: 0,
                    right: 500,
                    bottom: 200,
                },
                true,
            ),
            monitor(
                MonitorBounds {
                    left: 0,
                    top: 0,
                    right: 200,
                    bottom: 200,
                },
                false,
            ),
        ]);

        let position = layout.initial_window_position(None, 40, 40);
        assert_eq!(position, Some(IVec2::new(380, 80)));
    }

    #[test]
    fn active_monitor_prefers_containing_monitor_left_to_right() {
        let layout = DesktopMonitorLayout::new([
            monitor(
                MonitorBounds {
                    left: 0,
                    top: 0,
                    right: 100,
                    bottom: 100,
                },
                true,
            ),
            monitor(
                MonitorBounds {
                    left: 100,
                    top: 0,
                    right: 200,
                    bottom: 100,
                },
                false,
            ),
        ]);

        let monitor = layout.active_monitor(Vec2::new(150.0, 50.0));
        assert_eq!(monitor.map(|monitor| monitor.bounds.left), Some(100));
    }

    #[test]
    fn active_monitor_prefers_containing_monitor_right_to_left() {
        let layout = DesktopMonitorLayout::new([
            monitor(
                MonitorBounds {
                    left: 0,
                    top: 0,
                    right: 100,
                    bottom: 100,
                },
                true,
            ),
            monitor(
                MonitorBounds {
                    left: 100,
                    top: 0,
                    right: 200,
                    bottom: 100,
                },
                false,
            ),
        ]);

        let monitor = layout.active_monitor(Vec2::new(20.0, 50.0));
        assert_eq!(monitor.map(|monitor| monitor.bounds.left), Some(0));
    }

    #[test]
    fn active_monitor_handles_stacked_vertical_layouts() {
        let layout = DesktopMonitorLayout::new([
            monitor(
                MonitorBounds {
                    left: 0,
                    top: 0,
                    right: 100,
                    bottom: 100,
                },
                true,
            ),
            monitor(
                MonitorBounds {
                    left: 0,
                    top: 100,
                    right: 100,
                    bottom: 200,
                },
                false,
            ),
        ]);

        let monitor = layout.active_monitor(Vec2::new(50.0, 150.0));
        assert_eq!(monitor.map(|monitor| monitor.bounds.top), Some(100));
    }

    #[test]
    fn active_monitor_supports_negative_desktop_coordinates() {
        let layout = DesktopMonitorLayout::new([
            monitor(
                MonitorBounds {
                    left: -200,
                    top: 0,
                    right: 0,
                    bottom: 100,
                },
                false,
            ),
            monitor(
                MonitorBounds {
                    left: 0,
                    top: 0,
                    right: 200,
                    bottom: 100,
                },
                true,
            ),
        ]);

        let monitor = layout.active_monitor(Vec2::new(-150.0, 40.0));
        assert_eq!(monitor.map(|monitor| monitor.bounds.left), Some(-200));
    }

    #[test]
    fn active_monitor_chooses_nearest_monitor_for_gap_points() {
        let layout = DesktopMonitorLayout::new([
            monitor(
                MonitorBounds {
                    left: 0,
                    top: 0,
                    right: 100,
                    bottom: 100,
                },
                true,
            ),
            monitor(
                MonitorBounds {
                    left: 140,
                    top: 0,
                    right: 260,
                    bottom: 160,
                },
                false,
            ),
        ]);

        let monitor = layout.active_monitor(Vec2::new(130.0, 30.0));
        assert_eq!(monitor.map(|monitor| monitor.bounds.left), Some(140));
    }

    #[test]
    fn clamp_window_position_switches_to_monitor_on_cursor_crossing() {
        let layout = DesktopMonitorLayout::new([
            monitor(
                MonitorBounds {
                    left: 0,
                    top: 0,
                    right: 100,
                    bottom: 100,
                },
                true,
            ),
            monitor(
                MonitorBounds {
                    left: 100,
                    top: 0,
                    right: 200,
                    bottom: 100,
                },
                false,
            ),
        ]);

        let clamped = layout.clamp_window_position_for_cursor(
            Vec2::new(150.0, 40.0),
            Vec2::new(95.0, 8.0),
            32,
            32,
        );
        assert_eq!(clamped, Some(Vec2::new(100.0, 8.0)));
    }

    #[test]
    fn clamp_window_position_handles_uneven_monitor_sizes() {
        let layout = DesktopMonitorLayout::new([
            monitor(
                MonitorBounds {
                    left: 0,
                    top: 0,
                    right: 120,
                    bottom: 120,
                },
                true,
            ),
            monitor(
                MonitorBounds {
                    left: 120,
                    top: -40,
                    right: 360,
                    bottom: 160,
                },
                false,
            ),
        ]);

        let clamped = layout.clamp_window_position_for_cursor(
            Vec2::new(300.0, -10.0),
            Vec2::new(340.0, -50.0),
            48,
            48,
        );
        assert_eq!(clamped, Some(Vec2::new(312.0, -40.0)));
    }
}
