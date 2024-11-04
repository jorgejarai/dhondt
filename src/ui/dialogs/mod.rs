pub mod add;
pub mod confirm;
pub mod edit;
pub mod results;
pub mod save;

use cursive::{views::Dialog, Cursive};

pub fn error_msgbox(s: &mut Cursive, message: &str) {
    s.add_layer(Dialog::text(message).title("Error").dismiss_button("OK"));
}

pub fn msgbox(s: &mut Cursive, title: &str, message: &str) {
    s.add_layer(Dialog::text(message).title(title).dismiss_button("OK"));
}
