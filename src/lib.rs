mod models;

use std::{io::stdin, process};

use chrono::Utc;
use models::model::{Fish, Species};
use once_cell::sync::Lazy;
use surrealdb::{Surreal, engine::remote::ws::Client};

pub async fn log_fish(db: &Lazy<Surreal<Client>>)-> Result<(), Box<dyn std::error::Error>>{

    loop {
        // get species
        let species = get_species();

        // get timestamp
        let now = Utc::now();

        // get fish weight
        let weight = get_weight();

        // get fish length
        let length = get_length();

        // get location
        let fish_location = get_location();

        // get bait
        let fish_bait = get_bait();

        // get notes
        let fish_notes = get_notes();

        // create fish
        let _created: Vec<Fish> = db
            .create("fish")
            .content(Fish {
                species,
                weight,
                length,
                time_caught: now,
                location: fish_location.trim().to_string(),
                bait: fish_bait.trim().to_string(),
                notes: fish_notes.trim().to_string(),
            })
            .await?;

        // get the number of fish stored in the database
        let fish: Vec<Fish> = db.select("fish").await?;

        println!("You caught {} fish!\n", fish.len());
    }
}

fn get_species() -> Species {
    loop {
        println!("What fish species did you catch?");
        let fish_species = &mut String::new();
        stdin()
            .read_line(fish_species)
            .expect("Failed to read line");
        if fish_species.trim() == "quit" {
            process::exit(0);
        }

        let species = match fish_species.to_lowercase().trim() {
            "largemouth bass" => Species::LargemouthBass,
            "smallmouth bass" => Species::SmallmouthBass,
            "catfish" => Species::Catfish,
            "crappie" => Species::Crappie,
            "walleye" => Species::Walleye,
            "trout" => Species::Trout,
            "salmon" => Species::Salmon,
            "muskie" => Species::Muskie,
            "pike" => Species::Pike,
            "bluegill" => Species::Bluegill,
            "perch" => Species::Perch,
            "carp" => Species::Carp,
            "gar" => Species::Gar,
            "bowfin" => Species::Bowfin,
            "drum" => Species::Drum,
            "sturgeon" => Species::Sturgeon,
            _ => {
                println!("That's not a valid species!");
                println!("Valid species are: Largemouth Bass, Smallmouth Bass, Catfish, Crappie, Walleye, Trout, Salmon, Musky, Pike, Bluegill, Perch, Carp, Gar, Bowfin, Drum, Sturgeon\n");
                continue;
            }
        };

        return species;
    }
}

fn get_weight() -> f32 {
    println!("What did the fish weigh in pounds?");
    let fish_weight = &mut String::new();
    stdin().read_line(fish_weight).expect("Failed to read line");
    if fish_weight.trim() == "quit" {
        process::exit(0);
    }
    match fish_weight.trim().parse::<f32>() {
        Ok(_) => return fish_weight.trim().parse::<f32>().unwrap(),
        Err(_) => {
            println!("That's not a valid weight!");
            return get_weight();
        }
    }
}

fn get_length() -> f32 {
    println!("What was the length of the fish in inches?");
    let fish_length = &mut String::new();
    stdin().read_line(fish_length).expect("Failed to read line");
    if fish_length.trim() == "quit" {
        process::exit(0);
    }
    match fish_length.trim().parse::<f32>() {
        Ok(_) => fish_length.trim().parse::<f32>().unwrap(),
        Err(_) => {
            println!("That's not a valid length!");
            return get_length();
        }
    }
}

fn get_location() -> String {
    println!("Where did you catch the fish?");
    let fish_location = &mut String::new();
    stdin()
        .read_line(fish_location)
        .expect("Failed to read line");
    if fish_location.trim() == "quit" {
        process::exit(0);
    }
    fish_location.trim().to_string()
}

fn get_bait() -> String {
    println!("What bait did you use?");
    let fish_bait = &mut String::new();
    stdin().read_line(fish_bait).expect("Failed to read line");
    if fish_bait.trim() == "quit" {
        process::exit(0);
    }
    fish_bait.trim().to_string()
}

fn get_notes() -> String {
    println!("Any notes about the catch?");
    let fish_notes = &mut String::new();
    std::io::stdin()
        .read_line(fish_notes)
        .expect("Failed to read line");
    if fish_notes.trim() == "quit" {
        process::exit(0);
    }
    fish_notes.trim().to_string()
}