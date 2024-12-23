//! # example
//! Get all display info
//! ```
//! use display_info::DisplayInfo;
//! use std::time::Instant;
//!
//! let start = Instant::now();
//!
//! let display_infos = DisplayInfo::all().unwrap();
//! for display_info in display_infos {
//!   println!("display_info {display_info:?}");
//! }
//! let display_info = DisplayInfo::from_point(100, 100).unwrap();
//! println!("display_info {display_info:?}");
//! println!("运行耗时: {:?}", start.elapsed());
//! ```

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "linux")]
use linux::{get_all, get_from_point, ScreenRawHandle};

#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "macos")]
use macos::{get_all, get_from_point, ScreenRawHandle};

#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "windows")]
use windows::{get_all, get_from_point, ScreenRawHandle};

use anyhow::Result;

#[derive(Debug, Clone)]
pub struct DisplayInfo {
    /// The Display Name
    pub name: String,
    /// Unique identifier associated with the display.
    pub id: u32,
    /// Native screen raw handle
    pub raw_handle: ScreenRawHandle,
    /// The display x coordinate.
    pub x: i32,
    /// The display x coordinate.
    pub y: i32,
    /// The display pixel width.
    pub width: u32,
    /// The display pixel height.
    pub height: u32,
    /// The width of a display in millimeters. This value may be 0.
    pub width_mm: i32,
    /// The height of a display in millimeters. This value may be 0.
    pub height_mm: i32,
    /// Can be 0, 90, 180, 270, represents screen rotation in clock-wise degrees.
    pub rotation: f32,
    /// Output device's pixel scale factor.
    pub scale_factor: f32,
    /// The display refresh rate.
    pub frequency: f32,
    /// Whether the screen is the main screen
    pub is_primary: bool,
}

impl DisplayInfo {
    pub fn all() -> Result<Vec<DisplayInfo>> {
        get_all()
    }

    pub fn from_point(x: i32, y: i32) -> Result<DisplayInfo> {
        get_from_point(x, y)
    }

    pub fn from_name(name: impl ToString) -> Result<DisplayInfo> {
        let name = name.to_string();
        let display_infos = get_all()?;

        display_infos
            .iter()
            .find(|&d| d.name == name)
            .cloned()
            .ok_or_else(|| anyhow::anyhow!("Get display info failed"))
    }
}
