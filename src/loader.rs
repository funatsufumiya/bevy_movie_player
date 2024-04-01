use crate::movie::{LoadMode, Player};

pub trait MovieLoader {
    fn load(path: &str, load_mode: LoadMode) -> impl Player;
}