use image_viewer_rs::geo_tag::GeoTag;
use std::path::PathBuf;

use azul::prelude::*;

struct MyDataModel {
    image_id: Option<ImageId>,
}
impl Layout for MyDataModel {
    fn layout(&self, _: LayoutInfo<Self>) -> Dom<Self> {
        match self.image_id {
            Some(i) => Dom::image(i),
            None => {
                println!("None");
                Dom::div()
            }
        }
    }
}

fn main() {
    let g = GeoTag;
    let mut meta = rexiv2::Metadata::new_from_path("example.jpg").unwrap();
    println!("{:?}", meta.get_gps_info());
    println!("{:?}", meta.get_exif_tags());
    println!("{:?}", meta.get_tag_string("Exif.GPSInfo.GPSLatitudeRef"));
    println!("{:?}", meta.get_tag_string("Exif.GPSInfo.GPSLatitude"));
    println!("{:?}", meta.get_tag_string("Exif.GPSInfo.GPSLongitudeRef"));
    println!("{:?}", meta.get_tag_string("Exif.GPSInfo.GPSLongitude"));
    let gps_info_bk = rexiv2::GpsInfo {
        longitude: 139.717,
        latitude: 35.633091666666665,
        altitude: 73.59,
    };
    let gps_info_new = rexiv2::GpsInfo {
        longitude: 130.717,
        latitude: 35.633091666666665,
        altitude: 73.59,
    };
    println!("{:?}", meta.set_gps_info(&gps_info_bk));
    println!("{:?}", meta.get_gps_info());
    meta.save_to_file("example.jpg");
    // println!("{:?}", meta.delete_gps_info());
    // let mut gps_info = meta.get_gps_info().unwrap();
    // gps_info.longitude = 130.0;
    //
    let path = PathBuf::from("example.jpg");
    let image_source = ImageSource::File(path);
    // let image_id = ImageId
    let mut app = App::new(MyDataModel { image_id: None }, AppConfig::default()).unwrap();
    let image_id = app.add_css_image_id("example01");
    app.add_image(
        image_id,
        ImageSource::Embedded(include_bytes!("../example.jpg")),
    );
    {
        let mut data = app.app_state.data.lock().unwrap();
        data.image_id = Some(image_id);
    }
    // app.app_state.resources.add_image(image_source);
    let window = app
        .create_window(WindowCreateOptions::default(), css::native())
        .unwrap();
    app.run(window).unwrap();
}
