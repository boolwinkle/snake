extern crate ncurses;
extern crate queues;
extern crate rand;

use ncurses::*;
use queues::*;
use std::{thread, time};
use std::collections::HashSet;
//use rand::Rng;

#[derive(Clone, Copy, PartialEq, Hash, Eq)]
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

static GAME_WINDOW_HEIGHT: i32 = 12;
static GAME_WINDOW_WIDTH: i32 = 28;

fn create_win(specs: WinSpecs, border: bool) -> WINDOW {
  let win = newwin(specs.height, specs.width, specs.start_y, specs.start_x);
  if border { box_(win, 0, 0); }
  wrefresh(win);
  win
}

fn generate_food(free_pos: &mut HashSet<Pair>, c: &mut i32, win: WINDOW) -> Pair {

    //let mut rng = rand::thread_rng();
    //let food_index = rng.gen::<i32>() % (GAME_WINDOW_HEIGHT * GAME_WINDOW_WIDTH);
    let mut food: Pair = Pair{x: 1, y: 1};

    let mut n = 0;
    for pos in free_pos.iter() {
        if n == *c {
            food = *pos;
        } n += 1;
    } *c = (*c + 1) % (GAME_WINDOW_HEIGHT * GAME_WINDOW_WIDTH) ;
    mvwprintw(win, food.x, food.y, "$");

    food
}

fn check_opposite_dir(dir: &mut i32, dir_old: i32){
    if *dir == KEY_LEFT && dir_old == KEY_RIGHT ||
       *dir == KEY_RIGHT && dir_old == KEY_LEFT ||
       *dir == KEY_DOWN && dir_old == KEY_UP    ||
       *dir == KEY_UP && dir_old == KEY_DOWN {
           *dir = dir_old;
       }
}

fn fill_free_pos(set: &mut HashSet<Pair>) {
    for x in 0..GAME_WINDOW_HEIGHT {
        for y in 0..GAME_WINDOW_WIDTH {
            let p: Pair = Pair{x: x, y: y};
            set.insert(p);
        }
    }
}

fn move_snake(win: WINDOW, q: &mut Queue<Pair>, head: &mut Pair, str: &str, food_counter: &mut i32,
    free_pos: &mut HashSet<Pair>, food: &mut Pair, len: &mut i32, t: &mut std::time::Duration) {

        head.x = (head.x + GAME_WINDOW_HEIGHT) % GAME_WINDOW_HEIGHT;
        head.y = (head.y + GAME_WINDOW_WIDTH) % GAME_WINDOW_WIDTH;

        let head = Pair {x: head.x, y: head.y};
        mvwprintw(win, head.x, head.y, str);
        q.add(head).unwrap();
        free_pos.remove(&head);

        if head != *food {
            let tail: Pair = q.remove().unwrap();
            free_pos.insert(tail);
            mvwprintw(win, tail.x, tail.y, " ");
        }
        else {
            *food = generate_food(free_pos, food_counter, win);
            *len += 1;
            *t = *t * 99 / 100;
            mvwprintw(stdscr(), 0, 0, &format!("length: {}", len));
        }

        wrefresh(win);
    }

fn init_snake(win: WINDOW, len: i32, q: &mut Queue<Pair>, 
    start_x: i32, start_y: &mut i32, set: &mut HashSet<Pair>) {
        for _ in 0..len {
            *start_y += 1;
            let p: Pair = Pair {x: start_x, y: *start_y};
            q.add(p).unwrap();
            set.remove(&p);
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
    let _border_window: WINDOW = create_win(border_specs, true);

    //initialize snake
    let mut cooldown = time::Duration::from_millis(150);            
    let mut dir: i32 = KEY_RIGHT;
    let mut dir_old: i32 = dir;
    let mut init_len = 2;
    let mut snake_q: Queue<Pair> = queue![];
    let mut snake_x: i32 = 2;
    let mut snake_y: i32 = 2;
    let mut snake_str = ">";

    //free positions set
    let mut free_pos: HashSet<Pair> = HashSet::new();
    fill_free_pos(&mut free_pos);

    init_snake(game_window, init_len, &mut snake_q, snake_x, &mut snake_y, &mut free_pos);
    mvwprintw(stdscr(), 0, 0, &format!("length: {}", init_len));

    //generate food
    let mut food_counter = 0;
    let mut food: Pair = generate_food(&mut free_pos, &mut food_counter, game_window);

    loop {
        dir = getch();
        if [KEY_RIGHT, KEY_LEFT, KEY_UP, KEY_DOWN].contains(&dir)
            { check_opposite_dir(&mut dir, dir_old) }
        else 
            { dir = dir_old; }
        match dir {
            KEY_RIGHT => { snake_y +=  1; snake_str = ">" }
            KEY_LEFT  => { snake_y += -1; snake_str = "<" }
            KEY_UP    => { snake_x += -1; snake_str = "^" }
            KEY_DOWN  => { snake_x +=  1; snake_str = "v" }
            _ => {}
        }

        let snake_head: Pair = Pair{x: snake_x, y: snake_y};
        if !free_pos.contains(&snake_head) && snake_head != food {
            break;
        }

        let mut snake_head: Pair = Pair{x: snake_x, y: snake_y};

        move_snake(game_window, &mut snake_q, &mut snake_head,
            snake_str, &mut food_counter, &mut free_pos, &mut food, &mut init_len, &mut cooldown);

        dir_old = dir;
        thread::sleep(cooldown);
    }

    nodelay(stdscr(), true);
    getch();
    endwin();
}

fn main() {
    snake();
}