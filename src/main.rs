fn main() {
    let meta = rexiv2::Metadata::new_from_path("example.jpg").unwrap();
    println!("{:?}", meta.get_gps_info());
    println!("{:?}", meta.get_exif_tags());
    println!("{:?}", meta.get_tag_string("Exif.GPSInfo.GPSLatitudeRef"));
    println!("{:?}", meta.get_tag_string("Exif.GPSInfo.GPSLongitudeRef"));
}
