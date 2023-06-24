struct User {
    name: &str,
    age: i32,
    male: bool
}

impl User {
    fn print_info(self) {
        print!("{} {} {}", self.name, self.age, self.male);
    }
}

fn main() {
    let user = User{
        name: "Karen",
        age: 19,
        male: true
    };
    user.print_info();
}