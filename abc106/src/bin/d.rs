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

fn main() {
    input!{
        n: usize,
        m: usize,
        q: usize,
        lr: [[usize; 2]; m],
        pq: [[usize; 2]; q],
    }
    let mut xy: [[u16; 509]; 509] = [[0; 509]; 509];
    let mut cc: [[u16; 509]; 509] = [[0; 509]; 509];
    lr.into_iter().for_each(|lr_| xy[lr_[0]][lr_[1]] += 1);
    (1..n+1).for_each(|i| (1..n+1).for_each(|j| {
        cc[i][j] = cc[i-1][j] + cc[i][j-1] - cc[i-1][j-1] + xy[i][j];
    }));

    pq.iter().for_each(|pq_| {
        let ans = cc[pq_[1]][pq_[1]] - cc[pq_[0] - 1][pq_[1]] - cc[pq_[1]][pq_[0] - 1] + cc[pq_[0] - 1][pq_[0] - 1];
        println!("{}", ans);
    })
}
