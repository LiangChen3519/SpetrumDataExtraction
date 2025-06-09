use crate::data_structure::SpectrumData;
use csv::Writer;
use std::error::Error;
use std::fs;

// convert extracted file *records* which return from "parse_spectrum_data" into csv file
pub fn convert_to_csv(records: Vec<SpectrumData>, file_name: &str) -> Result<(), Box<dyn Error>> {
    if let Ok(exists) = fs::exists(file_name) {
        if exists {
            // if file exist, we delected
            fs::remove_file(file_name).unwrap();
        } 
           // let mut file = fs::File::create(file_name).unwrap();
            let mut file = fs::File::create(file_name).unwrap();
            let mut writer = Writer::from_writer(file);
            //insert the hearder
            // Write header
            writer.write_record(&[
                "IDData",
                "DateTime",
                "Temperature",
                "Wavelength",
                "Absorbance",
            ])?;
            // writeln!(writer, "IDData,DateTime,Temperature,Wavelength,Absorbance")?;
            // loop the list of records
            for record in records {
                let len = record.wavelength.len().min((record.absorbance).len());
                for i in 0..len {
                    writer.write_record(&[
                        &record.id_data,
                        &record.datetime,
                        &record.temperature,
                        &record.wavelength[i].to_string(),
                        &record.absorbance[i].to_string(),
                    ])?;
                }
            }
    }

    Ok(())
}
