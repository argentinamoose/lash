use std::env;
use std::io;
use std::fs;
use std::path::PathBuf;

fn main() {
    loop{
        println!(">");
        let current_dir = env::current_dir().unwrap(); //tells program what directory we are in
        let msg: &str = "Invalid input"; //error message
        let mut input: String = String::new(); //user input
        io::stdin().read_line(&mut input).unwrap();
        let split_input = input.split_whitespace().collect::<Vec<&str>>(); //splits input into a command and arguments
        match split_input[0].trim().to_lowercase().as_str() { //finds the command being used and runs that function
            "exit" => break,
            "help" => help(),
            "echo" => echo(split_input[1]),
            "cd" => cd(split_input[1]).expect(msg),
            "mkdir" => mkdir(current_dir, split_input[1]).expect(msg),
            "pwd" => pwd(current_dir),
            "ls" => ls(current_dir),
            "touch" => touch(current_dir, split_input[1]).expect(msg),
            "rm" => rm(current_dir, split_input[1]).expect(msg),
            _ => no_command(split_input[0]),
        }
    }
}

fn no_command(input: &str) {
    println!("lash: command not found: {}", input) //repeats the input if nothing of use is found
}

fn help() { //shows available commands
    println!("Available commands:
        exit - exit the program
        help - display this message
        echo <text> - print text to console
        pwd - print the current directory
        cd <directory> - change directories
        ls - list current directory's contents
        mkdir <name> - create a directory with the given name
        touch <filename> - create a file with the given name
        rm <filename> - delete a file"
    );
}

fn echo(input: &str) {
    println!("{}", input); //prints the argument
}

fn cd(dir: &str) -> io::Result<()> {
    env::set_current_dir(&dir)?; //change the directory that we are working from
    Ok(())
}

fn pwd(current_dir: PathBuf) {
    println!("{}", current_dir.display()); //print the current directory
}

fn ls(current_dir: PathBuf) {
    for entry in std::fs::read_dir(&current_dir).unwrap() {
        match entry {
            Ok(value) => println!("{}", value.path().display()), //display each file and directory in the current directory
            Err(error) => println!("Error: {}", error), //displays error if there's an error   
        }
    }
}

fn touch(current_dir: PathBuf, input: &str) -> io::Result<()> {
    if input.starts_with('/') {
        fs::File::create(&input)?; // create a file at the absolute location provided
    }
    else {
        let path = format!("{}/{}", current_dir.display(), input);
        fs::File::create(&path)?; // create a file at the relative location provided
    }
    Ok(())
}

fn mkdir(current_dir: PathBuf, input: &str) -> io::Result<()> {
    if input.starts_with('/') {
        fs::create_dir_all(&input)?; //create a directory at the absolute location provided
    }
    else {
        let path = format!("{}/{}", current_dir.display(), input);
        fs::create_dir_all(&path)?; //create a directory at the relative location provided
    }
    Ok(())
}

fn rm(current_dir: PathBuf, input: &str) -> io::Result<()> {
    if input.starts_with('/') {
        fs::remove_file(&input)?; //delete the file at the absolute location provided
    }
    else {
        let path = format!("{}/{}", current_dir.display(), input);
        fs::remove_file(&path)?; //delete the file at the relative location provided
    }
    Ok(())
}