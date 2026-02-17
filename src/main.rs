use ansi_term::Colour::{Blue, Green, Red, Yellow};
use ansi_term::Style;

use std::io::{stdout, Write};

use crossterm::{
    cursor::Hide,
    cursor::MoveTo,
    event, execute,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    terminal::SetSize,
    terminal::{Clear, ClearType},
    ExecutableCommand,
};

use sysinfo::{Cpu, Disk, System};

fn main() {
    //refresh all
    let mut sys = System::new_all();
    // non - changing vars
    let total_ram = (sys.total_memory() as f32) / (1024 as f32) / (1024 as f32) / (1024 as f32);
    let total_swap = (sys.total_swap() as f32) / (1024 as f32) / (1024 as f32) / (1024 as f32);
    let mut used_ram = 0;
    let mut used_swap = 0;
    let total_cores: usize = sys.cpus().len();
    let cpu_name: String = sys
        .cpus()
        .first()
        .map(|cpu| cpu.brand().to_string())
        .unwrap_or("Unknown CPU".to_string());
    let prog_version = Green.paint("v0.3");
    let mut i: i8 = 0;

    //clearscreen
    execute!(stdout(), Clear(ClearType::All), MoveTo(0, 0)).expect("Unable to clear screen");
    execute!(stdout(), Clear(ClearType::Purge)).expect("Unable to purge terminal history");
    execute!(stdout(), Hide).expect("Unable to Hide the cursor");
    execute!(stdout(), SetSize(80, 40)).expect("Unable to set the terminal size");

    std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);

    println!(
        "              --SYSTEM INFO--             {}\n",
        prog_version
    );
    println!("CPU SECTION:                        [ {} ]", cpu_name);
    println!("   Total Cores: {} Cores", total_cores);
    for cpu in sys.cpus() {
        i += 1;
        print!("   Core {} \n", i);
    }
    println!(
        "MEMORY SECTION:              [ {:.2} GB physical memory ] [ {:.2} GB virtual memory]",
        total_ram, total_swap
    );
    println!("   Physical memory used: {}", used_ram);
    println!("   Virtual memory used:  {}", used_ram);

    loop {
        sys.refresh_all();

        show_ram(&sys, total_cores);
        show_cpu_usage(&sys);

        std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
    }
}

// function to show ram and virtual ram
fn show_ram(system: &System, total_cores: usize) {
    let used_ram = (system.used_memory() as f32) / (1024 as f32) / (1024 as f32) / (1024 as f32);
    stdout()
        .execute(MoveTo(25, total_cores as u16 + 5))
        .expect("Unable to update");
    print!("{:.2}", used_ram);
    stdout().flush().expect("Unable to update");
    let used_swap = (system.used_swap() as f32) / (1024 as f32) / (1024 as f32) / (1024 as f32);
    stdout()
        .execute(MoveTo(25, total_cores as u16 + 6))
        .expect("Unable to update");
    print!("{:.2}", used_swap);
    stdout().flush().expect("Unable to update");
}

fn show_cpu_usage(system: &System) {
    let mut cpu_counter = 3;
    //11, 5 start
    for cpu in system.cpus() {
        cpu_counter += 1;
        stdout()
            .execute(MoveTo(13, cpu_counter))
            .expect("Unable to update");
        print!("{:.2}  ", cpu.cpu_usage());
        stdout().flush().expect("Unable to update");
    }
    cpu_counter = 3;
}
