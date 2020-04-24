use imgdata::{Manager, ManagerOptions};

const RESOURCE_PREFIX: &'static str = "resources/planets";
const COLOR_FILE: &'static str = "colors.ron";

pub fn read(planet: &str, category: &str, size: &str) -> Manager {
    let size = parse_size(size);
    let opts = ManagerOptions {
        image_path: image_path(planet, category, size),
        color_file_path: color_file_path(planet, category),
    };
    return Manager::new(opts);
}

pub fn image_path(planet: &str, category: &str, size: &str) -> String {
    format!(
        "{}/{}/{}/map{}.png",
        RESOURCE_PREFIX, planet, category, size,
    )
}

pub fn color_file_path(planet: &str, category: &str) -> String {
    format!("{}/{}/{}/{}", RESOURCE_PREFIX, planet, category, COLOR_FILE,)
}

pub fn parse_size(size: &str) -> &str {
    match size {
        "large" => "",
        "tiny" => "-tiny",
        _ => panic!("Unsupported size!"),
    }
}
