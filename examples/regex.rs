#[macro_use]
extern crate bson;
#[macro_use]
extern crate serde;

extern crate mongodb;
extern crate mongodb_cursor_pagination;

use bson::Bson;
use mongodb::{options::FindOptions, Client};
use mongodb_cursor_pagination::{CursorDirections, FindResult, PaginatedCursor};

#[derive(Debug, Deserialize)]
pub struct MyFruit {
    name: String,
    how_many: i32,
}

fn main() {
    let client =
        Client::with_uri_str("mongodb://localhost:27017/").expect("Failed to initialize client.");
    let db = client.database("mongodb_cursor_pagination");

    let docs = vec![
        doc! { "name": "Apple", "how_many": 2, "spanish": "Manzana" },
        doc! { "name": "Avocado", "how_many": 5, "spanish": "Aguacate" },
        doc! { "name": "Orange", "how_many": 3, "spanish": "Naranja" },
        doc! { "name": "Bananas", "how_many": 10, "spanish": "Bananas" },
        doc! { "name": "Blackberry", "how_many": 12, "spanish": "Mora" },
        doc! { "name": "Blueberry", "how_many": 10, "spanish": "Arandano" },
        doc! { "name": "Lingonberry", "how_many": 5, "spanish": "Arandano roja" },
        doc! { "name": "Raspberry", "how_many": 5, "spanish": "Frambuesa" },
        doc! { "name": "Strawberry", "how_many": 5, "spanish": "Fresa" },
        doc! { "name": "Grapes", "how_many": 12, "spanish": "Uvas" },
    ];

    // should result in...
    // Orange     | 3
    // Avocado    | 5
    // Apple      | 5
    // Blueberry  | 10
    // Bananas    | 10
    // Grapes     | 12
    // Blackberry | 12

    db.collection("myfruits")
        .insert_many(docs, None)
        .expect("Unable to insert data");

    // query page 1, 2 at a time
    let mut options = create_options(2, 0);
    let filter = doc! { "$or": [
        { "name": Bson::RegExp(String::from("berry"), String::from("i")) },
        { "spanish": Bson::RegExp(String::from("ana"), String::from("i")) },
    ] };
    let mut find_results: FindResult<MyFruit> = PaginatedCursor::new(Some(options), None, None)
        .find(&db.collection("myfruits"), Some(&filter))
        .expect("Unable to find data");
    print_details("First page", &find_results);

    // get the second page
    options = create_options(2, 0);
    let mut cursor = find_results.page_info.next_cursor;
    find_results = PaginatedCursor::new(Some(options), cursor, Some(CursorDirections::Next))
        .find(&db.collection("myfruits"), Some(&filter))
        .expect("Unable to find data");
    print_details("Second page", &find_results);

    // get previous page
    options = create_options(2, 0);
    cursor = find_results.page_info.start_cursor;
    find_results = PaginatedCursor::new(Some(options), cursor, Some(CursorDirections::Previous))
        .find(&db.collection("myfruits"), Some(&filter))
        .expect("Unable to find data");
    print_details("Previous (first) page", &find_results);

    // get the second page again
    options = create_options(2, 0);
    cursor = find_results.page_info.next_cursor;
    find_results = PaginatedCursor::new(Some(options), cursor, Some(CursorDirections::Next))
        .find(&db.collection("myfruits"), Some(&filter))
        .expect("Unable to find data");
    print_details("Second page (again)", &find_results);

    // get the third page
    options = create_options(2, 0);
    cursor = find_results.page_info.next_cursor;
    find_results = PaginatedCursor::new(Some(options), cursor, Some(CursorDirections::Next))
        .find(&db.collection("myfruits"), Some(&filter))
        .expect("Unable to find data");
    print_details("Third page", &find_results);

    // get previous page
    options = create_options(2, 0);
    cursor = find_results.page_info.start_cursor;
    find_results = PaginatedCursor::new(Some(options), cursor, Some(CursorDirections::Previous))
        .find(&db.collection("myfruits"), Some(&filter))
        .expect("Unable to find data");
    print_details("Previous (second) page", &find_results);

    db.collection("myfruits")
        .drop(None)
        .expect("Unable to drop collection");
}

fn create_options(limit: i64, skip: i64) -> FindOptions {
    FindOptions::builder()
        .limit(limit)
        .skip(skip)
        .sort(doc! { "name": 1 })
        .build()
}

fn print_details(name: &str, find_results: &FindResult<MyFruit>) {
    println!(
        "{}:\nitems: {:?}\ntotal: {}\nnext: {:?}\nprevious: {:?}\nhas_previous: {}\nhas_next: {}",
        name,
        find_results.items,
        find_results.total_count,
        find_results.page_info.next_cursor,
        find_results.page_info.start_cursor,
        find_results.page_info.has_previous_page,
        find_results.page_info.has_next_page,
    );
    println!("-----------------");
}
