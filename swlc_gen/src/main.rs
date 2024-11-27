mod funcs;
use crate::funcs::*;
use std::fs;
use std::path::Path;


//based upon: https://users.rust-lang.org/t/external-code-generation-still-use-cargo/89701/2S

fn main() {
    // define the result directory where the generated file will be stored temporarily
    let res_dir = "./swlc_gen/data/output/";
    let _ = fs::create_dir_all(res_dir);
    let result_dir = Path::new(res_dir);
    
    // define the file where the generated code will be stored
    let file_to_generate = "generated_code.rs";
    let file_to_generate_pb = result_dir.join(file_to_generate);
    let file_to_generate_p = file_to_generate_pb.as_path();
    
    // define the directory where txt files of the WLC are stored
    let wlc_src_files = "./swlc_gen/data/input/";
    let file_names = get_files_names(wlc_src_files);
    
    // get cmd option
    let cmd_option = std::env::args().nth(1);
    if cmd_option.is_some_and(|x| x.eq("c")) {
        // copy the file
        let src_file = "swlc_gen/data/output/generated_code.rs";
        let target_file = "swlc_com/src/generated_code.rs";
        copy_generated_file(src_file, target_file);
    } else {
        // generate the code
        match insert_header(file_to_generate_p) {
            Ok(_) => println!("  > Inserting header succeeded."),
            Err(err) => println!("ERROR: Inserting header failed: [{}]", err),
        }
        match process_all_source_files(wlc_src_files, file_names, file_to_generate_p) {
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
    }
}
