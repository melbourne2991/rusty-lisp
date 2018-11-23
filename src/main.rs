use std::env;
mod compiler;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        let file_a = &args[1];
        let file_b = &args[2];
        let file_c = &args[3];
        let file_d = &args[4];

        read_files(vec![file_a, file_b, file_c, file_d]);
    } else {
        println!("No file provided!");
    }
}

fn read_files(filenames: Vec<&String>) {
    let ast = compiler::parse(filenames);
    // print!("{:#?}", ast)
    print!("Mission complete")
}
