#[macro_export]
macro_rules! hashmap {
    (,) => {
        compile_error!("The hashmap macro cannot be invoked with a single comma. Please provide key-value pairs or remove the comma if no pairs are intended.");
    };
    ($($x:expr => $y:expr),* $(,)?) => {
        {
            let mut temp_hashmap = ::std::collections::HashMap::new();
            $(
                temp_hashmap.insert($x, $y);
            )*
            temp_hashmap
        }
    };
}
