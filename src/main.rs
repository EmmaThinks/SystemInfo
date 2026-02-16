use clearscreen;
use sysinfo::{Disk, System};

fn main() {
    let mut sys = System::new_all();
    let total_ram: f32 = (sys.total_memory() / 1024 / 1024 / 1024) as f32;
    let total_swap = sys.total_swap() / 1024 / 1024 / 1024;

    loop {
        clearscreen::clear().expect("No se pudo limpiar la pantalla");
        sys.refresh_all();
        // VARIABLES:
        let used_ram = sys.used_memory() / 1024 / 1024 / 1024;
        let used_swap = sys.used_swap() / 1024 / 1024 / 1024;

        println!("SYSTEM INFO 0.1:\n");

        // GEN INFORMATION:

        println!();

        println!("Resources:");

        // RAM MEMORY:
        println!(
            "RAM:

    Total ========> [ {0} ]
    Used =========> [ {1} ]
    Swap Total ===> [ {2} ]
    Swap Used ====> [ {3} ]
    ",
            total_ram, used_ram, total_swap, used_swap
        );

        // CPU USAGE & TOTAL CORES:
        println!("CPU:\n   Total Cores:{}", sys.cpus().len());
        println!("Discos:");
        println!("");

        std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
    }
}
