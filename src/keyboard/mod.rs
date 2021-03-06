/*
 * This file is part of espanso.
 *
 * Copyright (C) 2019 Federico Terzi
 *
 * espanso is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * espanso is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with espanso.  If not, see <https://www.gnu.org/licenses/>.
 */

use crate::config::Configs;
use serde::{Deserialize, Serialize};

#[cfg(target_os = "windows")]
pub mod windows;

#[cfg(target_os = "linux")]
mod linux;

#[cfg(target_os = "macos")]
pub mod macos;

pub trait KeyboardManager {
    fn send_string(&self, active_config: &Configs, s: &str);
    fn send_enter(&self, active_config: &Configs);
    fn trigger_paste(&self, active_config: &Configs);
    fn delete_string(&self, active_config: &Configs, count: i32);
    fn move_cursor_left(&self, active_config: &Configs, count: i32);
    fn trigger_copy(&self, active_config: &Configs);
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum PasteShortcut {
    Default,     // Default one for the current system
    CtrlV,       // Classic Ctrl+V shortcut
    CtrlShiftV,  // Could be used to paste without formatting in many applications
    ShiftInsert, // Often used in Linux systems
    CtrlAltV,    // Used in some Linux terminals (urxvt)
    MetaV,       // Corresponding to Win+V on Windows and Linux, CMD+V on macOS
}

impl Default for PasteShortcut {
    fn default() -> Self {
        PasteShortcut::Default
    }
}

// WINDOWS IMPLEMENTATION
#[cfg(target_os = "windows")]
pub fn get_manager() -> impl KeyboardManager {
    windows::WindowsKeyboardManager {}
}

// LINUX IMPLEMENTATION
#[cfg(target_os = "linux")]
pub fn get_manager() -> impl KeyboardManager {
    linux::LinuxKeyboardManager {}
}

// MAC IMPLEMENTATION
#[cfg(target_os = "macos")]
pub fn get_manager() -> impl KeyboardManager {
    macos::MacKeyboardManager {}
}
