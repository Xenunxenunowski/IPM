mod activity_evaluator;

use std::thread;
use std::time::Duration;
use sysinfo::{NetworkExt, NetworksExt, ProcessExt, System, SystemExt};
use get_sys_info::{BatteryLife, Platform, System as OtherSystem};
fn main() {

// Please note that we use "new_all" to ensure that all list of
// components, network interfaces, disks and users are already
// filled!
    let mut next_sys =  OtherSystem::new();
    let mut sys = System::new_all();

// First we update all information of our `System` struct.
    sys.refresh_all();

// We display all disks' information:

// Network interfaces name, data received and data transmitted:
    println!("=> networks:");
    for (interface_name, data) in sys.networks() {
        println!("{}: {}/{} KiB", interface_name, data.received()*8/1024, data.transmitted()*8/1024);
    }

// Components temperature:

    println!("=> system:");
// RAM and swap information:
    println!("total memory     : {:.2} GB", sys.total_memory() as f32/1024.0);
    println!("used memory      : {:.2} GB", sys.used_memory() as f32 /1024.0);
    println!("total swap       : {:.2} GB", sys.total_swap() as f32/1024.0);
    println!("used swap        : {:.2} GB", sys.used_swap() as f32 /1024.0);
    match next_sys.battery_life() {
        Ok(val) => {
            println!("battery left     : {}",val.remaining_capacity);
            println!("battery time left: {} h \n",val.remaining_time.as_secs_f64()/3600.0);
        }
        Err(_) => {
            println!("battery left     : {}", "No data");
            println!("battery time left: {}", "No data \n");
        }
    }

// Display system information:
    println!("System name:             {}", sys.name().unwrap());
    println!("System kernel version:   {}", sys.kernel_version().unwrap());
    println!("System OS version:       {}", sys.os_version().unwrap());
    println!("System host name:        {}", sys.host_name().unwrap());
// Number of CPUs:
    println!("CPUs: {}", sys.cpus().len());

    //thread::sleep(Duration::from_secs(10));
    println!("\n \n \n");
    activity_evaluator::evaluate_activity();
    //thread::sleep(Duration::from_secs(10));
// Display processes ID, name na disk usage:
}
