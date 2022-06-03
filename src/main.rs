extern crate ncurses;
extern crate queues;

use ncurses::*;
use queues::*;
use std::{thread, time};
use std::vec::*;

#[derive(Clone)]
#[derive(Copy)]
#[derive(PartialEq)]
struct Pair {
    x: i32,
    y: i32
}

fn snake() {
    initscr();
    cbreak();
    keypad(stdscr(), true);
    nodelay(stdscr(), true);
    //curs_set(visibility: CURSOR_VISIBILITY)

    let mut snake = ">";

    let mut x: i32 = 5; // starting position
    let mut y: i32 = 5;

    let t = time::Duration::from_millis(50); // snake moving cooldown
    
    let mut dir: i32 = KEY_RIGHT;                   // direction vars
    let mut dir_try: i32; let mut dir_old: i32;
    
    let init_len: i32 = 30;

    let mut max_row: i32 = 0; let mut max_col: i32 = 0;     // screen width and height 
    getmaxyx(stdscr(), &mut max_row, &mut max_col);

    let mut field: Vec<i32> = Vec::with_capacity((max_row * max_col) as usize);     // field matrix for 
    unsafe { field.set_len((max_row * max_col) as usize); }                         // colision detection and 
                                                                                    // food placement
    let mut snake_q: Queue<Pair> = queue![];

    for _ in 0..init_len {
        y += 1;
        snake_q.add(Pair {x: x, y: y});
        mvprintw(x % max_row, y % max_col, &snake);

        field[(x * max_col + y) as usize] = 1;
    }

    loop {
        dir_try = getch();
        if dir_try == KEY_LEFT || dir_try == KEY_RIGHT || dir_try == KEY_UP || dir_try == KEY_DOWN {
            dir_old = dir;
            dir = dir_try;

            if dir == KEY_LEFT && dir_old == KEY_RIGHT { dir = KEY_RIGHT; }
            else if dir == KEY_RIGHT && dir_old == KEY_LEFT { dir = KEY_LEFT; }
            else if dir == KEY_DOWN && dir_old == KEY_UP{ dir = KEY_UP; }
            else if dir == KEY_UP && dir_old == KEY_DOWN { dir = KEY_DOWN; }
        }
        if dir == KEY_LEFT {
            y = y - 1;
            snake = "<";
        }
        else if dir == KEY_RIGHT {
            y = y + 1;
            snake = ">"
        }
        else if dir == KEY_DOWN {
            x = x + 1;
            snake = "v";
        }
        else if dir == KEY_UP {
            x = x - 1;
            snake = "^";
        }
        
        let tail: Pair = snake_q.remove().unwrap();
        let head: Pair = Pair{x: x, y: y};
        snake_q.add(head);

        if !(0 <= x && x < max_row) || !(0 <= y && y < max_col) // if out of bounds
        || field[(head.x * max_col + head.y) as usize] == 1 { // colision detection
            break;
        }

        mvprintw(tail.x % max_row, tail.y % max_col, " ");
        mvprintw(head.x % max_row, head.y % max_col, &snake);

        field[(tail.x * max_col + tail.y) as usize] = 0;
        field[(head.x * max_col + head.y) as usize] = 1;

        refresh();
        thread::sleep(t);
    }
}

fn main() {
    snake();
}
