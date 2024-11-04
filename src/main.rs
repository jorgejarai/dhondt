pub mod csv_parser;
pub mod dhondt;
pub mod numeric;
pub mod ui;

use std::{collections::HashMap, process::exit};

use clap::{CommandFactory, Parser};
use cursive::{
    align::HAlign,
    event::{Event, Key},
    view::{Nameable, Resizable},
    views::{Dialog, EditView, LinearLayout, OnEventView, PaddedView, TextView},
};
use cursive_table_view::TableView;
use dhondt::{DHondtError, PartyCandidate};

use crate::csv_parser::parse_file;
use crate::ui::table_columns::{PartyResults, PartyResultsColumn};
use ui::dialogs::{
    add::new_party_dialog,
    confirm::{confirm_clear, confirm_quit},
    edit::edit_party_dialog,
    results::{generate_report, start_calculation},
    save::save_to_file,
};

/// D'Hondt calculator
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Initial value for seat count
    #[arg(short)]
    count: Option<u32>,

    /// Path to CSV file with initial data
    #[arg(short)]
    data: Option<String>,

    /// Save output to file (uses stdout if no path provided)
    #[arg(short)]
    output: Option<Option<String>>,
}

fn main() {
    let args = Args::parse();

    if args.output.is_some() && (args.count.is_none() || args.data.is_none()) {
        let mut cmd = Args::command();
        cmd.error(
            clap::error::ErrorKind::MissingRequiredArgument,
            "-c and -d are required when using -o",
        )
        .exit();
    }

    let initial_data: Vec<PartyResults> = match args.data {
        Some(ref csv_path) => match parse_file(csv_path.as_str()) {
            Ok(data) => data,
            Err(err) => {
                eprintln!("error: couldn't parse CSV file: {err}");
                exit(1);
            }
        },
        None => Vec::new(),
    };

    if args.output.is_some() {
        let seat_count = args.count.unwrap();

        let mut parties: HashMap<PartyCandidate, u32> = HashMap::new();
        initial_data.iter().for_each(|p| {
            parties.insert((p.party.clone(), p.name.clone()), p.votes);
        });

        match dhondt::calculate(seat_count, &parties) {
            Ok(results) => {
                let output = generate_report(seat_count, &parties, &results);
                let output_path = args.output.unwrap();

                if output_path.is_none() {
                    println!("{}", output);
                } else {
                    let output_path = output_path.unwrap();
                    if let Err(err) = save_to_file(&output_path, &output) {
                        eprintln!("error: couldn't save results to file: {err}");
                        exit(1);
                    }

                    return;
                }
            }
            Err(err) => {
                eprintln!(
                    "error: {}",
                    match err {
                        DHondtError::NoParties => "please input some parties first",
                        DHondtError::NoVotes => "no parties have any votes",
                        DHondtError::ZeroSeats => "can't distribute zero seats",
                    },
                );

                exit(1);
            }
        };

        return;
    }

    let mut s = cursive::default();
    let mut table = TableView::<PartyResults, PartyResultsColumn>::new()
        .column(PartyResultsColumn::Party, "Party", |c| c)
        .column(PartyResultsColumn::Name, "Name", |c| c)
        .column(PartyResultsColumn::Votes, "Votes", |c| {
            c.width(11).align(HAlign::Right)
        });

    table.set_on_submit(edit_party_dialog);

    s.add_layer(
        OnEventView::new(
            Dialog::around(
                LinearLayout::vertical()
                    .child(
                        LinearLayout::horizontal()
                            .child(TextView::new("Seats to assign: "))
                            .child(EditView::new().with_name("seat_count").fixed_width(5)),
                    )
                    .child(PaddedView::lrtb(
                        0,
                        0,
                        1,
                        0,
                        table.with_name("votes_table").min_size((75, 20)),
                    )),
            )
            .title("Party results")
            .button("Add candidate", new_party_dialog)
            .button("Calculate", start_calculation)
            .button("Clear", confirm_clear),
        )
        .on_event(Key::Esc, confirm_quit)
        .on_event(Event::CtrlChar('s'), start_calculation),
    );

    if !initial_data.is_empty() {
        s.call_on_name(
            "votes_table",
            move |table: &mut TableView<PartyResults, PartyResultsColumn>| {
                table.set_items(initial_data);
            },
        )
        .unwrap();
    }

    if let Some(val) = args.count {
        s.call_on_name("seat_count", |view: &mut EditView| {
            view.set_content(val.to_string());
        });
    }

    s.run();
}
