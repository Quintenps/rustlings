// clippy2.rs
// Make me compile! Execute `rustlings hint clippy2` for hints :)

fn main() {
    let mut res = 42;
    let option: Option<i32> = None;
    if let Some(x) = option {
        res += x;
    }
    println!("{}", res);
}
