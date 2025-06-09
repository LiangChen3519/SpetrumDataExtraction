use std::fs;
use std::io::{BufWriter, Write};
use crate::write_csv::convert_to_csv;

mod parser;
mod data_structure;

#[path="../test/read_by_line.rs"]
mod read_by_line;
mod write_csv;

fn main() {
    let file_path = "..\\input_data\\Tribox_9B8C_Spectra_2025-05-07_00-00-00_to_2025-05-16_09-45-00.dat";
    if let Ok(exists) = fs::exists(file_path) {
        if exists {
            println!("File exists");
            println!("now starting to extract data...");
            //read_by_line::read_by_line(file_path);
            match parser::read_file_content(&file_path) {
                Ok(contents) =>{
                    let my_records = parser::parse_spectrum_data(&contents);
                    //println!("{:?}", my_records);
                    let file_name = "mydata.txt";
                    // creat a file
                    let mut file = fs::File::create(file_name).unwrap();
                    let mut writer = BufWriter::new(&mut file);
                    println!("now starting to write data to file...");
                    for record in &my_records{
                        writeln!(writer,"IDData:{}",record.id_data).unwrap();
                        writeln!(writer,"DateTime:{}",record.datetime).unwrap();
                        writeln!(writer,"Temperature:{}",record.temperature).unwrap();
                        writeln!(writer,"WaveLength:{:?}",record.wavelength).unwrap();
                        writeln!(writer,"Absorbance:{:?}",record.absorbance).unwrap();
                    }
                    // here we save as a csv file
                    //let contents = parser::read_file_content(&file_path);
                    match convert_to_csv(my_records,"my_data.csv") {
                        Ok(csv_contents) => {
                            println!("CSV is created successfully")
                        }Err(e) => println!("Error in converting to csv: {}", e),
                    }
                }
                Err(e) => println!("Error in reading file: {}", e),
            }
            
        }
        else {
            println!("File is not exists");       
        }
    }else { 
        println!("Check file path!!");      
    }
}
