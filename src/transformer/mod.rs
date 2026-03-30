mod error;

use error::DicomTransformerError as DTError;
use std::env::args;

use dicom::{
    dictionary_std::tags,
    object::file::{OpenFileOptions, open_file},
    pixeldata::PixelDecoder,
};

/// Read and log DICOM dose information from a file
///
/// # Arguments
/// * `file_path` - Path to the DICOM file
///
/// # Returns
/// * `Ok(())` on success
/// * `Err(DTError)` on failure
#[allow(unused)]
fn read_dose(file_path: &str) -> Result<(), DTError> {
    let span = tracing::debug_span!("read_dose", file = file_path);
    let _guard = span.enter();

    tracing::debug!("Starting dose reading from DICOM file");

    let obj = OpenFileOptions::new().open_file(file_path)?;

    let dose_units: String = obj.element(tags::DOSE_UNITS)?.to_str()?.to_string();
    let scaling = obj.element(tags::DOSE_GRID_SCALING)?.to_float64()?;
    let pixel_rep = obj.element(tags::PIXEL_REPRESENTATION)?.to_int::<u16>()? as i64;
    let bits_allocated = obj.element(tags::BITS_ALLOCATED)?.to_int::<u16>()?;
    let pixel_data = obj.element(tags::PIXEL_DATA)?.to_bytes()?;

    tracing::debug!("Decoding pixel data");
    let decoded_data = obj.decode_pixel_data().map_err(|e| {
        tracing::error!("Failed to decode pixel data: {}", e);
        DTError::DTWriteError
    })?;

    tracing::debug!(
        dose_units = dose_units,
        scaling = scaling,
        pixel_representation = pixel_rep,
        bits_allocated = bits_allocated,
        pixel_data_length = pixel_data.len(),
        samples_per_pixel = decoded_data.samples_per_pixel(),
        "Successfully read DICOM dose data"
    );

    tracing::info!(
        dose_units = dose_units,
        scaling = scaling,
        "Dose information extracted"
    );

    Ok(())
}

/// Main entry point for the DICOM transformer
///
/// Reads a DICOM file from command-line arguments and extracts key information.
///
/// # Returns
/// * `Ok(())` on success
/// * `Err(DTError)` on failure
#[allow(unused)]
fn entrypoint() -> Result<(), DTError> {
    let span = tracing::info_span!("dicom_entrypoint");
    let _guard = span.enter();

    let Some(file_path) = args().nth(1) else {
        tracing::error!("No DICOM file path provided");
        return Err(DTError::DTPathError);
    };

    tracing::info!(path = file_path, "Processing DICOM file");

    let obj = open_file(file_path.clone()).map_err(|e| {
        tracing::error!(error = %e, path = file_path, "Failed to open DICOM file");
        DTError::DTPathError
    })?;

    // Extract patient information
    let patient_name: String = match obj.element(tags::PATIENT_NAME) {
        Ok(elem) => elem.to_str()?.to_string(),
        Err(e) => {
            tracing::warn!(error = %e, "Failed to read patient name, using default");
            "Unknown".to_string()
        }
    };

    let dose_unit: String = obj.element(tags::DOSE_UNITS)?.to_str()?.to_string();
    let pixel_size_allocated = obj.element(tags::BITS_ALLOCATED)?.to_str()?.to_string();
    let number_of_frames = obj.element(tags::NUMBER_OF_FRAMES)?.to_str()?.to_string();
    let columns: u16 = obj.element(tags::COLUMNS)?.to_int::<u16>()?;
    let rows: u16 = obj.element(tags::ROWS)?.to_int::<u16>()?;

    tracing::debug!(
        patient_name = patient_name,
        dose_unit = dose_unit,
        bits_allocated = pixel_size_allocated,
        number_of_frames = number_of_frames,
        columns = columns,
        rows = rows,
        "Extracted DICOM file metadata"
    );

    let data = obj
        .decode_pixel_data_frame(79)
        .map_err(|e| {
            tracing::error!(error = %e, frame = 79, "Failed to decode pixel data frame");
            DTError::DTWriteError
        })?
        .data()
        .to_vec();

    let mut vec: Vec<&[u8; 4]> = vec![];
    data.chunks_exact(4).for_each(|chunk| {
        let arr: &[u8; 4] = chunk.try_into().unwrap();
        vec.push(arr);
    });

    let vec32: Vec<u32> = vec
        .iter()
        .map(|&bytes| u32::from_le_bytes(*bytes))
        .collect();

    let frame: Vec<Vec<u32>> = vec32
        .chunks(columns as usize)
        .map(|chunk| chunk.to_vec())
        .collect();

    tracing::info!(
        patient_name = patient_name,
        dose_unit = dose_unit,
        frames = number_of_frames,
        dimensions = format!("{}x{}", columns, rows),
        "DICOM file processing completed successfully"
    );

    tracing::debug!(
        pixel_at_79_79 = frame[79][79],
        frame_size = frame.len(),
        "Frame 79 analysis"
    );

    Ok(())
}
