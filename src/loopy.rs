use super::moddy::*;
use super::cgroup::*;
use text_colorizer::*;
use std::fs;
use std::process;
use super::globals::*;
use super::grabby::*;


struct UserChoice {
}

impl UserChoice {
    const MANAGECONTROLLERS: &'static str = "0";
    const CREATECGROUP: &'static str = "1";
    const MANAGECGROUP: &'static str = "2";
    const EXIT3: &'static str = "3";
    const READCGROUPSETTING: &'static str = "0";
    const UPDATECGROUPSETTING: &'static str = "1";
    const DELETECGROUP: &'static str = "2";
    const ADDPID: &'static str = "3";
    const GOBACK4: &'static str = "4";
}


pub fn get_user_input(mut input: String) -> String {
    input.clear();
    std::io::stdin().read_line(&mut input).unwrap();
    input.pop();
    input
}


//main loop handling user input to command line which calls open sub loops
pub fn top_level_loop(cgroups: &mut Vec<Cgroup>, controllers: &mut Vec<String>) -> Option<Cgroup>{

    let mut input = String::new();

    loop {

        //get user input
        println!("{} \n(0) Manage Controllers? \n(1) Create a Cgroup?\n(2) Manage a Cgroup? \n(3) Exit? \n", "What would you like to do?".blue());
        input = get_user_input(input);

        //act based on user choice
        if input == UserChoice::MANAGECONTROLLERS {
            modify_controllers_loop(Some(controllers));
            return None;
        }else if input == UserChoice::CREATECGROUP{
            println!("Enter new cgroup name:");
            input = get_user_input(input);
            match create_cgroup(&input.to_string()){
                Ok(cgr) => return Some(cgr),
                _ => return None,
            }
        }else if input == UserChoice::MANAGECGROUP {
            loop{
                println!("{} \n(0) Read Cgroup Setting? \n(1) Update Cgroup Setting?\n(2) Delete Cgroup? \n(3) Add pid to Cgroup? \n(4) Go Back?\n", "What would you like to do?".blue());
                input = get_user_input(input);
                if input == UserChoice::READCGROUPSETTING {
                    read_cgroup_settings_loop(cgroups, controllers);
                }else if input == UserChoice::UPDATECGROUPSETTING {
                    update_cgroup_settings_loop(cgroups, controllers);
                }else if input == UserChoice::DELETECGROUP {
                    return delete_cgroup_loop(cgroups);
                }else if input == UserChoice::ADDPID{
                    add_pid_loop(cgroups);
                }else if input == UserChoice::GOBACK4{
                    break;
                }else{
                    println!("\nUnknown input please try again..\n");
                    continue;
                }
            }
            return None;
        //return cgroup with impossible name
        //to confirm exit
        }else if input == UserChoice::EXIT3{
            return Some(Cgroup::new(">>".to_string()));
        }else{
            println!("\n{} unknown choice please try again\n", "Error".red());
            continue;
        }

        
    }
}


//to read from cgroup settings files for a specific cgroup/controller combo
pub fn read_cgroup_settings_loop(cgroups: &mut Vec<Cgroup>, controllers: &Vec<String>){


    let mut controller_index = 0;
    let mut controller_index_tuple_vec = Vec::new();

    //see which controller user wants to read from
    print!("\n{}", "Available Controllers:\n".blue());
    for controller in controllers {
        println!("({}) {} ", controller_index, controller);
        controller_index_tuple_vec.push((controller_index, controller.to_string()));
        controller_index += 1;
    }
    let mut found = 0;
    let mut controller = String::new();
    let mut user_index = String::new();
    while found == 0 {
        println!("\n\nType the number corresponding to the controller you wish to read from (type .. to go back):");
        user_index = get_user_input(user_index);
        if user_index == ".." {
            return;
        }
        for i in 0..controller_index_tuple_vec.len() {
            if user_index == controller_index_tuple_vec[i].0.to_string(){
                found = 1;
                controller =controller_index_tuple_vec[i].1.to_string();
                break;
            }
        }

        if found == 0 {
            println!("Unkown choice please try again");
        }
    }


    let mut cgroup_index = 0;
    let mut cgroup_index_tuple_vec = Vec::new();

    //see which cgroup user wants to read from
    print!("\n{}", "Available Cgroups:\n".blue());
    for cgroup in &mut *cgroups {
        println!("({}) {} ", cgroup_index, cgroup.name);
        cgroup_index_tuple_vec.push((cgroup_index, cgroup.name.to_string()));
        cgroup_index += 1;
    }
    found = 0;
    let mut cgroup = String::new();
    user_index = String::new();
    while found == 0 {
        println!("\n\nType the number corresponding to the cgroup you wish to read from (type .. to go back):");
        user_index = get_user_input(user_index);
        if user_index == ".." {
            return;
        }
        for i in 0..cgroup_index_tuple_vec.len() {
            if user_index == cgroup_index_tuple_vec[i].0.to_string(){
                found = 1;
                cgroup = cgroup_index_tuple_vec[i].1.to_string();
                break;
            }
        }

        if found == 0 {
            println!("Unkown choice please try again");
        }
    }


    //filter settings files based on cgroup/controller
    let paths = fs::read_dir(format!("{CGROUPROOT}/{cgroup}")).unwrap();
    let mut path_strings = Vec::new();
    for path in paths {
        path_strings.push(path.unwrap().path().display().to_string());
    }
    let mut filtered_paths = Vec::new();
    for path in &path_strings {
        if path.contains(&controller) {
            filtered_paths.push(path);
        }
    }

    let mut s_file_index = 0;
    let mut s_file_index_tuple_vec = Vec::new();

    //see which settings file to read from 
    println!("\n{}\n", "Available Settings Files\n".blue());
    for s_file in &filtered_paths {
        println!("({}) {} ", s_file_index, s_file);
        s_file_index_tuple_vec.push((s_file_index, s_file.to_string()));
        s_file_index += 1;
    }
    found = 0;
    let mut s_file = String::new();
    user_index = String::new();
    while found == 0 {
        println!("\n\nType the number corresponding to the settings file you wish to read from (type .. to go back):");
        user_index= get_user_input(user_index);
        if user_index == ".." {
            return;
        }
        for i in 0..s_file_index_tuple_vec.len() {
            if user_index == s_file_index_tuple_vec[i].0.to_string(){
                found = 1;
                s_file = s_file_index_tuple_vec[i].1.to_string();
                break;
            }
        }

        if found == 0 {
            println!("Unkown choice please try again");
        }
    }


    match read_file_contents(&s_file) {
        Ok(contents) => println!("\n{}{}", "Value is: ".green(), contents),
        _ => println!("No data from file {s_file}"),
    }

}

//to update cgroup settings files for a specific cgroup/controller combo
pub fn update_cgroup_settings_loop(cgroups: &mut Vec<Cgroup>, controllers: &Vec<String>){


    let mut controller_index = 0;
    let mut controller_index_tuple_vec = Vec::new();

    //see which controller user wants to update
    print!("\n{}", "Available Controllers:\n".blue());
    for controller in controllers {
        println!("({}) {} ", controller_index, controller);
        controller_index_tuple_vec.push((controller_index, controller.to_string()));
        controller_index += 1;
    }
    let mut found = 0;
    let mut controller = String::new();
    let mut user_index = String::new();
    while found == 0 {
        println!("\n\nType the number corresponding to the controller you wish to update (type .. to go back):");
        user_index = get_user_input(user_index);
        if user_index == ".." {
            return;
        }
        for i in 0..controller_index_tuple_vec.len() {
            if user_index == controller_index_tuple_vec[i].0.to_string(){
                found = 1;
                controller =controller_index_tuple_vec[i].1.to_string();
                break;
            }
        }

        if found == 0 {
            println!("Unkown choice please try again");
        }
    }

    let mut cgroup_index = 0;
    let mut cgroup_index_tuple_vec = Vec::new();

    //see which cgroup user wants to update
    print!("\n{}", "Available Cgroups:\n".blue());
    for cgroup in &mut *cgroups {
        println!("({}) {} ", cgroup_index, cgroup.name);
        cgroup_index_tuple_vec.push((cgroup_index, cgroup.name.to_string()));
        cgroup_index += 1;
    }
    found = 0;
    let mut cgroup = String::new();
    user_index = String::new();
    while found == 0 {
        println!("\n\nType the number corresponding to the cgroup you wish to read from (type .. to go back):");
        user_index = get_user_input(user_index);
        if user_index == ".." {
            return;
        }
        for i in 0..cgroup_index_tuple_vec.len() {
            if user_index == cgroup_index_tuple_vec[i].0.to_string(){
                found = 1;
                cgroup = cgroup_index_tuple_vec[i].1.to_string();
                break;
            }
        }

        if found == 0 {
            println!("Unkown choice please try again");
        }
    }


    //filter settings files based on cgroup/controller
    let paths = fs::read_dir(format!("{CGROUPROOT}/{cgroup}")).unwrap();
    let mut path_strings = Vec::new();
    for path in paths {
        path_strings.push(path.unwrap().path().display().to_string());
    }
    let mut filtered_paths = Vec::new();
    for path in &path_strings {
        if path.contains(&controller) {
            filtered_paths.push(path);
        }
    }


    let mut s_file_index = 0;
    let mut s_file_index_tuple_vec = Vec::new();

    //see which settings file to update
    println!("\n{}\n", "Available Settings Files\n".blue());
    for s_file in &filtered_paths {
        println!("({}) {} ", s_file_index, s_file);
        s_file_index_tuple_vec.push((s_file_index, s_file.to_string()));
        s_file_index += 1;
    }
    found = 0;
    let mut s_file = String::new();
    let mut user_index = String::new();
    while found == 0 {
        println!("\n\nType the number corresponding to the settings file you wish to update (type .. to go back):");
        user_index= get_user_input(user_index);
        if user_index == ".." {
            return;
        }
        for i in 0..s_file_index_tuple_vec.len() {
            if user_index == s_file_index_tuple_vec[i].0.to_string(){
                found = 1;
                s_file = s_file_index_tuple_vec[i].1.to_string();
                break;
            }
        }

        if found == 0 {
            println!("Unkown choice please try again");
        }
    }

    println!("\n\nType the new settings value for the file:");
    let mut data = String::new();
    data = get_user_input(data);

    match update_file_contents(&s_file, &data) {
        Ok(()) => (),
        _ => println!("No data from file {s_file}"),
    }

}


//to delete a specific cgroup
fn delete_cgroup_loop(cgroups: &mut Vec<Cgroup>) -> Option<Cgroup> {
    let mut cgroup_index = 0;
    let mut cgroup_index_tuple_vec = Vec::new();

    //see which cgroup user wants to choose from
    print!("\n{}", "Available Cgroups:\n".blue());
    for cgroup in &mut *cgroups {
        println!("({}) {} ", cgroup_index, cgroup.name);
        cgroup_index_tuple_vec.push((cgroup_index, cgroup.name.to_string()));
        cgroup_index += 1;
    }
    let mut found = 0;
    let mut cgroup = String::new();
    let mut user_index = String::new();
    while found == 0 {
        println!("\n\nType the number corresponding to the cgroup you wish to read from (type .. to go back):");
        user_index = get_user_input(user_index);
        if user_index == ".." {
            return None;
        }
        for i in 0..cgroup_index_tuple_vec.len() {
            if user_index == cgroup_index_tuple_vec[i].0.to_string(){
                found = 1;
                cgroup = cgroup_index_tuple_vec[i].1.to_string();
                break;
            }
        }

        if found == 0 {
            println!("Unkown choice please try again");
        }
    }

    remove_cgroup(&cgroup)

}


//to modify active cgroup controllers 
#[allow(dead_code)]
#[allow(unused_assignments)]
pub fn modify_controllers_loop(current_controllers: Option<&mut Vec<String>>) -> Option<Vec<String>> {

    let mut avail_controllers: String = "".to_string();

    match read_file_contents(&format!("{CGROUPROOT}/cgroup.controllers")){
        Ok(contents) => {
            avail_controllers = contents.to_string();
        }
        _ => {
            println!("{} could not read {CGROUPROOT}/cgroup.controllers, please check that cgroups are mounted correctly... Terminating...", "Error".red());
            process::exit(1);
        },
    }

    let mut active_controllers: String = "".to_string();

    match read_file_contents(&format!("{CGROUPROOT}/cgroup.subtree_control")){
        Ok(contents) => {
            active_controllers = contents.to_string();
            println!("{} {}", "Active Controllers:".blue(), &active_controllers);
        },
        _ => {
            println!("{} could not read {CGROUPROOT}/cgroup.controllers, please check that cgroups are mounted correctly... Terminating...", "Error".red());
            process::exit(1);
        },
    }

    let mut input = String::new();

    println!("Here are a list of available controllers. Enter Y to activate or N to deactivate or L to leave as is.\n");

    let result = get_text_separated_by_substring(" ", &avail_controllers);

    let mut avail_controller_tuples = Vec::new();

    match result{
        Ok(avail_controller_vec) =>  {
            for i in 0..avail_controller_vec.len() {
                let mut controller = avail_controller_vec[i].to_string();
                if controller.ends_with('\n') {
                    controller.pop();
                }

                println!("Activate {}?", controller);
                loop{
                    input = get_user_input(input);
                    if input == "Y" || input == "y"{
                        modify_active_controller(&format!("+{}", &controller)).expect("error modifying active controllers");
                        break;
                    }else if input == "N" || input == "n"{
                        modify_active_controller(&format!("-{}", &controller)).expect("error modifying active controllers");
                        break;
                    }else if input == "L" || input == "l" {
                        break;
                    }else{
                        println!("Didn't understand that please try again");
                        continue;
                    }
                }

                avail_controller_tuples.push((i.to_string(), &avail_controller_vec[i]));
            }
        },
        _ => println!("{} could not read available controllers", "Error".red()),
    }

    //print updated active controllers
    match read_file_contents(&format!("{CGROUPROOT}/cgroup.subtree_control")){
        Ok(contents) => {
            active_controllers = contents.to_string();
            println!("\n{}\n{}", "Active Controllers:".blue(), &active_controllers);
        },
        _ => {
            println!("{} could not read {CGROUPROOT}/cgroup.subtree_control, please check that cgroups are mounted correctly... Terminating...", "Error".red());
            process::exit(1);
        },
    }

   

    match current_controllers {
        Some(mut_input) => {
            mut_input.clear();
            let active = get_text_separated_by_substring(" ", &active_controllers);
            match active{
                Ok(active_controller_vec) =>  {
                    for elem in active_controller_vec {
                        mut_input.push(elem);
                    }
                    None
                },
                _ => {
                    println!("{} could not get active controllers... Terminating", "Error".red());
                    process::exit(1);
                },
        
            }
        },
        None => {
            //return the active controllers
            let active = get_text_separated_by_substring(" ", &active_controllers);

            match active{
                Ok(active_controller_vec) =>  Some(active_controller_vec),
                _ => {
                    println!("{} could not get active controllers... Terminating", "Error".red());
                    process::exit(1);
                },

            }
        },
    }

    

    

}





//to add pid to a cgroup
pub fn add_pid_loop(cgroups: &mut Vec<Cgroup>) {
    let mut cgroup_index = 0;
    let mut cgroup_index_tuple_vec = Vec::new();

    //see which cgroup user wants to choose from
    print!("\n{}", "Available Cgroups:\n".blue());
    for cgroup in &mut *cgroups {
        println!("({}) {} ", cgroup_index, cgroup.name);
        cgroup_index_tuple_vec.push((cgroup_index, cgroup.name.to_string()));
        cgroup_index += 1;
    }
    let mut found = 0;
    let mut cgroup = String::new();
    let mut user_index = String::new();
    while found == 0 {
        println!("\n\nType the number corresponding to the cgroup you wish to read from (type .. to go back):");
        user_index = get_user_input(user_index);
        if user_index == ".." {
            return;
        }
        for i in 0..cgroup_index_tuple_vec.len() {
            if user_index == cgroup_index_tuple_vec[i].0.to_string(){
                found = 1;
                cgroup = cgroup_index_tuple_vec[i].1.to_string();
                break;
            }
        }

        if found == 0 {
            println!("Unkown choice please try again");
        }
    }

    let mut pid = String::new();
    loop {
        println!("\n\nType the pid to add to {} (type 000 to go back):", cgroup);
        pid = get_user_input(pid);
        let all_digits = pid.chars().all(char::is_numeric);
        if all_digits {
            if pid == "000" {
                return;
            }else{
                append_pid_command(&pid, &cgroup).expect("couldn't execute append pid command");
            }
        }else{
            println!("Invalid pid string, only numbers allowed... Please try again")
        }
    }
}
