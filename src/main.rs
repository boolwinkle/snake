extern crate ncurses;
extern crate queues;
extern crate rand;

use ncurses::*;
use queues::*;
use std::{thread, time};
use std::vec::*;
use rand::*;

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
    curs_set(ncurses::CURSOR_VISIBILITY::CURSOR_INVISIBLE);

    let lines: i32 = 10;
    let cols: i32 = 10;

    //let game_window: WINDOW = newwin(lines, cols, 5, 5);
    wborder(stdscr(), '|' as u32, '|' as u32, '-'  as u32, '-' as u32, '+' as u32, '+' as u32, '+' as u32, '+' as u32);

    let mut snake = ">";

    let mut x: i32 = 5; // starting position
    let mut y: i32 = 5;
    
    let mut dur: u64 = 150;
    let mut t = time::Duration::from_millis(dur); // snake moving cooldown
    
    let mut dir: i32 = KEY_RIGHT;                   // direction vars
    let mut dir_try: i32; let mut dir_old: i32;
    
    let init_len: i32 = 3;

    let mut max_row: i32 = lines; let mut max_col: i32 = cols;     // screen width and height 
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

    // generate food
    let food_coords: i32 = (rand::random::<u32>() % (max_row * max_col) as u32) as i32;
    let mut food: Pair = Pair{x: food_coords / max_col, y: food_coords % max_col};
    while field[food_coords as usize] == 1 ||
        ((1 <= food.x && food.x < max_row - 1) && (1 <= food.y && food.y < max_col - 1)) {
                let food_coords: i32 = (rand::random::<u32>() % (max_row * max_col) as u32) as i32;
                food = Pair{x: food_coords / max_col, y: food_coords % max_col};
            }

    let food: Pair = Pair{x: food_coords / max_col, y: food_coords % max_col};
    mvprintw(food.x, food.y, "$");
    field[(food.x * max_col + food.y) as usize] = 2;

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
        
        let head: Pair = Pair{x: x, y: y};
        snake_q.add(head);

        if !(1 <= x && x < max_row - 1) || !(1 <= y && y < max_col - 1) // if out of bounds
        || field[(head.x * max_col + head.y) as usize] == 1 {   // colision detection
            break;
        }

        else if field[(head.x * max_col + head.y) as usize] == 2 {
            // gen new food
            let food_coords: i32 = (rand::random::<u32>() % (max_row * max_col) as u32) as i32;
            let mut food: Pair = Pair{x: food_coords / max_col, y: food_coords % max_col};
            while field[food_coords as usize] == 1 ||
            (!(1 <= food.x && food.x < max_row - 1) || !(1 <= food.y && food.y < max_col - 1)) {
                let food_coords: i32 = (rand::random::<u32>() % (max_row * max_col) as u32) as i32;
                food = Pair{x: food_coords / max_col, y: food_coords % max_col};
            }
            mvprintw(food.x, food.y, "$");
            field[(food.x * max_col + food.y) as usize] = 2;
            dur = dur * 49 / 50;
            t = time::Duration::from_millis(dur);
        }
        
        else {
            let tail: Pair = snake_q.remove().unwrap();
            mvprintw(tail.x % max_row, tail.y % max_col, " ");
            field[(tail.x * max_col + tail.y) as usize] = 0;
        }

        mvprintw(head.x % max_row, head.y % max_col, &snake);
        field[(head.x * max_col + head.y) as usize] = 1;

        refresh();
        thread::sleep(t);
    }

    nodelay(stdscr(), false);
    getch();
    endwin();
}

fn main() {
    snake();
}
