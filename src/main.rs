
mod common;
mod shader;
mod macros;
mod camera;
mod model;
mod mesh;

#[cfg(feature = "chapter-1")]
mod _1_getting_started;
#[cfg(feature = "chapter-1")]
use _1_getting_started::*;

#[cfg(feature = "chapter-2")]
mod _2_lighting;
#[cfg(feature = "chapter-2")]
use _2_lighting::*;

#[cfg(feature = "chapter-3")]
mod _3_model_loading;
#[cfg(feature = "chapter-3")]
use _3_model_loading::*;

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
		#[cfg(feature = "chapter-1")] "1_3_1" => main_1_3_1(),
		#[cfg(feature = "chapter-1")] "1_3_2" => main_1_3_2(),
		#[cfg(feature = "chapter-1")] "1_3_3" => main_1_3_3(),
		#[cfg(feature = "chapter-1")] "1_3_4" => main_1_3_4(),
		#[cfg(feature = "chapter-1")] "1_3_5" => main_1_3_5(),
		#[cfg(feature = "chapter-1")] "1_3_6" => main_1_3_6(),
		#[cfg(feature = "chapter-1")] "1_4_1" => main_1_4_1(),
		#[cfg(feature = "chapter-1")] "1_4_2" => main_1_4_2(),
		#[cfg(feature = "chapter-1")] "1_4_3" => main_1_4_3(),
		#[cfg(feature = "chapter-1")] "1_4_4" => main_1_4_4(),
		#[cfg(feature = "chapter-1")] "1_4_5" => main_1_4_5(),
		#[cfg(feature = "chapter-1")] "1_4_6" => main_1_4_6(),
		#[cfg(feature = "chapter-1")] "1_5_1" => main_1_5_1(),
		#[cfg(feature = "chapter-1")] "1_5_2" => main_1_5_2(),
		#[cfg(feature = "chapter-1")] "1_5_3" => main_1_5_3(),
		#[cfg(feature = "chapter-1")] "1_6_1" => main_1_6_1(),
		#[cfg(feature = "chapter-1")] "1_6_2" => main_1_6_2(),
		#[cfg(feature = "chapter-1")] "1_6_3" => main_1_6_3(),
		#[cfg(feature = "chapter-1")] "1_7_1" => main_1_7_1(),
		#[cfg(feature = "chapter-1")] "1_7_2" => main_1_7_2(),
		#[cfg(feature = "chapter-1")] "1_7_3" => main_1_7_3(),
		#[cfg(feature = "chapter-1")] "1_7_4" => main_1_7_4(),

		#[cfg(feature = "chapter-2")] "2_1"   => main_2_1(),
		#[cfg(feature = "chapter-2")] "2_2_1" => main_2_2_1(),
		#[cfg(feature = "chapter-2")] "2_2_2" => main_2_2_2(),
		#[cfg(feature = "chapter-2")] "2_2_3" => main_2_2_3(),
		#[cfg(feature = "chapter-2")] "2_2_4" => main_2_2_4(),
		#[cfg(feature = "chapter-2")] "2_3_1" => main_2_3_1(),
		#[cfg(feature = "chapter-2")] "2_3_2" => main_2_3_2(),
		#[cfg(feature = "chapter-2")] "2_4_1" => main_2_4_1(),
		#[cfg(feature = "chapter-2")] "2_4_2" => main_2_4_2(),
		#[cfg(feature = "chapter-2")] "2_4_3" => main_2_4_3(),
		#[cfg(feature = "chapter-2")] "2_5_1" => main_2_5_1(),
		#[cfg(feature = "chapter-2")] "2_5_2" => main_2_5_2(),
		#[cfg(feature = "chapter-2")] "2_5_3" => main_2_5_3(),
		#[cfg(feature = "chapter-2")] "2_5_4" => main_2_5_4(),
		#[cfg(feature = "chapter-2")] "2_6_1" => main_2_6_1(),

		#[cfg(feature = "chapter-3")] "3_1"   => main_3_1(),
		#[cfg(feature = "chapter-3")] "3_2"   => main_3_2(),

        _     => println!("Unknown tutorial id")
    }
}