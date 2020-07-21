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

fn parse(s: &[char]) {
    match s {
        p if p.is_empty() => println!("{}", "YES"),
        p if p.starts_with(&['e', 'r', 'a', 's', 'e', 'r']) => parse(&p[6..]),
        p if p.starts_with(&['e', 'r', 'a', 's', 'e', 'd']) => parse(&p[5..]),
        p if p.starts_with(&['e', 'r', 'a', 's', 'e']) => parse(&p[5..]),
        p if p.starts_with(&['d', 'r', 'e', 'a', 'm', 'e', 'r', 'a']) => parse(&p[5..]),
        p if p.starts_with(&['d', 'r', 'e', 'a', 'm', 'e', 'r']) => parse(&p[7..]),
        p if p.starts_with(&['d', 'r', 'e', 'a', 'm', 'd']) => parse(&p[5..]),
        p if p.starts_with(&['d', 'r', 'e', 'a', 'm']) => parse(&p[5..]),
        _ => println!("{}", "NO"),
    }
}

fn main() {
    input!{
        s: chars
    }
    parse(&s[..]);
}
