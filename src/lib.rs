pub mod task;

use ncurses::*;

pub struct Vec2 {
    pub x: u16,
    pub y: u16,
}

impl Vec2 {
    fn from(x: u16, y: u16) -> Vec2 {
        Self { x, y }
    }
}

pub struct Widget {
    pub position: Vec2,
    pub content: String,
    pub has_highlight: bool,
}

pub struct UserCursor {
    pub position: Vec2,
    pub max_x: u16,
    pub max_y: u16,
}

impl UserCursor {
    pub fn from(max_x: u16, max_y: u16) -> UserCursor {
        Self {
            position: Vec2::from(0, 0),
            max_x,
            max_y,
        }
    }
}

pub struct App {
    is_ended: bool,
    cursor: UserCursor,
}

const NORMAL: i16 = 0;
const HIGHLIGHT: i16 = 1;

impl App {
    fn init() {
        initscr();
        noecho();
        curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);

        start_color();
        init_pair(HIGHLIGHT, COLOR_BLACK, COLOR_WHITE);
        init_pair(NORMAL, COLOR_WHITE, COLOR_BLACK);
    }

    pub fn new(cursor: UserCursor) -> App {
        App::init();

        Self {
            is_ended: false,
            cursor,
        }
    }

    pub fn update(&mut self) -> char {
        let key = getch() as u8 as char;

        match key {
            'q' => {
                self.release();
            }
            'h' => {
                if self.cursor.position.x != 0 {
                    self.cursor.position.x -= 1;
                }
            }
            'j' => {
                if self.cursor.position.y != 0 {
                    self.cursor.position.y -= 1;
                }
            }
            'k' => {
                if self.cursor.position.y + 1 < self.cursor.max_y {
                    self.cursor.position.y += 1;
                }
            }
            'l' => {
                if self.cursor.position.x + 1 < self.cursor.max_x {
                    self.cursor.position.x += 1;
                }
            }
            _ => (),
        };

        key
    }

    pub fn render(&self, widgets: Vec<Widget>) {
        for widget in widgets {
            mv(widget.position.y as i32, widget.position.x as i32);

            let mut pair = NORMAL;

            if widget.has_highlight {
                pair = HIGHLIGHT;
            }

            attron(COLOR_PAIR(pair));
            addstr(widget.content.as_str());
            attroff(COLOR_PAIR(pair));
        }

        refresh();
    }

    fn release(&mut self) {
        self.is_ended = true;
        endwin();
    }
}
