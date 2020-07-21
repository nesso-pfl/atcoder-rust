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

fn plan(before: Vec<i64>, after: Vec<i64>) -> Option<Vec<i64>> {
    let dtime = after[0] - before[0];
    let distance = (after[1] - before[1]).abs() + (after[2] - before[2]).abs();

    if dtime >= distance && (dtime - distance) % 2 == 0 { Some(after) }
    else { None }
}

fn main() {
    input!{
        n: u32,
        d: [[i64; 3]; n]
    }
    let result = d.into_iter()
        .fold(Some(vec![0, 0, 0]), |acc, x| acc.and_then(|a| plan(a, x)));
    
    match result {
        Some(_) => println!("{}", "Yes"),
        None => println!("{}", "No"),
    }
}