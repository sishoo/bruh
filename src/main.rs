use chrono::{Datelike, Weekday};
use std::fs::read_dir;
use std::io;
use std::thread::sleep;
use std::time::{Duration, Instant};
use std::ffi::OsString;

#[inline(always)]
fn clear() {
    print!("\x1B[2J\x1B[1;1H");
}

fn printerface_start() {
    let day = match chrono::offset::Local::now().date().weekday() {
        Weekday::Mon => "Monday",
        Weekday::Tue => "Tuesday",
        Weekday::Wed => "HUMP DAY!!!!!!",
        Weekday::Thu => "Thursday",
        Weekday::Fri => "Friday",
        Weekday::Sat => "Saturday",
        Weekday::Sun => "Sunday",
    };
    println!("+-------------+");
    println!("+     ¤69     +");
    println!("+-------------+");
    println!();
    println!("\tWelcome to the LETHAL-COMPANY-MOD-MANAGER-9 OS");
    println!("\t                                Courtesy of the Oracle");
    println!("\tHappy {}.", day);
}

fn printerface_mods(mods: &Vec<OsString>) {
    let day = match chrono::offset::Local::now().date().weekday() {
        Weekday::Mon => "Monday",
        Weekday::Tue => "Tuesday",
        Weekday::Wed => "HUMP DAY!!!!!!",
        Weekday::Thu => "Thursday",
        Weekday::Fri => "Friday",
        Weekday::Sat => "Saturday",
        Weekday::Sun => "Sunday",
    };
    println!("+-------------+");
    println!("+     ¤69     +");
    println!("+-------------+");
    println!();
    println!("\tWelcome to the LETHAL-COMPANY-MOD-MANAGER-9 OS");
    println!("\t                                Courtesy of the Oracle");
    println!("\tHappy {}.", day);
    println!("\tCurrent Mods:");
    for r#mod in mods {
        println!("{:?}", r#mod);
    }
}

fn startup_animation() {
    let duration = Duration::from_secs(6);
    let start_time = Instant::now();

    while start_time.elapsed() < duration {
        println!("Loading.");
        sleep(Duration::from_secs(1));
        clear();
        println!("Loading..");
        sleep(Duration::from_secs(1));
        clear();
        println!("Loading...");
        sleep(Duration::from_secs(1));
        clear();
    }
}

fn main() {
    loop {
        clear();
        startup_animation();
        printerface_start(); // printerface? i hardly even know er!!!!!
        println!("Please enter your Lethal Company directory path to get started: ");
        let mut folder_path = String::new();
        let res = io::stdin().read_line(&mut folder_path);
        if res.is_err() {
            clear();
            println!("ERRRRRRRRRR!!@^#!^@#^!@# Failed to open folder, restarting OS...");
            sleep(Duration::from_secs(1));
            continue;
        }
        let mut folder_path = folder_path.trim().to_string();
        folder_path.push_str(r"\BepInEx\plugins");
        println!("{:?}", folder_path);
        let mods = read_dir(folder_path);
        if mods.is_err() {
            clear();
            println!("ERRRRRRRRRR!!@^#!^@#^!@# Failed to find mods, restarting OS...");
            sleep(Duration::from_secs(1));
            continue;
        }
        let mods: Vec<OsString> = mods
            .unwrap()
            .filter(|entry| {
                if entry.is_ok() {
                    return !(entry.as_ref().unwrap().path().is_dir());
                }
                false
            })
            .map(|entry| entry.unwrap().file_name())
            .collect();
        printerface_mods(&mods);

    }
}
