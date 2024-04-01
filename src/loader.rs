use crate::movie::{Movie, LoadMode};

pub trait MovieLoader {
    fn load(path: &str, load_mode: LoadMode) -> Movie;
}