#![feature(test)]
#[cfg(test)]
extern crate test;

pub fn fib_fast(num: u64) -> u64 {
    if num == 0 {
        return 0;
    }
    let mut tmp1 = 1;
    let mut tmp2 = 0;

    for _i in 0..num {
        let temp = tmp1 + tmp2;
        tmp2 = tmp1;
        tmp1 = temp;
    }

    tmp1
}

pub fn fib_slow(num: u64) -> u64 {
    if num == 0 {
        return 0;
    } else if num == 1 {
        return 1;
    }

    fib_slow(num - 1) + fib_slow(num - 2)
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;
    #[test]
    fn test_correct_first_10() {
        assert_eq!(fib_slow(0), 0);
        assert_eq!(fib_fast(0), 0);
    }

    #[bench]
    fn bench_slow(b: &mut Bencher) {
        b.iter(|| {
            let n = test::black_box(20);
            let mut num = 0;
            for i in 0..n {
                num = fib_slow(i);
            }
            num
        })
    }

    #[bench]
    fn bench_fast(b: &mut Bencher) {
        b.iter(|| {
            let n = test::black_box(20);
            let mut num = 0;
            for i in 0..n {
                num = fib_fast(i);
            }
            num
        })
    }
}
