use std::fs;
use regex::Regex;
use crate::data_structure::SpectrumData;


pub fn parse_spectrum_data(content: &str) ->  Vec<SpectrumData> {
    let re_block = Regex::new(r"(?s)\[Spectrum\](.*?)\[END\]\s+of\s+\[Spectrum\]\r?\n").unwrap();
    let re_data = Regex::new(r"\[DATA\](?s)(.*?)\[END\] of \[DATA\]").unwrap();
    let mut records = Vec::new();
    //let matches: Vec<_> = re_data .captures_iter(input_path).collect();
    //println!("Found {} spectrum blocks", matches.len());
    for caps in re_block.captures_iter(content) {
        let block = caps.get(1).unwrap().as_str().trim();
        //println!("block: {}", block);
        // now we want to get the key from the blocks
        let id_data = extract_value_by_key(block, "IDData");
        let datetime = extract_value_by_key(block,"DateTime");
        let temperature = extract_value_by_key(block,"Temperature");
        // now parse the data
        let data_block = re_data.captures(block)
            .and_then(|caps| caps.get(1))
            .map(|m| m.as_str())
            .unwrap_or("")
            .trim();
        let mut wavelength = Vec::new();
        let mut absorbance = Vec::new();
        for line in data_block.lines().skip(1) {
            let parts: Vec<_> = line.split_whitespace().collect();
            if parts.len() >= 2{
                if let (Ok(w), Ok(a)) = (parts[0].parse::<f64>(), parts[1].parse::<f64>()) {
                    wavelength.push(w);
                    absorbance.push(a);
                }
            }else { 
                break;
            }
        }
        records.push(
            SpectrumData{
                id_data,
                datetime,
                temperature,
                wavelength,
                absorbance,
            });
    }
    records
}

// this function is extract value based on the key from each brock
pub fn extract_value_by_key(block: &str, key: &str) -> String {
    let pattern = format!(r"{}\s*=\s*(.*)", regex::escape(key));
    Regex::new(&pattern)
        .ok()
        .and_then(|re| re.captures(block))
        .and_then(|caps| caps.get(1))
        .map(|m| m.as_str().to_string())
        .unwrap_or_default()
}

// read the content of the file based on the file path
pub fn read_file_content(file_path: &str) -> Result<String, String> {
    let contents = fs::read_to_string(file_path).map_err(|e| e.to_string())?;
    Ok(contents)
}


