// this is a struct for storing spectrum data, like a big container
#[derive(Debug)]
pub struct SpectrumData {
    pub id_data: String, // Frequency in Hz
    pub datetime: String, // Date and time of the measurement
    pub temperature: String, // Temperature in Celsius
    pub wavelength: Vec<f64>,
    pub absorbance: Vec<f64>,
}