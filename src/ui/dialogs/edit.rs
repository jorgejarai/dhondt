use std::collections::HashMap;

use cursive::{
    event::Key,
    view::{Nameable, Resizable},
    views::{Dialog, EditView, LinearLayout, OnEventView, TextView},
    Cursive,
};
use cursive_table_view::TableView;

use crate::ui::{
    read_input,
    table_columns::{PartyResults, PartyResultsColumn},
    validation::validate_number,
};

use super::{confirm::confirm_delete, error_msgbox};

pub fn edit_party(s: &mut Cursive, index: usize) {
    let party = read_input(s, "edit_party").unwrap();
    let name = read_input(s, "edit_name").unwrap();

    if party.trim().is_empty() {
        error_msgbox(s, "Please provide a party name");
        return;
    }

    if name.trim().is_empty() {
        error_msgbox(s, "Please provide a name");
        return;
    }

    let mut parties: HashMap<String, u32> = HashMap::new();
    s.call_on_name(
        "votes_table",
        |table: &mut TableView<PartyResults, PartyResultsColumn>| {
            table.borrow_items().iter().for_each(|p| {
                parties.insert(p.party.clone(), p.votes);
            });
        },
    )
    .unwrap();

    let prev_name = s
        .call_on_name(
            "votes_table",
            move |table: &mut TableView<PartyResults, PartyResultsColumn>| {
                (
                    table.borrow_item(index).unwrap().clone().party,
                    table.borrow_item(index).unwrap().clone().name,
                )
            },
        )
        .unwrap();

    let mut is_name_repeated = false;
    if prev_name.0 != party.trim() || prev_name.1 != name.trim() {
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
    }

    if is_name_repeated {
        error_msgbox(s, "Candidate has already been entered");
        return;
    }

    let votes = read_input(s, "edit_votes").unwrap();
    let votes = validate_number(s, votes.as_str());
    if let Ok(votes) = votes {
        s.call_on_name(
            "votes_table",
            move |table: &mut TableView<PartyResults, PartyResultsColumn>| {
                let row = table.borrow_item_mut(index).unwrap();
                row.party = party.trim().into();
                row.name = name.trim().into();
                row.votes = votes;
            },
        )
        .unwrap();
    }

    s.pop_layer();
}

pub fn edit_party_dialog(s: &mut Cursive, _row: usize, index: usize) {
    s.add_layer(
        OnEventView::new(
            Dialog::around(
                LinearLayout::vertical()
                    .child(
                        LinearLayout::horizontal()
                            .child(TextView::new("Party: "))
                            .child(EditView::new().with_name("edit_party").fixed_width(26)),
                    )
                    .child(
                        LinearLayout::horizontal()
                            .child(TextView::new("Name:  "))
                            .child(EditView::new().with_name("edit_name").fixed_width(26)),
                    )
                    .child(
                        OnEventView::new(
                            LinearLayout::horizontal()
                                .child(TextView::new("Votes: "))
                                .child(EditView::new().with_name("edit_votes").fixed_width(9)),
                        )
                        .on_event(Key::Enter, move |s: &mut Cursive| {
                            edit_party(s, index);
                        }),
                    ),
            )
            .title("Edit party")
            .button("OK", move |s: &mut Cursive| {
                edit_party(s, index);
            })
            .dismiss_button("Cancel")
            .button("Delete", move |s: &mut Cursive| {
                confirm_delete(s, index);
            }),
        )
        .on_event(Key::Esc, |s| {
            s.pop_layer();
        })
        .on_event(Key::Enter, move |s: &mut Cursive| {
            edit_party(s, index);
        }),
    );

    let value = s
        .call_on_name(
            "votes_table",
            move |table: &mut TableView<PartyResults, PartyResultsColumn>| {
                table.borrow_item(index).unwrap().clone()
            },
        )
        .unwrap();

    s.call_on_name("edit_party", |edit: &mut EditView| {
        edit.set_content(value.party);
    });

    s.call_on_name("edit_name", |edit: &mut EditView| {
        edit.set_content(value.name);
    });

    s.call_on_name("edit_votes", |edit: &mut EditView| {
        edit.set_content(value.votes.to_string());
    });
}
