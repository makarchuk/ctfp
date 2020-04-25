use super::chapter1;
use std::marker::PhantomData;

struct Reader<F, R, A>
where
    // F: impl Fn(R) -> A
    F: Fn(R) -> A + Sized,
{
    phantom: std::marker::PhantomData<(A, R)>,
    f: F,
}

impl<F, R, A> Reader<F, R, A>
where
    F: Fn(R) -> A + Sized + 'static,
    A: 'static,
    R: 'static,
{
    fn map<G, B>(self, mapper: G) -> Reader<Box<dyn Fn(R)->B>, R, B>
    where
        G: Fn(A) -> B + Sized + 'static,
        B: 'static,
    {
        return Reader {
            phantom: PhantomData,
            f: Box::new(move |r: R| mapper((self.f)(r))),
        };
    }

    //I don't feel like implementing `Fn`
    fn get(self, r: R) -> A {
        return (self.f)(r);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_composition() {
        // strlen_reader :: String -> usize
        let strlen_reader = Reader {
            phantom: PhantomData,
            f: Box::new(|x: String| x.len()),
        };

        let composition = chapter1::compose(
            Box::new(|x: usize| x * 2),
            Box::new(|x: usize| x + 1),
        );
        assert_eq!(
            composition("Hello".len()),
            strlen_reader
                .map(Box::new(composition))
                .get("Hello".to_owned()),
        )
    }

    #[test]
    fn test_identity() {
        let strlen_reader = Reader {
            phantom: PhantomData,
            f: Box::new(|x: String| x.len()),
        };
        assert_eq!(
            strlen_reader.map(chapter1::id).get("test".to_owned()),
            chapter1::id("test".len()),
        );
    }

}
