use image_viewer_rs::geo_tag::GeoTag;
use std::{fs, path::PathBuf, time::Duration};

use azul::{
    dialogs::{open_directory_dialog, open_file_dialog, open_multiple_files_dialog},
    prelude::*,
    widgets::button::Button,
};

struct MyDataModel {
    image_id: Option<ImageId>,
    image_ids: Option<Vec<ImageId>>,
}
impl Layout for MyDataModel {
    fn layout(&self, _: LayoutInfo<Self>) -> Dom<Self> {
        let mut images = Dom::div().with_class("images");
        match &self.image_ids {
            Some(ids) => {
                for _ in (0..5) {
                    images.add_child(Dom::image(*ids.first().unwrap()).with_class("img"));
                }
            }
            None => {}
        }

        Dom::div()
            .with_class("all")
            .with_child(
                Dom::div()
                    .with_class("menu")
                    .with_child(
                        Button::with_label("open from directory")
                            .dom()
                            .with_class("select-from-folder")
                            .with_callback(On::LeftMouseUp, Callback(select_from_folder)),
                    )
                    .with_child(
                        Button::with_label("open from files")
                            .dom()
                            .with_class("select-from-files")
                            .with_callback(On::LeftMouseUp, Callback(select_from_files)),
                    ),
            )
            .with_child(images)
    }
    // fn layout(&self, _: LayoutInfo<Self>) -> Dom<Self> {
    //     match self.image_id {
    //         Some(i) => {
    //             // Dom::image(i)
    //             // (0..3).map(|_i| Dom::image(i)).collect::<Dom<Self>>()
    //             // Dom::div().with_class("container").with_child(
    //             //     (0..3)
    //             //         .map(|i| {
    //             //             Dom::div()
    //             //                 .with_class("img")
    //             //                 .with_child(Dom::label(i.to_string()).with_class("lab"))
    //             //                 .with_child(Dom::div())
    //             //         })
    //             //         .collect::<Dom<Self>>(),
    //             // )
    //             let mut container = Dom::div().with_class("container");
    //             for _ in (0..5) {
    //                 container.add_child(
    //                     Dom::image(i)
    //                         .with_class("img")
    //                         .with_callback(On::LeftMouseUp, Callback(my_andler)),
    //                 );
    //             }
    //             container
    //             // .with_child(Dom::div().with_class("img"))
    //         }
    //         None => {
    //             println!("None");
    //             Dom::div()
    //         }
    //     }
    // }
}
fn select_from_folder(
    app_state: &mut AppState<MyDataModel>,
    event: &mut CallbackInfo<MyDataModel>,
) -> UpdateScreen {
    open_directory_dialog(None)
        // .and_then(|path| fs::read_to_string(path.clone()).ok())
        .and_then(|path| Some(1))
        .and_then(|content| Some(Redraw))
        .unwrap_or(DontRedraw)
}

fn select_from_files(
    app_state: &mut AppState<MyDataModel>,
    event: &mut CallbackInfo<MyDataModel>,
) -> UpdateScreen {
    open_multiple_files_dialog(None, None)
        // .and_then(|path| fs::read_to_string(path.clone()).ok())
        .and_then(|paths| {
            println!("{:?}", paths);
            let image_id = app_state.add_css_image_id("example01");
            app_state.add_image(
                image_id,
                ImageSource::Embedded(include_bytes!("../example.jpg")),
            );
            {
                let mut data = app_state.data.lock().unwrap(); //data is myDataModel
                data.image_ids = Some(vec![image_id]);
            }
            Some(1)
        })
        .and_then(|content| Some(Redraw))
        .unwrap_or(DontRedraw)
}

fn my_andler(
    app_state: &mut AppState<MyDataModel>,
    event: &mut CallbackInfo<MyDataModel>,
) -> UpdateScreen {
    // open_directory_dialog(None)
    println!("--------------{:?}", app_state.windows[event.window_id]);
    open_multiple_files_dialog(None, None)
        // .and_then(|path| fs::read_to_string(path.clone()).ok())
        .and_then(|path| Some(1))
        .and_then(|content| Some(Redraw))
        .unwrap_or(DontRedraw)
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
    let mut app = App::new(
        MyDataModel {
            image_id: None,
            image_ids: None,
        },
        AppConfig::default(),
    )
    .unwrap();
    let image_id = app.add_css_image_id("example01");
    app.add_image(
        image_id,
        ImageSource::Embedded(include_bytes!("../example.jpg")),
    );
    {
        let mut data = app.app_state.data.lock().unwrap(); //data is myDataModel
        data.image_id = Some(image_id);
    }
    // app.app_state.resources.add_image(image_source);
    println!("{:?}", env!("CARGO_MANIFEST_DIR"));
    macro_rules! CSS_PATH {
        () => {
            concat!(env!("CARGO_MANIFEST_DIR"), "/example.css")
        };
    }
    let window = {
        let hot_reloader = css::hot_reload(CSS_PATH!(), Duration::from_millis(500));
        app.create_hot_reload_window(WindowCreateOptions::default(), hot_reloader)
            .unwrap()
        // let css = css::override_native(include_str!("../example.css")).unwrap();
        // app.create_window(WindowCreateOptions::default(), css.clone())
        //     .unwrap()
    };
    app.run(window).unwrap();
}
