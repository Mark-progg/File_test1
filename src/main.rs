use std::fs::File;
use std::io::{Write, BufReader, BufRead, Error};
use std::{thread, time};
use std::time::Duration;

fn Input_thread () {

}

fn Output_thread () {
    
}

fn Control_thread () {
    
}

fn main() -> Result<(), Error> {

    let path1 = "Text1.txt";
    let path2 = "Text2.txt";

    let mut line = String::new();
    println!("Enter path to Read:");
    let b1 = std::io::stdin().read_line(&mut line).unwrap();
    println!("Entered path is:{}", line);
    println!("No of bytes:{}", b1);


    let Input_thread = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    let Output_thread = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    let Control_thread = thread::spawn(|| {
            println!("Control");
    });

    Control_thread.join().unwrap();

    Ok(())

}
