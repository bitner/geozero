//! GDAL conversions.
pub(crate) mod gdal_reader;
pub(crate) mod gdal_writer;

pub use gdal_reader::*;
pub use gdal_writer::*;

pub(crate) mod conversion {
    use super::gdal_writer::*;
    use crate::error::Result;
    use crate::{CoordDimensions, GeozeroGeometry};
    use gdal::vector::Geometry;

    /// Convert to GDAL geometry.
    pub trait ToGdal {
        /// Convert to 2D GDAL geometry.
        fn to_gdal(&self) -> Result<Geometry>;
        /// Convert to GDAL geometry with dimensions.
        fn to_gdal_ndim(&self, dims: CoordDimensions) -> Result<Geometry>;
    }

    impl<T: GeozeroGeometry> ToGdal for T {
        fn to_gdal(&self) -> Result<Geometry> {
            self.to_gdal_ndim(CoordDimensions::default())
        }
        fn to_gdal_ndim(&self, dims: CoordDimensions) -> Result<Geometry> {
            let mut gdal = GdalWriter::new();
            gdal.dims = dims;
            self.process_geom(&mut gdal)?;
            Ok(gdal.geom)
        }
    }
}
