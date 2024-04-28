// Note: we allow some warnings since we want to show
//       how not to program and then improve the style
#[derive(Clone, Debug)]
struct Name<'a> {
    name: &'a str, // name of person
    id: u64,       // unique id
}

#[allow(clippy::needless_return)]
#[allow(clippy::needless_range_loop)]
// C-like error handling: return -1 on error
// or, return NULL pointer and C-like length
fn get_index_1(book: &[Name], n: usize, id: u64) -> i32 {
    // warning: needless_range_loop
    for i in 0..=n {
        if book[i].id == id {
            return i as i32;
        }
    }
    // warning: needless_return
    return -1; // return error value if ID is not found
}

#[allow(clippy::needless_range_loop)]
// C-like error handling: return -1 on error
// or, return NULL pointer and C-like length
fn get_index_2(book: &[Name], id: u64) -> i32 {
    let n = book.len();
    // warning: needless_range_loop
    for i in 0..=n {
        if book[i].id == id {
            return i as i32;
        }
    }
    -1 // return error value if ID is not found
}

// C-like error handling: return -1 on error
// or, return NULL pointer
fn get_index_3(book: &[Name], id: u64) -> i32 {
    for (i, value) in book.iter().enumerate() {
        if value.id == id {
            return i as i32;
        }
    }
    -1 // return error value if ID is not found
}


// Rust-like implementation
fn get_index<'a>(book: &'a [Name<'a>], id: u64) -> Option<(usize, Name<'a>)> {
    for (i, value) in book.iter().enumerate() {
        if value.id == id {
            return Some((i, value.clone()))
        }
    }
    None // return error value if ID is not found
}

fn main() {
    let address_book = [
        Name {
            name: "Alice",
            id: 1,
        },
        Name { name: "Bob", id: 2 },
        Name {
            name: "Charlie",
            id: 3,
        },
    ];
    let to_print = [3, 1];

    println!("Loop 1: Printing address book entries {to_print:?}");
    for id in to_print {
        let i = get_index_1(&address_book, address_book.len(), id) as usize;
        println!("Id {id}: Name={}", address_book[i].name);
    }
    let to_print = [3, 1];

    println!("Loop 2: Printing address book entries {to_print:?}");
    for id in to_print {
        let i = get_index_2(&address_book, id) as usize;
        println!("Id {id}: Name={name}", name=address_book[i].name);
    }

    // to crash:
    // let _i: i32 = get_index_2(&address_book, 4);

    // no crash using provided iterator
    let _i: i32 = get_index_3(&address_book, 4);

    // to crash loop3 - add non-existing entry
    let _to_print = [3, 1, 4];
    // and comment out next statement
    let to_print = [3, 1];

    println!("Loop 3: Printing address book entries {to_print:?}");
    for id in to_print {
        let i: usize = get_index_3(&address_book, id) as usize;
        println!("Id {id}: Name={:?}", address_book[i].name);
    }

    let to_print = [3, 1, 4];
    println!("Loop 4: Printing address book entries {to_print:?}");
    for id in to_print {
        if let Some((_i, v)) = get_index(&address_book, id) {
            println!("Id {id}: Name={:?}", v.name);
        } else {
            println!("Id {id} does not exists!");
        }
    }

}
