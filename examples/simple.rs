use tasques::{Tasque, TasqueManager};

fn main() {
    /*
    let tm = TasqueManager::new();

    tm.on_start(|ctx| {
        // Do things
        // td.signal("aaa");
    });

    tm.on_signal("aaa", |ctx| {
        println!("AAA something happened");
    });
     */

    let mut tm = TasqueManager::new();

    tm.register(Tasque::on_start(|| println!("Hello, world!")));
    tm.register(Tasque::requires("oobleck", || println!("hi")));

    tm.run();
}
