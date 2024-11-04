use clipboard::{ClipboardContext, ClipboardProvider};
use cursive::{
    event::Key,
    view::Resizable,
    views::{Dialog, OnEventView, ScrollView, TextView},
    Cursive,
};
use cursive_table_view::TableView;
use std::collections::HashMap;

use crate::{
    dhondt::{self, DHondtError, PartyCandidate},
    numeric::format_num,
    ui::{
        read_input,
        table_columns::{PartyResults, PartyResultsColumn},
        validation::validate_number,
    },
};

use super::{error_msgbox, msgbox, save::save_dialog};

pub fn start_calculation(s: &mut Cursive) {
    let seat_count = read_input(s, "seat_count").unwrap();
    let Ok(seat_count) = validate_number(s, seat_count.as_str()) else {
        return;
    };

    let mut parties: HashMap<PartyCandidate, u32> = HashMap::new();

    s.call_on_name(
        "votes_table",
        |table: &mut TableView<PartyResults, PartyResultsColumn>| {
            table.borrow_items().iter().for_each(|p| {
                parties.insert((p.party.clone(), p.name.clone()), p.votes);
            });
        },
    )
    .unwrap();

    match dhondt::calculate(seat_count, &parties) {
        Ok(results) => show(s, seat_count, parties, results),
        Err(err) => error_msgbox(
            s,
            match err {
                DHondtError::NoParties => "Please input some parties first",
                DHondtError::NoVotes => "No parties have any votes",
                DHondtError::ZeroSeats => "Can't distribute zero seats",
            },
        ),
    };
}

pub fn generate_report(
    seat_count: u32,
    parties: &HashMap<PartyCandidate, u32>,
    results: &HashMap<String, Vec<String>>,
) -> String {
    let mut output = String::new();

    output.push_str(&format!("Seats to assign: {seat_count}\n\n"));

    let party_totals: HashMap<String, u32> = parties.iter().fold(HashMap::new(), |mut acc, p| {
        *acc.entry(p.0 .0.clone()).or_insert(0) += p.1;
        acc
    });
    let mut party_totals_sorted: Vec<(String, u32)> =
        party_totals.iter().map(|p| (p.0.clone(), *p.1)).collect();
    party_totals_sorted.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    let mut lines: Vec<(String, String, String, String)> = vec![];

    let votes_total: u32 = parties.iter().fold(0, |acc, p| acc + p.1);
    let votes_total_formatted = format_num(votes_total);

    lines.push((
        "Total of votes:".into(),
        votes_total_formatted,
        String::new(),
        String::new(),
    ));
    lines.push((String::new(), String::new(), String::new(), String::new()));

    for party in &party_totals_sorted {
        let party_name = party.0.clone();
        let votes = party.1;
        let percentage = votes as f32 / votes_total as f32 * 100.0;
        let seats = results.get(&party_name).unwrap_or(&vec![]).len();

        lines.push((
            format!("{party_name}:"),
            format_num(votes),
            format!("{percentage:.2}%"),
            format!("{seats}"),
        ));

        let mut party_candidates: Vec<(String, u32)> = parties
            .iter()
            .filter(|c| c.0 .0 == party_name)
            .map(|c| (c.0 .1.clone(), *c.1))
            .collect();
        party_candidates.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        for (i, candidate) in party_candidates.iter().enumerate() {
            let name = candidate.0.clone();
            let votes = candidate.1;
            let percentage = votes as f32 / votes_total as f32 * 100.0;
            let elected = results
                .get(&party_name)
                .unwrap_or(&vec![])
                .iter()
                .any(|n| n == &name);

            let elected_sym = if !elected
                && party_candidates[0..i].iter().any(|c| {
                    c.1 == votes
                        && results
                            .get(&party_name)
                            .unwrap_or(&vec![])
                            .iter()
                            .any(|n| n == &c.0)
                }) {
                "?"
            } else if elected {
                "✓"
            } else {
                " "
            };

            lines.push((
                format!("  - {name}"),
                format_num(votes),
                format!("{percentage:.2}%"),
                elected_sym.into(),
            ));
        }

        lines.push((String::new(), String::new(), String::new(), String::new()));
    }

    let mut max_widths = (0, 0, 0, 0);

    for (col1, col2, col3, col4) in &lines {
        max_widths.0 = max_widths.0.max(col1.len());
        max_widths.1 = max_widths.1.max(col2.len());
        max_widths.2 = max_widths.2.max(col3.len());
        max_widths.3 = max_widths.3.max(col4.len());
    }

    for (col1, col2, col3, col4) in &lines {
        output.push_str(&format!(
            "{:<width1$}    {:>width2$}    {:>width3$}    {:>width4$}\n",
            col1,
            col2,
            col3,
            col4,
            width1 = max_widths.0,
            width2 = max_widths.1,
            width3 = max_widths.2,
            width4 = max_widths.3,
        ));
    }

    output
}

pub fn show(
    s: &mut Cursive,
    seat_count: u32,
    parties: HashMap<PartyCandidate, u32>,
    results: HashMap<String, Vec<String>>,
) {
    let report_text = generate_report(seat_count, &parties, &results);

    s.add_layer(
        OnEventView::new(
            Dialog::around(ScrollView::new(TextView::new(report_text.clone())))
                .title("Seat distribution")
                .dismiss_button("OK")
                .button("Copy to clipboard", {
                    let report_text = report_text.clone();

                    move |s| {
                        let mut ctx: ClipboardContext = match ClipboardProvider::new() {
                            Ok(ctx) => ctx,
                            Err(_) => {
                                error_msgbox(s, "Couldn't access clipboard");
                                return;
                            }
                        };

                        match ctx.set_contents(report_text.clone()) {
                            Ok(_) => msgbox(s, "Copy to clipboard", "Results copied to clipboard"),
                            Err(_) => error_msgbox(s, "Couldn't copy to clipboard"),
                        };
                    }
                })
                .button("Save", {
                    let report_text = report_text.clone();

                    move |s| {
                        save_dialog(s, report_text.clone());
                    }
                })
                .min_width(70),
        )
        .on_event(Key::Esc, |s| {
            s.pop_layer();
        }),
    );
}
