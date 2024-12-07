use core::ops::Deref;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::process::exit;

#[derive(Debug, Default, Clone)]
pub struct Tanach {
    pub nr_of_books_counted: u16,
    pub nr_of_books_expected: u16,
    pub books: Vec<Book>, // order of the books is fixed
                          // TODO add type of notes in hash<string, String> ,e.g. m, patach ipv meteg oid
}
impl Tanach {
    pub fn new() -> Self {
        Default::default()
    }
    pub fn add_book(&mut self, book_name: impl Into<String>, book_meta_data: &BookMetaData) {
        let book_name = book_name.into();
        if !self.books.iter().any(|book| book.book_name == book_name) {
            //println!("Book will be added");
            self.nr_of_books_counted += 1;
            self.books
                .push(Book::new(book_name, book_meta_data.clone()));
        } else {
            println!(
                "\n\nError: \n\tYou are trying to add the book [{}] for the second time!",
                book_name
            );
            println!("\tPlease check your input files for duplicates (names and content) !");
            exit(exitcode::DATAERR);
        }
    }
    pub fn add_verses_per_chapter(
        &mut self,
        book_name: String,
        chapter_number: u16,
        verses_per_chapter: u16,
    ) {
        let book_index = self
            .books
            .iter()
            .position(|book| book.book_name == book_name);
        let book_index = match book_index {
            Some(i) => i,
            _ => {
                println!(
                    "\n\nError: \n\tYou are trying to add chapter number [{}] to the book [{}], ",
                    chapter_number, book_name
                );
                println!("\tbut the book [{}] is not processed yet.", book_name);
                println!("\tPlease check your input files for duplicates (names and content)!");
                exit(exitcode::DATAERR);
            }
        };
        self.books[book_index]
            .book_meta_data
            .verses_per_chapter
            .insert(chapter_number, verses_per_chapter);
        self.books[book_index]
            .book_meta_data_counted
            .nr_of_chapters_counted += 1;
    }

    pub fn add_verse(&mut self, vers_ref: VerseReference, verse: String) {
        let book_index = self
            .books
            .iter()
            .position(|book| book.book_name == vers_ref.book_name);
        let book_index = match book_index {
            Some(i) => i,
            _ => {
                println!(
                    "\tbut the book [{}] is not processed yet.",
                    vers_ref.book_name
                );
                println!("\tPlease check your input files for duplicates (names and content)!");
                exit(exitcode::DATAERR);
            }
        };
        self.books[book_index]
            .references_and_verses
            .push((vers_ref.clone(), verse.clone()));

        //  if self.books[book_index] //////////////////////////////
        //      .book_meta_data_counted
        //      .verses_per_chapter_counted
        //      .contains_key(&vers_ref.chapter_no)
        //  {
        //      // println!("verses_per_chapter_counted::KEY  FOUND");
        //      *self.books[book_index]
        //          .book_meta_data_counted
        //          .verses_per_chapter_counted
        //          .get_mut(&vers_ref.chapter_no)
        //          .unwrap() += 1;
        // } else {
        //      // println!("verses_per_chapter_counted::KEY NOT  FOUND");
        //      self.books[book_index]
        //          .book_meta_data_counted
        //          .verses_per_chapter_counted
        //          .insert(vers_ref.chapter_no, 1);
        //  } ////////////////////////////////////////////

        if let std::collections::hash_map::Entry::Vacant(e) = self.books[book_index]
            .book_meta_data_counted
            .verses_per_chapter_counted
            .entry(vers_ref.chapter_no)
        {
            // println!("verses_per_chapter_counted::KEY NOT  FOUND");
            e.insert(1);
        } else {
            // println!("verses_per_chapter_counted::KEY  FOUND");
            *self.books[book_index]
                .book_meta_data_counted
                .verses_per_chapter_counted
                .get_mut(&vers_ref.chapter_no)
                .unwrap() += 1;
        }
        self.books[book_index]
            .book_meta_data_counted
            .nr_of_verses_counted += 1;
    }
}

impl Deref for Tanach {
    type Target = Vec<Book>;
    fn deref(&self) -> &Self::Target {
        &self.books
    }
}
#[derive(Debug, Default, Clone)]
pub struct Book {
    pub book_name: String,
    pub book_meta_data: BookMetaData,
    pub book_meta_data_counted: BookMetaDataCounted,
    pub references_and_verses: Vec<(VerseReference, String)>,
}
impl Book {
    pub fn new(book_name: impl Into<String>, book_meta_data: BookMetaData) -> Self {
        let book_name = book_name.into();
        Self {
            book_name,
            book_meta_data,
            book_meta_data_counted: BookMetaDataCounted::new(),
            references_and_verses: vec![],
        }
    }
}
impl Deref for Book {
    type Target = Vec<(VerseReference, String)>;
    fn deref(&self) -> &Self::Target {
        &self.references_and_verses
    }
}

#[derive(Debug, Default, Clone)]
pub struct BookMetaData {
    pub uxlc_version: String, // Unicode/XML Leningrad Codex
    pub build_version: String,
    pub build_date: String,
    pub nr_of_chapters: u16,
    pub nr_of_verses: u16,
    pub verses_per_chapter: HashMap<u16, u16>,
    // TODO add field for notes like: [m],[X] etc
    // TODO may be two hashes ? one for the verse_positions, verse and note
}

impl BookMetaData {
    pub fn new() -> Self {
        Default::default()
    }
}
#[derive(Debug, Default, Clone)]
pub struct BookMetaDataCounted {
    pub nr_of_chapters_counted: u16, // Chapter 1
    pub nr_of_verses_counted: u16,   //chapterverses
    pub verses_per_chapter_counted: HashMap<u16, u16>,
}
impl BookMetaDataCounted {
    pub fn new() -> Self {
        Default::default()
    }
}

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct VerseReference {
    pub book_name: String,
    pub chapter_no: u16,
    pub verse_no: u16,
}
impl VerseReference {
    pub fn new(book_name: String, chapter_no: u16, verse_no: u16) -> Self {
        Self {
            book_name,
            chapter_no,
            verse_no,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn import_tanach() {
        let mut tanach = Tanach {
            nr_of_books_expected: 2,
            ..Default::default()
        };
        assert_eq!(tanach.nr_of_books_expected, 2);
        assert_eq!(tanach.nr_of_books_counted, 0);
        assert_eq!(tanach.books.len(), 0);

        //adding book: Genesis
        let book_meta_data = BookMetaData {
            uxlc_version: "1.7".to_string(),
            build_version: "26.8".to_string(),
            build_date: "7 Dec 2022  09:20".to_string(),
            nr_of_chapters: 50,
            nr_of_verses: 1533,
            verses_per_chapter: HashMap::new(),
        };
        tanach.add_book("Genesis", &book_meta_data);
        assert_eq!(tanach.nr_of_books_expected, 2);
        assert_eq!(tanach.nr_of_books_counted, 1);
        assert_eq!(tanach.books.len(), 1);
        assert_eq!(tanach.books[0].book_name, "Genesis");
        assert_eq!(tanach.books[0].book_meta_data.uxlc_version, "1.7");
        assert_eq!(tanach.books[0].book_meta_data.build_version, "26.8");
        assert_eq!(
            tanach.books[0].book_meta_data.build_date,
            "7 Dec 2022  09:20"
        );
        assert_eq!(tanach.books[0].book_meta_data.nr_of_chapters, 50);
        assert_eq!(tanach.books[0].book_meta_data.nr_of_verses, 1533);
        assert_eq!(
            tanach.books[0]
                .book_meta_data_counted
                .nr_of_chapters_counted,
            0
        );
        assert_eq!(
            tanach.books[0].book_meta_data_counted.nr_of_verses_counted,
            0
        );
        assert_eq!(
            tanach.books[0]
                .book_meta_data_counted
                .verses_per_chapter_counted
                .len(),
            0
        );
        assert_eq!(tanach.books[0].references_and_verses.len(), 0);

        //adding: verses/chapter to book "Genesis"
        tanach.add_verses_per_chapter("Genesis".to_string(), 1, 44);
        tanach.add_verses_per_chapter("Genesis".to_string(), 2, 77);
        tanach.add_verses_per_chapter("Genesis".to_string(), 3, 88);
        assert_eq!(tanach.books.len(), 1);
        assert_eq!(tanach.books[0].book_name, "Genesis");
        assert_eq!(tanach.books[0].book_meta_data.nr_of_chapters, 50);
        assert_eq!(tanach.books[0].book_meta_data.nr_of_verses, 1533);
        assert_eq!(tanach.books[0].book_meta_data.verses_per_chapter[&1], 44);
        assert_eq!(tanach.books[0].book_meta_data.verses_per_chapter[&2], 77);
        assert_eq!(tanach.books[0].book_meta_data.verses_per_chapter[&3], 88);
        assert_eq!(
            tanach.books[0].book_meta_data.verses_per_chapter.get(&4),
            None
        );
        assert_eq!(
            tanach.books[0]
                .book_meta_data_counted
                .nr_of_chapters_counted,
            3
        );
        assert_eq!(
            tanach.books[0].book_meta_data_counted.nr_of_verses_counted,
            0
        );
        assert_eq!(
            tanach.books[0]
                .book_meta_data_counted
                .verses_per_chapter_counted
                .len(),
            0
        );
        assert_eq!(
            tanach.books[0]
                .book_meta_data_counted
                .verses_per_chapter_counted
                .get(&1),
            None
        );
        assert_eq!(tanach.books[0].references_and_verses.len(), 0);

        // adding: 1 verse to book "Genesis"
        let verse_ref = VerseReference::new("Genesis".to_string(), 1, 10);
        tanach.add_verse(verse_ref, "בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃".to_string());
        assert_eq!(tanach.books[0].references_and_verses.len(), 1);
        assert_eq!(tanach.books[0].book_meta_data.nr_of_chapters, 50);
        assert_eq!(tanach.books[0].book_meta_data.nr_of_verses, 1533);
        assert_eq!(tanach.books[0].book_meta_data.verses_per_chapter[&1], 44);
        assert_eq!(tanach.books[0].book_meta_data.verses_per_chapter[&2], 77);
        assert_eq!(tanach.books[0].book_meta_data.verses_per_chapter[&3], 88);
        assert_eq!(
            tanach.books[0]
                .book_meta_data_counted
                .nr_of_chapters_counted,
            3
        );
        assert_eq!(
            tanach.books[0].book_meta_data_counted.nr_of_verses_counted,
            1
        );
        assert_eq!(
            tanach.books[0]
                .book_meta_data_counted
                .verses_per_chapter_counted
                .len(),
            1
        );
        assert_eq!(
            tanach.books[0]
                .book_meta_data_counted
                .verses_per_chapter_counted
                .get(&1),
            Some(&1)
        );
        assert_eq!(
            tanach.books[0].references_and_verses[0].0.book_name,
            "Genesis"
        );
        assert_eq!(tanach.books[0].references_and_verses[0].0.chapter_no, 1);
        assert_eq!(tanach.books[0].references_and_verses[0].0.verse_no, 10);
        assert_eq!(
            tanach.books[0].references_and_verses[0].1,
            "בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃"
        );

        //adding book: Exodus
        let book_meta_data = BookMetaData {
            uxlc_version: "1.78".to_string(),
            build_version: "236.8".to_string(),
            build_date: "8 Jan 2002  00:00".to_string(),
            nr_of_chapters: 58,
            nr_of_verses: 153,
            verses_per_chapter: HashMap::new(),
        };
        tanach.add_book("Exodus", &book_meta_data);
        assert_eq!(tanach.nr_of_books_expected, 2);
        assert_eq!(tanach.nr_of_books_counted, 2);
        assert_eq!(tanach.books.len(), 2);
        assert_eq!(tanach.books[1].book_name, "Exodus");
        assert_eq!(tanach.books[1].book_meta_data.uxlc_version, "1.78");
        assert_eq!(tanach.books[1].book_meta_data.build_version, "236.8");
        assert_eq!(
            tanach.books[1].book_meta_data.build_date,
            "8 Jan 2002  00:00"
        );
        assert_eq!(tanach.books[1].book_meta_data.nr_of_chapters, 58);
        assert_eq!(tanach.books[1].book_meta_data.nr_of_verses, 153);
        assert_eq!(
            tanach.books[1]
                .book_meta_data_counted
                .nr_of_chapters_counted,
            0
        );
        assert_eq!(
            tanach.books[1].book_meta_data_counted.nr_of_verses_counted,
            0
        );
        assert_eq!(
            tanach.books[1]
                .book_meta_data_counted
                .verses_per_chapter_counted
                .len(),
            0
        );
        assert_eq!(tanach.books[1].references_and_verses.len(), 0);

        //adding: verses/chapter to book "Exodus"
        tanach.add_verses_per_chapter("Exodus".to_string(), 1, 4);
        tanach.add_verses_per_chapter("Exodus".to_string(), 2, 7);
        tanach.add_verses_per_chapter("Exodus".to_string(), 3, 8);
        tanach.add_verses_per_chapter("Exodus".to_string(), 10, 98);
        assert_eq!(tanach.books.len(), 2);
        assert_eq!(tanach.books[1].book_name, "Exodus");
        assert_eq!(tanach.books[1].book_meta_data.nr_of_chapters, 58);
        assert_eq!(tanach.books[1].book_meta_data.nr_of_verses, 153);
        assert_eq!(tanach.books[1].book_meta_data.verses_per_chapter[&1], 4);
        assert_eq!(tanach.books[1].book_meta_data.verses_per_chapter[&2], 7);
        assert_eq!(tanach.books[1].book_meta_data.verses_per_chapter[&3], 8);
        assert_eq!(
            tanach.books[1].book_meta_data.verses_per_chapter.get(&4),
            None
        );
        assert_eq!(
            tanach.books[1].book_meta_data.verses_per_chapter.get(&5),
            None
        );
        assert_eq!(
            tanach.books[1].book_meta_data.verses_per_chapter.get(&6),
            None
        );
        assert_eq!(
            tanach.books[1].book_meta_data.verses_per_chapter.get(&7),
            None
        );
        assert_eq!(
            tanach.books[1].book_meta_data.verses_per_chapter.get(&8),
            None
        );
        assert_eq!(
            tanach.books[1].book_meta_data.verses_per_chapter.get(&9),
            None
        );
        assert_eq!(tanach.books[1].book_meta_data.verses_per_chapter[&10], 98);
        assert_eq!(
            tanach.books[1].book_meta_data.verses_per_chapter.get(&11),
            None
        );
        assert_eq!(
            tanach.books[1]
                .book_meta_data_counted
                .nr_of_chapters_counted,
            4
        );
        assert_eq!(
            tanach.books[1].book_meta_data_counted.nr_of_verses_counted,
            0
        );
        assert_eq!(
            tanach.books[1]
                .book_meta_data_counted
                .verses_per_chapter_counted
                .len(),
            0
        );
        assert_eq!(
            tanach.books[1]
                .book_meta_data_counted
                .verses_per_chapter_counted
                .get(&1),
            None
        );
        assert_eq!(tanach.books[1].references_and_verses.len(), 0);

        // adding: verses to book "Exodus"
        let mut verse_ref = VerseReference::new("Exodus".to_string(), 10, 1230);
        tanach.add_verse(verse_ref.clone(), "בְּרֵאשִׁ֖ית בָּרָ֣א וְאֵ֥ת הָאָֽרֶץ׃".to_string());
        verse_ref.chapter_no = 23;
        verse_ref.verse_no = 234;
        tanach.add_verse(verse_ref.clone(), "בְּרֵאשִׁ֖ית בָּרָ֣א וְאֵ֥ת ׃".to_string());
        verse_ref.verse_no = 237;
        tanach.add_verse(verse_ref.clone(), "בְּרֵאשִׁ֖ית בָּרָ֣א וְאֵ֥בְּרֵאשִׁ֖יתת ׃".to_string());
        assert_eq!(tanach.books.len(), 2);
        assert_eq!(tanach.books[1].book_name, "Exodus");
        assert_eq!(tanach.books[1].book_meta_data.nr_of_chapters, 58);
        assert_eq!(tanach.books[1].book_meta_data.nr_of_verses, 153);
        assert_eq!(tanach.books[1].book_meta_data.verses_per_chapter[&1], 4);
        assert_eq!(tanach.books[1].book_meta_data.verses_per_chapter[&2], 7);
        assert_eq!(tanach.books[1].book_meta_data.verses_per_chapter[&3], 8);
        assert_eq!(
            tanach.books[1].book_meta_data.verses_per_chapter.get(&4),
            None
        );
        assert_eq!(
            tanach.books[1].book_meta_data.verses_per_chapter.get(&5),
            None
        );
        assert_eq!(
            tanach.books[1].book_meta_data.verses_per_chapter.get(&6),
            None
        );
        assert_eq!(
            tanach.books[1].book_meta_data.verses_per_chapter.get(&7),
            None
        );
        assert_eq!(
            tanach.books[1].book_meta_data.verses_per_chapter.get(&8),
            None
        );
        assert_eq!(
            tanach.books[1].book_meta_data.verses_per_chapter.get(&9),
            None
        );
        assert_eq!(tanach.books[1].book_meta_data.verses_per_chapter[&10], 98);
        assert_eq!(
            tanach.books[1].book_meta_data.verses_per_chapter.get(&11),
            None
        );
        assert_eq!(
            tanach.books[1]
                .book_meta_data_counted
                .nr_of_chapters_counted,
            4
        );
        assert_eq!(
            tanach.books[1].book_meta_data_counted.nr_of_verses_counted,
            3
        );
        assert_eq!(
            tanach.books[1]
                .book_meta_data_counted
                .verses_per_chapter_counted
                .len(),
            2
        );
        assert_eq!(
            tanach.books[1]
                .book_meta_data_counted
                .verses_per_chapter_counted
                .get(&10),
            Some(&1)
        );
        assert_eq!(
            tanach.books[1]
                .book_meta_data_counted
                .verses_per_chapter_counted
                .get(&23),
            Some(&2)
        );
        assert_eq!(tanach.books[1].references_and_verses.len(), 3);
        assert_eq!(
            tanach.books[1].references_and_verses[0].0.book_name,
            "Exodus"
        );
        assert_eq!(tanach.books[1].references_and_verses[0].0.chapter_no, 10);
        assert_eq!(tanach.books[1].references_and_verses[0].0.verse_no, 1230);
        assert_eq!(
            tanach.books[1].references_and_verses[0].1,
            "בְּרֵאשִׁ֖ית בָּרָ֣א וְאֵ֥ת הָאָֽרֶץ׃"
        );
        assert_eq!(
            tanach.books[1].references_and_verses[1].0.book_name,
            "Exodus"
        );
        assert_eq!(tanach.books[1].references_and_verses[1].0.chapter_no, 23);
        assert_eq!(tanach.books[1].references_and_verses[1].0.verse_no, 234);
        assert_eq!(
            tanach.books[1].references_and_verses[1].1,
            "בְּרֵאשִׁ֖ית בָּרָ֣א וְאֵ֥ת ׃"
        );
        assert_eq!(
            tanach.books[1].references_and_verses[2].0.book_name,
            "Exodus"
        );
        assert_eq!(tanach.books[1].references_and_verses[2].0.chapter_no, 23);
        assert_eq!(tanach.books[1].references_and_verses[2].0.verse_no, 237);
        assert_eq!(
            tanach.books[1].references_and_verses[2].1,
            "בְּרֵאשִׁ֖ית בָּרָ֣א וְאֵ֥בְּרֵאשִׁ֖יתת ׃"
        );
    }
}
