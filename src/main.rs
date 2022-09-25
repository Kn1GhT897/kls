mod dir_vec_builder;
mod output;

use std::env;
use std::path::Path;

fn main() {
    use dir_vec_builder::DirVecBuilder;
    use output::Output;
    let args: Vec<String> = env::args().collect();

    let path = Path::new(&args[args.len() - 1]);
    let options: String = if args.len() == 3 {
        args.get(1).unwrap().clone()
    } else {
        String::from("_")
    };


    let vector = DirVecBuilder::build_vec(options, path);

    match vector {
        Ok(item) => Output::output(item),
        Err(e) => {
            println!("{}", e.to_string());
            return
        }
    }

}
