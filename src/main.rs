enum Shell {
    Hard,
    Soft
}

struct Taco {
    shell: Shell,
    has_meat: bool 
}

impl Taco {
    fn cook(&self) -> String {
        return String::from("CANYOUSMELLWHATTHEROCKISCOOKING?");
    }
}

fn main() {
    let my_taco: Taco = Taco {
        shell: Shell::Hard,
        has_meat: true
    };

    if my_taco.has_meat {
        println!("MEATY!");
    }

    println!("{}", my_taco.cook());
}
