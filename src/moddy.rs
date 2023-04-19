use text_colorizer::*;
use std::fs;
use runas::Command as RootCommand;
use super::cgroup::*;
use super::globals::*;
use std::fs::OpenOptions; 
use std::io::Write;

pub fn read_file_contents(file_path: &str) -> Result<String, &'static str>  {

    let read_result = fs::read_to_string(file_path);

    match read_result{
        Ok(contents) => Ok(contents.to_string()),
        _ => Err("Could not open file {file_path}"),
    }
   
}

pub fn update_file_contents(file_path: &str, val: &str) -> Result<(), &'static str>  {

    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(file_path)
        .unwrap();

        // println!("{CGROUPROOT}/{}/{}", &self.name, filename);
        match write!(file, "{}", val) {
            Ok(v) => {
                println!("{} wrote to file {}", "Success".green(), file_path);
                Ok(())
            },
            Err(e) => Err("could not write to file {file_path}"),
        }
         
   
}



pub fn modify_active_controller(val: &str) -> Result<(), &'static str> {
        
        
    let status = RootCommand::new("sh")
                        .arg("-c")
                        .arg(format!("./modify_active_controllers.sh {}", val))
                        .status()
                        .expect("failed to execute add_pid_cgroup.sh");

    match status.code() {
        Some(code) if code == 0 => {
            println!("{}", "Success".green());
            Ok(())
        },
        Some(code) => {
            println!("{}", "Error".red());
            Err("Could not create group")
        },
        _  => Err("Could not create group"),
    }

}


pub fn create_cgroup(name: &str) -> Result<Cgroup, &'static str> {
 
    let status = RootCommand::new("sh")
                .arg("-c")
                .arg(format!("mkdir {CGROUPROOT}/{} && chown {USERNAME}:{USERNAME} {CGROUPROOT}/{}/*", name, name))
                .status()
                .expect("failed to create cgroup");

    match status.code() {
        Some(code) if code == 0 => {
            println!("{} created cgroup {name} with exit status {code}", "Success".green());
            Ok(Cgroup::new(name.to_string()))
        },
        Some(code) => {
            println!("{} not create cgroup {name} with exit status {code}", "Error".red());
            Err("Could not create group")
        },
        _  => Err("Could not create group"),
    }
}



pub fn remove_cgroup(name: &str) -> Result<Cgroup, &'static str> {
  
    let status = RootCommand::new("sh")
                .arg("-c")
                .arg(format!("rmdir {CGROUPROOT}/{}", name))
                .status()
                .expect("failed to remove cgroup");

    match status.code() {
        Some(code) if code == 0 => {
            println!("{} removed cgroup {name} with exit status {code}", "Success".green());
            let mut removed_cgroup = Cgroup::new(name.to_string());
            removed_cgroup.delete = 1;
            Ok(removed_cgroup)
        },
        Some(code) => {
            println!("{} not delete cgroup {name} with exit status {code}", "Error".red());
            Err("Could not delete cgroup")
        },
        _  => Err("Could not delete cgroup"),
    }
}


