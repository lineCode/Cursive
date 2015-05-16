extern crate cursive;

use cursive::Cursive;
use cursive::view::TextView;

fn main() {
    let mut siv = Cursive::new();

    // We can quit by pressing q
    siv.add_global_callback('q' as i32, |s,_| s.quit());

    siv.add_layer(TextView::new("Hello World!\nPress q to quit the application."));

    siv.add_global_callback('a' as i32, |s, _| {
        //
    });

    siv.run();
}