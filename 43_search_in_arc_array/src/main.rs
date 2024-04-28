use std::sync::Arc;

#[derive(Clone, Debug)]
struct Name {
    name: String,  // let's keep an owned string
    id: u64,       // unique id
}

// No cloning, no index, no lifetimes
fn get_reference(book: &[Arc<Name>], id: u64) -> Option<&Arc<Name>> {
    for value in book.iter() {
        if value.id == id {
            return Some(value)
        }
    }
    None // return error value if ID is not found
}

fn main() {
    let address_book = [
        Arc::new(Name {
            name: "Alice".to_string(),
            id: 1,
        }),
        Arc::new(Name { name: "Bob".to_string(), id: 2 }),
        Arc::new(Name {
            name: "Charlie".to_string(),
            id: 3,
        }),
    ];
    let to_print = [3, 1, 4];
    println!("Printing address book entries {to_print:?}");
    for id in to_print {
        if let Some(v) = get_reference(&address_book, id) {
            println!("Id {id}: Name={:?}", v.name);
        } else {
            println!("Id {id} does not exists!");
        }
    }
}
