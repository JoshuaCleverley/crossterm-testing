use crossterm::{cursor, QueueableCommand};
use std::io::{Cursor, Stdout, Write};

pub struct Rect {
    pub x: u16,
    pub y: u16,
    pub w: u16,
    pub h: u16,
}

impl Rect {
    pub fn render(&self, mut stdout: &Stdout) {
        stdout.queue(cursor::MoveTo(self.x, self.y)).unwrap();
        let inner_rect = Rect {
            x: self.x + 1,
            y: self.y + 1,
            w: self.w - 2,
            h: self.h - 2,
        };

        for y in 0..(self.h as u32) {
            for x in 0..(self.w as u32) {
                if !inner_rect.collides_point(self.x + x as u16, self.y + y as u16) {
                    let horizontal = b"\xE2\x94\x81";
                    let vertical = b"\xe2\x94\x82";

                    let top_left = b"\xe2\x94\x8d";
                    let top_right = b"\xe2\x94\x91";
                    let bottom_left = b"\xe2\x94\x95";
                    let bottom_right = b"\xe2\x94\x99";

                    let mut char = horizontal;
                    if x == 0 || x == self.w as u32 - 1 {
                        char = vertical;
                    }
                    if x == 0 {
                        if y == 0 {
                            char = top_left;
                        } else if y == self.h as u32 - 1 {
                            char = bottom_left;
                        }
                    }
                    if x == self.w as u32 - 1 {
                        if y == 0 {
                            char = top_right;
                        } else if y == self.h as u32 - 1 {
                            char = bottom_right;
                        }
                    }

                    stdout
                        .queue(cursor::MoveTo(self.x + x as u16, self.y + y as u16))
                        .unwrap()
                        .write(char)
                        .unwrap();
                }
            }
        }
    }

    pub fn render_text(
        &self,
        mut stdout: &Stdout,
        text: &[u8],
        centered_horizontal: bool,
        centered_vertical: bool,
    ) {
        let mut text_start: (u16, u16) = (2, 1);

        if centered_horizontal {
            text_start.0 = self.w / 2 - text.len() as u16 / 2;
        }
        if centered_vertical {
            text_start.1 = self.h / 2;
        }

        stdout
            .queue(cursor::MoveTo(self.x + text_start.0, self.y + text_start.1))
            .unwrap()
            .write(text)
            .unwrap();
    }

    pub fn collides_point(&self, x: u16, y: u16) -> bool {
        x >= self.x && x < self.x + self.w && y >= self.y && y < self.y + self.h
    }
    pub fn collides_rect(&self, _rect: Rect) -> bool {
        unimplemented!()
    }
}
