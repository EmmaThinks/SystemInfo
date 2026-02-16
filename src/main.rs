use clearscreen;
use sysinfo::System;

fn main() {
    let mut sys = System::new_all();
    let total_ram = sys.total_memory() / 1024 / 1024 / 1024;

    loop {
        clearscreen::clear().expect("No se pudo limpiar la pantalla");
        sys.refresh_all();
        //variables de recursos aqui

        let used_ram = sys.used_memory() / 1024 / 1024 / 1024;

        println!("INFORMACION DEL SISTEMA:\n");

        println!("Recursos:");
        println!("RAM:\n   Total:{0}\n   Used:{1}", total_ram, used_ram);
        println!("CPU:");
        println!("Discos:");
        println!("Recursos:");

        std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
    }
}
