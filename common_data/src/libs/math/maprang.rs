use num_traits::NumCast;

pub fn map_range<X, Y>(val: X, in_min: X, in_max: X, out_min: Y, out_max: Y) -> Y
where
    X: NumCast,
    Y: NumCast,
{
    macro_rules! unwrap_or_panic {
        ($result:expr, $arg:expr) => {
            $result.unwrap_or_else(|| panic!("[map_range] failed to cast {} arg to `f64`", $arg))
        };
    }

    let val_f: f64 = unwrap_or_panic!(NumCast::from(val), "first");
    let in_min_f: f64 = unwrap_or_panic!(NumCast::from(in_min), "second");
    let in_max_f: f64 = unwrap_or_panic!(NumCast::from(in_max), "third");
    let out_min_f: f64 = unwrap_or_panic!(NumCast::from(out_min), "fourth");
    let out_max_f: f64 = unwrap_or_panic!(NumCast::from(out_max), "fifth");

    NumCast::from((val_f - in_min_f) / (in_max_f - in_min_f) * (out_max_f - out_min_f) + out_min_f)
        .unwrap_or_else(|| panic!("[map_range] failed to cast result to target type"))
}
