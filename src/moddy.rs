use text_colorizer::*;
use std::fs;
use runas::Command as RootCommand;
use super::cgroup::*;
use super::globals::*;
use std::fs::OpenOptions; 
use std::io::Write;
use std::fs::File;
use std::io::{ self, BufRead, BufReader };

pub fn read_file_contents(file_path: &str) -> Result<String, &'static str>  {

    let read_result = fs::read_to_string(file_path);

    match read_result{
        Ok(contents) => Ok(contents.to_string()),
        _ => Err("Could not open file {file_path}"),
    }
   
}

//returns an iterator over the lines of a file
pub fn read_lines(filename: String) -> io::Lines<BufReader<File>> {
    // Open the file in read-only mode.
    let file = File::open(filename).unwrap(); 
    // Read the file line by line, and return an iterator of the lines of the file.
    return io::BufReader::new(file).lines(); 
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
                        .arg(format!("echo {} > /sys/fs/cgroup/cgroup.subtree_control", val))
                        .status()
                        .expect("failed to modify active controllers file");

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

pub fn append_pid_command(pid: &str, cgroup: &str) -> Result<(), &'static str> {
    let status = RootCommand::new("sh")
                        .arg("-c")
                        .arg(format!("echo {} > \"/sys/fs/cgroup/{}/cgroup.procs\"", pid, cgroup))
                        .status()
                        .expect("failed to add pid to cgroup");

    match status.code() {
        Some(code) if code == 0 => {
            println!("{}", "Success".green());
            Ok(())
        },
        Some(code) => {
            println!("{}", "Error".red());
            Err("Could add pid to cgroup")
        },
        _  => Err("Could not add pid to cgroup"),
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



pub fn remove_cgroup(name: &str) -> Option<Cgroup>{
  
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
            Some(removed_cgroup)
        },
        Some(code) => {
            println!("{} not delete cgroup {name} with exit status {code}", "Error".red());
            None
        },
        _  => None,
    }
}




//creates a file string of the form
//rmdir {path_cgroup1} && rmdir {path_cgroup2}...
//to allow for bulk deletion of cgroups as specified
//in existing_cgroups.json additionally updates 
//existing cgroups.json so that 
pub fn bulk_remove_cgroup(cgroups: Vec<String>) {

    let mut command_string = "rmdir ".to_string();

    let append_string = " && rmdir ";

    let mut cgroups_iter = cgroups.iter().peekable();

    while let Some(cgroup) = cgroups_iter.next()  {
        
        for c in CGROUPROOT.chars() {
            command_string.push(c);
        }
        command_string.push('/');
        for c in cgroup.chars() { 
            command_string.push(c);
        }
        if cgroups_iter.peek().is_some() {
            for c in append_string.chars() {
                command_string.push(c);
            }
        }
    }
    
    let status = RootCommand::new("sh")
                .arg("-c")
                .arg(command_string)
                .status()
                .expect("failed to remove cgroups");

    match status.code() {
        Some(code) if code == 0 => {
            println!("{} removed cgroups with exit status {code}", "Success".green());
            ()
        },
        Some(code) => {
            println!("{} could not delete cgroups with exit status {code}", "Error".red());
            ()
        },
        _  => (),
    }


    
    println!("{:?}", cgroups);

    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .open("./existing_cgroups.json")
        .unwrap();

    

    //read contents of json file before modifying
    let lines = read_lines("./existing_cgroups.json".to_string());

    drop(file);

    fs::remove_file("existing_cgroups.json").expect("Couldn't delete existing_cgroups.json");

    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .open("./existing_cgroups.json")
        .unwrap();

    //only add cgroups that weren't deleted
    for line in lines {

        // println!("{}", line.unwrap());
        let deserialized: Cgroup = serde_json::from_str(&line.unwrap()).unwrap();
        
        println!("{:?}", deserialized);
        
        
        if deserialized.delete == 0 {
            writeln!(file, "{{\"name\":\"{}\",\"delete\":{}}}", deserialized.name, deserialized.delete).expect("couldn't rewrite to existing_cgroups.json");
        }
    }



}



//runs on shutdown either from clean user controlled input
//or when receiving a ctrl+c (a.k.a. SIGINT signal)
pub fn file_cleanup(cgroups: & Vec<Cgroup>) {
    
    
    //delete existing file since cgroups is the most up to date
    fs::remove_file("existing_cgroups.json").expect("Couldn't delete existing_cgroups.json");

    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .open("./existing_cgroups.json")
        .unwrap();

    //recreate file with up to date data
    for cgroup in cgroups {

        writeln!(file, "{{\"name\":\"{}\",\"delete\":{}}}", cgroup.name, cgroup.delete).expect("couldn't rewrite to existing_cgroups.json");
        
    }

        
    
}


