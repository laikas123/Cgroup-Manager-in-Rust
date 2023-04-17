use std::process::Command;
use std::fs::OpenOptions;
use text_colorizer::*;
use std::io::prelude::*;

pub mod timeget;
use timeget::*;


static LOGDIR: &str = "/home/logan/Desktop/log";

struct UserChoice {
}

impl UserChoice {
    const NEWCGROUP: &'static str = "0";
    const ADDPID: &'static str = "1";
    const SETMAXMEM: &'static str = "2";
}


struct CpuCgroup {}
struct MemCgroup {}
struct PidCgroup {}

struct CgroupInfo<T> {
    _type: T,
    name: String,
}



//functions for a cgroup using
//the memory controller
impl CgroupInfo<MemCgroup> {

    fn new(cgroup: String) -> Self {
        CgroupInfo {_type: MemCgroup{}, name: cgroup}
    }

    fn get_current_mem(&self) -> String{
        let output = Command::new("/bin/cat")
                        .arg(format!("/sys/fs/cgroup/{}/memory.current", self.name))
                        .output()
                        .expect("failed to execute get_current_mem for cgroup {self.name}");

        let s = match std::str::from_utf8(&output.stdout) {
            Ok(v) => v,
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        };

        // println!("result: {}", s);

        s.to_string()

    }

    fn write_current_mem_to_file(&self) {

        let file_data = self.get_current_mem();
        let timestamp = generate_timestamp_string();

        println!("{}",format!("{LOGDIR}/{}/memory.current", &self.name));

        let mut file = OpenOptions::new()
        .create_new(true)
        .write(true)
        .append(true)
        .open(format!("{LOGDIR}/{}/memory.current", &self.name))
        .unwrap();

        if let Err(e) = write!(file, "{}", format!("{} {}", timestamp, file_data)) {
            eprintln!("Couldn't write to file: {}", e);
        }
    }
}







//note input.clear() is necessary to clear input buffer
//and input.pop removes unwanted new line
fn main() {

    let hours_mins_secs = get_hours_mins_secs();
    println!("hours mins secs {}, {}, {}", hours_mins_secs.0, hours_mins_secs.1, hours_mins_secs.2);

    // let test = CgroupInfo::<MemCgroup>{_type: MemCgroup{}, name:"bloopy".to_string()};

    let test2 = CgroupInfo::<MemCgroup>::new("bloopy".to_string());

    test2.get_current_mem();

    println!("{}", generate_timestamp_string());

    test2.write_current_mem_to_file();

    loop {

        //get user input
        println!("What would you like to do? \n(0) Create cgroup \n(1) Add to existing cgroup?\n(2) Set max memory for cgroup?\n");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        input.pop();

        //figure out user choice and perform action based
        //on further input
        if input == UserChoice::NEWCGROUP {
            println!("Enter new cgroup name:");
            input.clear();
            std::io::stdin().read_line(&mut input).unwrap();
            input.pop();
            create_cgroup(&input);
        }else if input == UserChoice::ADDPID {
            println!("Enter pid followed by a space followed by cgroup name:");
            input.clear();
            std::io::stdin().read_line(&mut input).unwrap();
            input.pop();
            let pid_and_cgroup: Vec<&str> = input.split(" ").collect();
            let pid = pid_and_cgroup[0];
            let cgroup = pid_and_cgroup[1];
            add_pid_to_cgroup(pid, cgroup);
        }else if input == UserChoice::SETMAXMEM {
            println!("Enter max memory in bytes followed by a space followe by cgroup name:");
            input.clear();
            std::io::stdin().read_line(&mut input).unwrap();
            input.pop();
            let bytes_and_cgroup: Vec<&str> = input.split(" ").collect();
            let bytes = bytes_and_cgroup[0];
            let cgroup = bytes_and_cgroup[1];
            change_mem_max_for_cgroup(bytes, cgroup);
        }else{
            println!("\nError unknown choice please try again\n");
            input.clear();
            continue;
        }

        
    }


}


fn create_cgroup(name: &str) -> Result<(), &'static str> {
    let status = Command::new("mkdir")
                        .arg(format!("/sys/fs/cgroup/{}", name))
                        .status()
                        .expect("failed to execute mkdir");

    match status.code() {
        Some(code) if code == 0 => {
            println!("{} created cgroup {name} with exit status {code}", "Success".green());
            Ok(())
        },
        Some(code) => {
            println!("{} not create cgroup {name} with exit status {code}", "Error".red());
            Err("Could not create group")
        },
        _  => Err("Could not create group"),
    }
}

fn add_pid_to_cgroup(pid: &str, cgroup: &str) -> Result<(), &'static str> {
    let status = Command::new("sh")
                        .arg("-c")
                        .arg(format!("/root/add_pid_cgroup.sh {} {}", pid, cgroup))
                        .status()
                        .expect("failed to execute add_pid_cgroup.sh");

    match status.code() {
        Some(code) if code == 0 => {
            println!("{} added {pid} to {cgroup} with exit status {code}", "Success".green());
            Ok(())
        },
        Some(code) => {
            println!("{} could not add {pid} to cgroup {cgroup} with exit status {code}", "Error".red());
            Err("Could not create group")
        },
        _  => Err("Could not create group"),
    }
        
}


//takes the max memory amount in bytes
//and the cgroup 
fn change_mem_max_for_cgroup(bytes: &str, cgroup: &str) -> Result<(), &'static str> {
    let status = Command::new("sh")
                        .arg("-c")
                        .arg(format!("/root/set_mem_max.sh {} {}", bytes, cgroup))
                        .status()
                        .expect("failed to execute add_pid_cgroup.sh");

    match status.code() {
        Some(code) if code == 0 => {
            println!("{} wrote {bytes} to memory.max for {cgroup} with exit status {code}", "Success".green());
            Ok(())
        },
        Some(code) => {
            println!("{} failed to write {bytes} to memory.max for {cgroup} with exit status {code}", "Error".red());
            Err("Could not create group")
        },
        _  => Err("Could not create group"),
    }
        
}








