use tasques::{Tasque, TasqueManager};

fn main() {
    let mut tm = TasqueManager::new();

    tm.register(
        Tasque::new(String::from("do_thing"), || {
            println!("Hello, world!");
        })
        .on_start(),
    );

    tm.register(
        Tasque::new(String::from("another_thing"), || {
            println!("YES IT WORKED");
        })
        .requires(String::from("do_thing"))
    );

    tm.run_with(4);
}
