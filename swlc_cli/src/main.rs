//use std::process::exit;

use swlc_core::config::*;
use swlc_core::model_content::get_instanstance_of_tanach;
// use swlc_core::help::*;
// use swlc_core::search::*;
use swlc_core::tools::check_counters;
use swlc_core::tools::reorder_books;

fn main() {
    // load (and or create) the configuration file
    let config = Config::read_from_file();
    display_current_configuration(&config);
    //config.update(Some("Chrono".to_string()),None,None);
    //let new_config = Config::read_from_file();
    //display_current_configuration(&new_config);

    //get the tanach structure
    let mut tanach = get_instanstance_of_tanach();
    // check counters
    check_counters(&tanach);
    //dbg!("{:?}",&tanach);
    reorder_books(&mut tanach, &config.book_order);

    //display_help();

    //infinite loop for <commands> and <items> to search for in the tanach
    /*
    loop {
        println!("\nPlease type a command OR a string to search for (h for help):");
        let search_or_command = get_user_input();
        match search_or_command.as_str() {
            "h" => display_help(),
            "a" => display_list_of_abbreviations(),
            "c" => display_current_configuration(&config),
            "r" => set_search_range(&mut config, &tanach),
            "o" => set_search_order(&mut config),
            "x" => exit(0),
            _ => {
                println!("Searching for <{}> in the WLC.", &search_or_command);
                match search_text_in_tanach(&tanach, &search_or_command) {
                    Ok(result) => println!("Result: {:?}", result),
                    Err(e) => println!("Error: {}", e),
                }
                /*let char_types = get_character_types(&search_or_command);
                println!("{:?}",char_types);
                if char_types.non_hebrew {
                    println!("  Sorry, the given text is NOT a valid Hebrew text.");
                    println!("  Use only Hebrew chars and white spaces (unicode definition) are allowed.");
                    println!("  Please try again");
                } else {
                    search_text(&search_or_command);
                }*/
            }
        }
    }
    */
}
