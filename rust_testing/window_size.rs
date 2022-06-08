extern crate ncurses;

use ncurses::*;
use std::string::*;

fn main() {
    let msg = "Just a string";		                /* message to be appeared on the screen */
    let mut row: i32 = 0; let mut col: i32 = 0;		                /* to store the number of rows and *
                                                    * the number of colums of the screen */
    initscr();				                        /* start the curses mode */
    getmaxyx(stdscr(),&mut row, &mut col);		                /* get the number of rows and columns */
    mvprintw(row/2 ,(col - msg.len() as i32)/2, msg);
                                                    /* print the message at the center of the screen */
    let msg2 = format!("This screen has {} rows and {} columns\n", row, col);
    mvprintw(row-2, 0, &msg2);
    printw("Try resizing your window(if possible) and then run this program again");
    refresh();
    getch();
    endwin();
}