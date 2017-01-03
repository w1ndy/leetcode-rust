use List::*;

enum List {
    Cons(i32, Box<List>),
    Nil,
}

impl List {
    fn stringify(&self) -> String {
        match *self {
            Cons(a, ref tail) => format!("{}, {}", a, tail.stringify()),
            Nil => format!("Nil")
        }
    }
}

fn dissolve_number(num: i32) -> List {
    match num {
        0 => Nil,
        n => Cons(n % 10, Box::new(dissolve_number(n / 10)))
    }
}

fn add_with_inc(a: &List, b: &List, inc: bool) -> List {
    match (a, b, inc) {
        (&Nil, &Nil, false) => Nil,
        (&Nil, &Nil, true) => Cons(1, Box::new(Nil)),
        (&Nil, &Cons(j, ref btail), false) => Cons(j, Box::new(add_with_inc(&Nil, btail, false))),
        (&Nil, &Cons(j, ref btail), true) => Cons((j + 1) % 10, Box::new(add_with_inc(&Nil, btail, (j + 1) >= 10))),
        (&Cons(i, ref atail), &Nil, false) => Cons(i, Box::new(add_with_inc(atail, &Nil, false))),
        (&Cons(i, ref atail), &Nil, true) => Cons((i + 1) % 10, Box::new(add_with_inc(atail, &Nil, (i + 1) >= 10))),
        (&Cons(i, ref atail), &Cons(j, ref btail), false) => Cons((i + j) % 10, Box::new(add_with_inc(atail, btail, (i + j) >= 10))),
        (&Cons(i, ref atail), &Cons(j, ref btail), true) => Cons((i + j + 1) % 10, Box::new(add_with_inc(atail, btail, (i + j + 1) >= 10)))
    }
}

fn add(a: &List, b: &List) -> List {
    add_with_inc(a, b, false)
}

fn main() {
    let (la, lb) = (dissolve_number(123), dissolve_number(223));
    let lsum = add(&la, &lb);
    println!("{}", lsum.stringify());
}

