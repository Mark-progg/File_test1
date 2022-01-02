use std::fs::File;
use std::io::{Write, BufReader, BufRead, Error};
use std::{thread, time};
use std::time::Duration;
use std::fs::OpenOptions;
use std::io::Read;
use std::fs;

use std::io::ErrorKind;

fn Input_thread () {

}

fn Output_thread () {
    
}

fn Control_thread () {
    
}

fn main() -> Result<(), Error> {

    let mut path_Input = String::new();
    println!("Enter path name to Input:");
    let b2 = std::io::stdin().read_line(&mut path_Input).unwrap();



    let mut path_Output = String::new();
    println!("Enter path name to Output:");
    let b3 = std::io::stdin().read_line(&mut path_Output).unwrap();


/*
    let mut line = String::new();  //считать строку и вывести ее в консоль
    println!("Enter message to write in file:");
    let b1 = std::io::stdin().read_line(&mut line).unwrap();
    println!("Entered message is:{}", line);
    println!("No of bytes:{}", b1); //конец упражнения

    let mut output = OpenOptions::new().read(true).write(true).create(true).open(&path_Output.trim())?;  //запись в файл(+его создание)
    output.write_all(line.as_bytes()).expect("Write in file failed");



    let mut output2 = File::open(&path_Output.trim()).unwrap(); //не знаю почему чтобы считать из файла нужно создать новый дексриптор 
    let mut output_line = String::new(); //создание строки в которую загоняем строку из файла
    output2.read_to_string(&mut output_line).unwrap();
    println!("In the file:{:?}", &mut output_line);
*/

    let Output_thread = thread::spawn(move|| {
        println!("Output_thread");


        
        thread::sleep(Duration::from_millis(1));
    });

    let Input_thread = thread::spawn(move || {
        loop {
        println!("{:?}", path_Input);
        let mut Input_file= File::open(&mut path_Input.trim());

        let mut Input_file = match Input_file {
            Ok(file) => {
                thread::sleep(Duration::from_millis(5000));
                file},
            Err(error) => match error.kind() {
                ErrorKind::NotFound => {
                    thread::sleep(Duration::from_millis(1000));
                    println!("Err() ErrorKind::NotFound ");
                    continue;
                },
                other_error => {
                    panic!("Problem opening the file: {:?}", other_error)
                }
            },
        };

    };
    });

    let Control_thread = thread::spawn(|| {
        println!("Control_thread finished the work ");
        thread::sleep(Duration::from_millis(10000));
    });

    Control_thread.join().unwrap();
    println!("End programm");

    Ok(())
}
