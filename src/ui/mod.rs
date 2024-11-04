use cursive::{views::EditView, Cursive};

pub mod dialogs;
pub mod table_columns;
pub mod validation;

pub fn read_input(s: &mut Cursive, name: &str) -> Option<String> {
    let value = s.call_on_name(name, |view: &mut EditView| view.get_content())?;

    Some(value.to_string())
}
