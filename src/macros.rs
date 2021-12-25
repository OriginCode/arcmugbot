#[macro_export]
macro_rules! vecdeque {
    (@single $($x:tt)*) => (());
    (@count $($rest:expr),*) => (<[()]>::len(&[$(vecdeque!(@single $rest)),*]));

    ($($value:expr,)+) => { vecdeque!($($value),+) };
    ($($value:expr),*) => {
        {
            let _cap = vecdeque!(@count $($value),*);
            let mut _map = ::std::collections::VecDeque::with_capacity(_cap);
            $(
                _map.push_back($value);
            )*
            _map
        }
    };
    ($value:expr;$count:expr) => {
        {
            let c = $count;
            let mut _map = ::std::collections::VecDeque::with_capacity(c);
            for _ in 0..c {
                _map.push_back($value);
            }
            _map
        }
    };
}
