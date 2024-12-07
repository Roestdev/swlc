use itertools::Itertools;
use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    pub static ref FULL_NAME_2_ABBREVIATION: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("Genesis", "Gen");
        m.insert("Exodus", "Exo");
        m.insert("Leviticus", "Lev");
        m.insert("Numbers", "Num");
        m.insert("Deuteronomy", "Deu");
        m.insert("Joshua", "Jos");
        m.insert("Judges", "Jdg");
        m.insert("1 Samuel", "1Sa");
        m.insert("2 Samuel", "2Sa");
        m.insert("1 Kings", "1Ki");
        m.insert("2 Kings", "2Ki");
        m.insert("Isaiah", "Isa");
        m.insert("Jeremiah", "Jer");
        m.insert("Ezekiel", "Eze");
        m.insert("Hosea", "Hos");
        m.insert("Joel", "Joe");
        m.insert("Amos", "Amo");
        m.insert("Obadiah", "Oba");
        m.insert("Jonah", "Jon");
        m.insert("Micah", "Mic");
        m.insert("Nahum", "Nah");
        m.insert("Habakkuk", "Hab");
        m.insert("Zephaniah", "Zep");
        m.insert("Haggai", "Hag");
        m.insert("Zechariah", "Zec");
        m.insert("Malachi", "Mal");
        m.insert("Psalms", "Psa");
        m.insert("Proverbs", "Pro");
        m.insert("Job", "Job");
        m.insert("Song of Solomon", "Sng");
        m.insert("Ruth", "Rth");
        m.insert("Lamentations", "Lam");
        m.insert("Ecclesiastes", "Ecc");
        m.insert("Esther", "Est");
        m.insert("Daniel", "Dan");
        m.insert("Ezra", "Ezr");
        m.insert("Nehemiah", "Neh");
        m.insert("1 Chronicles", "1Ch");
        m.insert("2 Chronicles", "2Ch");
        m
    };
}

lazy_static! {
    pub static ref ABBREVIATIONS_2_FULL_NAME: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("1ch", "1 Chronicles");
        m.insert("1c", "1 Chronicles");
        m.insert("1ki", "1 Kings");
        m.insert("1k", "1 Kings");
        m.insert("1sa", "1 Samuel");
        m.insert("1s", "1 Samuel");
        m.insert("2ch", "2 Chronicles");
        m.insert("2c", "2 Chronicles");
        m.insert("2ki", "2 Kings");
        m.insert("2k", "2 Kings");
        m.insert("2sa", "2 Samuel");
        m.insert("2s", "2 Samuel");
        m.insert("Amo", "Amos");
        m.insert("amo", "Amos");
        m.insert("am", "Amos");
        m.insert("a", "Amos");
        m.insert("Dan", "Daniel");
        m.insert("dan", "Daniel");
        m.insert("da", "Daniel");
        m.insert("Deu", "Deuteronomy");
        m.insert("deu", "Deuteronomy");
        m.insert("de", "Deuteronomy");
        m.insert("Exo", "Exodus");
        m.insert("exo", "Exodus");
        m.insert("ex", "Exodus");
        m.insert("Eze", "Ezekiel");
        m.insert("eze", "Ezekiel");
        m.insert("Ecc", "Ecclesiastes");
        m.insert("ecc", "Ecclesiastes");
        m.insert("ec", "Ecclesiastes");
        m.insert("Est", "Esther");
        m.insert("est", "Esther");
        m.insert("es", "Esther");
        m.insert("Ezr", "Ezra");
        m.insert("ezr", "Ezra");
        m.insert("Gen", "Genesis");
        m.insert("gen", "Genesis");
        m.insert("ge", "Genesis");
        m.insert("g", "Genesis");
        m.insert("Hag", "Haggai");
        m.insert("hag", "Haggai");
        m.insert("Hos", "Hosea");
        m.insert("hos", "Hosea");
        m.insert("ho", "Hosea");
        m.insert("nah", "Nahum");
        m.insert("na", "Nahum");
        m.insert("hab", "Habakkuk");
        m.insert("h", "Habakkuk");
        m.insert("isa", "Isaiah");
        m.insert("is", "Isaiah");
        m.insert("i", "Isaiah");
        m.insert("jer", "Jeremiah");
        m.insert("je", "Jeremiah");
        m.insert("job", "Job");
        m.insert("joe", "Joel");
        m.insert("jon", "Jonah");
        m.insert("jos", "Joshua");
        m.insert("jdg", "Judges");
        m.insert("jd", "Judges");
        m.insert("jud", "Judges");
        m.insert("ju", "Judges");
        m.insert("lam", "Lamentations");
        m.insert("la", "Lamentations");
        m.insert("lev", "Leviticus");
        m.insert("mal", "Malachi");
        m.insert("mic", "Micah");
        m.insert("neh", "Nehemiah");
        m.insert("ne", "Nehemiah");
        m.insert("num", "Numbers");
        m.insert("nu", "Numbers");
        m.insert("oba", "Obadiah");
        m.insert("ob", "Obadiah");
        m.insert("o", "Obadiah");
        m.insert("pro", "Proverbs");
        m.insert("pr", "Proverbs");
        m.insert("psa", "Psalms");
        m.insert("ps", "Psalms");
        m.insert("rth", "Ruth");
        m.insert("rt", "Ruth");
        m.insert("ru", "Ruth");
        m.insert("r", "Ruth");
        m.insert("sng", "Song of Solomon");
        m.insert("sos", "Song of Solomon");
        m.insert("so", "Song of Solomon");
        m.insert("s", "Song of Solomon");
        m.insert("zep", "Zephaniah");
        m.insert("zec", "Zechariah");
        m
    };
}

pub fn display_help() {
    println!("\n");
    println!("--------------------------------------------------------------------");
    println!("This program accepts strings (utf8), which are either a command or");
    println!("an item to search for in the Westminster Leningrad Codex (WLC).");
    println!("\nCommands::");
    println!("    h   =>  display this Help.");
    println!("    a   =>  display used book Abbreviations.");
    println!("    c   =>  display the current Configuration.");
    println!("    o   =>  set a new search Order.");
    println!("    r   =>  set a new search Range.");
    println!("    x   =>  eXit this program.");
    println!("\nItem::");
    println!("    The item you want to search for is a string (utf8) where all");
    println!("    characters belong to one of the sets below:");
    println!("      - A member of the Hebrew character class.");
    println!("      - A white-space conform https://www.unicode.org/reports/tr31/");
    println!("--------------------------------------------------------------------");
}

/*todo pub fn display_about(){}*/
pub fn display_list_of_abbreviations() {
    let x: Vec<(&str, &str)> = FULL_NAME_2_ABBREVIATION.clone().into_iter().collect();
    println!("\n-----------------------------------------------------");
    println!("List of abbreviations to be used in the search range:\n");
    println!("{:<21}{:<}", "Book name", "Abbr.");
    println!("--------------------------");
    for item in x.into_iter().sorted() {
        println!("{:<16}->{:>6}", item.0, item.1);
    }
}
