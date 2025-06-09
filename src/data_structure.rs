// this is a struct for storing spectrum data, like a big container
#[derive(Debug)]
pub struct SpectrumData {
    pub id_data: String, // Id of the data
    pub datetime: String, // Date and time of the measurement
    pub temperature: String, // Temperature in Celsius
    pub wavelength: Vec<f64>, // wavelength from 200 to 360 nm
    pub absorbance: Vec<f64>, //..
}