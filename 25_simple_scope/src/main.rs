// returns the value of variable v
// value 10 is bound with identifier v
// We call v a variable v 
fn variant_1() -> u32 {
    let v=1_0;
    return v
}

// function returns the value of last statement
fn variant_1a() -> u32 {
    let v=1_0_;
    v
}

// function returns the value of last statement
fn variant_1b() -> u32 {
    let v=1__0;
    print_type_of("v", &v);
    v
}

// function returns no value since last statement 
// is empty
fn variant_1c() {
    let v=1_0;
    v;
}

// this is equivalent to returning ()
fn variant_1d() {
    let v=1_0;
    v;
    ()
}

fn variant_2a() -> u32 {
    let v=1_0;
    let v=1_1;
    let v=1_2;
    v
}

fn variant_2() -> u32 {
    let v=10;
    {
        let v = 1_1;
    }
    v
}

fn variant_3() -> u32 {
    let v=10;
    {
        let v = 1_1;
        return v
    }
    v
}

fn variant_4() -> u32 {
    let v=10;
    {
        let v = 1_1;
        v;
    }
    v
}

fn variant_5() -> u32 {
    let mut v = 1_0;
    v= {
        let v = 1_1;
        v
    };
    v
}

fn variant_5b() -> u32 {
    let mut v = 1_0;
    v= {
        let v = 1_1;
        v
    };
    v
}


fn variant_6() -> u32 {
    let mut v = 1_0;
    v= {
        let v = 1_1;
        return v
    };
    let v = 1_2;
    v
}

struct Scope {
    scope: u32,
    vid: u32,
}
// prints "Dropping variable id_{scope}_{vid}" 
// when variable is dropped

fn new_id(scope: u32, vid: u32) -> Scope {
    println!("new variable: id_{scope}_{vid}");
    Scope{ scope, vid }
}

impl Drop for Scope {
    fn drop(&mut self) {
        println!("Dropping variable id_{}_{}", self.scope, self.vid);
    }
}

fn print_type_of<T>(identifier: &str, _: &T) {
    println!("The type of '{identifier}' is '{}'", std::any::type_name::<T>())
}


fn variant_7() {

    let v = new_id(1,1); // scope 0, instance 1
    let v = new_id(1,2); // scope 0, instance 2
    let v = new_id(1,2); // scope 0, instance 3
}

fn variant_2b() {
    let v = new_id(1,1); // scope 0, instance 1
    let v = new_id(1,2); // scope 0, instance 2
    let v = new_id(1,3); // scope 0, instance 3
}


fn main() {
    let v = 1_1;
    println!("A: {v}");
    {
        println!("B: {v}");
        let v = 2_2;
        println!("C: {v}");
        {
            println!("D: {v}");
            let v = 3_2;
            println!("E: {v}");
        }
        println!("F: {v}");
    }
    println!("G: {v}");
    let v = 1_4; 
    println!("H: {v}");
}

fn variant_3() {
    let v = new_id(1,1); // scope 0, instance 1
    {
        let v = new_id(2,2); // scope 0, instance 2
        {
            let v = new_id(3,3); // scope 0, instance 2
        }
    }
    let v = new_id(1,4); // scope 0, instance 3
}

fn main() {
    let my_variable=1_0;
    print_type_of("my_variable", &my_variable);
    println!("Output variant_1: {}!", variant_1());
    println!("Output variant_1a: {}!", variant_1a());
    variant_1b();
    variant_1c();
    println!("Output variant_2: {}!", variant_2());
    println!("Output variant_2a: {}!", variant_2a());
    println!("Output variant_3: {}!", variant_3());
    println!("Output variant_4: {}!", variant_4());
    println!("Output variant_5: {}!", variant_5());
    println!("Output variant_6: {}!", variant_6());
    variant_7();
}
