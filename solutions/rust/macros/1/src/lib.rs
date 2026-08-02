#[macro_export]
macro_rules! hashmap {
    () => {::std::collections::HashMap::new()};
    (,) => {aosdijgpowehg};
    ($($key:expr => $val:expr),* $(,)?) => {
        {
            let mut _map = ::std::collections::HashMap::new();
            $(
                _map.insert($key, $val);
            )*
            _map
        }
    };
}

/// This module contains doctests, which allows writing tests where a code
/// snippet is supposed to fail to compile. These tests also have "ignore"
/// attributes, make sure to remove them when solving this exercise locally.
pub mod compile_fail_tests;
