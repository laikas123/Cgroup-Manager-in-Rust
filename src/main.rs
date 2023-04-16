use std::process::Command;
use std::fs::OpenOptions;
use std::io::prelude::*;
use text_colorizer::*;



//for readability in main function if else branch
#[derive(PartialEq)]
enum InputChoice{
    new_cgroup,
    add_pid_cgroup,
}

//note input.clear() is necessary to clear input buffer
//and input.pop removes unwanted new line
fn main() {

    loop {

        //get user input
        println!("What would you like to do? \n(0) Create cgroup \n(1) Add to existing cgroup?\n");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        input.pop();

        //create new cgroup
        if input.parse::<u8>().unwrap() == InputChoice::new_cgroup as u8 {
            println!("Enter new cgroup name:");
            input.clear();
            std::io::stdin().read_line(&mut input).unwrap();
            input.pop();
            create_cgroup(&input);
        //add pid to cgroup
        }else if input.parse::<u8>().unwrap() == InputChoice::add_pid_cgroup as u8{
            println!("Enter pid followed by a space followed by cgroup name:");
            input.clear();
            std::io::stdin().read_line(&mut input).unwrap();
            input.pop();
            let pid_and_cgroup: Vec<&str> = input.split(" ").collect();
            let pid = pid_and_cgroup[0];
            let cgroup = pid_and_cgroup[1];
            add_pid_to_cgroup(pid, cgroup);
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







