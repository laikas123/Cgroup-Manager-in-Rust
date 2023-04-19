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

    let mut active_controllers = print_startup_info_modify_controllers();

    let mut cgroups: Vec<Cgroup> = Vec::new();

    loop {
        match top_level_loop(&mut cgroups, &mut active_controllers){
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

    modify_controllers_loop(mut None::<Vec<String>>)

}






