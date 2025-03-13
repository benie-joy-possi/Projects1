use std::env;
use sys_info;
fn main() {
    match env::current_dir() {
        Ok(path) =>   println!(" The current working directory is: {:?}",path),
     Err(_) => println!("we could not get the current working directory")    
    }
    match sys_info::os_type() {
        Ok(os_name) => println!("the OS name is: {}", os_name),
        Err(_) => println!("could not get the operatting system name")
        
    }
    match sys_info::cpu_num() {
        Ok(cpu_num) => println!("This computer has {} cpu cores", cpu_num),
        Err(_) => println!("could not get number of cpu cores")
        
    }
   
}
