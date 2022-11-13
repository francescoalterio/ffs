use std::fs;
use std::env;

fn main() {
    let myArgs: Vec<String> = env::args().collect();
    excecute_functions(&myArgs[1], &myArgs[2]);
}

fn excecute_functions(action: &String, name: &String) {
    match action.as_str() {
        "cd" => create_dir(name),
        "dd" => delete_dir(name),
        "df" => delete_file(name),
        "cf" => create_file(name),
        "rn" => rename(name),
        "dad" => delete_all_dir(name),
        _=> println!("Unsupported functionality")
    }
}

fn create_dir(name: &String) {
    fs::create_dir(name);
    println!("DIR CREATED {}", name);
}
    
fn delete_dir(name: &String) {
    fs::remove_dir(name);
    println!("DIR DELETED {}", name);
}

fn delete_file(name: &String) {
    fs::remove_file(name);
    println!("FILE DELETED {}", name);
}

fn create_file(name: &String) {
    fs::File::create(name);
    println!("FILE CREATED {}", name);
}

fn rename(last_name: &String) {
    let myArgs: Vec<String> = env::args().collect();
    let new_name = &myArgs[3];
    fs::rename(last_name, new_name);
    println!("FILE RENAMED {}", last_name);
}

fn delete_all_dir(name: &String) {
    fs::remove_dir_all(name);
    println!("ALL DIR DELETED {}", name);
}