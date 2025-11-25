#[macro_export]
macro_rules! hashmap {
    () => {
        {
            ::std::collections::HashMap::new()
        }
    };
    
    ( $($key:literal => $val:expr),+ $(,)? ) => {
        {
            ::std::collections::HashMap::from([
                $(
                    ($key, $val),
                )*
            ])
        }
    };
}