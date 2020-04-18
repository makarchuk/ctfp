use std::collections::HashMap;
use std::hash::Hash;

struct Memoize<F, A, B>
where
    F: Fn(A) -> B,
    A: Eq+Hash+Clone,
    B: Clone,
{
    f: F,
    cache: HashMap<A, B>,
    pub calls_count: u32,
}

impl<F, A, B> Memoize<F, A, B>
where
    F: Fn(A) -> B,
    A: Eq+Hash+Clone,
    B: Clone,
{
    pub fn new(f: F) -> Self {
        return Self {
            f,
            cache: HashMap::new(),
            calls_count: 0,
        }
    }
}

impl<F, A, B> FnMut<(A,)> for Memoize<F, A, B>
where
    F: Fn(A) -> B,
    A: Eq+Hash+Clone,
    B: Clone,
{
    extern "rust-call" fn call_mut(&mut self, args: (A,)) -> Self::Output {
        if let Some(v) = self.cache.get(&args.0) {
            v.clone()
        } else {
            self.calls_count += 1;
            let value = (self.f)(args.0.clone());
            self.cache.insert(args.0, value.clone());
            value
        }
    }
}

impl<F, A, B> FnOnce<(A,)> for Memoize<F, A, B>
    where
        F: Fn(A) -> B,
        A: Eq+Hash+Clone,
        B: Clone,
{
    type Output = B;

    extern "rust-call" fn call_once(self, args: (A,)) -> Self::Output {
        (self.f)(args.clone().0)
    }
}
#[cfg(test)]
mod test {
    use super::*;
    # [test]
    fn test_memoize(){
        fn length(s: String) -> usize {
            return s.len()
        }
        let mut memoize = Memoize::new(length);
        assert_eq!(memoize("aaaaa".to_owned()), 5);
        assert_eq!(memoize.calls_count, 1);
        assert_eq!(memoize("aaaaa".to_owned()), 5);
        assert_eq!(memoize.calls_count, 1);
        assert_eq!(memoize("aaaa".to_owned()), 4);
        assert_eq!(memoize.calls_count, 2);
    }
}