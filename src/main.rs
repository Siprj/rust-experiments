#[derive(Debug, PartialEq, Eq)]
struct Bla {
    counter: u8,
}

impl Bla {
    fn add(&mut self) {
        self.counter += 5;
    }

    fn add2( bla: &mut Bla) {
        bla.counter += 5;
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Weather {
    Suny,
    Rainy
}

struct Omega {
    omega: u8
}

trait Window {
    fn look(&self) -> Weather;
}

impl Window for Bla {
    fn look(&self) -> Weather {
        Weather::Suny
    }
}

impl Window for Omega {
    fn look(&self) -> Weather {
        Weather::Rainy
    }
}

fn kwa<T>(v: T) where T: Window {
    println!("{:?}", v.look())
}

fn do_something(str: &str) {
    println!("{}", str);
}

fn main() {
    let mut bla = Bla {counter: 10};
    bla.add();
    Bla::add2(&mut bla);
    bla.add();



    println!("Hello, world! {}", bla.counter);

    let str = "blabla";
    do_something(&str[2..4]);
    kwa(bla);
    kwa(Omega{omega:10});
}
