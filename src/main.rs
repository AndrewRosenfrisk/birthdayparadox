// "Birthday Paradox Simulation, by Al Sweigart al@inventwithpython.com
//   2. Explore the surprising probabilities of the "Birthday Paradox".
//   3. More info at https://en.wikipedia.org/wiki/Birthday_problem
//   4. This code is available at https://nostarch.com/big-book-small-python-programming
//   5. Tags: short, math, simulation"
  use std::{env, sync::{atomic::{AtomicUsize, Ordering}, Arc}, thread, usize, vec};
  use std::time::Instant;
  use rand::prelude::*;
  use chrono::prelude::*;

  fn main() {
    println!("Birthday Paradox, by Al Sweigart al@inventwithpython.com
 
 The birthday paradox shows us that in a group of N people, the odds
 that two of them have matching birthdays is surprisingly large.
 This program does a Monte Carlo simulation (that is, repeated random
 simulations) to explore this concept.
 
 (It's not actually a paradox, it's just a surprising result.)");

    let args: Vec<String> = env::args().collect();

    let set_size = args[1].parse::<u16>().unwrap();
    let multi_thread_flag = args[2].parse::<bool>().unwrap_or(true);
    let single_set_flag = args[3].parse::<bool>().unwrap_or(false);
    let single_thread_flag = args[4].parse::<bool>().unwrap_or(false);

    if single_set_flag {
        single_birthday_set(set_size);
    }

    if multi_thread_flag {
        println!("Generating {set_size} random birthdays 100,000 times...");
        let now = Instant::now();
        let multithreaded_sim_match = multi_threaded_simulation(set_size);
        println!("Running multi_threaded_simulation() took {} milliseconds.", now.elapsed().as_millis());

        let probability = f64::floor(multithreaded_sim_match as f64/100000.0 * 100.0);
        println!("Out of 100,000 simulations of {} people, there was a", set_size);
        println!("matching birthday in that group {} times. This means", multithreaded_sim_match);
        println!("that {} people have a {}% chance of", set_size, probability);
        println!("having a matching birthday in their group. \nThat's probably more than you would think!");
    }

    if single_thread_flag {        
    println!("Generating {set_size} random birthdays 100,000 times...");
    let now = Instant::now();
    let single_thread_sim_match = single_threaded_simulation(set_size);
    println!("Running multi_threaded_simulation() took {} milliseconds.", now.elapsed().as_millis());

    let probability = f64::floor(single_thread_sim_match as f64/100000.0 * 100.0);
    println!("Out of 100,000 simulations of {} people, there was a", set_size);
    println!("matching birthday in that group {} times. This means", single_thread_sim_match);
    println!("that {} people have a {}% chance of", set_size, probability);
    println!("having a matching birthday in their group. \nThat's probably more than you would think!");
}
}

fn get_birthdays(set_size: u16) -> Vec<NaiveDate> {
    let mut birthdays: Vec<NaiveDate> = vec![];

    for _ in 1..=set_size {
        let mut rng = rand::thread_rng();
        rng.gen_range(0..=364);
        birthdays.push(NaiveDate::from_yo_opt(2024, rng.gen_range(1..=365)).unwrap());
    }
    birthdays
 }

fn get_match(birthdays: Vec<NaiveDate>) -> Option<Vec<NaiveDate>>{
    let mut uniques = Vec::from_iter(birthdays.iter().copied());
    uniques.sort();
    uniques.dedup();

    let mut matches: Vec<NaiveDate> = vec![];
    if uniques.len() == birthdays.len() {
        return None;
    } else {
        uniques
        .iter()
        .enumerate()
        .for_each(|unique| {
            if birthdays
                .iter()
                .enumerate()
                .filter(move |duplicate| { *duplicate.1 == *unique.1})
                .count() > 1 {
                    matches.push(unique.1.clone());
                }            
        });
        }
        return Some(matches)
 }

fn single_birthday_set(set_size: u16) {
    let months = ["Jan", "Feb", "Mar", "Apr", "May", "Jun",
    "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"];

    println!("Here are {set_size} birthdays:");
    let birthdays: Vec<NaiveDate> = get_birthdays(set_size);

    birthdays.iter().enumerate().for_each(|(i, d)| {
    let end = if i + 1 != birthdays.len() {
            ",".to_string()
        } else { 
            "".to_string()
        };
    let month_index: usize = d.month0().try_into().unwrap();
    println!("{:?}. {} {:?}{}", i + 1, months[month_index], d.day0() + 1, end);
    });

    let matches = get_match(birthdays);

    match matches {
    None => println!("There are no matching birthdays."),
    Some(x) => {
    println!("Here are {} matching birthdays:", x.len());
    x.iter().enumerate().for_each(|(i, d)| {
        let end = if i + 1 != x.len() {
            ",".to_string()
        } else {
            "".to_string()
        };
        let month_index: usize = d.month0().try_into().unwrap();
        println!("{:?}. {} {:?}{}", i + 1, months[month_index], d.day0() + 1, end);
    });
    }
    }
}
 
fn multi_threaded_simulation(set_size: u16) -> usize {
    let counter = Arc::new(AtomicUsize::new(0));
    let sim_match_parent = Arc::new(AtomicUsize::new(0));
    let mut handles = vec![];

    for i in 0..10 {
        let counter_clone = Arc::clone(&counter);
        let sim_match_child = Arc::clone(&sim_match_parent);
    
        let handle = thread::spawn(move || {
            for _ in 0..10000 {
                let sim_birthdays = get_birthdays(set_size);
                if get_match(sim_birthdays) != None {
                    sim_match_child.fetch_add(1, Ordering::Relaxed);
                }
            }
            counter_clone.fetch_add(1, Ordering::Relaxed);
            println!("Thread {}, +10k simulations complete...\n", i)
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    println!("100,000 multithreaded simulations run.");
    sim_match_parent.load(Ordering::Relaxed)
}

fn single_threaded_simulation(set_size: u16) -> usize {
    let mut sim_match: usize = 0;

for i in 0..100000 {
    if i % 10000 == 0 {
        println!("{} simulations run so far...\n", i);
    }

    let sim_birthdays = get_birthdays(set_size);

    if get_match(sim_birthdays) != None {
        sim_match += 1;
    }
}
    println!("100,000 single threaded simulations run.");
    sim_match
}
