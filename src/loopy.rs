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
    const READCGROUPSETTING: &'static str = "0";
    const UPDATECGROUPSETTING: &'static str = "1";
    const DELETECGROUP: &'static str = "2";
    const GOBACK3: &'static str = "3";
}


pub fn get_user_input(mut input: String) -> String {
    input.clear();
    std::io::stdin().read_line(&mut input).unwrap();
    input.pop();
    input
}


pub fn top_level_loop(cgroups: &mut Vec<Cgroup>, controllers: &mut Vec<String>) -> Option<Cgroup>{

    let mut input = String::new();

    loop {

        //get user input
        println!("{} \n(0) Manage Controllers \n(1) Create a Cgroup?\n(2) Manage a Cgroup?\n", "What would you like to do?".blue());
        input = get_user_input(input);

        //act based on user choice
        if input == UserChoice::MANAGECONTROLLERS {

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
                println!("{} \n(0) Read Cgroup Setting \n(1) Update Cgroup Setting?\n(2) Delete Cgroup? \n(3) Go Back()\n", "What would you like to do?".blue());
                input = get_user_input(input);
                if input == UserChoice::READCGROUPSETTING {
                    read_cgroup_settings_loop(cgroups, controllers);
                }else if input == UserChoice::UPDATECGROUPSETTING {
                    update_cgroup_settings_loop(cgroups, controllers);
                }else if input == UserChoice::DELETECGROUP {
                    delete_cgroup_loop(cgroups);
                }else if input == UserChoice::GOBACK3{
                    break;
                }else{

                }
            }
            return None;
        }else{
            println!("\n{} unknown choice please try again\n", "Error".red());
            continue;
        }

        
    }
}


pub fn read_cgroup_settings_loop(cgroups: &mut Vec<Cgroup>, controllers: &Vec<String>){


    //see which controller user wants to read from
    print!("\n{}: ", "Available Controllers: ".blue());
    for controller in controllers {
        print!("{} ", controller);
    }
    let mut found = 0;
    let mut controller = String::new();
    while found == 0 {
        println!("\n\nType the name of the controller you wish to read from:");
        controller = get_user_input(controller);
        for elem in controllers{
            println!("{}", *elem);
            if controller == *elem{
                found = 1;
                break;
            }
        }

        if found == 0 {
            println!("Unkown choice please try again");
        }
    }

    //see which cgroup user wants to read from
    print!("\n{}: ", "Available Cgroups: ".blue());
    for cgroup in &mut *cgroups {
        print!("{} ", cgroup.name);
    }
    found = 0;
    let mut cgroup = String::new();
    while found == 0 {
        println!("\n\nType the name of the cgroup you wish to read from:");
        cgroup = get_user_input(cgroup);
        for i in 0..cgroups.len() {
            if cgroup == cgroups[i].name{
                found = 1;
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


    //see which settings file to read from 
    println!("\n{}\n", "Available Settings Files\n".blue());
    // println!("{:?}", cgroups);
    for s_file in &filtered_paths {
        println!("{} ", s_file);
    }
    found = 0;
    let mut s_file = String::new();
    while found == 0 {
        println!("\n\nType the name of the settings file you wish to read from:");
        s_file = get_user_input(s_file);
        for i in 0..filtered_paths.len() {
            if s_file == *filtered_paths[i]{
                found = 1;
                break;
            }
        }

        if found == 0 {
            println!("Unkown choice please try again");
        }
    }


    match read_file_contents(&s_file) {
        Ok(contents) => println!("\n{}", contents),
        _ => println!("No data from file {s_file}"),
    }

}


pub fn update_cgroup_settings_loop(cgroups: &mut Vec<Cgroup>, controllers: &Vec<String>){


    //see which controller user wants to choose from
    print!("\n{}: ", "Available Controllers: ".blue());
    for controller in controllers {
        print!("{} ", controller);
    }
    let mut found = 0;
    let mut controller = String::new();
    while found == 0 {
        println!("\n\nType the name of the controller you wish to read from:");
        controller = get_user_input(controller);
        for elem in controllers{
            println!("{}", *elem);
            if controller == *elem{
                found = 1;
                break;
            }
        }

        if found == 0 {
            println!("Unkown choice please try again");
        }
    }

    //see which cgroup user wants to choose from
    print!("\n{}: ", "Available Cgroups: ".blue());
    for cgroup in &mut *cgroups {
        print!("{} ", cgroup.name);
    }
    found = 0;
    let mut cgroup = String::new();
    while found == 0 {
        println!("\n\nType the name of the cgroup you wish to read from:");
        cgroup = get_user_input(cgroup);
        for i in 0..cgroups.len() {
            if cgroup == cgroups[i].name{
                found = 1;
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


    //see which settings file to update
    println!("\n{}\n", "Available Settings Files\n".blue());
    // println!("{:?}", cgroups);
    for s_file in &filtered_paths {
        println!("{} ", s_file);
    }
    found = 0;
    let mut s_file = String::new();
    while found == 0 {
        println!("\n\nType the name of the settings file you wish to read from:");
        s_file = get_user_input(s_file);
        for i in 0..filtered_paths.len() {
            if s_file == *filtered_paths[i]{
                found = 1;
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


fn delete_cgroup_loop(cgroups: &mut Vec<Cgroup>) {
    //see which cgroup user wants to  delete
    print!("\n{}: ", "Available Cgroups: ".blue());
    for cgroup in &mut *cgroups {
        print!("{} ", cgroup.name);
    }
    let mut found = 0;
    let mut cgroup = String::new();
    while found == 0 {
        println!("\n\nType the name of the cgroup you wish to read delete:");
        cgroup = get_user_input(cgroup);
        for i in 0..cgroups.len() {
            if cgroup == cgroups[i].name{
                found = 1;
                break;
            }
        }

        if found == 0 {
            println!("Unkown choice please try again");
        }
    }

    remove_cgroup(&cgroup);

}





pub fn modify_controllers_loop(current_controllers: mut Option<Vec<String>>) -> Vec<String> {

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
                        modify_active_controller(&format!("+{}", &controller));
                        break;
                    }else if input == "N" || input == "n"{
                        modify_active_controller(&format!("-{}", &controller));
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
            println!("{} {}", "Active Controllers:".blue(), &active_controllers);
        },
        _ => {
            println!("{} could not read {CGROUPROOT}/cgroup.subtree_control, please check that cgroups are mounted correctly... Terminating...", "Error".red());
            process::exit(1);
        },
    }

    // if current_controllers.is_none() {
    //     current_controllers.clear();
        
    // }

    //return the active controllers
    let active = get_text_separated_by_substring(" ", &active_controllers);

    match active{
        Ok(active_controller_vec) =>  active_controller_vec,
        _ => {
            println!("{} could not get active controllers... Terminating", "Error".red());
            process::exit(1);
        },

    }

    

}






