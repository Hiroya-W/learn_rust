use std::thread;
use std::time::Duration;

use console::Term;

fn main() {
    let term = Term::stdout();
    let _ = term.write_line("Hello World");
    thread::sleep(Duration::from_millis(2000));
    let _ = term.clear_line();
}
