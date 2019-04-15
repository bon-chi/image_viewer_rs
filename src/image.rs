use rexiv2::{GpsInfo, Metadata};
use std::path::Path;
use std::result;

pub struct Image<'a> {
    path: &'a Path,
    meta_data: Metadata,
}
enum EW {
    E,
    W,
}
enum NS {
    N,
    S,
}

pub struct GeoTag {
    gps_info: GpsInfo,
    gps_longitude_ref: NS,
    gps_latitude_ref: EW,
}

enum Error {}
type Result<T> = result::Result<T, Error>;

impl<'a> Image<'a> {
    fn update_geo_tag(&mut self, geo_tag: &GeoTag) -> Result<()> {
        Ok(())
    }
}

impl GeoTag {
    fn new(longitude: f64, latitude: f64) -> GeoTag {
        use crate::image::EW;
        GeoTag {
            gps_longitude_ref: if longitude > 0.0 { NS::N } else { NS::N },
            gps_latitude_ref: if latitude > 0.0 { EW::E } else { EW::W },
            gps_info: GpsInfo {
                longitude: longitude.abs(),
                latitude: latitude.abs(),
                altitude: 0.0,
            },
        }
    }
}
