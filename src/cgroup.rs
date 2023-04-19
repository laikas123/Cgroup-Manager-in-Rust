use text_colorizer::*;
use std::process::Command;
use std::fs::OpenOptions;   
use runas::Command as RootCommand;
use std::io::Write;
use serde::{Serialize, Deserialize};
use super::globals::*;



#[derive(Serialize, Deserialize, Debug)]
pub struct Cgroup {
    pub name: String,
    pub delete: u8,
}

impl Cgroup {

    pub fn new(name: String) -> Self {
        Cgroup{name: name, delete: 0}       
    }

    pub fn get(&self, filename: &str) -> Result<String, &'static str>{

        
        let output = Command::new("/bin/cat")
                .arg(format!("{CGROUPROOT}/{}/{}", self.name, filename))
                .output()
                .expect("failed to execute get_current for cgroup {self.name}");


        println!("{}", format!("{CGROUPROOT}/{}/{}", filename, self.name));

        match std::str::from_utf8(&output.stdout) {
            Ok(s) => Ok(s.to_string()),
            Err(e) => Err("Couldn't read file"),
        }
        

    }

    pub fn set(&self, filename: &str, val: &str) {
        
        
        let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(format!("{CGROUPROOT}/{}/{}", &self.name, filename))
        .unwrap();

        println!("{CGROUPROOT}/{}/{}", &self.name, filename);
        if let Err(e) = write!(file, "{}", val) {
            eprintln!("Couldn't write to file: {}", e);
        }
    
            

        
    }

    //needs to be run as root
    pub fn add_pid(&self, pid: &str) -> Result<(), &'static str> {
        let status = RootCommand::new("sh")
                        .arg("-c")
                        .arg(format!("echo {} > \"/sys/fs/cgroup/{}/cgroup.procs\"", pid, &self.name))
                        .status()
                        .expect("failed to add pid to cgroup");

        // println!("add_pid_cgroup.sh {} {}", pid, &self.name);
        match status.code() {
            Some(code) if code == 0 => {
                println!("{} added {pid} to with exit status {code}", "Success".green());
                Ok(())
            },
            Some(code) => {
                println!("{} could not add {pid} to cgroup  with exit status {code}", "Error".red());
                Err("Could not create group")
            },
            _  => Err("Could not add {pid} to cgroup.procs"),
        }

    }


}