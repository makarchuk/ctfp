struct Pair<A, B> {
    first: A,
    second: B,
}

impl<A, B> Pair<A, B> {
    fn bimap<F, G, C, D>(f: F, g: G) -> impl Fn(Pair<A, B>) -> Pair<C, D>
    where
        F: Fn(A) -> C + Sized + 'static,
        G: Fn(B) -> D + Sized + 'static,
    {
        Box::new(move |p: Pair<A, B>| Pair {
            first: f(p.first),
            second: g(p.second),
        })
    }

    fn first<F, C>(f: F) -> impl Fn(Pair<A, B>) -> Pair<C, B>
    where
        F: Fn(A) -> C + Sized + 'static,
    {
        Box::new(move |p: Pair<A, B>| Pair {
            first: f(p.first),
            second: p.second,
        })
    }

    fn second<G, D>(g: G) -> impl Fn(Pair<A, B>) -> Pair<A, D>
    where
        G: Fn(B) -> D + Sized + 'static,
    {
        Box::new(move |p: Pair<A, B>| Pair {
            first: p.first,
            second: g(p.second),
        })
    }
}

#[derive(Eq, PartialEq, Debug)]
enum PreList<A, B> {
    None(),
    Cons(A, B),
}

impl<A, B> PreList<A, B> {
    fn bimap<F, G, C, D>(f: F, g: G) -> impl Fn(PreList<A, B>) -> PreList<C, D>
    where
        F: Fn(A) -> C + Sized + 'static,
        G: Fn(B) -> D + Sized + 'static,
    {
        Box::new(move |p: PreList<A, B>| match p {
            PreList::None() => PreList::None(),
            PreList::Cons(a, b) => PreList::Cons(f(a), g(b)),
        })
    }

    fn first<F, C>(f: F) -> impl Fn(PreList<A, B>) -> PreList<C, B>
    where
        F: Fn(A) -> C + Sized + 'static,
    {
        Box::new(move |p: PreList<A, B>| {
            match p {
                PreList::None() => PreList::None(),
                PreList::Cons(a, b) => PreList::Cons(f(a), b)
            }
        })
    }

    fn second<G, D>(g: G) -> impl Fn(PreList<A, B>) -> PreList<A, D>
    where
        G: Fn(B) -> D + Sized + 'static,
    {
        Box::new(move |p: PreList<A, B>| {
            match p {
                PreList::None() => PreList::None(),
                PreList::Cons(a, b) => PreList::Cons(a, g(b))
            }
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn pair_bimap() {
        let p: Pair<_, _> = Pair {
            first: 1,
            second: "test".to_owned(),
        };
        let lifted = Pair::bimap(
            Box::new(|a: i32| 2.0 * a as f64),
            Box::new(|b: String| b.len()),
        );
        let processed = lifted(p);
        assert_eq!(processed.first, 2.0);
        assert_eq!(processed.second, 4)
    }

    #[test]
    fn pair_first() {
        let p: Pair<_, _> = Pair {
            first: 1,
            second: "test".to_owned(),
        };
        let lifted = Pair::first(Box::new(|a: i32| 2.0 * a as f64));
        let processed = lifted(p);
        assert_eq!(processed.first, 2.0);
        assert_eq!(processed.second, "test".to_owned())
    }

    #[test]
    fn pair_second() {
        let p: Pair<_, _> = Pair {
            first: 1,
            second: "test".to_owned(),
        };
        let lifted = Pair::second(Box::new(|s: String| s.repeat(2)));
        let processed = lifted(p);
        assert_eq!(processed.first, 1);
        assert_eq!(processed.second, "testtest".to_owned())
    }

    #[test]
    fn pre_list() {
        let lifted = PreList::bimap(
            Box::new(|x: f64| x.round() as i32),
            Box::new(|x: usize| "a".to_owned().repeat(x))
        );
        assert_eq!(lifted(PreList::None()), PreList::None());
        assert_eq!(lifted(PreList::Cons(2.1, 2_usize)), PreList::Cons(2, "aa".to_owned()));

        let lifted = PreList::first(
            Box::new(|x: f64| x.round() as i32),
        );
        assert_eq!(lifted(PreList::None()), PreList::None());
        assert_eq!(lifted(PreList::Cons(2.1, 2)), PreList::Cons(2, 2));

        let lifted = PreList::second(
            Box::new(|x: usize| "a".to_owned().repeat(x))
        );
        assert_eq!(lifted(PreList::None()), PreList::None());
        assert_eq!(lifted(PreList::Cons(2.1, 7)), PreList::Cons(2.1, "aaaaaaa".to_owned()));
    }
}
