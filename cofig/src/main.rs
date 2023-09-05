
fn main() {
    if let Some(home) = directories::BaseDirs::new() {
        let path = home.home_dir();
        println!("{}", path.display());

        println!("{} {} {} {}", home.config_dir().display(), home.cache_dir().display(), home.config_dir().display(), home.data_dir().display())
    } else {
        println!("Unable to get home directory.");
    }
}

