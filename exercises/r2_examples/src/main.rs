

// #[derive(Debug)]
// struct Structure(i32);

// #[derive(Debug)]
// struct Deep(Structure);


// fn main() {
//     let name = String::from("xiao ming");
//     let a: u32  = 1;
//     println!("Hello, {:?}!", a);
//     println!("Hello, {:?}!", name);

//     println!("{} month in a year", 12);
//     println!("{1:?} {0:?} is the {actor:?} name.", "Slater","Christian", actor="actor's");

//     println!("Now {:?} will print!", Structure(3));

//     println!("Now {:?} will print!", Deep(Structure(8)))

// }



// #[derive(Debug)]
// struct Person<'a> {
//     name: &'a str,
//     age: u8
// }

// fn main(){
//     let name = "Peter";
//     let age = 27;
//     let peter = Person{name,age};
//     println!("{:#?}", peter);
// }

// use std::fmt;

// #[derive(Debug)]
// struct MinMax(i64, i64);

// impl fmt::Display for MinMax {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "{}, {}", self.0, self.1)
//     }
// }

// #[derive(Debug)]
// struct  Point2D {
//     x: f64,
//     y: f64,
// }

// impl fmt::Display for Point2D{
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result  {
//         write!(f, "x: {}, y:{}", self.x, self.y)
//     }
// }

// fn main(){
//     let minmax = MinMax(1,15);

//     println!("Compare structures");
//     println!("Display:{}", minmax);
//     println!("Debug: {:?}", minmax);

//     let big_range = MinMax(-300, 300);
//     let small_range = MinMax(-3, 3);

//     println!("The big range is {big} and the small is {small}", small = small_range, big = big_range);

//     let point = Point2D{x: 3.3, y: 8.1};
//     println!("Compare points:");
//     println!("Display: {}", point);
//     println!("Debug: {:?}", point);
// }


// #![allow(dead_code)]

// enum WebEvent{
//     PageLoad,
//     PageUnload,
//     KeyPress(char),
//     Click{x:i64, y: i64}
// }

// fn inspect(event: &WebEvent){
//     match event {
//         WebEvent::PageLoad => println!("page laoded"),
//         WebEvent::PageUnload => println!("page unloaded"),
//         _ => println!("Other")
//     }
// }

// impl WebEvent {
//     fn run(&self){
//         // let t = *self;
//         inspect(self);
//     }
// }

// fn main(){
//     let load = WebEvent::PageLoad;
//     let click = WebEvent::Click { x: 20, y: 20 };

//     inspect(&load);
//     inspect(&click);

//     let unload = WebEvent::PageUnload;
//     let load = WebEvent::PageLoad;
//     unload.run();
// }


// #[allow(dead_code)]

// enum Status {
//     Rich,
//     Poor
// }

// enum Work {
//     Civilian,
//     Soldier,
// }

// fn main(){
//     use Status::{Poor, Rich};
//     use Work::*;

//     let status = Poor;
//     match status {
//         Poor => println!("Poor"),
//         _ => println!(" ======")
//     }

//     let work = Soldier;
//     match work {
//         Civilian => println!("civilian"),
//         _ => println!(" ======")
//     }
// }

// use List::*;

// enum List {
//     Cons(u32, Box<List>),
//     Nil,
// }

// impl List{
//     fn new()-> List {Nil}

//     fn prepend(self, elem: u32)-> List{
//         Cons(elem, Box::new(self))
//     }

//     fn len(&self) -> u32{
//         match *self {
//             Cons(_, ref tail) => 1 + tail.len(),
//             Nil => 0,
//         }
//     }

//     fn stringify(&self) -> String {
//         match *self {
//             Cons(head, ref tail) => {
//                 format!("{}, {}", head, tail.stringify())
//             },
//             Nil => {
//                 format!("Nil")
//             }
//         }
//     }
// }

// fn main(){
//     let mut list = List::new();

//     list = list.prepend(1);
//     list = list.prepend(2);
//     list = list.prepend(3);

//     println!("linked list has length: {}", list.len());
//     println!("{}", list.stringify())
// }

// use std::fmt::{self, write};

// struct Circle{
//     radius: i32
// }

// impl fmt::Display for Circle {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
//         write!(f, "Circle of radius {}", self.radius)
//     }
// }

// fn main(){
//     let circle = Circle{radius: 6};
//     println!("{}", circle)
// }

// fn main(){
//     let mut c = 0;

//     let r = loop {
//         c += 1;

//         if c == 10 {
//             break c * 2;
//         }
//     };

//     assert_eq!(r , 20)
// }

// fn main(){
//     // for n in 1..100 {
//     //     println!("{}", n);
//     // }

//     // for n in 1..= 100 {
//     //     println!("={}", n)
//     // }

//     let mut names = vec!["Bob", "Frank", "Ferris"];

//     for name in names.iter_mut(){
//         match name {
//             &mut "Ferris" => println!("There is a restacean among us!"),
//             _ => println!("hello {}", name)
//         }
//     }
// }


// fn main(){
//     fn function (i: i32) -> i32 { i +1};

//     let colosure_annotated = |i: i32| -> i32 { i + 1};


//     let i = 1;

//     println!("function: {}", function(i));

//     println!("closure_annotated: {}", colosure_annotated(i));
//     // println!("closure_inferred: {}", colosure_inferred(i));

//     let one = || 3;

//     println!("closure returning one: {}", one());
// }

// fn main(){
//     use std::mem;

//     let color = String::from("green");

//     let print = || println!("color: {}", color);

//     // print();
//     let _reborrow = &color;
//     print();

//     // let _color_moved = color;

//     let mut count = 0;
//     let mut inc = || {
//         count += 1;
//         print!("color={}", _reborrow);
//     };

//     inc();
// }

// fn main(){
//     let haystack = vec![1,2,3];

//     let contains = move |needle| haystack.contains(needle);

//     println!("{}", contains(&1));
//     println!("{}", contains(&4));
// }


// fn apply<F>(f:F) where F: FnOnce(){
//     f();
// }

// fn apply_to_3<F>(f:F)-> i32 where  F: Fn(i32) -> i32 {
//     f(3)
// }

// fn main(){
//     use std::mem;

//     let greeting = "hello";
//     let mut fareWell = "goodbay".to_owned();

//     let diary = || {
//         println!("I said {}.", greeting);

//         fareWell.push_str("!!!!");

//         println!("Then I screamed {}.", fareWell);
        
//         mem::drop(fareWell);
//     };

//     apply(diary);

//     let double = |x| 2 * x;

//     println!("3 doubled:{}", apply_to_3(double));
// }


mod my_mod{

    fn private_function(){
        println!("called `my_mod::private_function()`");
    }

    pub fn function(){
        private_function();
        println!("called `my_mode::function()`");
    }


    pub mod nested{
        pub fn function(){
            println!("called `my_mod::nested::function()`")
        }

        #[allow(dead_code)]
        fn private_function(){
            println!("called 1my_mod::nested::private_function()")
        }

        pub(in crate::my_mod) fn public_function_in_my_mod(){
            println!("public_function_in_my_mod")
        }

        pub(self) fn public_function_self_in_my_mod(){
            println!("public_function_self_in_my_mod")
        }

        pub(super) fn public_function_super_in_my_mod(){
            println!("public_function_super_in_my_mod")
        }
    }

}

fn main(){
    my_mod::function();

    my_mod::nested::function();

    // my_mod::public_function_in_my_mod();
}