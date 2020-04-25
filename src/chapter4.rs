#[derive(Debug)]
pub struct Optional<T> {
    value: Option<T>,
    is_valid: bool,
}

fn id<T>(o: T) -> Optional<T> {
    Optional {
        value: Some(o),
        is_valid: true,
    }
}

pub fn compose<F, G, A, B, C>(f: F, g: G) -> impl Fn(A) -> Optional<C> + Sized
where
    F: Fn(A) -> Optional<B> + Sized,
    G: Fn(B) -> Optional<C> + Sized,
{
    return Box::new(move |a: A| {
        let v1 = f(a);
        if v1.is_valid {
            g(v1.value.unwrap())
        } else {
            Optional {
                value: None,
                is_valid: false,
            }
        }
    });
}

fn safe_sqrt(n: f64) -> Optional<f64> {
    if n <= 0.0 {
        Optional {
            value: None,
            is_valid: false,
        }
    } else {
        Optional {
            value: Some(n.sqrt()),
            is_valid: true,
        }
    }
}

fn safe_reciprocal(n: f64) -> Optional<f64> {
    if n == 0.0 {
        Optional {
            value: None,
            is_valid: false,
        }
    } else {
        Optional {
            value: Some(1.0 / n),
            is_valid: true,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_safe_root_reciprocal_happy() {
        let safe_root_reciprocal = compose(compose(safe_sqrt, id), compose(id, safe_reciprocal));
        let result = safe_root_reciprocal(3.0);
        assert!(result.is_valid);
        assert_eq!((1.0_f64 / 3.0_f64.sqrt()), result.value.unwrap())
    }

    #[test]
    fn test_safe_root_reciprocal_unhappy() {
        let safe_root_reciprocal = compose(safe_sqrt, safe_reciprocal);
        assert_none(safe_root_reciprocal(-2.0));
        assert_none(safe_root_reciprocal(0.0));
    }

    fn assert_none(o: Optional<f64>) {
        assert!(!o.is_valid);
        assert!(o.value.is_none());
    }
}
