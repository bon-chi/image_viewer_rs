use image_viewer_rs::geo_tag::GeoTag;
use std::{
    collections::BTreeMap,
    ffi::OsStr,
    fs::File,
    io::Read,
    path::{Path, PathBuf},
    time::Duration,
};

use azul::{
    dialogs::{open_directory_dialog, open_file_dialog, open_multiple_files_dialog},
    prelude::*,
    widgets::button::Button,
    widgets::label::Label,
    widgets::text_input::*,
};

struct MyDataModel {
    image_id: Option<ImageId>,
    image_ids: Option<Vec<ImageId>>,
    images: BTreeMap<String, MyImage>,
    gps_input: TextInputState,
}

struct MyImage {
    id: ImageId,
    path: PathBuf,
    longitude: f64,
    latitude: f64,
    choosen: bool,
}

impl MyImage {
    fn toggle(&mut self) {
        self.choosen = !self.choosen;
    }
}

impl Layout for MyDataModel {
    fn layout(&self, info: LayoutInfo<Self>) -> Dom<Self> {
        let mut images = Dom::div().with_class("images");
        // match &self.image_ids {
        //     Some(ids) => {
        //         for _ in (0..5) {
        //             images.add_child(Dom::image(*ids.first().unwrap()).with_class("img"));
        //         }
        //     }
        //     None => {}
        // }
        for (_, i) in &self.images {
            images.add_child(
                Dom::div()
                    .with_class("image-container")
                    .with_callback(On::LeftMouseUp, Callback(toggle_image))
                    .with_child(
                        Dom::div()
                            .with_class("image-header")
                            .with_child(
                                Label::new(
                                    i.path.file_name().unwrap().to_str().unwrap().to_string(),
                                )
                                .dom()
                                .with_class("title"),
                            )
                            .with_child(
                                Label::new(if i.choosen { "check" } else { "blank" })
                                    .dom()
                                    .with_class("check"),
                            ),
                    )
                    .with_child(Dom::image(i.id).with_class("img"))
                    .with_child(
                        Label::new(format!("long: {}, lat: {}", i.longitude, i.latitude))
                            .dom()
                            .with_class("gps"),
                    ),
            );
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
                    )
                    .with_child(
                        Dom::div()
                            .with_class("gps-container")
                            .with_child(
                                TextInput::new()
                                    .bind(info.window, &self.gps_input, &self)
                                    .dom(&self.gps_input)
                                    .with_class("gps-input"),
                            )
                            .with_child(
                                Button::with_label("paste")
                                    .dom()
                                    .with_class("paste-button")
                                    .with_callback(On::LeftMouseUp, Callback(paste_gps)),
                            )
                            .with_child(
                                Button::with_label("set")
                                    .dom()
                                    .with_class("gps-button")
                                    .with_callback(On::LeftMouseUp, Callback(set_gps)),
                            ),
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
            let paths = paths.iter().map(|p| PathBuf::from(p)).collect::<Vec<_>>();
            // let path = Path::new(paths.first().unwrap());
            // println!("{:?}", paths);
            let mut images: BTreeMap<String, MyImage> = BTreeMap::new();
            for path in paths {
                let image_id =
                    app_state.add_css_image_id(path.file_name().unwrap().to_str().unwrap());
                app_state.add_image(
                    image_id,
                    ImageSource::File(path.clone()), // TODO: use thumbnail
                );
                let meta = rexiv2::Metadata::new_from_path(&path).unwrap();
                let longitude = meta.get_gps_info().unwrap().longitude;
                let latitude = meta.get_gps_info().unwrap().latitude;
                images.insert(
                    path.file_name().unwrap().to_str().unwrap().to_string(),
                    MyImage {
                        id: image_id,
                        path: path,
                        choosen: false,
                        longitude: longitude,
                        latitude: latitude,
                    },
                );
            }
            // let images: Vec<(MyImage, bool)> = paths
            //     .into_iter()
            //     .map(|path| {
            //         let image_id =
            //             app_state.add_css_image_id(path.file_name().unwrap().to_str().unwrap());
            //         // let mut buffer = Vec::new();
            //         // let mut f = File::open("../example.jpg").unwrap();
            //         // f.read_to_end(&mut buffer);
            //         app_state.add_image(
            //             image_id,
            //             // ImageSource::Embedded(include_bytes!("../example.jpg")), // TODO: use thumbnail
            //             ImageSource::File(path.clone()), // TODO: use thumbnail
            //                                              // ImageSource::Embedded(&buffer),
            //                                              // TODO: use thumbnail
            //         );
            //         (
            //             MyImage {
            //                 id: image_id,
            //                 path: path,
            //             },
            //             false,
            //         )
            //     })
            //     .collect();
            let image_id = app_state.add_css_image_id("example01");
            app_state.add_image(
                image_id,
                ImageSource::Embedded(include_bytes!("../example.jpg")),
            );
            {
                let mut data = app_state.data.lock().unwrap(); //data is myDataModel
                data.image_ids = Some(vec![image_id]);
                data.images = images;
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

fn parse_gps(s: &str) -> rexiv2::GpsInfo {
    let ss = s.split(',').collect::<Vec<&str>>();
    let latitude: f64 = ss[0].parse().unwrap();
    let longtitude: f64 = ss[1].parse().unwrap();
    rexiv2::GpsInfo {
        longitude: longtitude,
        latitude: latitude,
        altitude: 0.0,
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
    let mut app = App::new(
        MyDataModel {
            image_id: None,
            image_ids: None,
            images: BTreeMap::new(),
            gps_input: TextInputState::new(""),
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
        let hot_reloader = css::hot_reload(CSS_PATH!(), Duration::from_millis(3000));
        app.create_hot_reload_window(WindowCreateOptions::default(), hot_reloader)
            .unwrap()
        // let css = css::override_native(include_str!("../example.css")).unwrap();
        // app.create_window(WindowCreateOptions::default(), css.clone())
        //     .unwrap()
    };
    app.run(window).unwrap();
}

fn toggle_image(
    app_state: &mut AppState<MyDataModel>,
    event: &mut CallbackInfo<MyDataModel>,
) -> UpdateScreen {
    let node_id = event.hit_dom_node;
    let child_id = event.get_node(node_id).unwrap().first_child.unwrap();
    let grand_child_id = event.get_node(child_id).unwrap().first_child.unwrap();
    let grand_child = event.get_node_content(grand_child_id).unwrap();
    let label = match &grand_child.node_type {
        azul::dom::NodeType::Label(dom_string) => match dom_string {
            azul::dom::DomString::Heap(string) => string,
            _ => panic!("never reach"),
        },
        _ => panic!("never reach"),
    };
    // let child = node_id.children(event.get_node_hierarchy());
    // let child2 = child.node.unwrap().children(child.node_layout);
    println!("{:?}", grand_child.node_type);
    {
        let mut data = app_state.data.lock().unwrap(); //data is myDataModel
        println!("{:?}", label);
        println!("{:?}", &data.images.get(label).is_some());
        if let Some(ref mut image) = data.images.get_mut(label) {
            image.toggle();
        }
    }
    Redraw
}

fn set_gps(
    app_state: &mut AppState<MyDataModel>,
    event: &mut CallbackInfo<MyDataModel>,
) -> UpdateScreen {
    let mut data = app_state.data.lock().unwrap(); //data is myDataModel
    let gps_info = parse_gps(&data.gps_input.text);
    for (_, image) in &mut data.images {
        if image.choosen {
            // image.path
            let mut meta = rexiv2::Metadata::new_from_path(&image.path).unwrap();
            println!("{:?}", meta.set_gps_info(&gps_info));
            meta.save_to_file(&image.path);
        }
    }
    Redraw
}

fn paste_gps(
    app_state: &mut AppState<MyDataModel>,
    event: &mut CallbackInfo<MyDataModel>,
) -> UpdateScreen {
    let gps_string = app_state.get_clipboard_string().unwrap();
    println!("{}", gps_string);
    let mut data = app_state.data.lock().unwrap(); //data is myDataModel
    data.gps_input.text = gps_string;
    Redraw
}
