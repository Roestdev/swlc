use crate::help::ABBREVIATIONS_2_FULL_NAME;
//use crate::model::Tanach;
use crate::model_definition::{Tanach, VerseReference};
// use crate::tools::get_index_for_verse_reference;

use crate::tools::*;
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::env;
//use std::fmt::Error;
use std::fs;
use std::fs::OpenOptions;
use std::io;
use std::io::{BufRead, Write};
use std::path::{Path, PathBuf};
//use std::process::exit;
use toml;

use std::cmp::Ordering;

const CONFIG_FILE_NAME: &str = "config.toml";
#[derive(Serialize, Deserialize, Debug)]
struct ConfigToml {
    program_data: Option<ConfigTomlWlcData>,
    search_options: Option<ConfigTomlSearch>,
}
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
struct ConfigTomlWlcData {
    data_directory: Option<String>,
}
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
struct ConfigTomlSearch {
    //book_order: Vec<String>,
    //book_order_id: String,
    book_order: BookOrder,
    start_of_search: VerseReference,
    end_of_search: VerseReference,
}
#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub enum BookOrder {
    #[default]
    Tanach,
    Canonical,
    Chronological,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    //pub book_order_id: String,
    pub book_order: BookOrder,
    pub start_of_search: VerseReference,
    pub end_of_search: VerseReference,
    pub data_directory: PathBuf,
}

impl Default for Config {
    fn default() -> Self {
        let (_, data_dir) = get_project_dirs();
        Config {
            data_directory: data_dir,
            start_of_search: VerseReference {
                book_name: "Genesis".to_string(),
                chapter_no: 1,
                verse_no: 1,
            },
            end_of_search: VerseReference {
                book_name: "2 Chronicles".to_string(),
                chapter_no: 36,
                verse_no: 23,
            },
            //book_order_id: "Tanach".to_string(),
            book_order: BookOrder::Tanach,
        }
    }
}
impl Config {
    pub fn read_from_file() -> Self {
        // get configuration files paths first
        let config_file_name = CONFIG_FILE_NAME;
        let (config_dir, _) = get_project_dirs();
        let config_file_path = config_dir.join(config_file_name);
        // check if a config file is present
        if !config_file_path.is_file() {
            println!("No configuration file found!");
            println!("Creating a new default configuration file.");
            let mut new_config = Config::default();
            new_config.write_to_file();
            println!("A new default configuration file has been created.");
        }
        // read the configuration file
        let mut content: String = "".to_owned();
        let result = fs::read_to_string(config_file_path.clone());
        if result.is_ok() {
            content = result.unwrap();
        }
        if content.is_empty() {
            println!("Configuration file found, but is empty!");
            println!("Creating a new default configuration file.");
            let mut new_config = Config::default();
            new_config.write_to_file();
            println!("A new default configuration file has been created.");
        }

        let config_toml: ConfigToml = toml::from_str(&content).expect(
            "\n==> Failed to create Config Object out of config file.
                         \n==> Please remove the configuration file and restart the program!\n\n.",
        );

        let directory = match config_toml.program_data {
            Some(wlc) => wlc.data_directory.unwrap_or_else(|| {
                println!("Missing field api_key in table wlc.");
                "unknown".to_owned()
            }),
            None => {
                println!("Missing table wlc.");
                "unknown".to_owned()
            }
        };
        // check search
        let (book_order, start_searching, end_searching): (
            BookOrder,
            VerseReference,
            VerseReference,
        ) = match config_toml.search_options {
            Some(search) => {
                let search_book_order = search.book_order;
                let search_start = search.start_of_search;
                let search_stop = search.end_of_search;
                (search_book_order, search_start, search_stop)
            }
            None => {
                println!("Missing table search.");
                let empty_bible_ref = VerseReference {
                    book_name: "unknown".to_string(),
                    chapter_no: 0,
                    verse_no: 0,
                };
                (
                    BookOrder::default(),
                    empty_bible_ref.clone(),
                    empty_bible_ref.clone(),
                )
            }
        };
        Config {
            data_directory: PathBuf::from(directory),
            start_of_search: start_searching,
            end_of_search: end_searching,
            book_order,
        }
    }
    pub fn write_to_file(&mut self) {
        let config_toml = ConfigToml {
            program_data: Some(ConfigTomlWlcData {
                data_directory: Some(self.data_directory.to_string_lossy().to_string()),
            }),
            search_options: Some(ConfigTomlSearch {
                book_order: self.book_order.clone(),
                start_of_search: self.start_of_search.clone(),
                end_of_search: self.end_of_search.clone(),
            }),
        };
        let config_file_name = CONFIG_FILE_NAME;
        let (config_dir, _) = get_project_dirs();
        let config_file_path = config_dir.join(config_file_name);
        let mut file = OpenOptions::new()
            .truncate(true)
            .create(true)
            .write(true)
            .read(true)
            .open(config_file_path)
            .unwrap(); //TODO
        let toml = toml::to_string(&config_toml).unwrap();
        let _ = file.write_all(toml.as_bytes());
    }

    pub fn update(
        &mut self,
        new_book_order: Option<BookOrder>,
        new_start: Option<VerseReference>,
        new_stop: Option<VerseReference>,
    ) {
        if new_book_order.is_some() {
            self.book_order = new_book_order.unwrap();
        }
        if new_start.is_some() {
            self.start_of_search = new_start.unwrap();
        }
        if new_stop.is_some() {
            self.end_of_search = new_stop.unwrap();
        }
        self.write_to_file();
    }
}
pub fn get_project_dirs() -> (PathBuf, PathBuf) {
    let args: Vec<String> = env::args().collect();
    let binding = args[0].clone();
    let file = binding.as_str();
    let path = Path::new(file);
    let binding = path
        .file_name()
        .expect("Unable to extract file name; maybe UTF8 related?")
        .to_os_string();
    let app_name = binding.to_str().unwrap();
    let project_dirs =
        ProjectDirs::from("rs", "", app_name).ok_or("Could not determine home data_directory path");
    let binding = project_dirs.expect("TODO REASON");
    let config_dir = binding.config_dir().to_path_buf();
    let data_dir = binding.data_dir().to_path_buf();
    (config_dir, data_dir)
}
pub fn get_user_input() -> String {
    let stdin = io::stdin();
    stdin
        .lock()
        .lines()
        .next()
        .expect("There was no next line")
        .expect("The line could not be read")
}
pub fn display_current_configuration(config: &Config) {
    println!("\n----------------------------------------------------------");
    println!("The current configuration is:\n");
    //println!("Search order: {}", config.book_order_id);
    println!("Search order:      {:?}", config.book_order);
    println!(
        "Start search at:   book = {}, chapter = {}, verse = {}",
        config.start_of_search.book_name,
        config.start_of_search.chapter_no,
        config.start_of_search.verse_no
    );
    println!(
        "Stop search at:    book = {}, chapter = {}, verse = {}",
        config.end_of_search.book_name,
        config.end_of_search.chapter_no,
        config.end_of_search.verse_no
    );
    println!("Data directory:    {:#?}", config.data_directory);
}

pub fn set_search_order(config: &mut Config) {
    println!("\n--------------------------");
    println!("Set a new search order.\n");
    println!("Please select a new search_order: (ta: TAnach , ca: CAnonical, ch: CHronological)");
    let search_order = get_user_input().to_lowercase();
    match search_order.as_str().trim_end() {
        "ta" => {
            // Tanach ordering
            let start_of_search = VerseReference {
                book_name: "Genesis".to_string(),
                chapter_no: 1,
                verse_no: 1,
            };
            let end_of_search = VerseReference {
                book_name: "2 Chronicles".to_string(),
                chapter_no: 36,
                verse_no: 23,
            };
            config.update(
                Some(BookOrder::Tanach),
                Some(start_of_search),
                Some(end_of_search),
            );
        }
        "ca" => {
            // Canonical ordering
            let start_of_search = VerseReference {
                book_name: "Genesis".to_string(),
                chapter_no: 1,
                verse_no: 1,
            };
            let end_of_search = VerseReference {
                book_name: "Maleachi".to_string(),
                chapter_no: 4,
                verse_no: 6,
            };
            config.update(
                Some(BookOrder::Canonical),
                Some(start_of_search),
                Some(end_of_search),
            );
        }
        "ch" => {
            // Chronological ordering
            // https://www.biblegateway.com/blog/2016/02/when-was-each-book-of-the-bible-written/
            let start_of_search = VerseReference {
                book_name: "Job".to_string(),
                chapter_no: 1,
                verse_no: 1,
            };
            let end_of_search = VerseReference {
                book_name: "Nehemiah".to_string(),
                chapter_no: 13,
                verse_no: 31,
            };
            config.update(
                Some(BookOrder::Chronological),
                Some(start_of_search),
                Some(end_of_search),
            );
        }
        _ => {
            println!("Non-existing search order, please enter:");
            println!("'ta': TAnach ,\n 'ca': CAnonical,\n 'ch': CHronological");
        }
    }
}

pub fn set_search_range(config: &mut Config, tanach: &Tanach) {
    println!("\n--------------------------------------------------");
    println!("Set a new search range.\n");
    println!("Please enter data for the START of the search range");
    println!("Enter a book abbreviation: (Gen | Exo ...)");
    let user_input = get_user_input();
    let binding = user_input.to_lowercase();
    let mut new_start_book_name = binding.trim_end();
    if let Some(i) = ABBREVIATIONS_2_FULL_NAME.get(new_start_book_name) {
        new_start_book_name = i;
        println!("Found: {:?}!", new_start_book_name);
    } else {
        println!("Used abbreviation '{}' is not valid.", new_start_book_name);
        println!("Please try again, press 'a' to get a list of valid abbreviations.");
        return;
    }
    println!("Chapter number: (1 < n < 255)");
    let user_input = get_user_input();
    let new_start_chapter_number = user_input.as_str().trim_end();
    println!("Verse number: (1 < n < 255)");
    let user_input = get_user_input();
    let new_start_verse_number = user_input.as_str().trim_end();
    let start_search_reference = VerseReference {
        book_name: new_start_book_name.to_string(),
        chapter_no: new_start_chapter_number.parse::<u16>().unwrap_or(255),
        verse_no: new_start_verse_number.parse::<u16>().unwrap_or(255),
    };
    //
    println!("\nPlease enter the data for the END of the search range");
    println!("Enter a book abbreviation: (Gen | Exo ...)");
    let user_input = get_user_input();
    let binding = user_input.to_lowercase();
    let mut new_end_book_name = binding.trim_end();
    if let Some(i) = ABBREVIATIONS_2_FULL_NAME.get(new_end_book_name) {
        new_end_book_name = i;
        println!("Found: {:?}!", new_end_book_name);
    } else {
        println!("Used abbreviation '{}' is not valid.", new_end_book_name);
        println!("Please try again, press 'a' to get a list of valid abbreviations.");
        return;
    }
    println!("Chapter number: (1 < n < 255)");
    let user_input = get_user_input();
    let new_end_chapter_number = user_input.as_str().trim_end();
    println!("Verse number: (1 < n < 255)");
    let user_input = get_user_input();
    let new_end_verse_number = user_input.as_str().trim_end();
    let end_search_reference = VerseReference {
        book_name: new_end_book_name.to_string(),
        chapter_no: new_end_chapter_number.parse::<u16>().unwrap_or(255),
        verse_no: new_end_verse_number.parse::<u16>().unwrap_or(255),
    };
    println!(
        "\nYour entered the following:\n- start search reference > book:{}, chapter:{}. verse:{}",
        new_start_book_name, new_start_chapter_number, new_start_verse_number
    );
    println!(
        "- end search reference >   book:{}, chapter:{}. verse:{}",
        new_end_book_name, new_end_chapter_number, new_end_verse_number
    );
    // let s = get_index_for_verse_reference(tanach, &start_search_reference);
    // let e = get_index_for_verse_reference(tanach, &end_search_reference);
    //println!("s-index-> {:?}, e-index-> {:?}", s, e);
    let mut start_reference = usize::MIN;
    match get_index_for_verse_reference(tanach, &start_search_reference) {
        Some(reference) => start_reference = reference,
        None => {
            println!("The given 'start search reference' is not found.\nPlease try again.");
            return;
        }
    }
    let mut end_reference = usize::MIN;
    match get_index_for_verse_reference(tanach, &end_search_reference) {
        Some(reference) => end_reference = reference,
        None => {
            println!("The given 'end search reference' is not found.\nPlease try again.");
            return;
        }
    }
    match start_reference.cmp(&end_reference) {
        Ordering::Less => {
            println!("start < end ");
            config.update(
                None,
                Some(start_search_reference.clone()),
                Some(end_search_reference.clone()),
            );
        }
        Ordering::Equal | Ordering::Greater => {
            println!("The entered search range is not valid.");
            println!("The end of the range must be larger then the start.");
            println!(
                "Start of range: {}, end of range: {} (index)",
                start_reference, end_reference
            );
        }
    }
}

pub fn get_book_order(selector: &BookOrder) -> Vec<String> {
    let tanach = vec![
        // // Torah
        // "Genesis".to_string(),
        // "Exodus".to_string(),
        // "Leviticus".to_string(),
        // "Numbers".to_string(),
        // "Deuteronomy".to_string(),
        // // Former Prophets
        // "Joshua".to_string(),
        // "Judges".to_string(),
        // "1 Samuel".to_string(),
        // "2 Samuel".to_string(),
        // "1 Kings".to_string(),
        // "2 Kings".to_string(),
        // // Latter Major Prophets
        // "Isaiah".to_string(),
        // "Jeremiah".to_string(),
        // "Ezekiel".to_string(),
        // // Latter Minor Prophets
        // "Hosea".to_string(),
        // "Joel".to_string(),
        // "Amos".to_string(),
        "Obadiah".to_string(),
        // "Jonah".to_string(),
        // "Micah".to_string(),
        "Nahum".to_string(),
        "Habakkuk".to_string(),
        // "Zephaniah".to_string(),
        "Haggai".to_string(),
        // "Zechariah".to_string(),
        // "Malachi".to_string(),
        // // Wisdom
        // "Psalms".to_string(),
        // "Proverbs".to_string(),
        // "Job".to_string(),
        // // Festival
        // "Song of Songs".to_string(),
        // "Ruth".to_string(),
        // "Lamentations".to_string(),
        // "Ecclesiastes".to_string(),
        // "Esther".to_string(),
        // // History
        // "Daniel".to_string(),
        // "Ezra".to_string(),
        // "Nehemiah".to_string(),
        // "1 Chronicles".to_string(),
        // "2 Chronicles".to_string(),
    ];
    let canonical = vec![
        // Pentateuch
        // "Genesis".to_string(),
        // "Exodus".to_string(),
        // "Leviticus".to_string(),
        // "Numbers".to_string(),
        // "Deuteronomy".to_string(),
        // // Historical books
        // "Joshua".to_string(),
        // "Judges".to_string(),
        // "Ruth".to_string(),
        // "1 Samuel".to_string(),
        // "2 Samuel".to_string(),
        // "1 Kings".to_string(),
        // "2 Kings".to_string(),
        // "1 Chronicles".to_string(),
        // "2 Chronicles".to_string(),
        // "Ezra".to_string(),
        // "Nehemiah".to_string(),
        // "Esther".to_string(),
        // // Poetic and wisdom books
        // "Job".to_string(),
        // "Psalms".to_string(),
        // "Proverbs".to_string(),
        // "Ecclesiastes".to_string(),
        // "Song of Songs".to_string(),
        // // Great prophets
        // "Isaiah".to_string(),
        // "Jeremiah".to_string(),
        // "Lamentations".to_string(),
        // "Ezekiel".to_string(),
        // "Daniel".to_string(),
        // // Minor prophets
        // "Hosea".to_string(),
        // "Joel".to_string(),
        // "Amos".to_string(),
        "Obadiah".to_string(),
        // "Jonah".to_string(),
        // "Micah".to_string(),
        "Nahum".to_string(),
        "Habakkuk".to_string(),
        // "Zephaniah".to_string(),
        "Haggai".to_string(),
        // "Zechariah".to_string(),
        // "Malachi".to_string(),
    ];

    let chronological = vec![
        // see: https://www.biblegateway.com/blog/2016/02/when-was-each-book-of-the-bible-written/
        // "Job".to_string(),
        // "Genesis".to_string(),
        // "Exodus".to_string(),
        // "Leviticus".to_string(),
        // "Numbers".to_string(),
        // "Deuteronomy".to_string(),
        // "Psalms".to_string(),
        // "Joshua".to_string(),
        // "Judges".to_string(),
        // "Ruth".to_string(),
        // "Song of Songs".to_string(),
        // "Proverbs".to_string(),
        // "Ecclesiastes".to_string(),
        // "1 Samuel".to_string(),
        // "2 Samuel".to_string(),
        "Obadiah".to_string(),
        // "Joel".to_string(),
        // "Jonah".to_string(),
        // "Amos".to_string(),
        // "Hosea".to_string(),
        // "Micah".to_string(),
        // "Isaiah".to_string(),
        "Nahum".to_string(),
        // "Zephaniah".to_string(),
        "Habakkuk".to_string(),
        // "Ezekiel".to_string(),
        // "Lamentations".to_string(),
        // "Jeremiah".to_string(),
        // "1 Kings".to_string(),
        // "2 Kings".to_string(),
        // "Daniel".to_string(),
        "Haggai".to_string(),
        // "Zechariah".to_string(),
        // "Ezra".to_string(),
        // "1 Chronicles".to_string(),
        // "2 Chronicles".to_string(),
        // "Esther".to_string(),
        // "Malachi".to_string(),
        // "Nehemiah".to_string(),
    ];
    match selector {
        BookOrder::Tanach => tanach,
        BookOrder::Canonical => canonical,
        BookOrder::Chronological => chronological,
    }
}
