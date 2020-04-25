use math::round::ceil;
use rand::SeedableRng;
use rand_distr::{Distribution, Normal};
use rand_pcg;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub struct TileOptions {
    pub parent_x: u8,
    pub parent_y: u8,
    pub width: usize,
    pub height: usize,
    pub max_value: f64,
    pub min_value: f64,
    pub mean: f64,
    pub std_dev: f64,
}

impl Hash for TileOptions {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let max_value = ceil(self.max_value, 0) as u64;
        let min_value = ceil(self.min_value, 0) as u64;
        let mean = ceil(self.mean, 0) as u64;
        let std_dev = ceil(self.std_dev, 0) as u64;
        self.parent_x.hash(state);
        self.parent_y.hash(state);
        self.width.hash(state);
        self.height.hash(state);
        max_value.hash(state);
        min_value.hash(state);
        mean.hash(state);
        std_dev.hash(state);
    }
}

pub struct NormalDistTile {
    pub width: usize,
    pub height: usize,
    pub subtiles: Vec<u8>,
}

impl NormalDistTile {
    pub fn hash(opts: &TileOptions) -> u64 {
        let mut hasher = DefaultHasher::new();
        opts.hash(&mut hasher);
        return hasher.finish();
    }

    pub fn new(opts: TileOptions) -> NormalDistTile {
        let subtiles = opts.width * opts.height;
        let seed = NormalDistTile::hash(&opts);
        let mut rng = rand_pcg::Pcg32::seed_from_u64(seed);
        let normal_dist = Normal::new(opts.mean, opts.std_dev).unwrap();
        let subtiles = normal_dist
            .sample_iter(&mut rng)
            .take(subtiles)
            .map(|x| apply_thresholds(x, &opts))
            .collect::<Vec<u8>>();
        return NormalDistTile {
            width: opts.width,
            height: opts.height,
            subtiles: subtiles,
        };
    }

    pub fn get(&self, x: u8, y: u8) -> u8 {
        return self.subtiles[coord_to_index(x, y, self.width)];
    }
}

pub fn apply_thresholds(x: f64, opts: &TileOptions) -> u8 {
    let x = ceil(x, 0);
    if x < opts.min_value {
        return opts.min_value as u8;
    }
    if x > opts.max_value {
        return opts.max_value as u8;
    }
    return x as u8;
}

pub fn coord_to_index(x: u8, y: u8, width: usize) -> usize {
    // The coordinate system assumes top-left is 0,0. For texample, the
    // following array layout has the first coordinate of 0,0 as the value
    // 0:
    //      [  0  1  2  3
    //         4  5  6  7
    //         8  9 10 11
    //        12 13 14 15]
    //
    // and the last coorinate of 3,3 as the value 15.
    let w = width as u8;
    return (x + y * w as u8) as usize;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coord_to_index() {
        // tiny square tile
        assert_eq!(coord_to_index(0, 0, 1), 0);
        // small square tile
        // [ 0  1  2  3
        //   4  5  6  7
        //   8  9 10 11
        //  12 13 14 15]
        assert_eq!(coord_to_index(0, 0, 4), 0);
        assert_eq!(coord_to_index(3, 0, 4), 3);
        assert_eq!(coord_to_index(0, 3, 4), 12);
        assert_eq!(coord_to_index(3, 3, 4), 15);
        // small, wide rectangle
        // [ 0  1  2  3  4  5
        //   6  7  8  9 10 11
        //  12 13 14 15 16 17]
        assert_eq!(coord_to_index(0, 0, 6), 0);
        assert_eq!(coord_to_index(4, 0, 6), 4);
        assert_eq!(coord_to_index(5, 0, 6), 5);
        assert_eq!(coord_to_index(0, 1, 6), 6);
        assert_eq!(coord_to_index(5, 1, 6), 11);
        assert_eq!(coord_to_index(2, 2, 6), 14);
        assert_eq!(coord_to_index(5, 2, 6), 17);
        // small, tall rectangle
        // [ 0  1  2
        //   3  4  5
        //   6  7  8
        //   9 10 11
        //  12 13 14
        //  15 16 17]
        assert_eq!(coord_to_index(0, 0, 3), 0);
        assert_eq!(coord_to_index(2, 0, 3), 2);
        assert_eq!(coord_to_index(1, 1, 3), 4);
        assert_eq!(coord_to_index(2, 1, 3), 5);
        assert_eq!(coord_to_index(1, 3, 3), 10);
        assert_eq!(coord_to_index(2, 4, 3), 14);
        assert_eq!(coord_to_index(2, 5, 3), 17);
    }
}
