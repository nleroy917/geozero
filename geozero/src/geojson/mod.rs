//! GeoJSON conversions.
pub(crate) mod geojson_reader;
pub(crate) mod geojson_writer;

pub use geojson_reader::*;
pub use geojson_writer::*;

pub(crate) mod conversion {
    use super::geojson_writer::*;
    use crate::error::Result;
    use crate::{GeozeroDatasource, GeozeroGeometry};

    /// Convert to GeoJSON.
    pub trait ToJson {
        /// Convert to GeoJSON String.
        fn to_json(&self) -> Result<String>;
    }

    impl<T: GeozeroGeometry> ToJson for T {
        fn to_json(&self) -> Result<String> {
            let mut out: Vec<u8> = Vec::new();
            let mut p = GeoJsonWriter::new(&mut out);
            self.process_geom(&mut p)?;
            String::from_utf8(out).map_err(|_| {
                crate::error::GeozeroError::Geometry("Invalid UTF-8 encoding".to_string())
            })
        }
    }

    /// Consume features as GeoJSON.
    pub trait ProcessToJson {
        /// Consume features as GeoJSON String.
        fn to_json(&mut self) -> Result<String>;
    }

    impl<T: GeozeroDatasource> ProcessToJson for T {
        fn to_json(&mut self) -> Result<String> {
            let mut out: Vec<u8> = Vec::new();
            let mut p = GeoJsonWriter::new(&mut out);
            self.process(&mut p)?;
            String::from_utf8(out).map_err(|_| {
                crate::error::GeozeroError::Geometry("Invalid UTF-8 encoding".to_string())
            })
        }
    }
}

// #[cfg(feature = "with-wkb")]
// mod wkb {
//     use super::geojson_writer::*;
//     use crate::error::Result;
//     use crate::geojson::GeoJson;
//     use crate::wkb::{FromWkb, WkbDialect};
//     use std::io::Read;

//     impl FromWkb for GeoJson<'_> {
//         fn from_wkb<R: Read>(rdr: &mut R, dialect: WkbDialect) -> Result<Self> {
//             let mut out: Vec<u8> = Vec::new();
//             let mut p = GeoJsonWriter::new(&mut out);
//             crate::wkb::process_wkb_type_geom(rdr, &mut p, dialect)?;
//             let json = String::from_utf8(out).map_err(|_| {
//                 crate::error::GeozeroError::Geometry("Invalid UTF-8 encoding".to_string())
//             })?;
//             Ok(GeoJson(&json))
//         }
//     }
// }