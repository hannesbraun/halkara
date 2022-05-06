#[macro_export]
macro_rules! unwrap_or_return {
    ( $e:expr ) => {
        match $e {
            Some(x) => x,
            None => return,
        }
    };
}

/// Linear congruential generator
struct Lcg {
    m: u32,
    state: u32,
}

impl Lcg {
    fn new() -> Lcg {
        let m = 2147483647;
        Lcg {
            m,
            state: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map_or_else(|_| 69696969, |duration| duration.as_secs() as u32)
                % m,
        }
    }

    /// Int in range (max_val is exclusive)
    fn rand_int_range(&mut self, max_val: u32) -> u32 {
        let a = 48271;
        let c = 0;

        self.state = (a * self.state + c) % self.m;
        ((self.state as f64 / self.m as f64) * max_val as f64) as u32
    }
}

/// Knuth shuffle
pub fn shuffle<T>(vec: &mut Vec<T>) {
    let n = vec.len();
    let mut lcg = Lcg::new();
    for i in 0..n - 1 {
        let j = i + lcg.rand_int_range(n as u32) as usize * (n - i) / n;
        vec.swap(i, j);
    }
}

pub fn shuffle_n<T>(vec: &mut Vec<T>, n: usize) {
    for _ in 0..n {
        shuffle(vec);
    }
}
