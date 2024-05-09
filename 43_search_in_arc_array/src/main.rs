use std::sync::Arc;

#[derive(Clone, Debug)]
struct Name {
    name: String, // using an owned string - instead of a borrowed &str
    id: u64,      // unique id
}

fn show_borrowing() {
    let v = Name {
        name: "Alice".to_string(),
        id: 1,
    };
    let rv = &v;
    println!("rv={rv:?}")
}

fn get_entry(book: &[Name], id: u64) -> Option<Name> {
    book.iter().find(|value| value.id == id).cloned()
}

#[allow(clippy::manual_find)]
// No cloning, no index, no lifetimes
fn get_reference_1(book: &[Arc<Name>], id: u64) -> Option<&Arc<Name>> {
    for value in book.iter() {
        if value.id == id {
            return Some(value);
        }
    }
    None // return error value if ID is not found
}

// Using functional programming
fn get_reference_2(book: &[Arc<Name>], id: u64) -> Option<&Arc<Name>> {
    book.iter().find(|value| value.id == id)
}

// Return an owned object
fn get_reference_3(book: &[Arc<Name>], id: u64) -> Option<Arc<Name>> {
    book.iter()
        .find(|value: &&Arc<Name>| value.id == id)
        .cloned()
}

fn init_address_book() -> Vec<Arc<Name>> {
    vec![
        Arc::new(Name {
            name: "Alice".to_string(),
            id: 1,
        }),
        Arc::new(Name {
            name: "Bob".to_string(),
            id: 2,
        }),
        Arc::new(Name {
            name: "Charlie".to_string(),
            id: 3,
        }),
    ]
}

fn init_book() -> Vec<Name> {
    vec![
        Name {
            name: "Alice".to_string(),
            id: 1,
        },
        Name {
            name: "Bob".to_string(),
            id: 2,
        },
        Name {
            name: "Charlie".to_string(),
            id: 3,
        },
    ]
}

fn main() {
    let id_3: Option<&Arc<Name>>;
    let id_1: Option<Arc<Name>>;
    {
        let address_book = init_address_book();
        let to_print = [3, 1, 4];
        println!("Printing address book entries {to_print:?}");
        for id in to_print {
            if let Some(v) = get_reference_2(&address_book, id) {
                println!("Id {id}: Name={:?}", v.name);
            } else {
                println!("Id {id} does not exists");
            }
        }
        id_3 = get_reference_1(&address_book, 3);
        println!("id_3 = {id_3:?}"); // does not work after scope
        id_1 = get_reference_3(&address_book, 1);
    }
    let book = init_book();
    println!("id=1 => result={:?}", get_entry(&book, 1));

    let r = book.iter().map(|value| value.id).collect::<Vec<_>>();
    println!("Map Result: {:?}", r);

    let r = book
        .iter()
        .map(|value| value.name.clone())
        .collect::<Vec<_>>();
    println!("Map Result: {:?}", r);

    println!(
        "Map Result: {:?}",
        book.iter()
            .map(|value| { format!("{}-{}", value.name, value.id) })
            .collect::<Vec<_>>()
    );

    // problem: cannot use id_3 here..

    println!("id_1 = {id_1:?}"); // does not work after scope
    show_borrowing();
}
