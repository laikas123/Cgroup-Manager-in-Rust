use text_colorizer::*;
use std::process;
use std::thread;
use std::fs::File;
use std::fs;
use std::io::{ self, BufRead, BufReader };
use std::fs::OpenOptions;
use std::io::Write;

pub mod moddy;
use moddy::*;


pub mod grabby;
use grabby::*;

pub mod loopy;
use loopy::*;

pub mod cgroup;
use cgroup::*;

pub mod globals;
use globals::*;


//note input.clear() is necessary to clear input buffer
//and input.pop removes unwanted new line
fn main() {

    
    

    let mut active_controllers = print_startup_info_modify_controllers();

    let mut cgroups: Vec<Cgroup> = Vec::new();

    let cgroups_ref = &cgroups;

    add_remove_existing_cgroups(&mut cgroups);

    

    loop {
        match top_level_loop(&mut cgroups, &mut active_controllers){
            Some(new_cgroup) => {
                if new_cgroup.delete == 0 && &new_cgroup.name != ">>" {
                    cgroups.push(new_cgroup);
                }else if &new_cgroup.name == ">>" {
                    println!("{}", "Exiting cleanly".blue());
                    file_cleanup(&cgroups);
                    process::exit(0);
                }else{
                    for i in 0..cgroups.len(){
                        if cgroups[i].name == new_cgroup.name {
                            cgroups.remove(i);
                            break;
                        }
                    }
                }
            },
            _ => continue,
        }
    }
    


}

//TODO
//make sure that on single creates and deletes the .json file is up to date... Since handling ctrl+c is a pain...
//finish documenting and put a bow on it for now

fn print_startup_info_modify_controllers() -> Vec<String> {

    println!("\n\n\n{}", "Starting up...".purple());

    println!("{} {CGROUPROOT}/\n\n(To change please modify global variable: {} in globals.rs file)\n", "Assumed Root Cgroup Directory:".blue(), "\"CGROUPROOT\"".yellow());
    println!("{} {USERNAME}\n\n(To change please modify global variable: {} in globals.rs file)\n", "Assumed Username:".blue(), "\"USERNAME\"".yellow());

    match modify_controllers_loop(None) {
        Some(output) => output,
        _ => {
            println!("{} couldn't update controllers properly terminating...", "Error".red());
            process::exit(1);
        }
    }

}



//runs on startup, used to import existing cgroups 
//to be managed, and delete those the user no longer wants
fn add_remove_existing_cgroups(cgroups: &mut Vec<Cgroup>) {

    let mut delete_list: Vec<String> = Vec::new();

    // Stores the iterator of lines of the file in lines variable.
    let lines = read_lines("./existing_cgroups.json".to_string());
    // Iterate over the lines of the file, and in this case print them.
    for line in lines {
        // println!("{}", line.unwrap());
        let deserialized: Cgroup = serde_json::from_str(&line.unwrap()).unwrap();
        
        //check the cgroup actually exists on the device
        let mut exists = 0;
        let paths = fs::read_dir(format!("{CGROUPROOT}")).unwrap();
        let mut path_strings = Vec::new();
        for path in paths {
            path_strings.push(path.unwrap().path().display().to_string());
        }
        for path in &path_strings {
            if path.contains(&deserialized.name) {
                exists = 1;
            }
        }

        if exists == 0 {
            println!("{} cgroup {} doesn't exist on device, please remove from {}", "Warning".truecolor(255, 165, 0), &deserialized.name, "existing_cgroups.json".yellow());
            continue;
        }
        if deserialized.delete == 1 {
            delete_list.push(deserialized.name.to_string());
        }else{
            cgroups.push(deserialized);
        }
    }


    println!("{}", "Current cgroups: ".blue());
    for cgroup in cgroups {
        print!(" {}", cgroup.name.to_string());
    }

    println!("\n\n{}", "Cgroups to delete: ".truecolor(255, 165, 0));
    for cgroup in &delete_list {
        print!(" {}", cgroup);
    }
    
    loop{
        println!("\nType Y to confirm and N to stop delete and exit\n");
        let mut input = String::new();
        input = get_user_input(input);
        if input == "Y" || input == "y"{
            bulk_remove_cgroup(delete_list);
            break;
        }else if input == "N" || input == "n"{
            break;
        }else{
            println!("Error unknown choice please try again...\n");
            continue;
        }
    }

    
}


//runs on shutdown either from clean user controlled input
//or when receiving a ctrl+c (a.k.a. SIGINT signal)
fn file_cleanup(cgroups: & Vec<Cgroup>) {
    
    
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






