use cursive_table_view::TableViewItem;
use std::cmp::Ordering;

use crate::numeric::format_num;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum PartyResultsColumn {
    Party,
    Name,
    Votes,
}

#[derive(Clone, Debug, serde::Deserialize)]
pub struct PartyResults {
    pub party: String,
    pub name: String,
    pub votes: u32,
}

impl TableViewItem<PartyResultsColumn> for PartyResults {
    fn to_column(&self, column: PartyResultsColumn) -> String {
        match column {
            PartyResultsColumn::Party => self.party.clone(),
            PartyResultsColumn::Name => self.name.clone(),
            PartyResultsColumn::Votes => format_num(self.votes),
        }
    }

    fn cmp(&self, other: &Self, column: PartyResultsColumn) -> Ordering
    where
        Self: Sized,
    {
        match column {
            PartyResultsColumn::Party => self.party.cmp(&other.party),
            PartyResultsColumn::Name => self.name.cmp(&other.name),
            PartyResultsColumn::Votes => self.votes.cmp(&other.votes),
        }
    }
}
