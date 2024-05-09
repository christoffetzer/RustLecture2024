fn apply<F>(func: F, arg: i32) -> i32
where
    F: Fn(i32) -> i32,
{
    func(arg)
}

fn apply_1(func: Fi32, arg: i32) -> i32 {
    func(arg)
}

fn square(x: i32) -> i32 {
    x * x
}

type Fi32 = fn(i32) -> i32;

fn function_return(selector: i32) -> Fi32 {
    match selector {
        1..=3 => |a| a + 1,
        4..=5 => |a| a + 2,
        6..=7 => |a| a + 3,
        _ => |a| a,
    }
}

fn closure_return(increment: i32) -> Box<dyn Fn(i32) -> i32> {
    Box::new(move |a| a + increment)
}

fn main() {
    let result = apply(square, 5);
    println!("The result is: {}", result);
    let result = apply(|a| a + 1, 5);
    println!("The result is: {}", result);
    let result = apply_1(|a| a + 3, 5);
    println!("The result is: {}", result);
    println!(
        "The result for function_return(3)(2)= {}",
        function_return(3)(2)
    );
    println!(
        "The result for function_return(5)(2)= {}",
        function_return(5)(2)
    );

    println!(
        "The result for closure_return(5)(7)= {}",
        closure_return(5)(7)
    );
}
