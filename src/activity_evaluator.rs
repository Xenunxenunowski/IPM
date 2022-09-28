use std::thread;
use std::time::Duration;
use get_sys_info::Platform;
use sysinfo::{Cpu, CpuExt, System, SystemExt};
use crate::OtherSystem;

/*
time,cpu usage,network usage should dictate as to weather or not an action should be taken

cpu usage + network usage should dictate as to what kind of action to take[ sleep/hibernate , shutdown]

 */
pub fn evaluate_activity(){
    let mut next_sys =  OtherSystem::new();
    let mut sys = System::new_all();
    sys.refresh_all();
    thread::sleep(Duration::from_secs(2));
    sys.refresh_all();
    let mut accumulative_cpu_usage:f32=0.0;
    for x in sys.cpus() {
        accumulative_cpu_usage += x.cpu_usage();
    }

    accumulative_cpu_usage =accumulative_cpu_usage / sys.cpus().len() as f32;
    println!("{}",accumulative_cpu_usage);
    thread::sleep(Duration::from_secs(15));
    use user_idle::UserIdle;
    let idle = UserIdle::get_time().unwrap();
    let idle_seconds = idle.as_seconds();
    let idle_minutes = idle.as_minutes();
    println!("{} seconds ,{} minutes",idle_seconds,idle_minutes);
}