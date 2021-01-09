#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

use cetkaik_full_state_transition::*;

#[must_use]
pub fn yield_random_next(current_state: &state::A, config: Config) -> Option<state::A> {
    use rand::seq::SliceRandom;
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let (hop1zuo1_candidates, candidates) = current_state.get_candidates(config);
    let candidate = match (&hop1zuo1_candidates[..], &candidates[..]) {
        ([], []) => return None, // stuck; yield no more
        ([], candidates) => candidates.choose(&mut rng).unwrap(),
        (hop1zuo1_candidates, []) => hop1zuo1_candidates.choose(&mut rng).unwrap(),
        (hop1zuo1_candidates, candidates) => {
            if rng.gen::<f64>() < 0.1_f64 {
                // choose randomly from hop1zuo1_candidates
                hop1zuo1_candidates.choose(&mut rng).unwrap()
            } else {
                // choose randomly from candidates
                candidates.choose(&mut rng).unwrap()
            }
        }
    }
    .to_owned();

    let hand_not_resolved = match candidate {
        message::PureMove::InfAfterStep(msg) => {
            let (c, maybe_ciurl) = apply_inf_after_step(&current_state, msg, config)
                .map_err(|e| format!("Internal error in yield_random_next: {}", e))
                .unwrap()
                .choose();

            let c_candidates = c.get_candidates(config);

            let c_msg = *c_candidates.choose(&mut rng).expect("This cannot fail, because it is always legal to cancel");

            let (hand_not_resolved, maybe_ciurl) = apply_after_half_acceptance(&c, c_msg, config)
                .map_err(|e| format!("Internal error in yield_random_next: {}", e))
                .unwrap()
                .choose();

            hand_not_resolved
        }
        message::PureMove::NormalMove(msg) => {
            let (hand_not_resolved, maybe_ciurl) = apply_normal_move(&current_state, msg, config)
                .map_err(|e| format!("Internal error in yield_random_next: {}", e))
                .unwrap()
                .choose();
            hand_not_resolved
        }
    };
    let next_state = match resolve(&hand_not_resolved, Config::cerke_online_alpha()) {
        state::HandResolved::HandExists { if_tymok, if_taxot } => {
            // FIXME: always chooses taxot
            match if_taxot {
                IfTaxot::VictoriousSide(victor) => return None,
                IfTaxot::NextSeason(prob_distribution) => prob_distribution.choose_when_no_ciurl(),
            }
        }
        state::HandResolved::NeitherTymokNorTaxot(next_state) => next_state,
        state::HandResolved::GameEndsWithoutTymokTaxot(_) => return None,
    };

    return Some(next_state);
}
