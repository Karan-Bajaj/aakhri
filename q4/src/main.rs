#![warn(clippy::all)]
mod cuda;

use std::env;
use std::error::Error;
use std::path::{Path, PathBuf};
use rustacuda_core::DeviceCopy;

pub struct Voter {
    x: f32,
    y: f32,
    z: f32,
}
unsafe impl DeviceCopy for Voter {}

pub struct ElectionOutcome {
    a_votes: i32,
    b_votes: i32
}
unsafe impl DeviceCopy for ElectionOutcome {}

fn main() -> Result<(), Box<dyn Error>> {
    let mut ctx = Some(cuda::CudaContext::init()?);
    println!("Context created; reading input file...");
    let voters = read_input_file("input/voters.csv".to_string());
    println!("{} voters successfully read.", voters.len());

    let election_outcomes = ctx.as_mut().unwrap().compute(voters)?;
    process_results(election_outcomes);

    Ok(())
}

fn process_results(outcomes: Vec<ElectionOutcome>) {
    let mut a_wins = 0;
    let mut b_wins = 0;
    let mut tie = 0;
    let total_outcomes = outcomes.len() as f32;

    for o in outcomes {
        if o.a_votes == o.b_votes {
            tie +=1;
        } else if o.a_votes > o.b_votes {
            a_wins += 1;
        } else {
            b_wins += 1;
        }
    }

    let a_win_pct = a_wins as f32 / total_outcomes * 100.0;
    let b_win_pct = b_wins as f32 / total_outcomes * 100.0;
    let tie_pct = tie as f32 / total_outcomes * 100.0;

    println!(
        "A wins {:.2}% of the time; B wins {:.2}% of the time; Tie {:.2}% of the time.",
        a_win_pct, b_win_pct, tie_pct
    )
}

fn read_input_file(input_file: String) -> Vec<Voter> {
    let mut reader = csv::Reader::from_path(input_file).unwrap();
    let mut voters = Vec::new();
    for row in reader.records() {
        let row = row.unwrap();
        voters.push(Voter {
            x: row[0].parse::<f32>().unwrap(),
            y: row[1].parse::<f32>().unwrap(),
            z: row[2].parse::<f32>().unwrap()
        });
    }
    voters
}
