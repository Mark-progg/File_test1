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
    let b1 = std::io::stdin().read_line(&mut path_Input).unwrap();



    let mut path_Output = String::new();
    println!("Enter path name to Output:");
    let b2 = std::io::stdin().read_line(&mut path_Output).unwrap();

    let Output_thread = thread::spawn(move|| {
        loop{

            let mut line_Output = String::new(); //взяли строчку у юзера
            let b3 = std::io::stdin().read_line(&mut line_Output).unwrap();

            let mut Output_file= File::create(&mut path_Output.trim());//создали файл

            let mut Output_file = match Output_file {
                Ok(file) => file,
                Err(error) => panic!("Problem creating the file: {:?}", error),
            };

            Output_file.write_all(path_Output.as_bytes()).expect("Write in file failed");
            thread::sleep(Duration::from_millis(1000));
            fs::remove_file(&mut path_Output.trim()).unwrap();
        };
    });

    let Input_thread = thread::spawn(move || {
        loop {
            let mut Input_file= File::open(&mut path_Input.trim());

            let mut Input_file = match Input_file {
                Ok(file) => {
                    //thread::sleep(Duration::from_millis(3));
                    file},
                Err(error) => match error.kind() {
                    ErrorKind::NotFound => {
                        thread::sleep(Duration::from_millis(1));
                        //println!("Input_thread: Err(): ErrorKind::NotFound ");
                        continue;
                    },
                    other_error => {
                        panic!("Problem opening the file: {:?}", other_error)
                    }
                },
            };
            let mut input_line = String::new(); //создание строки в которую загоняем строку из файла
            Input_file.read_to_string(&mut input_line).unwrap();
            println!("{}", input_line);
            thread::sleep(Duration::from_millis(1000));

        };
    });




    let Control_thread = thread::spawn(|| {
        loop {
        
            let mut Control_file= File::open("X.txt");

            let mut Control_file = match Control_file {
                Ok(file) => {
                    break;
                    file},
                Err(error) => match error.kind() {
                    ErrorKind::NotFound => {
                        thread::sleep(Duration::from_millis(500));
                        //println!("Control_thread: Err(): ErrorKind::NotFound ");
                        continue;
                    },
                    other_error => {
                        panic!("Problem opening the file: {:?}", other_error)
                    }
                },
            };

            };
    });

    Control_thread.join().unwrap();
    println!("End programm");
    Ok(())
}

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