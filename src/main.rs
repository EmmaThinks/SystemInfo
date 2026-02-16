use ansi_term::Colour::{Blue, Green, Red, Yellow};
use ansi_term::Style;
use std::arch::x86_64::CpuidResult;
use std::io::stdout;

use crossterm::{
    cursor::MoveTo,
    event, execute,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
    terminal::{Clear, ClearType},
    ExecutableCommand,
};

use sysinfo::{Cpu, Disk, System};

fn main() {
    //refresh all
    let mut sys = System::new_all();
    // non - changing vars
    let total_ram = (sys.total_memory() / 1024 / 1024 / 1024);
    let total_swap = sys.total_swap() / 1024 / 1024 / 1024;
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

    std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);

    println!(
        "              --SYSTEM INFO--             {}\n",
        prog_version
    );
    println!("CPU SECTION:                        [ {} ]", cpu_name);
    println!("   Total Cores: {} Cores", total_cores);
    for cpu in sys.cpus() {
        i += 1;
        print!("   Core {} {}\n", i, cpu.frequency());
    }

    loop {
        sys.refresh_all();
        // VARIABLES:
        let used_ram = sys.used_memory() / 1024 / 1024 / 1024;
        let used_swap = sys.used_swap() / 1024 / 1024 / 1024;

        std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
    }
}
