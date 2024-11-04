use cursive::{
    views::{Dialog, EditView},
    Cursive,
};
use cursive_table_view::TableView;

use crate::ui::table_columns::{PartyResults, PartyResultsColumn};

pub fn confirm_quit(s: &mut Cursive) {
    let is_votes_table_empty = s
        .call_on_name(
            "votes_table",
            move |table: &mut TableView<PartyResults, PartyResultsColumn>| {
                table.borrow_items().is_empty()
            },
        )
        .unwrap();
    let is_seat_count_empty = s
        .call_on_name("seat_count", |view: &mut EditView| view.get_content())
        .unwrap()
        .to_string()
        .trim()
        .is_empty();

    if is_votes_table_empty && is_seat_count_empty {
        s.quit();
    }

    s.add_layer(
        Dialog::text("Do you really want to quit?")
            .title("Confirm quit")
            .dismiss_button("No")
            .button("Yes", |s| {
                s.quit();
            }),
    );
}

pub fn confirm_clear(s: &mut Cursive) {
    let is_votes_table_empty = s
        .call_on_name(
            "votes_table",
            move |table: &mut TableView<PartyResults, PartyResultsColumn>| {
                table.borrow_items().is_empty()
            },
        )
        .unwrap();
    let is_seat_count_empty = s
        .call_on_name("seat_count", |view: &mut EditView| view.get_content())
        .unwrap()
        .to_string()
        .trim()
        .is_empty();

    if is_votes_table_empty && is_seat_count_empty {
        return;
    }

    s.add_layer(
        Dialog::text("Do you really want to clear all data?")
            .title("Confirm clear")
            .dismiss_button("No")
            .button("Yes", |s| {
                s.call_on_name(
                    "votes_table",
                    move |table: &mut TableView<PartyResults, PartyResultsColumn>| {
                        table.clear();
                    },
                )
                .unwrap();
                s.call_on_name("seat_count", |view: &mut EditView| {
                    view.set_content("");
                })
                .unwrap();
                s.pop_layer();
            }),
    );
}

pub fn confirm_delete(s: &mut Cursive, index: usize) {
    s.add_layer(
        Dialog::text("Do you want to remove this candidate?")
            .title("Confirm")
            .dismiss_button("No")
            .button("Yes", move |s| {
                s.call_on_name(
                    "votes_table",
                    |table: &mut TableView<PartyResults, PartyResultsColumn>| {
                        table.remove_item(index);
                    },
                )
                .unwrap();
                s.pop_layer();
                s.pop_layer();
            }),
    );
}
