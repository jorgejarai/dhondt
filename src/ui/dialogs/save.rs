use std::{fs::File, io::Write, path::Path};

use cursive::{
    event::Key,
    view::{Nameable, Resizable},
    views::{Dialog, EditView, LinearLayout, OnEventView, TextView},
    Cursive,
};

use crate::ui::read_input;

use super::{error_msgbox, msgbox};

pub fn save_to_file(path: &str, contents: &str) -> std::io::Result<()> {
    let mut file = File::create(path)?;
    file.write_all(contents.as_bytes())?;

    Ok(())
}

pub fn save_dialog(s: &mut Cursive, results: String) {
    s.add_layer(
        OnEventView::new(
            Dialog::around(
                LinearLayout::horizontal()
                    .child(TextView::new("Path: "))
                    .child(EditView::new().with_name("save_path").fixed_width(25)),
            )
            .title("Save to file")
            .dismiss_button("Cancel")
            .button("Save", move |s| {
                let path = read_input(s, "save_path").unwrap();

                if path.trim().is_empty() {
                    error_msgbox(s, "Path must not be empty");
                    return;
                }

                if Path::exists(Path::new(path.as_str())) {
                    s.add_layer(
                        Dialog::text("File already exists. Do you want to overwrite?")
                            .button("Yes", {
                                let results = results.clone();

                                move |s| {
                                    s.pop_layer();
                                    match save_to_file(path.as_str(), &results.clone()) {
                                        Ok(_) => {
                                            s.pop_layer();
                                            msgbox(s, "Save to file", "File saved successfully");
                                        }
                                        Err(err) => error_msgbox(
                                            s,
                                            format!("Could not save results: {}", err).as_str(),
                                        ),
                                    }
                                }
                            })
                            .button("No", |s| {
                                s.pop_layer();
                            }),
                    );
                    return;
                }

                match save_to_file(path.as_str(), &results.clone()) {
                    Ok(_) => {
                        s.pop_layer();
                        msgbox(s, "Save to file", "File saved successfully");
                    }
                    Err(_) => error_msgbox(s, "Could not save results"),
                }
            }),
        )
        .on_event(Key::Esc, |s| {
            s.pop_layer();
        }),
    );
}
