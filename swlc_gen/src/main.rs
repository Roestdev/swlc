use std::fs;
use std::path::Path;

mod funcs;
use crate::funcs::{
    copy_generated_file, format_generated_file, get_files_names, insert_footer, insert_header,
    process_all_source_files,
};
fn main() {
    // get cmd option
    let cmd_option = std::env::args().nth(1);

    if cmd_option.is_none() {
        // define the result directory where the generated file will be stored temporarily
        let res_dir = "./swlc_gen/data/output/";
        let _ = fs::create_dir_all(res_dir);
        let result_dir = Path::new(res_dir);

        // define the file name and location of the generated file
        let file_to_generate = "model_content.rs";
        let file_to_generate_pb = result_dir.join(file_to_generate);
        let file_to_generate_p = file_to_generate_pb.as_path();

        // define the directory where WLC txt files are stored
        let wlc_src_files = "./swlc_gen/data/input/";
        let found_files_names = get_files_names(wlc_src_files);

        // generate the code
        match insert_header(file_to_generate_p) {
            Ok(_) => println!("  > Inserting header succeeded."),
            Err(err) => println!("ERROR: Inserting header failed: [{}]", err),
        }
        match process_all_source_files(wlc_src_files, found_files_names, file_to_generate_p) {
            Ok(_) => println!("  > Processing files succeeded."),
            Err(err) => println!("ERROR: Processing files failed: [{}]", err),
        }
        match insert_footer(file_to_generate_p) {
            Ok(_) => println!("  > Inserting footer succeeded."),
            Err(err) => println!("ERROR: Inserting footer failed: [{}]", err),
        }
        match format_generated_file(file_to_generate_p) {
            Ok(_) => println!("  > Formatting file succeeded."),
            Err(err) => println!("ERROR: Formatting file failed: [{}]", err),
        }
        println!("Code generation is complete.");
    } else if cmd_option.is_some_and(|x| x.to_lowercase().eq("c")) {
        // copy file
        let src_file = "swlc_gen/data/output/model_content.rs";
        let target_file = "swlc_core/src/model_content.rs";
        copy_generated_file(src_file, target_file);
    } else {
        println!("Usage:");
        println!("  > cargo run -p swlc_gen    (generate file)");
        println!("  > cargo run -p swlc_gen c  (copy generated file)");
    }
}
