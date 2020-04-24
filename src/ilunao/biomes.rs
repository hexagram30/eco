use crate::io;
use imgdata::Manager;

pub fn biomes() -> Manager {
    io::read("ilunao", "biomes", "large")
}

pub fn biomes_tiny() -> Manager {
    io::read("ilunao", "biomes", "tiny")
}
