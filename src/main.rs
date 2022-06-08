extern crate ncurses;
extern crate queues;
extern crate rand;

use ncurses::*;
use queues::*;
use std::{thread, time};

#[derive(Clone)]
#[derive(Copy)]
#[derive(PartialEq)]
struct Pair {
    x: i32,
    y: i32
}

struct WinSpecs {
    height: i32,
    width: i32,
    start_x: i32,
    start_y: i32
}

static GAME_WINDOW_HEIGHT: i32 = 24;
static GAME_WINDOW_WIDTH: i32 = 56;

fn create_win(specs: WinSpecs, border: bool) -> WINDOW {
  let win = newwin(specs.height, specs.width, specs.start_y, specs.start_x);
  if border { box_(win, 0, 0); }
  wrefresh(win);
  win
}

fn move_snake(win: WINDOW, q: &mut Queue<Pair>, x: &mut i32, y: &mut i32, str: &str) {

    *x = (*x + GAME_WINDOW_HEIGHT) % GAME_WINDOW_HEIGHT;
    *y = (*y + GAME_WINDOW_WIDTH) % GAME_WINDOW_WIDTH;

    let head = Pair {x: *x, y: *y};
    mvwprintw(win, head.x, head.y, str);
    q.add(head).unwrap();

    let tail: Pair = q.remove().unwrap();
    mvwprintw(win, tail.x, tail.y, " ");

    wrefresh(win);
}

fn init_snake(win: WINDOW, len: i32, snake_q: &mut Queue<Pair>, start_x: i32, start_y: &mut i32) {
    for _ in 0..len {
        *start_y += 1;
        snake_q.add(Pair {x: start_x, y: *start_y}).unwrap();
        mvwprintw(win, start_x, *start_y, ">");
    }
    wrefresh(win);
}

fn snake() {

    //ncurses setup
    initscr();
    cbreak();
    keypad(stdscr(), true);
    nodelay(stdscr(), true);
    curs_set(ncurses::CURSOR_VISIBILITY::CURSOR_INVISIBLE);
    refresh();

    //center game window
    let mut max_x = 0;
    let mut max_y = 0;
    getmaxyx(stdscr(), &mut max_y, &mut max_x);
    let start_y = (max_y - GAME_WINDOW_HEIGHT) / 2;
    let start_x = (max_x - GAME_WINDOW_WIDTH) / 2;

    //create game window
    let game_window_specs: WinSpecs = WinSpecs{
        height: GAME_WINDOW_HEIGHT, 
        width: GAME_WINDOW_WIDTH, 
        start_x: start_x, 
        start_y: start_y
    };
    let border_specs: WinSpecs = WinSpecs {
        height: GAME_WINDOW_HEIGHT + 2, 
        width: GAME_WINDOW_WIDTH + 2, 
        start_x: start_x - 1, 
        start_y: start_y - 1
    };
    
    let game_window: WINDOW = create_win(game_window_specs, false);
    let border_window: WINDOW = create_win(border_specs, true);

    //initialize snake
    let cooldown = time::Duration::from_millis(150);            
    let mut dir: i32 = KEY_RIGHT;
    let mut dir_old: i32 = dir;
    let init_len = 5;
    let mut snake_q: Queue<Pair> = queue![];
    let mut snake_x: i32 = 2;
    let mut snake_y: i32 = 2;
    let mut snake_str = ">";

    init_snake(game_window, init_len, &mut snake_q, snake_x, &mut snake_y);

    loop {
        dir = getch();
        if ![KEY_RIGHT, KEY_LEFT, KEY_UP, KEY_DOWN].contains(&dir) { dir = dir_old }
        match dir {
            KEY_RIGHT => { snake_y +=  1; snake_str = ">" }
            KEY_LEFT  => { snake_y += -1; snake_str = "<" }
            KEY_UP    => { snake_x += -1; snake_str = "^" }
            KEY_DOWN  => { snake_x +=  1; snake_str = "v" }
            _ => {}
        }

        move_snake(game_window, &mut snake_q, &mut snake_x, &mut snake_y, snake_str);

        dir_old = dir;
        thread::sleep(cooldown);
    }

    //endwin();
}

fn main() {
    snake();
}

/*

    TODO:
        prettier code:
            make functions
        bugfix:
            idea - have a set with used coords

 */