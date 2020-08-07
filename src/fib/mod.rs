use std::collections::HashMap;

/// `0   1   2   3   4   5   6   7   8   9`
/// `1   1   2   3   5   8   13  21  34  55`

type FibNum = u128;
type FibMap = Vec<FibNum>;

pub fn fib(idx: FibNum) -> FibNum {
    match idx {
        0..=1 => 1,
        _ => fib(idx - 2) + fib(idx - 1),
    }
}

pub fn fib1(idx: FibNum) -> FibNum {
    let mut map: FibMap = vec![0; 1024];
    fib1_internal(idx, &mut map)
}

pub fn fib2(idx: FibNum) -> FibNum {
    let mut prev: FibNum = 0;
    let mut last: FibNum = 1;
    let mut sum: FibNum = 0;

    match idx {
        0 => 1,
        _ => {
            for _ in 0..idx {
                sum = prev + last;
                prev = last;
                last = sum;
            }

            sum
        }
    }
}

fn fib1_internal(idx: FibNum, map: &mut FibMap) -> FibNum {
    match idx {
        0..=1 => {
            map.insert(idx as usize, 1);
            1
        }
        _ => {
            let mut prev: FibNum = 0;
            let mut last: FibNum = 0;

            if let Some(v) = map.get((idx - 2) as usize) {
                if *v != 0 {
                    prev = *v;
                } else {
                    prev = fib1_internal(idx - 2, map);
                }
            }

            if let Some(v) = map.get((idx - 1) as usize) {
                if *v != 0 {
                    last = *v;
                } else {
                    last = fib1_internal(idx - 1, map);
                }
            }

            let sum = prev + last;
            map.insert(idx as usize, sum);
            sum
        }
    }
}
