use cursive::{
    event::{Event, Key},
    view::{Nameable, Resizable},
    views::{Dialog, EditView, LinearLayout, OnEventView, TextView},
    Cursive,
};
use cursive_table_view::TableView;
use std::sync::Mutex;

use crate::ui::{
    read_input,
    table_columns::{PartyResults, PartyResultsColumn},
    validation::validate_number,
};

use super::error_msgbox;

static LAST_PARTY_NAME: Mutex<String> = Mutex::new(String::new());

pub fn add_party(s: &mut Cursive) {
    let party = read_input(s, "party").unwrap();
    let name = read_input(s, "name").unwrap();

    if party.trim().is_empty() {
        error_msgbox(s, "Please provide a party name");
        return;
    }

    if name.trim().is_empty() {
        error_msgbox(s, "Please provide a name");
        return;
    }

    let mut is_name_repeated = false;
    s.call_on_name(
        "votes_table",
        |table: &mut TableView<PartyResults, PartyResultsColumn>| {
            is_name_repeated = table
                .borrow_items()
                .iter()
                .any(|c| c.name == name.trim() && c.party == party.trim());
        },
    )
    .unwrap();

    if is_name_repeated {
        error_msgbox(s, "Candidate has already been entered");
        return;
    }

    let votes = read_input(s, "votes").unwrap();
    let votes = validate_number(s, votes.as_str());
    if let Ok(votes) = votes {
        s.call_on_name(
            "votes_table",
            |table: &mut TableView<PartyResults, PartyResultsColumn>| {
                table.insert_item(PartyResults {
                    party: party.trim().into(),
                    name: name.trim().into(),
                    votes,
                });
            },
        )
        .unwrap();

        *LAST_PARTY_NAME.lock().unwrap() = party.trim().into();
    }

    s.pop_layer();
}

pub fn new_party_dialog(s: &mut Cursive) {
    s.add_layer(
        OnEventView::new(
            Dialog::around(
                LinearLayout::vertical()
                    .child(
                        LinearLayout::horizontal()
                            .child(TextView::new("Party: "))
                            .child(EditView::new().with_name("party").fixed_width(25)),
                    )
                    .child(
                        LinearLayout::horizontal()
                            .child(TextView::new("Name:  "))
                            .child(EditView::new().with_name("name").fixed_width(25)),
                    )
                    .child(
                        OnEventView::new(
                            LinearLayout::horizontal()
                                .child(TextView::new("Votes: "))
                                .child(EditView::new().with_name("votes").fixed_width(9)),
                        )
                        .on_event(Key::Enter, add_party),
                    ),
            )
            .title("Add candidate")
            .button("OK", add_party)
            .dismiss_button("Cancel"),
        )
        .on_event(Key::Esc, |s| {
            s.pop_layer();
        })
        .on_event(Event::Ctrl(Key::Enter), add_party),
    );

    let _ = s.call_on_name("party", |edit: &mut EditView| {
        edit.set_content(LAST_PARTY_NAME.lock().unwrap().clone());
    });
}
