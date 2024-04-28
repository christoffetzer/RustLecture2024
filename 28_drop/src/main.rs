

struct Scope {
    scope: u32,
    vid: u32,
}

fn new_id(scope: u32, vid: u32) -> Scope {
    println!("new variable: id_{scope}_{vid}");
    Scope{ scope, vid }
}

// prints "Dropping variable id_{scope}_{vid}" 
// when variable is dropped

impl Drop for Scope {
    fn drop(&mut self) {
        println!("Dropping variable id_{}_{}", self.scope, self.vid);
    }
}

fn main() { 
      let s = new_id(0,0); 
         takes_ownership(s); 
       println!("Bye"); // won't compile if added 
     } 
       
     fn takes_ownership(some_string: Scope) { 
         println!("{}", some_string.vid); 
      } 
impl<T> Stack<T> for LLStack<T> {
    fn new() -> Self {
        LLStack::<T> { head: None }
    }

    fn push(&mut self, element: T) {
        let new_node = Box::new(Node {
            element: element,
            next: self.head.take(),
        });

        self.head = Some(new_node);
    }

    fn pop(&mut self) -> Option<T> {…}

    fn peek(&self) -> Option<&T> {…}

}