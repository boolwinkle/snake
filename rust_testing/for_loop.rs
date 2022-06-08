use std::collections::HashSet;

fn main() {

    let mut set: HashSet<i32> = HashSet::new();

    for i in 0..100 {
        set.insert(i);
    }

    let n = 5;
    let mut c = 0;
    for i in set.iter() {
        if c == n {
            println!("{i}");
            break;
        }
        c += 1;
    }
}