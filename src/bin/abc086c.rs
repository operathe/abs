use proconio::input;

fn can_travel(t: i32, x: i32, y: i32, prev_t: i32, prev_x: i32, prev_y: i32) -> bool {
    let distance = (x - prev_x).abs() + (y - prev_y).abs();
    let time_diff = t - prev_t;
    distance <= time_diff && (distance % 2 == time_diff % 2)
}

fn main() {
    input! {
        n: usize,
        travels: [(i32, i32, i32); n],
    }

    let mut prev_x = 0;
    let mut prev_y = 0;
    let mut prev_t = 0;

    for (t, x, y) in travels {
        if !can_travel(t, x, y, prev_t, prev_x, prev_y) {
            println!("No");
            return;
        }
        prev_t = t;
        prev_x = x;
        prev_y = y;
    }

    println!("Yes");
}
