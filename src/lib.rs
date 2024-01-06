#![feature(arbitrary_self_types)]
#![allow(unused_imports)]

use autocxx::prelude::*;
use image::io::Reader;
use image::ImageResult;
use std::fmt::{Display, Formatter};
use std::io::Cursor;
use std::pin::Pin;

mod reader_options {
    use autocxx::prelude::*;
    include_cpp! {
        #include "ReaderOptions.h"
        safety!(unsafe_references_wrapped)
        generate!("ZXing::ReaderOptions")
    }
    pub use ffi::*;
}
mod reader_options_ext {
    use autocxx::prelude::*;
    include_cpp! {
        #include "ReaderOptionsExt.h"
        name!(ffi3)
        safety!(unsafe_references_wrapped)
        generate!("ZXing::ReaderOptionsExt")
        extern_cpp_type!("ZXing::ReaderOptions", crate::reader_options::ZXing::ReaderOptions)
        extern_cpp_type!("ZXing::BarcodeFormat", crate::base::ZXing::BarcodeFormat)
    }
    pub use ffi3::*;
}

mod base {
    use autocxx::prelude::*;
    include_cpp! {
        #include "ImageView.h"
        #include "ReadBarcode.h"
        #include "Result.h"
        #include "Flags.h"
        name!(ffi2)
        safety!(unsafe_ffi)
        generate!("ZXing::ReadBarcode")
        generate!("ZXing::ImageView")
        generate!("ZXing::Result")
        generate!("ZXing::Results")
        extern_cpp_type!("ZXing::ReaderOptions", crate::reader_options::ZXing::ReaderOptions)
        extern_cpp_type!("ZXing::TextMode", crate::reader_options::ZXing::TextMode)
    }
    pub use ffi2::*;
}

#[repr(i32)]
pub enum BarcodeFormat {
    ///< Used as a return value if no valid barcode has been detected
    None = 0,
    ///< Aztec
    Aztec = 1,
    ///< Codabar
    Codabar = 2,
    ///< Code39
    Code39 = 4,
    ///< Code93
    Code93 = 8,
    ///< Code128
    Code128 = 16,
    ///< GS1 DataBar, formerly known as RSS 14
    DataBar = 32,
    ///< GS1 DataBar Expanded, formerly known as RSS EXPANDED
    DataBarExpanded = 64,
    ///< DataMatrix
    DataMatrix = 128,
    ///< EAN-8
    EAN8 = 256,
    ///< EAN-13
    EAN13 = 512,
    ///< ITF (Interleaved Two of Five)
    ITF = 1024,
    ///< MaxiCode
    MaxiCode = 2048,
    ///< PDF417
    PDF417 = 4096,
    ///< QR Code
    QRCode = 8192,
    ///< UPC-A
    UPCA = 16384,
    ///< UPC-E
    UPCE = 32768,
    ///< Micro QR Code
    MicroQRCode = 65536,
    ///< Rectangular Micro QR Code
    RMQRCode = 131072,
    ///< DX Film Edge Barcode
    DXFilmEdge = 262144,
    LinearCodes = 313214,
    MatrixCodes = 211073,
    Any = 524287,
}

impl From<base::ZXing::BarcodeFormat> for BarcodeFormat {
    fn from(value: base::ZXing::BarcodeFormat) -> Self {
        use base::ZXing::BarcodeFormat as BF;
        match value {
            BF::None => BarcodeFormat::None,
            BF::Aztec => BarcodeFormat::Aztec,
            BF::Codabar => BarcodeFormat::Codabar,
            BF::Code39 => BarcodeFormat::Code39,
            BF::Code93 => BarcodeFormat::Code93,
            BF::Code128 => BarcodeFormat::Code128,
            BF::DataBar => BarcodeFormat::DataBar,
            BF::DataBarExpanded => BarcodeFormat::DataBarExpanded,
            BF::DataMatrix => BarcodeFormat::DataMatrix,
            BF::EAN8 => BarcodeFormat::EAN8,
            BF::EAN13 => BarcodeFormat::EAN13,
            BF::ITF => BarcodeFormat::ITF,
            BF::MaxiCode => BarcodeFormat::MaxiCode,
            BF::PDF417 => BarcodeFormat::PDF417,
            BF::QRCode => BarcodeFormat::QRCode,
            BF::UPCA => BarcodeFormat::UPCA,
            BF::UPCE => BarcodeFormat::UPCE,
            BF::MicroQRCode => BarcodeFormat::MicroQRCode,
            BF::RMQRCode => BarcodeFormat::RMQRCode,
            BF::DXFilmEdge => BarcodeFormat::DXFilmEdge,
            BF::LinearCodes => BarcodeFormat::LinearCodes,
            BF::MatrixCodes => BarcodeFormat::MatrixCodes,
            BF::Any => BarcodeFormat::Any,
        }
    }
}

impl Display for BarcodeFormat {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            BarcodeFormat::None => write!(f, "None"),
            BarcodeFormat::Aztec => write!(f, "Aztec"),
            BarcodeFormat::Codabar => write!(f, "Codabar"),
            BarcodeFormat::Code39 => write!(f, "Code39"),
            BarcodeFormat::Code93 => write!(f, "Code93"),
            BarcodeFormat::Code128 => write!(f, "Code128"),
            BarcodeFormat::DataBar => write!(f, "DataBar"),
            BarcodeFormat::DataBarExpanded => write!(f, "DataBarExpanded"),
            BarcodeFormat::DataMatrix => write!(f, "DataMatrix"),
            BarcodeFormat::EAN8 => write!(f, "EAN8"),
            BarcodeFormat::EAN13 => write!(f, "EAN13"),
            BarcodeFormat::ITF => write!(f, "ITF"),
            BarcodeFormat::MaxiCode => write!(f, "MaxiCode"),
            BarcodeFormat::PDF417 => write!(f, "PDF417"),
            BarcodeFormat::QRCode => write!(f, "QRCode"),
            BarcodeFormat::UPCA => write!(f, "UPCA"),
            BarcodeFormat::UPCE => write!(f, "UPCE"),
            BarcodeFormat::MicroQRCode => write!(f, "MicroQRCode"),
            BarcodeFormat::RMQRCode => write!(f, "RMQRCode"),
            BarcodeFormat::DXFilmEdge => write!(f, "DXFilmEdge"),
            BarcodeFormat::LinearCodes => write!(f, "LinearCodes"),
            BarcodeFormat::MatrixCodes => write!(f, "MatrixCodes"),
            BarcodeFormat::Any => write!(f, "Any"),
        }
    }
}

pub struct ImageView {
    _data: Vec<u8>,
    image: UniquePtr<base::ZXing::ImageView>,
}

impl ImageView {
    pub fn new(data: &[u8]) -> ImageResult<Self> {
        let image = Reader::new(Cursor::new(data))
            .with_guessed_format()?
            .decode()?
            .into_luma8();

        let width = image.width() as i32;
        let height = image.height() as i32;

        let data = image.into_vec();

        Ok(Self {
            image: unsafe {
                base::ZXing::ImageView::new(
                    data.as_ptr(),
                    c_int(width),
                    c_int(height),
                    base::ZXing::ImageFormat::Lum,
                    c_int(0),
                    c_int(0),
                )
            }
            .within_unique_ptr(),
            _data: data,
        })
    }
}

pub struct BarcodeResult {
    result: UniquePtr<base::ZXing::Result>,
}

impl BarcodeResult {
    pub fn format(&self) -> BarcodeFormat {
        self.result.format().into()
    }

    pub fn text(&self) -> String {
        self.result
            .text(reader_options::ZXing::TextMode::Plain.within_unique_ptr())
            .to_string()
    }
}

pub fn read_barcode(image: ImageView) -> BarcodeResult {
    let options = reader_options_ext::ZXing::ReaderOptionsExt::new().within_unique_ptr();
    let mut options = CppUniquePtrPin::new(options);
    options.as_cpp_mut_ref().addFormat(base::ZXing::BarcodeFormat::Codabar.within_unique_ptr());
    BarcodeResult {
        result: base::ZXing::ReadBarcode(&image.image, unsafe { options.as_cpp_mut_ref().asOptions().as_mut() })
            .within_unique_ptr(),
    }
}
