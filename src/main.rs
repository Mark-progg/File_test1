use std::fs::File;
use std::io::{Write, BufReader, BufRead, Error};
use std::{thread, time};
use std::time::Duration;
use std::fs::OpenOptions;
use std::io::Read;
use std::fs;
use std::io::ErrorKind;


fn main() -> Result<(), Error> {

    let mut path_input = String::new(); //ввод контейнера 1 
    println!("Enter path name to Input:");
    std::io::stdin().read_line(&mut path_input).unwrap();

    let mut path_output = String::new(); //ввод контейнера 2
    println!("Enter path name to Output:");
    std::io::stdin().read_line(&mut path_output).unwrap();

    let output_thread = thread::spawn(move|| {
        loop{

            let mut line_output = String::new(); //взяли строчку у юзера
            std::io::stdin().read_line(&mut line_output).unwrap();

            let mut output_file = match File::create(&mut path_output.trim()) {
                Ok(file) => file,
                Err(error) => panic!("Problem creating the file: {:?}", error),
            };

            output_file.write_all(line_output.as_bytes()).expect("Write in file failed");
            thread::sleep(Duration::from_millis(1000));
            fs::remove_file(&mut path_output.trim()).unwrap();
        };
    });

    let input_thread = thread::spawn(move || {
        loop {
            let mut input_file = match File::open(&mut path_input.trim()) {
                Ok(file) => file,
                Err(error) => match error.kind() {
                    ErrorKind::NotFound => {
                        thread::sleep(Duration::from_millis(1));
                        continue;
                    },
                    other_error => {
                        panic!("Problem opening the file: {:?}", other_error)
                    }
                },
            };
            let mut input_line = String::new(); //создание строки в которую загоняем строку из файла
            input_file.read_to_string(&mut input_line).unwrap();
            println!("{}", input_line);
            thread::sleep(Duration::from_millis(1000));

        };
    });

    let control_thread = thread::spawn(|| {
        loop {
        
            match File::open("X.txt") {
                Ok(file) => break,
                Err(error) => match error.kind() {
                    ErrorKind::NotFound => {
                        thread::sleep(Duration::from_millis(500));
                        continue;
                    },
                    other_error => {
                        panic!("Problem opening the file: {:?}", other_error)
                    }
                },
            };

            };
    });

    control_thread.join().unwrap();
    println!("End programm");
    Ok(())
}



/*
    let mut line = String::new();  //считать строку и вывести ее в консоль
    println!("Enter message to write in file:");
    let b1 = std::io::stdin().read_line(&mut line).unwrap();
    println!("Entered message is:{}", line);
    println!("No of bytes:{}", b1); //конец упражнения

    let mut output = OpenOptions::new().read(true).write(true).create(true).open(&path_output.trim())?;  //запись в файл(+его создание)
    output.write_all(line.as_bytes()).expect("Write in file failed");



    let mut output2 = File::open(&path_output.trim()).unwrap(); //не знаю почему чтобы считать из файла нужно создать новый дексриптор 
    let mut output_line = String::new(); //создание строки в которую загоняем строку из файла
    output2.read_to_string(&mut output_line).unwrap();
    println!("In the file:{:?}", &mut output_line);
*/