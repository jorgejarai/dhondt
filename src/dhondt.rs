use std::collections::HashMap;

#[derive(Debug)]
pub enum DHondtError {
    ZeroSeats,
    NoVotes,
    NoParties,
}

pub type PartyCandidate = (String, String);

pub fn calculate(
    seat_count: u32,
    votes: &HashMap<PartyCandidate, u32>,
) -> Result<HashMap<String, Vec<String>>, DHondtError> {
    if seat_count == 0 {
        return Err(DHondtError::ZeroSeats);
    }

    if votes.is_empty() {
        return Err(DHondtError::NoParties);
    }

    let votes_by_party =
        votes
            .iter()
            .fold(HashMap::new(), |mut acc, ((party, _candidate), votes)| {
                *acc.entry(party.clone()).or_insert(0) += votes;
                acc
            });

    if votes_by_party.iter().all(|p| p.1 == &0) {
        return Err(DHondtError::NoVotes);
    }

    let most_voted_party = votes_by_party.iter().max_by_key(|p| p.1).unwrap();
    let mut seats = HashMap::new();
    *seats.entry(most_voted_party.0.clone()).or_default() = 1;

    loop {
        let assigned_seats: u32 = seats.iter().map(|p| p.1).sum();
        if assigned_seats == seat_count {
            break;
        }

        let next_seat_party = votes_by_party
            .iter()
            .max_by_key(|p| {
                let votes = votes_by_party.get(p.0).unwrap();
                let curr_seats = seats.get(p.0).unwrap_or(&0);

                votes / (curr_seats + 1)
            })
            .unwrap();

        *seats.entry(next_seat_party.0.clone()).or_default() += 1;
    }

    let elected_candidates = seats
        .iter()
        .map(|p| {
            let mut party_candidates: Vec<(String, u32)> = votes
                .iter()
                .filter(|c| c.0 .0 == *p.0)
                .map(|c| (c.0 .1.clone(), *c.1))
                .collect();
            party_candidates.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

            let party_elected_candidates: Vec<String> = party_candidates
                .iter()
                .take(*p.1 as usize)
                .map(|c| c.0.clone())
                .collect();

            (p.0.clone(), party_elected_candidates)
        })
        .collect();

    Ok(elected_candidates)
}
