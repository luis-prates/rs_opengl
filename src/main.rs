#[cfg(feature = "chapter-1")]
mod _1_getting_started;
#[cfg(feature = "chapter-1")]
use _1_getting_started::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("Call with the number of the tutorial, e.g. `1_1_2` for _1_2_hello_window_clear.rs");
        std::process::exit(1);
    }
    let tutorial_id = &args[1];

    match tutorial_id.as_str() {
        #[cfg(feature = "chapter-1")] "1_2_3" => main_1_2_3(),
        #[cfg(feature = "chapter-1")] "1_2_4" => main_1_2_4(),
		#[cfg(feature = "chapter-1")] "1_2_5" => main_1_2_5(),
        _     => println!("Unknown tutorial id")
    }
}