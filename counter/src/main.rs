use sysinfo::{ProcessExt, System, SystemExt};

fn main() {
    let mut system = System::new_all();

    loop {
        system.refresh_all();

        let processes = system.get_processes();
        let mut sorted_processes: Vec<_> = processes.values().collect();
        sorted_processes.sort_by(|a, b| b.cpu_usage().partial_cmp(&a.cpu_usage()).unwrap());

        println!("PID\tCPU%\tMemory\tName");
        for process in sorted_processes.iter().take(10) {
            let name = process.name();
            let pid = process.pid();
            let cpu_usage = process.cpu_usage();
            let memory_usage = process.memory();

            println!("{}\t{:.2}\t{} KB\t{}", pid, cpu_usage, memory_usage, name);
        }

        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
