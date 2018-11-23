use std::env;
mod compiler;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        let file_a = &args[1];
        let file_b = &args[2];

        read_files(vec![file_a, file_b]);
    } else {
        println!("No file provided!");
    }
}

fn read_files(filenames: Vec<&String>) {
    let ast = compiler::parse(filenames);
    print!("{:#?}", ast)
}
