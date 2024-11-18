// fn largest_i(a: i32, b: i32) -> i32 {
//     a.max(b)
// }

// fn largest_f(a: f32, b: f32) -> f32 {
//     a.max(b)
// }

// fn largest_item(a: Item, b: Item) -> Item {
//     a.max(b)
// }

fn largest<T: PartialOrd>(a: T, b: T) -> T {
    if a > b {
        a
    } else {
        b
    }
}

#[derive(PartialEq, Debug, PartialOrd)]
struct Item {
    price: i32,
}

impl Item {
    fn new(price: i32) -> Self {
        Item { price: price }
    }
}

#[derive(Debug)]
struct Person<T, U> {
    age: T,
    name: U,
}

impl<T, U> Person<T, U> {
    fn new(age: T, name: U) -> Person<T, U> {
        Person { age: age, name: name }
    }

    fn add(&mut self, extra: T) {
        // self.age += extra;
    }
}

fn classic_example_stack() {
    #[derive(Debug)]
    struct Stack<T> {
        items: Vec<T>,
    }

    impl<T> Stack<T> {
        fn new() -> Stack<T> {
            Stack { items: Vec::new() }
        }
        fn push(&mut self, item: T) {
            self.items.push(item);
        }

        fn pop(&mut self) -> Option<T> {
            self.items.pop()
        }
    }

    let mut stack = Stack::<i32>::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);

    println!("My stack holds {:?}", stack);
    stack.pop();
    println!("My stack holds {:?}", stack);
}

fn main() {
    println!("Hello, world!");
    println!("{}", largest(1.3, 4.1));

    let item1 = Item::new(3);
    let item2 = Item::new(5);
    let res = largest(item1, item2);

    println!("{:?}", res);

    let mut p = Person::new(5, "Bob");
    // p.add(10);
    let mut p2 = Person::new("ten", 1.0 / 3.0);

    println!("{:?}, {:?}", p, p2);

    classic_example_stack();
}
