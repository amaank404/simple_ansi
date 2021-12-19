#![allow(dead_code)]

/// This module provides cursor movement functionalities
pub mod cursor {
    /// Moves the cursor to home position (topleft) (line: 0, column: 0)
    pub fn home() {
        print!("\x1b[H");
    }

    /// Moves the cursor to a specific line and column
    ///
    /// # Arguments
    ///
    /// * `line` - A u64 number depicting the line number
    /// * `column` - A u64 number depicting the columnt number
    pub fn move_to(line: u64, column: u64) {
        print!("\x1b[{line};{column}H", line = line, column = column);
    }

    /// Move up by the given number of lines
    ///
    /// # Arguments
    ///
    /// * `lines` - Number of lines to move up by
    pub fn move_up(lines: u64) {
        print!("\x1b[{}A", lines);
    }

    /// Move down by the given number of lines
    ///
    /// # Arguments
    ///
    /// * `lines` - Number of lines to move down by
    pub fn move_down(lines: u64) {
        print!("\x1b[{}B", lines);
    }

    /// Move right by the given number of lines
    ///
    /// # Arguments
    ///
    /// * `lines` - Number of lines to move right by
    pub fn move_right(columns: u64) {
        print!("\x1b[{}C", columns);
    }

    /// Move left by the given number of lines
    ///
    /// # Arguments
    ///
    /// * `lines` - Number of lines to move left by
    pub fn move_left(columns: u64) {
        print!("\x1b[{}D", columns);
    }

    /// Move cursor to specified column
    ///
    /// # Arguments
    ///
    /// * `column` - Column to move to
    pub fn move_to_column(column: u64) {
        print!("\x1b[{}G", column);
    }

    /// Hide cursor
    pub fn hide() {
        print!("\x1b[?25l");
    }

    /// Show cursor
    pub fn show() {
        print!("\x1b[?25h");
    }
}

/// This module provides erasing functionalities for the cursor
pub mod erase {
    /// Erase from cursor to the end of screen
    pub fn till_endscreen() {
        print!("\x1b[0J");
    }

    /// Erase from cursor to beginning of the screen
    pub fn till_startscreen() {
        print!("\x1b[1J");
    }

    /// Erase entire screen
    pub fn screen() {
        print!("\x1b[2J");
    }

    /// Erase from cursor to the end of line
    pub fn till_endline() {
        print!("\x1b[0K");
    }

    /// Erase from cursor to beginning of the line
    pub fn till_startline() {
        print!("\x1b[1K");
    }

    /// Erase entire line
    pub fn line() {
        print!("\x1b[2K");
    }
}

/// Screen related functions
pub mod screen {
    /// Save current data on screen into terminal buffer for restoring it later
    pub fn save() {
        print!("\x1b[?47h");
    }

    /// Restore data from the terminal buffer into current screen
    pub fn restore() {
        print!("\x1b[?47l");
    }

    /// Different terminal modes supported by this library.
    /// You can pass them to `set_mode` function to set it
    /// and to reset it you can pass it to `reset_mode` function.
    ///
    /// All modes start with either `Mono` (Monochrome) or `Color`
    /// (Color) optionally followed by a number indicating the
    /// number of colors in the mode. Then follows `Text` or `Graphics`
    /// which are also meant for which type of mode you want to enter
    /// given the choice between these two. Then follows screen size in
    /// the format `{rows}x{columns}`.
    pub enum Mode {
        MonoText40x25 = 0,
        ColorText40x25 = 1,
        MonoText80x25 = 2,
        ColorText80x25 = 3,
        Color4Graphics320x200 = 4,
        MonoGraphics320x200 = 5,
        MonoGraphics640x200 = 6,
        ColorGraphics320x200 = 13,
        Color16Graphics640x200 = 14,
        MonoGraphics640x350 = 15,
        Color16Graphics640x350 = 16,
        MonoGraphics640x480 = 17,
        Color16Graphics640x480 = 18,
        Color256Graphics320x200 = 19,
    }

    /// Set a mode from `Mode` enum
    ///
    /// # Arguments
    ///
    /// * `mode` - Mode from the enum `Mode`
    pub fn set_mode(mode: Mode) {
        print!("\x1b[={}h", mode as u8);
    }

    /// Reset a mode from `Mode` enum
    ///
    /// # Arguments
    ///
    /// * `mode` - Mode from the enum `Mode`
    pub fn reset_mode(mode: Mode) {
        print!("\x1b[={}l", mode as u8);
    }

    /// Enable line wrapping
    pub fn enable_line_wrapping() {
        print!("\x1b[=7h");
    }

    /// Disable line wrapping
    pub fn disable_line_wrapping() {
        print!("\x1b[=7l");
    }
}
