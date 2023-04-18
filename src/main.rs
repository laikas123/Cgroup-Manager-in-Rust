use text_colorizer::*;
use std::process;

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

    let active_controllers = print_startup_info_modify_controllers();

    let mut cgroups: Vec<Cgroup> = Vec::new();

    loop {
        match top_level_loop(&mut cgroups, &active_controllers){
            Some(new_cgroup) => {
                if new_cgroup.delete == 0{
                    cgroups.push(new_cgroup);
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
    


    // let test2 = Cgroup::new("bloopy".to_string(), CgroupTypes::Mem);

    
    
    

  


}












fn print_startup_info_modify_controllers() -> Vec<String> {

    println!("\n\n\n{}", "Starting up...".purple());

    println!("{} {CGROUPROOT}/\n\n(To change please modify global variable: {} in globals.rs file)\n", "Assumed Root Cgroup Directory:".blue(), "\"CGROUPROOT\"".yellow());
    println!("{} {USERNAME}\n\n(To change please modify global variable: {} in globals.rs file)\n", "Assumed Username:".blue(), "\"USERNAME\"".yellow());

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






