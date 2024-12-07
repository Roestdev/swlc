use crate::config::*;
use crate::model_definition::*;
use std::process::exit;

pub fn check_counters(tanach: &Tanach) {
    for book in tanach.iter() {
        if book.book_meta_data.nr_of_chapters != book.book_meta_data_counted.nr_of_chapters_counted
        {
            println!(
                "Please check your input file for book: <{}>",
                book.book_name
            );
            println!("The number given for the total number of chapters seems incorrect.");
            println!(
                "Expected: {}, actual read: {}.",
                book.book_meta_data.nr_of_chapters,
                book.book_meta_data_counted.nr_of_chapters_counted
            );
            exit(exitcode::DATAERR);
        }
        if book.book_meta_data.nr_of_verses != book.book_meta_data_counted.nr_of_verses_counted {
            println!(
                "Please check your input file for book: <{}>",
                book.book_name
            );
            println!("The number given for the total number of verses seems incorrect.");
            println!(
                "Expected: {}, actual read: {}.",
                book.book_meta_data.nr_of_verses, book.book_meta_data_counted.nr_of_verses_counted
            );
            exit(exitcode::DATAERR);
        }
        // compare two hashmaps
        if book
            .book_meta_data
            .verses_per_chapter
            .ne(&book.book_meta_data_counted.verses_per_chapter_counted)
        {
            println!(
                "Please check your input file for book: <{}>",
                book.book_name
            );
            println!("One or more of the given numbers for verses per chapter seems incorrect.");
            println!(
                "Expected: {:?}, actual read: {:?}.",
                book.book_meta_data.verses_per_chapter,
                book.book_meta_data_counted.verses_per_chapter_counted
            );
            exit(exitcode::DATAERR);
        }
    }
}

pub fn reorder_books(tanach: &mut Tanach, book_order: &BookOrder) {
    let books_order = get_book_order(book_order);
    if tanach.books.len() == books_order.len() {
        for (index, item) in books_order.into_iter().enumerate() {
            let book_item = tanach.books.remove(
                tanach
                    .books
                    .iter()
                    .position(|x| x.book_name == item)
                    .expect("Book not found (used in <fn reorder_books()>)"),
            );
            tanach.books.insert(index, book_item);
        }
    } else {
        println!("In function reorder_books():");
        println!(
            "  Number of books found in tanach: {}\n  Defined number of books: {}",
            tanach.books.len(),
            books_order.len()
        );
        println!("Exiting program");
        exit(exitcode::DATAERR);
    }
}

/*
fn get_index_for_reference(
    tanach: Tanach,
    needle: VerseReference,
) -> Vec<usize> {
    tanach
        .books
        .into_iter()
        .flat_map(|b| {
            b.references_and_verses
                .into_iter()
                .position(|f| {
                    f.0.book_name == needle.book_name
                        && f.0.chapter_no == needle.chapter_no
                        && f.0.verse_no == needle.verse_no
                })
        })
        .collect()
}*/

/*fn get_index_of_all_verses(tanach: Tanach) -> Vec<(usize, (VerseReference, String))> {
    // assume we have a month: Month which is filled
    tanach
        .books
        .into_iter()
        .flat_map(|b| {
            b.references_and_verses.into_iter()
            // .filter(|b| {
            //     //r.0.book_name == "Haggai".to_string() && r.0.chapter_no == 1 && r.0.verse_no == 2
            //     b.0.book_name == needle.book_name && b.0.chapter_no == needle.chapter_no && b.0.verse_no == needle.verse_no
            // })
        })
        .enumerate()
        .collect()
}*/

/*fn get_index_for_all_lines_in_all_books(tanach: Tanach) -> Vec<(usize,(VerseReference,String))> {
    // assume we have a month: Month which is filled
    tanach
        .books
        .into_iter()
        .flat_map(|v| {
           v.references_and_verses
               .into_iter()
        })
        .enumerate()
        .collect()
}
*/
pub fn get_index_for_verse_reference(
    tanach: &Tanach,
    verse_reference: &VerseReference,
) -> Option<usize> {
    <Vec<Book> as Clone>::clone(&tanach.books)
        //tanach.books
        .into_iter()
        .flat_map(|b| b.references_and_verses.into_iter())
        .position(|v| {
            v.0.book_name == verse_reference.book_name
                && v.0.chapter_no == verse_reference.chapter_no
                && v.0.verse_no == verse_reference.verse_no
        })
}

fn add_index_to_verse_reference(
    tanach: &Tanach,
    verse_ref: &VerseReference,
) -> Vec<(usize, (VerseReference, String))> {
    // assume we have a month: Month which is filled
    <Vec<Book> as Clone>::clone(&tanach.books)
        .into_iter()
        .flat_map(|w| {
            w.references_and_verses.into_iter().filter(|r| {
                //r.0.book_name == "Haggai".to_string() && r.0.chapter_no == 1 && r.0.verse_no == 2
                r.0.book_name == verse_ref.book_name
                    && r.0.chapter_no == verse_ref.chapter_no
                    && r.0.verse_no == verse_ref.verse_no
            })
        })
        .enumerate()
        .collect()
}
