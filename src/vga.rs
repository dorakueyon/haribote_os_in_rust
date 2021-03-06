use core::fmt;

use crate::asm;
use crate::fonts::{FONTS, FONT_HEIGHT, FONT_WIDTH};

const COLOR_PALETTE: [[u8; 3]; 16] = [
	[0x00, 0x00, 0x00],	/*  0:黒 */
	[0xff, 0x00, 0x00],	/*  1:明るい赤 */
	[0x00, 0xff, 0x00],	/*  2:明るい緑 */
	[0xff, 0xff, 0x00],	/*  3:明るい黄色 */
	[0x00, 0x00, 0xff],	/*  4:明るい青 */
	[0xff, 0x00, 0xff],	/*  5:明るい紫 */
	[0x00, 0xff, 0xff],	/*  6:明るい水色 */
	[0xff, 0xff, 0xff],	/*  7:白 */
	[0xc6, 0xc6, 0xc6],	/*  8:明るい灰色 */
	[0x84, 0x00, 0x00],	/*  9:暗い赤 */
	[0x00, 0x84, 0x00],	/* 10:暗い緑 */
	[0x84, 0x84, 0x00],	/* 11:暗い黄色 */
	[0x00, 0x00, 0x84],	/* 12:暗い青 */
	[0x84, 0x00, 0x84],	/* 13:暗い紫 */
	[0x00, 0x84, 0x84],	/* 14:暗い水色 */
	[0x84, 0x84, 0x84]	/* 15:暗い灰色 */
];



#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]

pub enum Color {
    Black = 0,
    LightRed = 1,
    LightGreen = 2,
    LightYellow = 3,
    LightBlue = 4,
    LightPurple = 5,
    LightCyan = 6,
    White = 7,
    LightGray = 8,
    DarkRed = 9,
    DarkGreen = 10,
    DarkYellow = 11,
    DarkBlue = 12,
    DarkPurple = 13,
    DarkCyan = 14,
    DarkGray = 15,
}

#[derive(Debug)]
pub struct Screen {
    pub scrnx: i16,
    pub scrny: i16,
    pub vram: &'static mut u8,
    pub mouse: [[Color; MOUSE_CURSOR_WIDTH]; MOUSE_CURSOR_HEIGHT],
}

const MOUSE_CURSOR_WIDTH: usize = 16;
const MOUSE_CURSOR_HEIGHT: usize = 16;

impl Screen {
    //  SCRNX	EQU		0x0ff4
    //  SCRNY	EQU		0x0ff6
    //  VRAM	EQU		0x0ff8
    pub fn new() -> Screen {
        Screen {
            scrnx: unsafe { *(0x0ff4 as *const i16)},
            scrny: unsafe { *(0x0ff6 as *const i16)},
            vram: unsafe { &mut *( *(0x0ff8 as *const i32) as *mut u8) },
            mouse: [[Color::DarkCyan; MOUSE_CURSOR_WIDTH]; MOUSE_CURSOR_HEIGHT],
        }
    }
    
    pub fn init(&mut self) {
        self.init_pallet();
        self.init_screen();
        self.init_mouse_cursor();
        self.putblock(
            self.mouse,
            MOUSE_CURSOR_WIDTH as isize,
            MOUSE_CURSOR_HEIGHT as isize,
            (self.scrnx as isize - MOUSE_CURSOR_WIDTH as isize) / 2,
            (self.scrny as isize - MOUSE_CURSOR_HEIGHT as isize - 28) /2,
        )
    }

    pub fn init_pallet(&self) {
        let eflags = asm::load_eflags();
        asm::cli();
        asm::out8(0x03c8, 0);
        for i in 0..16 {
            asm::out8(0x03c9, COLOR_PALETTE[i][0] / 4);
            asm::out8(0x03c9, COLOR_PALETTE[i][0] / 4);
            asm::out8(0x03c9, COLOR_PALETTE[i][0] / 4);
        }
        asm::store_eflags(eflags);
    }
    
    pub fn init_screen(&mut self){
        use Color::*;

        let xsize = self.scrnx as isize;
        let ysize = self.scrny as isize; 

        self.boxfill8(DarkCyan, 0, 0, xsize - 1, ysize - 29);
        self.boxfill8(LightGray, 0, ysize - 28, xsize - 1, ysize - 28);
        self.boxfill8(White, 0, ysize - 27, xsize - 1, ysize - 27);
        self.boxfill8(LightGray, 0, ysize - 26, xsize - 1, ysize - 1);

        self.boxfill8(White, 3, ysize - 24, 59, ysize - 24);
        self.boxfill8(White, 2, ysize - 24, 2, ysize - 4);
        self.boxfill8(DarkYellow, 3, ysize - 4, 59, ysize - 4);
        self.boxfill8(DarkYellow, 59, ysize - 23, 59, ysize - 5);
        self.boxfill8(Black,  2, ysize -  3, 59, ysize - 3);
        self.boxfill8(Black, 60, ysize - 24, 60, ysize - 3);

        self.boxfill8(DarkGray, xsize - 47, ysize - 24, xsize - 4, ysize - 24);
        self.boxfill8(DarkGray, xsize - 47, ysize - 23, xsize - 47, ysize - 4);
        self.boxfill8(White, xsize - 47, ysize - 3, xsize - 4, ysize - 3);
        self.boxfill8(White, xsize - 3, ysize - 24, xsize - 3, ysize - 3);
    }

    pub fn boxfill8(&mut self, color: Color, x0: isize, y0: isize, x1: isize, y1: isize) {
        for y in y0..=y1 {
            for x in x0..=x1 {
                let ptr = unsafe { &mut *((self.vram as *mut u8).offset(y * self.scrnx as isize + x))} ;
                *ptr = color as u8;
            }
        }
    }
    pub fn print_char(&mut self, char: u8, color: Color, startx: isize, starty: isize) {
        let font = FONTS[char as usize];
        let color = color as u8;
        let offset = startx + starty * self.scrnx as isize;
        for y in 0..FONT_HEIGHT {
            for x in 0..FONT_WIDTH {
                if font[y][x] {
                    let cell = (y * self.scrnx as usize + x) as isize;
                    let ptr = unsafe { &mut *((self.vram as *mut u8).offset(cell + offset))};
                    *ptr = color;
                }
            }
        }
    }
    fn init_mouse_cursor(&mut self) {
        let cursor: [[u8; MOUSE_CURSOR_WIDTH]; MOUSE_CURSOR_HEIGHT] = [
            *b"**************..",
            *b"*OOOOOOOOOOO*...",
            *b"*OOOOOOOOOO*....",
            *b"*OOOOOOOOO*.....",
            *b"*OOOOOOOO*......",
            *b"*OOOOOOO*.......",
            *b"*OOOOOOO*.......",
            *b"*OOOOOOOO*......",
            *b"*OOOO**OOO*.....",
            *b"*OOO*..*OOO*....",
            *b"*OO*....*OOO*...",
            *b"*O*......*OOO*..",
            *b"**........*OOO*.",
            *b"*..........*OOO*",
            *b"............*OO*",
            *b".............***",
        ];
        
        for y in 0..MOUSE_CURSOR_HEIGHT {
            for x in 0..MOUSE_CURSOR_WIDTH {
                match cursor[y][x] {
                    b'*' => self.mouse[y][x] = Color::DarkRed,
                    b'O' => self.mouse[y][x] = Color::White,
                    _ => (),
                }
            }
        }
    }

    fn putblock(
        &mut self,
        image: [[Color; 16]; 16],
        pxsize: isize,
        pysize: isize,
        px0: isize,
        py0: isize,
    ) {
        for y in 0..pysize {
            for x in 0..pxsize {
                let ptr = unsafe {
                    &mut *((self.vram as *mut u8)
                            .offset((py0 + y) * (self.scrnx as isize) + (px0 + x)))
                };
                *ptr = image[y as usize][x as usize] as u8;
            }
        }
    }
}

pub struct ScreenWriter {
    initial_x: usize,
    x: usize,
    y: usize,
    color: Color,
    screen: Screen,
}

impl ScreenWriter {
    pub fn new(screen: Screen, color: Color, x: usize, y: usize) -> ScreenWriter {
        ScreenWriter {
            initial_x: x,
            x,
            y,
            color,
            screen
        }
    }

    fn newline(&mut self) {
        self.x = self.initial_x;
        self.y = self.y + FONT_HEIGHT;
    }
}

impl fmt::Write for ScreenWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let str_bytes = s.as_bytes();
        let height = self.screen.scrny as usize;
        let width = self.screen.scrnx as usize;
        for i in 0..str_bytes.len() {
            if str_bytes[i] == b'\n'  {
                self.newline();
                return Ok(());
            }
            if self.x + FONT_WIDTH < width && self.y + FONT_HEIGHT < height {
                self.screen
                    .print_char(str_bytes[i], self.color, self.x as isize, self.y as isize);
            } else if self.y + FONT_HEIGHT * 2 < height {
                self.newline();
                self.screen
                    .print_char(str_bytes[i], self.color, self.x as isize, self.y as isize)
            }

            if self.x + FONT_WIDTH < width {
                self.x = self.x + FONT_WIDTH;
            } else if self.y + FONT_HEIGHT < height {
                self.newline();
            } else {
                self.x = width;
                self.y = height;
            }
        }
        Ok(())
    }
}

