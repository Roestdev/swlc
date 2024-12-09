use chrono::prelude::*;
use regex::Regex;
use std::fs;
use std::fs::OpenOptions;
use std::io::ErrorKind;
use std::io::{BufRead, BufReader, Write};
use std::path::Path;
use std::process::exit;
use std::process::Command;

//use swlc_core::NR_OF_BOOKS_IN_TANACH;

pub fn get_files_names(wlc_dir: &str) -> Vec<String> {
    let entries = fs::read_dir(wlc_dir);
    let files = match entries {
        Ok(v) => v,
        Err(error) => {
            println!("ERROR::{}", error);
            println!("Program will halt with exit code: {}", exitcode::CONFIG);
            exit(exitcode::CONFIG);
        }
    };
    let file_names: Vec<String> = files
        .filter_map(|entry| {
            let path = entry.ok()?.path();
            if path.is_file() {
                path.file_name()?.to_str().map(|s| s.to_owned())
            } else {
                None
            }
        })
        .collect();

    /*
    println!(
        "Check if the correct number of WLC txt files are present in the source directory ..."
    );
    if usize::from(NR_OF_BOOKS_IN_TANACH) != file_names.len() {
        println!(
            "ERROR::Expected: {} file(s), found: {} file(s).",
            NR_OF_BOOKS_IN_TANACH,
            file_names.len()
        );
        println!("Program will halt with exit code: {}", exitcode::CONFIG);
        exit(exitcode::CONFIG);
    } else {
        println!("  > Correct number of files are found.");
    }
    */
    file_names
}

pub fn insert_header(output_file: &Path) -> Result<(), std::io::Error> {
    println!("Inserting a header to the generated code ...");
    let output_file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(output_file)
        .expect("  > Unable to insert the header into the output file.");

    writeln!(
        &output_file,
        "// ++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++\n\
         //              ===> DO NOT CHANGE THIS FILE !!! <===                   +\n\
         // This file was automatically generated at: {}    +\n\
         // ++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++\n",
        Local::now().format("%Y-%m-%dT%H:%M:%S.%3f")
    )?;

    writeln!(&output_file, "use std::collections::HashMap;")?;
    writeln!(&output_file, "use crate::model_definition::Tanach;")?;
    writeln!(&output_file, "use crate::model_definition::BookMetaData;")?;
    writeln!(&output_file, "use crate::model_definition::VerseReference;")?;
    writeln!(&output_file, "use crate::NR_OF_BOOKS_IN_TANACH;")?;
    writeln!(&output_file, "\n")?;

    writeln!(
        &output_file,
        "pub fn get_instanstance_of_tanach()  -> Tanach {{"
    )?;
    writeln!(&output_file, "    let mut tanach = Tanach {{")?;
    writeln!(
        &output_file,
        "        nr_of_books_expected: NR_OF_BOOKS_IN_TANACH,"
    )?;
    writeln!(&output_file, "        ..Default::default()")?;
    writeln!(&output_file, "    }};\n")?;
    Ok(())
}

pub fn process_all_source_files(
    wlc_dir: &str,
    file_names: Vec<String>,
    file_to_generate_p: &Path,
) -> Result<(), std::io::Error> {
    println!("Generate code for every WLC text file ...");
    for file in file_names {
        println!("  > Processing file: [{}{}]", wlc_dir, file);
        let wlc_dir = Path::new(wlc_dir);
        let file_to_read: &str = &file;
        let file_to_read_pb = wlc_dir.join(file_to_read);
        let file_to_read_p = file_to_read_pb.as_path();
        import_book_from_file(file_to_read_p, file_to_generate_p)?;
    }
    Ok(())
}

pub fn import_book_from_file(in_path: &Path, out_path: &Path) -> Result<(), std::io::Error> {
    let input_file = OpenOptions::new()
        // readonly
        .read(true)
        .open(in_path)
        .expect("Unable to process input file.");
    let output_file = OpenOptions::new()
        // append
        .append(true)
        .create(true)
        .open(out_path)
        .expect("Unable to process output file.");
    let mut book_name = "dummy".to_string();
    writeln!(&output_file, "let book_meta_data = BookMetaData {{")?;
    let _nr_of_chapters_2: u16 = 0;
    let _nr_of_verses_2: u16 = 0;
    let reader = BufReader::new(input_file);
    for line in reader.lines() {
        let line = line.unwrap_or("Error reading the line".to_string());
        let ch = line.chars().next().unwrap();
        if ch == '\u{202B}' {
            // found mixed text (LTR and RTL)

            //remove asterisk(s)
            let line = line.replace("*", "");

            // remove annotations
            let annotation_re = Regex::new(r"\u{202A}\[.]\u{202C}").unwrap();
            let after = annotation_re.replace_all(&line, "");
            let stripped_line = after.clone();

            // retrieve: chapter number, verse number and the verse itself.
            let verse_re = Regex::new(r"^\u{202B}\u{00A0}(?P<verse_no>\d+)\u{00A0}*\u{05C3}(?P<chapter_no>\d+)\u{00A0}{1,3}(?P<verse>\p{Hebrew}.+)\u{202C}$").unwrap();
            match verse_re.captures(&stripped_line) {
                Some(caps) => {
                    let chapter_no = caps["chapter_no"].parse::<u16>().unwrap();
                    let verse_no = caps["verse_no"].parse::<u16>().unwrap();
                    writeln!(
                        &output_file,
                        "\nlet verse_ref = VerseReference {{
                        book_name: \"{}\".to_string(),
                        chapter_no: {},
                        verse_no: {},
                    }};",
                        book_name, chapter_no, verse_no
                    )?;
                    //let _ = println!(&output_file, "==> Verseref:{:?}", &verse_ref);
                    //let _ = println!(&output_file, "==> Added verse:{}", &caps["verse"]);
                    writeln!(
                        &output_file,
                        "tanach.add_verse(verse_ref, \"{}\".to_string());",
                        &caps["verse"]
                    )?;
                }
                None => {
                    writeln!(&output_file, "ERROR: Unable to add this verse:")?;
                    writeln!(&output_file, "Line = [{}]", &line.clone())?;
                }
            }
        } else {
            //let _ = println!(&output_file, "==> RTL text");
            // Find CHAPTER nr and number of VERSES per CHAPTER
            // Example: "xxxx  Chapter 1   (31 verses). "
            let chapter_re = Regex::new(r"^\u{202A}x{4}\s+Chapter\s(?P<chapter_no>\d+).*\((?P<verses_per_chapter>\d+)\sverses").unwrap();
            if let Some(caps) = chapter_re.captures(&line) {
                let chapter_no = caps["chapter_no"].parse::<u16>().unwrap();
                let verses_per_chapter = caps["verses_per_chapter"].parse::<u16>().unwrap();
                writeln!(
                    &output_file,
                    "tanach.add_verses_per_chapter(\"{}\".to_string(), {}, {});\n",
                    book_name, chapter_no, verses_per_chapter
                )?;
            }

            // Find UXLC version
            // Example: "xxxx    Unicode/XML Leningrad Codex [UXLC 8.9] "
            let uxlc_re = Regex::new(r"^\u{202A}+x{4}\s+.+\s(?P<uxlc_version>\d+\.\d+)]").unwrap();
            if let Some(caps) = uxlc_re.captures(&line) {
                let uxlc_v = &caps["uxlc_version"];
                writeln!(&output_file, "uxlc_version: \"{}\".to_string(),", uxlc_v)?;
            }

            // Find BUILD VERSION and BUILD DATE
            // Example: "xxxx    Build: 26.9    -    1 Apr 2023  00:00 "
            let build_re = Regex::new(r"(?P<build_version>\d+\.\d+).*(?P<build_date>\d+\s+[A-Za-z]{1,3}\s+\d{4}\s+\d{2}:\d{2})").unwrap();
            if let Some(caps) = build_re.captures(&line) {
                let bld_date = &caps["build_date"];
                let bld_version = &caps["build_version"];
                writeln!(&output_file, "build_date: \"{}\".to_string(),", bld_date)?;
                writeln!(
                    &output_file,
                    "build_version: \"{}\".to_string(),",
                    bld_version
                )?;
            }

            // Find 'BOOK_NAME' and 'nr of CHAPTERs' and 'nr of VERSEs'
            // Example: " xxxx    Genesis (50 chapters, 1533 verses). "
            let book_name_re = Regex::new(r"^\u{202A}x{4}\s{4}(?P<book_name>\d*\s*[\w+|\s]+)\s\((?P<nr_of_chapters>\d+)\schapters,\s(?P<nr_of_verses>\d+)\sverses").unwrap();
            //let book_name_re = Regex::new(r"^\u{202A}x{4}\s{4}(?P<endof>End\sof\s)?(?P<book_name>\d*\s*[\w+|\s]+)\s\((?P<nr_of_chapters>\d+)\schapters,\s(?P<nr_of_verses>\d+)\sverses").unwrap();
            if let Some(caps) = book_name_re.captures(&line) {
                let book_name_temp = caps["book_name"].to_string();
                if !book_name_temp.contains("End of ") {
                    book_name = book_name_temp;
                    let nr_of_chapters: u16 = caps["nr_of_chapters"].parse().unwrap();
                    let nr_of_verses: u16 = caps["nr_of_verses"].parse().unwrap();
                    writeln!(&output_file, "nr_of_chapters: {},", nr_of_chapters)?;
                    writeln!(&output_file, "nr_of_verses: {},", nr_of_verses)?;
                    writeln!(&output_file, "verses_per_chapter: HashMap::new(),")?;
                    writeln!(&output_file, "}};")?;
                    writeln!(
                        &output_file,
                        "tanach.add_book(\"{}\", &book_meta_data.clone());",
                        book_name
                    )?;
                }
            }
        }
    }
    Ok(())
}

pub fn insert_footer(output_file: &Path) -> Result<(), std::io::Error> {
    println!("Inserting a footer to the generated code ...");
    let output_file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(output_file)?;
    //.expect("  > Unable to insert the footer into the output file.");

    writeln!(
        &output_file,
        //"check_counters(&tanach);\n\
        "tanach\n\
        }}\n\
         // ++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++\n\
         //                     ===> END OF GENERATED CODE <===                  +\n\
         // ++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++\n",
    )?;
    Ok(())
}

pub fn format_generated_file(file_to_generate_p: &Path) -> Result<(), std::io::Error> {
    // Format generated code for readability
    println!("Formatting the generated code with [rustfmt] ...");
    Command::new("rustfmt").arg(file_to_generate_p).spawn()?;
    //.expect("failed to execute command");
    Ok(())
}

pub fn copy_generated_file(from: &str, to: &str) {
    println!("Copying the generated file to the 'swlc_core' of the 'swlc' package...");
    match fs::copy(from, to) {
        Ok(count) => {
            println!(
                "  > Copying succesfull, a total of {} bytes were copied.",
                count
            );
        }
        Err(error) => match error.kind() {
            ErrorKind::NotFound => {
                println!("Something went wrong: {}", ErrorKind::NotFound)
            }
            ErrorKind::PermissionDenied => {
                println!("Something went wrong: {}", ErrorKind::PermissionDenied)
            }
            _ => {
                panic!("Problem copying the generated file:");
            }
        },
    }
}
