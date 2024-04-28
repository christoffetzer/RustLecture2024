struct Scope {
    scope: u32,
    vid: u32,
}

fn new_id(scope: u32, vid: u32) -> Scope {
    println!("new variable: id_{scope}_{vid}");
    Scope{ scope, vid }
}

impl Drop for Scope {
    fn drop(&mut self) {
        println!("Dropping variable id_{}_{}", self.scope, self.vid);
    }
}

struct Ephemeral<'a> {
    id: &'a str,
}

impl Drop for Ephemeral<'_> {
    fn drop(&mut self) {
        println!("Dropping Ephemeral {}!", self.id);
    }
}

fn main() {
    println!("scope 0");
    new_id(0,1);
    let _id0_2 = new_id(0,2);
    let _id0_2 = new_id(0,2);
    {
        println!("scope 1");
        let _id1_1 = new_id(1,1);
        {
            println!("scope 2");
            let _id2_1 = new_id(2,1);
            println!("end scope 2");
        }
        let _id1_2 = new_id(1,2);
        println!("end scope 1");
    }
    let _id0_2 = new_id(0,2);
    println!("End of program");
}

pub fn lifetime() {
    println!("scope 0");
    let _id0 = Ephemeral{ id: "0_1" };
    new_id(0,1);
    {
        println!("scope 1");
        let _id1 = Ephemeral{ id: "1_1" };
        {
            println!("scope 2");
            let _id2 = Ephemeral{ id: "2_1" };
            println!("end scope 2");
        }
        println!("id3");
        let _id3 = Ephemeral{ id: "1_1" };
        println!("end scope 1");
    }
    let _id4 = Ephemeral{ id: "0_2" };
    println!("End of program");
}
