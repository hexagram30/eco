use hxgm30eco::tile::{NormalDistTile, TileOptions};

pub fn main() {
    let opts = TileOptions {
        parent_x: 108,
        parent_y: 54,
        width: 10,
        height: 10,
        max_value: 10.0,
        min_value: 0.0,
        mean: 5.0,
        std_dev: 2.0,
    };
    let t = NormalDistTile::new(opts);
    println!(" {:?}", t.subtiles);
    println!("{}", t.get(0, 0));
    println!("{}", t.get(1, 0));
    println!("{}", t.get(2, 0));
    println!("{}", t.get(3, 0));

    println!("{}", t.get(0, 5));
    println!("{}", t.get(1, 5));
    println!("{}", t.get(2, 5));
    println!("{}", t.get(3, 5));

    println!("{}", t.get((t.width - 4) as u8, (t.height - 1) as u8));
    println!("{}", t.get((t.width - 3) as u8, (t.height - 1) as u8));
    println!("{}", t.get((t.width - 2) as u8, (t.height - 1) as u8));
    println!("{}", t.get((t.width - 1) as u8, (t.height - 1) as u8));
}

//  0  1  2  3  4  5  6  7  8  9
// 10 11 12 13 14 15 16 17 18 19
// 20 21 22 23 24 25 26 27 28 29
// ...
// 90 91 92 93 94 95 96 97 98 99
