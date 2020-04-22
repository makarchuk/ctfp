pub fn id<T>(x: T) -> T {
    x
}

pub fn compose<F, G, A, B, C>(f: F, g: G) -> impl Fn(A) -> C + Sized
where
    F: Fn(A) -> B + Sized,
    G: Fn(B) -> C + Sized,
{
    return Box::new(move |a: A| g(f(a)));
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_compose_respects_identity() {
        let to_str = |a: i32| -> String { format!("{}", a) };
        assert_eq!(compose(Box::new(to_str), Box::new(id))(1111), to_str(1111));

        assert_eq!(compose(Box::new(id), Box::new(to_str),)(2222), to_str(2222));
    }

    #[test]
    fn test_nested_compose() {
        let to_str = |a: i32| -> String { format!("{}", a) };
        let len = |x: String| -> usize { x.len() };

        assert_eq!(
            compose(compose(Box::new(to_str), Box::new(len)), Box::new(id))(1111),
            4
        );
    }
}
