
fn main() {

    // 1st example 
    #[allow(dead_code)]
    #[derive(Debug,Clone)]
    struct SomeData {
        id: u64,
    }
    let first_owner = SomeData { id: 1 };
    println!("first_owner = {first_owner:?}");
    let new_owner = first_owner;
    println!("new_owner ={new_owner:?}");
    // the following fails - if uncommented
    // println!("first_owner = {first_owner:?}");

    // 2nd example 
    #[derive(Debug,Clone,Copy)]
    struct SomeOtherData {
        id: u64,
    }

    let first_owner = SomeOtherData { id: 1 };
    println!("first_owner = {first_owner:?}");
    let mut owner_of_copy = first_owner; // copy!
    println!("owner_of_copy ={owner_of_copy:?}");
    println!("first_owner = {first_owner:?}");
    owner_of_copy.id = 2;
    println!("owner_of_copy ={owner_of_copy:?}");
    println!("first_owner = {first_owner:?}");

    // 3rd example 
    let first_owner = SomeData { id: 1 };
    println!("first_owner = {first_owner:?}");
    let borrower = &first_owner;
    println!("first_owner = {first_owner:?}");
    println!("borrower ={borrower:?}");

    let MyVariable=1_0;
    println!("MyVariable ={MyVariable}");

}
