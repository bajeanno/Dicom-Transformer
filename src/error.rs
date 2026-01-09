use dicom::{
    core::value::ConvertValueError,
    object::{AccessError, ReadError},
    pixeldata::Error as PixelDataError,
};
use thiserror::Error;

#[allow(dead_code)]
#[derive(Error, Debug)]
pub enum DicomTransformerError {
    #[error("Failed to write DICOM file")]
    DTWriteError,
    #[error("No file path provided")]
    DTPathError,
    #[error("Failed to open DICOM file: {0}")]
    DTOpenError(#[from] std::io::Error),
    #[error("Failed to convert value: {0}")]
    DTConvertError(#[from] ConvertValueError),
    #[error("Failed to access DICOM file: {0}")]
    DTAccessError(#[from] AccessError),
    #[error("Failed to access DICOM file: {0}")]
    DTReadError(#[from] ReadError),
    #[error("pixel failure: {0}")]
    DTPixelError(#[from] PixelDataError),

}
