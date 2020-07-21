macro_rules! input {
    (source = $s:expr, $($r:tt)*) => {
        let mut iter = $s.split_whitespace();
        input_inner!{iter, $($r)*}
    };
    ($($r:tt)*) => {
        let s = {
            use std::io::Read;
            let mut s = String::new();
            std::io::stdin().read_to_string(&mut s).unwrap();
            s
        };
        let mut iter = s.split_whitespace();
        input_inner!{iter, $($r)*}
    };
}

macro_rules! input_inner {
    ($iter:expr) => {};
    ($iter:expr, ) => {};

    ($iter:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($iter, $t);
        input_inner!{$iter $($r)*}
    };
}

macro_rules! read_value {
    ($iter:expr, ( $($t:tt),* )) => {
        ( $(read_value!($iter, $t)),* )
    };

    ($iter:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($iter, $t)).collect::<Vec<_>>()
    };

    ($iter:expr, chars) => {
        read_value!($iter, String).chars().collect::<Vec<char>>()
    };

    ($iter:expr, usize1) => {
        read_value!($iter, usize) - 1
    };

    ($iter:expr, $t:ty) => {
        $iter.next().unwrap().parse::<$t>().expect("Parse error")
    };
}

fn search(n: u32, target: u32) -> Option<(u32, u32, u32)> {
    (0..n+1).find_map(|x| {
        let cy = (0..n+1-x).find(|y| 10000 * x + 5000 * y + 1000 * (n - x - y) == target);
        cy.and_then(|y_| Some((x, y_, n - x - y_)))
    })
}

fn main() {
    input!{
        s: [u32; 2],
    }
    let n = s[0];
    let y = s[1];
    let result = search(n, y);
    match result {
        Some((a, b, c)) => println!("{} {} {}", a, b, c),
        None => println!("-1 -1 -1"),
    }
}
