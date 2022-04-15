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
    m: usize,
    state: usize,
}

impl Lcg {
    fn new(m: usize) -> Lcg {
        Lcg {
            m,
            state: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map_or_else(|_| 69696969, |duration| duration.as_secs() as usize)
                % m,
        }
    }

    fn rand_int(&mut self) -> usize {
        let a = std::cmp::min(2 + self.m / 5, self.m - 1);
        let c = std::cmp::min(self.m / 14, self.m - 1);

        self.state = (a * self.state + c) % self.m;
        self.state
    }
}

/// Knuth shuffle
pub fn shuffle<T>(vec: &mut Vec<T>) {
    let n = vec.len();
    let mut lcg = Lcg::new(n);
    for i in 0..n - 1 {
        let j = i + lcg.rand_int() * (n - i) / n;
        vec.swap(i, j);
    }
}
