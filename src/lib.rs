use std::fs::read_to_string;

pub fn read_input(day: u8) -> std::io::Result<String> {
    read_to_string(format!("inputs/day-{}.txt", day))
}

pub fn opt_exists<T, F>(opt: &Option<T>, predicate: F) -> bool
where
    F: FnOnce(&T) -> bool,
{
    match opt {
        Some(value) => predicate(value),
        None => false,
    }
}
