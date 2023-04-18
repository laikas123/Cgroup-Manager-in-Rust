use std::process::Command;
use std::fs::OpenOptions;
use text_colorizer::*;
use std::io::prelude::*;
use runas::Command as RootCommand;
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



enum CgroupTypes{
    Cpu,
    Mem,
    Pid
}

struct CgroupInfo {
    name: String,
    ctype: CgroupTypes,
    files: Vec<String>
}

impl CgroupInfo {

    fn new(name: String, ctype: CgroupTypes) -> Self {
        
        let mut files = Vec::<String>::new();
        
        match ctype {
            CgroupTypes::Cpu  => {
                files.push("cpu.stat".to_string());
                CgroupInfo{name: name, ctype: CgroupTypes::Cpu, files: files}
            },
            CgroupTypes::Mem => {
                files.push("memory.current".to_string());
                files.push("memory.min".to_string());
                files.push("memory.max".to_string());
                files.push("memory.stat".to_string());
                CgroupInfo{name: name, ctype: CgroupTypes::Mem, files: files}
            }
            CgroupTypes::Pid => {
                files.push("pids.current".to_string());
                files.push("pids.max".to_string());
                CgroupInfo{name: name, ctype: CgroupTypes::Pid, files: files}
            }
        }
    }


    fn get(&self, filename: &str) -> Result<String, &'static str>{

        
        let output = Command::new("/bin/cat")
                .arg(format!("/sys/fs/cgroup/{}/{}", self.name, filename))
                .output()
                .expect("failed to execute get_current for cgroup {self.name}");


        println!("{}", format!("/sys/fs/cgroup/{}/{}", filename, self.name));

        match std::str::from_utf8(&output.stdout) {
            Ok(s) => Ok(s.to_string()),
            Err(e) => Err("Couldn't read file"),
        }
        
            
        

    

        

    }

    fn set(&self, filename: &str, val: &str) {
        
        
        let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(format!("/sys/fs/cgroup/{}/{}", &self.name, filename))
        .unwrap();

        println!("/sys/fs/cgroup/{}/{}", &self.name, filename);
        if let Err(e) = write!(file, "{}", val) {
            eprintln!("Couldn't write to file: {}", e);
        }
    
            

        
    }


    fn add_pid(&self, pid: &str) -> Result<(), &'static str> {
        let status = RootCommand::new("sh")
                        .arg("-c")
                        .arg(format!("./add_pid_cgroup.sh {} {}", pid, &self.name))
                        .status()
                        .expect("failed to execute add_pid_cgroup.sh");

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
            _  => Err("Could not create group"),
        }

    }

    fn log_current(&self, filename: &str) {

        
        let file_data = self.get(filename).expect("error");

        let timestamp = generate_timestamp_string();

        let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(format!("{LOGDIR}/{}/{}", &self.name, filename))
        .unwrap();

        if let Err(e) = write!(file, "{}", format!("{} {}", timestamp, file_data)) {
            eprintln!("Couldn't write to file: {}", e);
        }
       
    }
}







//note input.clear() is necessary to clear input buffer
//and input.pop removes unwanted new line
fn main() {

    let test2 = CgroupInfo::new("bloopy".to_string(), CgroupTypes::Mem);

    for filename in &test2.files {
        println!("{}", filename);
    }




    let mut input = String::new();

    loop {

        //get user input
        println!("What would you like to do? \n(0) Create cgroup \n(1) Add to existing cgroup?\n(2) Set max memory for cgroup?\n");
        input = get_user_input(input);

        //act based on user choice
        if input == UserChoice::NEWCGROUP {
            println!("Enter new cgroup name:");
            input = get_user_input(input);
            create_cgroup(&input);
        }else if input == UserChoice::ADDPID {
            println!("Enter pid followed by a space followed by cgroup name:");
            input = get_user_input(input);
            let pid_and_cgroup: Vec<&str> = input.split(" ").collect();
            let pid = pid_and_cgroup[0];
            let cgroup = pid_and_cgroup[1];
            // add_pid_to_cgroup(pid, cgroup);
        }else if input == UserChoice::SETMAXMEM {
            println!("Enter max memory in bytes followed by a space followe by cgroup name:");
            input = get_user_input(input);
            let bytes_and_cgroup: Vec<&str> = input.split(" ").collect();
            let bytes = bytes_and_cgroup[0];
            let cgroup = bytes_and_cgroup[1];
        }else{
            println!("\nError unknown choice please try again\n");
            continue;
        }

        
    }


}


fn create_cgroup(name: &str) -> Result<(), &'static str> {
    let status = RootCommand::new("mkdir")
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




fn get_user_input(mut input: String) -> String {
    input.clear();
    std::io::stdin().read_line(&mut input).unwrap();
    input.pop();
    input
}







