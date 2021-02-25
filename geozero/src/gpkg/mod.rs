//! Geopackage geometry type encoding/decoding.
//!
//! All geometry types implementing [GeozeroGeometry](crate::GeozeroGeometry) can be encoded as Geopackage WKB geometry.
//!
//! Geometry types implementing [FromWkb](crate::wkb::FromWkb) can be decoded from Geopackage geometries.
//!
//! # Usage example
//!
//! Select geo-types geometries from a Geopackage:
//! ```
//! use geozero::{wkb, ToWkt};
//! use sqlx::sqlite::SqlitePoolOptions;
//!
//! # async fn rust_geo_query() -> Result<(), sqlx::Error> {
//! let pool = SqlitePoolOptions::new()
//!     .max_connections(5)
//!     .connect("sqlite://points.gpkg")
//!     .await?;
//!
//! let row: (wkb::Geometry<geo_types::Geometry<f64>>,) = sqlx::query_as("SELECT geom FROM pt2d")
//!     .fetch_one(&pool)
//!     .await?;
//! let wkbgeom = row.0;
//! println!("{}", wkbgeom.0.to_wkt().unwrap());
//! # Ok(())
//! # }
//! ```

mod geopackage;

pub use geopackage::*;