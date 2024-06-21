use std::{env, fmt::Debug, str::FromStr};

pub fn get_env<T: FromStr>(key: &str, default: Option<T>) -> T
where
    T::Err: Debug,
{
    match env::var(key) {
        Ok(value) => value.parse::<T>().unwrap_or_else(|e| {
            eprintln!("Cannot parse {}: {:?}", key, e);
            panic!("Invalid format for {}", key)
        }),
        Err(_) => {
            default.unwrap_or_else(|| panic!("{} must be set in .env file or have a default", key))
        }
    }
}
