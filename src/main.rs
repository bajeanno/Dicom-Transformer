use std::{
    env::args,
};

mod error;
use error::DicomTransformerError as DTError;

use dicom::{
    dictionary_std::tags,
    object::file::{OpenFileOptions, open_file},
    pixeldata::PixelDecoder,
};

#[allow(unused)]
fn read_dose(file_path: &str) -> Result<(), DTError> {
    let obj = OpenFileOptions::new().open_file(file_path)?;
    let dose_units: String = obj.element(tags::DOSE_UNITS)?.to_str()?.to_string();
    let scaling = obj.element(tags::DOSE_GRID_SCALING)?.to_float64()?;
    let pixel_rep = obj.element(tags::PIXEL_REPRESENTATION)?.to_int::<u16>()? as i64;
    let bits_allocated = obj.element(tags::BITS_ALLOCATED)?.to_int::<u16>()?;
    let pixel_data = obj.element(tags::PIXEL_DATA)?.to_bytes()?;
    let decoded_data = obj.decode_pixel_data().map_err(|_| DTError::DTWriteError)?;
    // let decoded_data_frame = obj.decode_pixel_data_frame(79);

    // let doses = doses_raw.iter().map(|&val| val as f64 * scaling).collect();
    println!("DEBUG: DOSE_UNITS = {}", dose_units);
    println!("DEBUG: DOSE_GRID_SCALING = {}", scaling);
    println!("DEBUG: PIXEL_REPRESENTATION = {}", pixel_rep);
    println!("DEBUG: BITS_ALLOCATED = {}", bits_allocated);
    println!("DEBUG: PIXEL_DATA length = {} bytes", pixel_data.len());
    println!("DEBUG: samples per pixels = {} bytes", decoded_data.samples_per_pixel());
    println!("Unité de dose : {}", dose_units);
    Ok(())
}

#[allow(unused)]
fn main() -> Result<(), DTError> {
    let Some(file_path) = args().nth(1) else {
        eprintln!("Usage: dicom_transformer <DICOM_FILE_PATH>");
        return Err(DTError::DTPathError);
    };

    let obj = open_file(file_path.clone())?;
    let patient_name: String = obj.element(tags::PATIENT_NAME)?.to_str()?.to_string();
    let dose_unit: String = obj.element(tags::DOSE_UNITS)?.to_str()?.to_string();
    let pixel_size_allocated = obj.element(tags::BITS_ALLOCATED)?.to_str()?.to_string();
    let pixel_size_stored = obj.element(tags::BITS_STORED)?.to_str()?.to_string();
    let number_of_frames = obj.element(tags::NUMBER_OF_FRAMES)?.to_str()?.to_string();
    let columns: u16 = obj.element(tags::COLUMNS)?.to_int::<u16>()?;
    let rows: u16 = obj.element(tags::ROWS)?.to_int::<u16>()?;
    // let doses = read_dose(&file_path)?;
    let data = obj.decode_pixel_data_frame(79)?.data().to_vec();
    data.iter().step_by(4);
    let mut vec : Vec<&[u8; 4]> = vec![];
    data.chunks_exact(4).for_each(|chunk| {
        let arr: &[u8; 4] = chunk.try_into().unwrap();
        vec.push(arr);
    });
    let vec32 : Vec<u32> = vec.iter().map(|&bytes| u32::from_le_bytes(*bytes)).collect();
    let frame: Vec<Vec<u32>> = vec32.chunks(columns as usize).map(|chunk| chunk.to_vec()).collect();

    // for pixel in doses {
    //     print!("{pixel} ");
    //     print!("{}", pixel * 2.0);
    // }

    println!();
    println!("Nom du patient = {}", patient_name);
    println!("Unité de la dose = {}", dose_unit);
    println!("Pixel size(bits) = {}", pixel_size_allocated);
    println!("Nombre de slices = {}", number_of_frames);
    println!("Colonnes = {}", columns);
    println!("Lignes = {}", rows);
    println!("Value of 79/79/79: {}", frame[79][79]);
    Ok(())
}
