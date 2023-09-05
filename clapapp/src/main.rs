use clap::{App, Arg};

fn main() {
    // 创建一个应用程序对象
    let app = App::new("My Rust CLI App")
        .version("1.0")
        .author("Your Name")
        .about("A simple Rust command-line application");

    // 添加命令行参数
    let matches = app
        .arg(Arg::with_name("input")
            .short("i")
            .long("input")
            .value_name("FILE")
            .help("Input file")
            .required(true) // 参数是必需的
            .takes_value(true)) // 参数需要一个值
        .arg(Arg::with_name("output")
            .short("o")
            .long("output")
            .value_name("FILE")
            .help("Output file")
            .required(true)
            .takes_value(true))
        .get_matches(); // 解析命令行参数

    // 从命令行参数中获取值
    let input_file = matches.value_of("input").unwrap();
    let output_file = matches.value_of("output").unwrap();

    println!("Input file: {}", input_file);
    println!("Output file: {}", output_file);

    // 在这里可以执行您的应用程序逻辑，使用input_file和output_file参数
}
