use std::fs::File;
use std::io::{Write, BufReader, BufRead, Error};
use std::{thread, time};
use std::time::Duration;
use std::fs::OpenOptions;
use std::io::Read;

fn Input_thread () {

}

fn Output_thread () {
    
}

fn Control_thread () {
    
}

fn main() -> Result<(), Error> {

    let path2 = "Text2.txt";
    let path3 = "Text3.txt";

    let mut path1 = String::new();
    println!("Enter path name to write in file:");
    let b2 = std::io::stdin().read_line(&mut path1).unwrap();
    //path1.trim();

    let mut line = String::new();  //считать строку и вывести ее в консоль
    println!("Enter message to write in file:");
    let b1 = std::io::stdin().read_line(&mut line).unwrap();
    println!("Entered message is:{}", line);
    println!("No of bytes:{}", b1); //конец упражнения

    let mut output = OpenOptions::new().read(true).write(true).create(true).open(&path1.trim())?;  //запись в файл(+его создание)
    output.write_all(line.as_bytes()).expect("Write in file failed");

    let mut output_line = String::new(); //создание строки в которую загоняем строку из файла
    output.read_to_string(&mut output_line).unwrap();
    println!("In the file:{}", output_line);
    print!("lol\n");

    let mut my_file = std::fs::File::open("my_document.txt").unwrap();
    let mut file_content = String::new();
    my_file.read_to_string( & mut file_content).unwrap();
    print!("The content is: {}", file_content);

    let Input_thread = thread::spawn(|| {
        for i in 1..3 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    let Output_thread = thread::spawn(|| {
        for i in 1..3 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    let Control_thread = thread::spawn(|| {
            println!("Control");
    });

    Control_thread.join().unwrap();
    Output_thread.join().unwrap();
    println!("Ololo");

    Ok(())
}
