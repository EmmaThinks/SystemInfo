use ansi_term::{ANSIGenericString, Color::Red, Colour::Green};

use std::{
    io::{self, stdout, Write},
    thread,
    time::{self, Duration},
};

use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{self, read, Event, KeyCode, KeyModifiers},
    execute,
    style::Print,
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType, SetSize},
    ExecutableCommand,
};

use sysinfo::{Components, Disk, Disks, System};

fn main() {
    //refresh all
    let mut sys = System::new_all();
    let mut component = Components::new_with_refreshed_list();
    let disks = Disks::new_with_refreshed_list();
    // non - changing vars
    let total_ram = (sys.total_memory() as f32) / (1024 as f32) / (1024 as f32) / (1024 as f32);
    let total_swap = (sys.total_swap() as f32) / (1024 as f32) / (1024 as f32) / (1024 as f32);
    let total_cores: usize = sys.cpus().len();
    let cpu_name: String = sys
        .cpus()
        .first()
        .map(|cpu| cpu.brand().to_string())
        .unwrap_or("Unknown CPU".to_string());
    let prog_version = Green.paint("v0.7");
    let mut i: i8 = 0;
    let dot_char = ".";

    //clearscreen
    execute!(stdout(), Clear(ClearType::All), MoveTo(0, 0)).expect("Unable to clear screen");
    execute!(stdout(), Clear(ClearType::Purge)).expect("Unable to purge terminal history");
    execute!(stdout(), Hide).expect("Unable to Hide the cursor");
    execute!(stdout(), SetSize(130, 40)).expect("Unable to set the terminal size");

    println!("              --OVERSEER--             {}\n", prog_version);
    println!("CPU SECTION:     [ {} ]", cpu_name);
    println!("   Total Cores: {} Cores", total_cores);
    for cpu in sys.cpus() {
        i += 1;
        print!("   Core {} \n", i);
    }
    println!(
        "MEMORY SECTION:     [ {:.2} GB physical memory ] [ {:.2} GB virtual memory]",
        total_ram, total_swap
    );

    println!(
        "   Physical memory used: [{}]",
        dot_char.repeat(((total_ram / 2.0) as f32).round() as usize)
    );
    println!(
        "   Virtual memory used:  [{}]",
        dot_char.repeat(((total_swap / 2.0) as f32).round() as usize)
    );
    println!("DISKS SECTION:");
    for disk in disks.list() {
        println!("   {}", disk.mount_point().display());
        stdout().flush().expect("Unable to update");
    }
    loop {
        sys.refresh_all();
        component.refresh(true);

        // all functions
        show_ram(&sys, total_cores, total_ram, total_swap);
        show_cpu_usage(&sys, &component);
        show_disk_usage(total_cores, &disks);

        // function with poll to let processor have time to calculate metrics while looks for events
        check_for_event(std::time::Duration::from_secs(1));
    }
}

// function to show ram and virtual ram
fn show_ram(system: &System, total_cores: usize, tot_ram: f32, tot_swap: f32) {
    let used_ram = (system.used_memory() as f32) / (1024 as f32) / (1024 as f32) / (1024 as f32);
    let used_swap = (system.used_swap() as f32) / (1024 as f32) / (1024 as f32) / (1024 as f32);
    let bar_char = "|";

    //ram part
    stdout()
        .execute(MoveTo(26, total_cores as u16 + 5))
        .expect("Unable to Update");
    print!("{}", bar_char.repeat((used_ram.floor() as usize) / 2));

    stdout()
        .execute(MoveTo(
            ((28.0 + (tot_ram / 2.0)).round()) as u16,
            total_cores as u16 + 5,
        ))
        .expect("Unable to update");
    print!("{:.2} GB ", used_ram);
    //end ram part

    // swap part
    stdout()
        .execute(MoveTo(26, total_cores as u16 + 6))
        .expect("Unable to Update");
    print!("{}", bar_char.repeat((used_swap.floor() as usize) / 2));

    stdout()
        .execute(MoveTo(
            ((28.0 + (tot_swap / 2.0)).round()) as u16,
            total_cores as u16 + 6,
        ))
        .expect("Unable to update");
    print!("{:.2} GB ", used_swap);
    stdout().flush().expect("Unable to update");
    //end swap part
}
//function to show cpu usage by core
fn show_cpu_usage(system: &System, comp: &Components) {
    let mut cpu_counter = 3;
    let bar = "||||||||||";
    //11, 5 start
    for cpu in system.cpus() {
        cpu_counter += 1;
        stdout()
            .execute(MoveTo(13, cpu_counter))
            .expect("Unable to Move");
        print!("[..........] {:.2}% ", cpu.cpu_usage());

        stdout()
            .execute(MoveTo(14, cpu_counter))
            .expect("Unable to Move");

        print!(
            "{}",
            &bar[0..((cpu.cpu_usage() as i32) / 10).min(10) as usize]
        );
        stdout().flush().expect("Unable to print");
    }

    for components in comp {
        if components.label() == "k10temp Tctl" {
            let temp = components.temperature().unwrap_or(0.0) as f32;
            stdout().execute(MoveTo(60, 2)).expect("Unable to Move");
            if temp > 85.0 {
                print!("\x1b[31m[{:.2} °C]  \x1b[31m  ", temp);
            } else if temp > 60.0 {
                print!("\x1b[33m[{:.2} °C]  \x1b[33m", temp);
            } else if temp > 30.0 {
                print!("\x1b[32m[{:.2} °C]  \x1b[32m", temp);
            } else if temp < 30.0 {
                print!("\x1b[36m[{:.2} °C]  \x1b[36m", temp);
            }
            print!("\x1B[0m");
            stdout().flush().expect("Unable to print");
        }
    }
}

fn show_disk_usage(total_cores: usize, disks: &Disks) {
    let mut row_counter = total_cores as u16 + 8;

    for disk in disks.list() {
        stdout()
            .execute(MoveTo(25, row_counter))
            .expect("unable to update");

        print!(
            " {} | {} GB     Read: {}/MBs | Write {}/MBs",
            (disk.total_space() - disk.available_space()) / 1024 / 1024 / 1024,
            disk.total_space() / 1024 / 1024 / 1024,
            disk.usage().read_bytes / 1024 / 1024,
            disk.usage().written_bytes / 1024 / 1024
        );

        row_counter += 1;
    }
    stdout().flush().expect("Unable to update disks");
}

fn check_for_event(timeout: Duration) {
    enable_raw_mode().expect("Unable to enter raw mode");
    if event::poll(timeout).expect("Unable to make poll") {
        match read().expect("Unable to get the event") {
            Event::Resize(columns, rows) => {}
            Event::Key(event) => {
                if event.code == event::KeyCode::Char('q') {
                    execute!(stdout(), Show).expect("Unable to Hide the cursor");
                    disable_raw_mode().expect("Unable to exit the program");
                    std::process::exit(0);
                }
            }
            _ => {}
        }
    }
    disable_raw_mode().expect("Unable to exit the program");
}
