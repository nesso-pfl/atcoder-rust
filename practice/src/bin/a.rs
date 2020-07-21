#[macro_use] mod input;

fn main() {
    input!{
        n: usize,
        v: [usize; 2],
        s: String,
    };
    println!("{} {}", n + v[0] + v[1], s);
}
