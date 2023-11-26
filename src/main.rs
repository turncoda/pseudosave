#![windows_subsystem = "windows"]

use native_windows_derive as nwd;
use native_windows_gui as nwg;

use lazy_static::lazy_static;

use nwd::NwgUi;
use nwg::stretch::geometry::{Rect, Size};
use nwg::stretch::style::{Dimension, FlexDirection, FlexWrap};
use nwg::NativeUi;
use std::cell::RefCell;
use std::env;

use gvas::properties::name_property::NameProperty;
use gvas::properties::str_property::StrProperty;
use gvas::properties::Property;
use gvas::GvasFile;
use std::fs::File;
use std::path::Path;

const ICON: &[u8] = include_bytes!("assets/crystal.ico");
const PK_LEVEL_NAME: &str = "lastSavedZoneSpawnIn";
const PK_START_TAG: &str = "spawnPointTag";
const UPGRADE_NAMES: [&str; 21] = [
    "attack",
    "airKick",
    "slide",
    "plunge",
    "wallRide",
    "Light",
    "projectile",
    "sprint",
    "powerBoost",
    "SlideJump",
    "guard",
    "chargeAttack",
    "extraKick",
    "airRecovery",
    "mobileHeal",
    "magicHaste",
    "HealBoost",
    "damageBoost",
    "healthPiece",
    "magicPiece",
    "outfitPro",
];
lazy_static! {
    static ref EMPTY_STR_PROP: Property = Property::from(StrProperty { value: None });
}

#[derive(Default, NwgUi)]
pub struct App {
    save_file: RefCell<Option<GvasFile>>,

    #[nwg_control(size: (960, 640), position: (160, 40), title: "Pseudoregalia Save File Editor")]
    #[nwg_events( OnWindowClose: [App::exit] )]
    window: nwg::Window,

    #[nwg_layout(parent: window, max_row: Some(12), max_column: Some(5) )]
    main_layout: nwg::GridLayout,

    #[nwg_resource(title: "Open File", action: nwg::FileDialogAction::Open, filters: "SAV(*.sav)")]
    dialog: nwg::FileDialog,

    #[nwg_control(text: "Open", focus: true)]
    #[nwg_layout_item(layout: main_layout, col: 0, row: 0)]
    #[nwg_events(OnButtonClick: [App::open_file])]
    open_btn: nwg::Button,

    #[nwg_control(text: "Write", enabled: false)]
    #[nwg_layout_item(layout: main_layout, col: 1, row: 8, col_span: 3)]
    #[nwg_events(OnButtonClick: [App::write_file])]
    write_btn: nwg::Button,

    #[nwg_control(readonly: true)]
    #[nwg_layout_item(layout: main_layout, col: 1, row: 0, col_span: 4)]
    file_name: nwg::TextInput,

    #[nwg_control(text: PK_LEVEL_NAME, h_align: HTextAlign::Right)]
    #[nwg_layout_item(layout: main_layout, col: 0, row: 1)]
    level_name_label: nwg::Label,

    #[nwg_control(readonly: true, placeholder_text: Some("<empty>"))]
    #[nwg_layout_item(layout: main_layout, col: 1, row: 1, col_span: 4)]
    level_name: nwg::TextInput,

    #[nwg_control(text: PK_START_TAG, h_align: HTextAlign::Right)]
    #[nwg_layout_item(layout: main_layout, col: 0, row: 2)]
    start_tag_label: nwg::Label,

    #[nwg_control(readonly: true, placeholder_text: Some("<empty>"))]
    #[nwg_layout_item(layout: main_layout, col: 1, row: 2, col_span: 4)]
    start_tag: nwg::TextInput,

    #[nwg_control]
    #[nwg_layout_item(layout: main_layout, col: 0, row: 3, col_span: 5, row_span: 5)]
    powerups_frame: nwg::Frame,

    #[nwg_layout(parent: powerups_frame)]
    powerups_layout: nwg::FlexboxLayout,

    #[nwg_control(readonly: true, flags: "VISIBLE|VSCROLL|AUTOVSCROLL")]
    #[nwg_layout_item(layout: main_layout, col: 0, row: 9, col_span: 5, row_span: 3)]
    log_box: nwg::TextBox,

    upgrades: RefCell<Vec<Upgrade>>,
}

#[derive(Default)]
struct Upgrade {
    frame: nwg::Frame,
    layout: nwg::FlexboxLayout,
    text_input: nwg::TextInput,
    label: nwg::Label,
    name: String,
}

impl App {
    fn open_file(&self) {
        if let Ok(d) = env::current_dir() {
            if let Some(d) = d.to_str() {
                self.dialog
                    .set_default_folder(d)
                    .expect("Failed to set default folder.");
            }
        }

        if self.dialog.run(Some(&self.window)) {
            self.file_name.set_text("");
            if let Ok(directory) = self.dialog.get_selected_item() {
                let dir = directory.into_string().unwrap();
                self.file_name.set_text(&dir);
                self.read_file();
                self.write_btn.set_enabled(true);
            }
        }
    }

    fn read_file(&self) {
        let mut file = File::open(&self.file_name.text()).unwrap();
        let gvas_file = GvasFile::read(&mut file).unwrap();
        if let Property::StrProperty(prop) = gvas_file
            .properties
            .get(PK_START_TAG)
            .unwrap_or(&EMPTY_STR_PROP)
        {
            self.start_tag
                .set_text(prop.value.as_ref().unwrap_or(&String::new()));
            self.start_tag.set_readonly(false);
        }
        if let Property::StrProperty(prop) = gvas_file
            .properties
            .get(PK_LEVEL_NAME)
            .unwrap_or(&EMPTY_STR_PROP)
        {
            self.level_name
                .set_text(prop.value.as_ref().unwrap_or(&String::new()));
            self.level_name.set_readonly(false);
        }
        let upgrades_map = &gvas_file
            .properties
            .get("upgrades")
            .unwrap()
            .get_map()
            .unwrap()
            .value;
        for upgrade_ui in self.upgrades.borrow_mut().iter_mut() {
            let value = upgrades_map
                .get(&Property::from(NameProperty {
                    value: upgrade_ui.name.clone(),
                }))
                .unwrap();
            upgrade_ui
                .text_input
                .set_text(&value.get_int().unwrap().value.to_string());
            upgrade_ui.text_input.set_readonly(false);
        }
        *self.save_file.borrow_mut() = Some(gvas_file);
    }

    fn update_save_file(&self, gvas_file: &mut GvasFile) -> bool {
        let mut is_changed = false;

        for (pk, new_value) in [
            (PK_LEVEL_NAME, self.level_name.text()),
            (PK_START_TAG, self.start_tag.text())
        ] {
            let mut prop = gvas_file
                .properties
                .get(pk)
                .unwrap_or(&EMPTY_STR_PROP)
                .get_str()
                .unwrap()
                .clone();
            let old_value = prop.value.clone().unwrap_or(String::new());
            if old_value != new_value {
                is_changed = true;
                prop.value.replace(new_value);
            }
            if prop.value == Some("".to_string()) || prop.value == None {
                gvas_file.properties.remove(pk);
            } else {
                gvas_file
                    .properties
                    .insert(pk.to_string(), Property::from(prop));
            }
        }

        let upgrades_prop = gvas_file.properties.get_mut("upgrades").unwrap();
        let upgrades_map = &mut upgrades_prop.get_map_mut().unwrap().value;
        for upgrade_ui in self.upgrades.borrow().iter() {
            let key = Property::from(NameProperty {
                value: upgrade_ui.name.clone(),
            });
            let value = i32::from_str_radix(&upgrade_ui.text_input.text(), 10).unwrap();
            let existing_prop = upgrades_map.get_mut(&key).unwrap();
            let existing_value = &mut existing_prop.get_int_mut().unwrap().value;
            if *existing_value != value {
                is_changed = true;
                *existing_value = value;
            }
        }

        is_changed
    }

    fn log(&self, msg: &str) {
        self.log_box.appendln(msg);
    }

    fn write_file(&self) {
        if let Some(sav) = self.save_file.borrow_mut().as_mut() {
            if !self.update_save_file(sav) {
                self.log("No change detected.");
                return;
            }
            let file_path_string = self.file_name.text().to_string();
            let file_path = Path::new(&file_path_string);
            let backup_file_path_string = format!("{}.bak", file_path_string);
            let backup_file_path = Path::new(&backup_file_path_string);
            if !backup_file_path.exists() {
                self.log(&format!(
                    "Created backup copy: {}",
                    backup_file_path.display()
                ));
                std::fs::copy(&file_path, &backup_file_path).unwrap();
            } else {
                self.log(&format!(
                    "Backup copy exists, leaving it as is: {}",
                    backup_file_path_string
                ));
            }
            let mut file = File::create(file_path).unwrap();
            let _ = sav.write(&mut file);
            self.log(&format!("Wrote to: {}", file_path_string));
        }
    }

    fn exit(&self) {
        nwg::stop_thread_dispatch();
    }
}

fn main() {
    nwg::init().expect("Failed to init Native Windows GUI");
    nwg::Font::set_global_family("Segoe UI").expect("Failed to set default font");
    let icon = nwg::Icon::from_bin(ICON).unwrap();

    let mut _app = App::build_ui(Default::default()).expect("Failed to build UI");
    _app.window.set_icon(Some(&icon));
    for upgrade_name in UPGRADE_NAMES {
        let mut upgrade = Upgrade::default();
        upgrade.name = upgrade_name.to_string();
        nwg::Frame::builder()
            .parent(&_app.powerups_frame)
            .build(&mut upgrade.frame)
            .unwrap();
        nwg::TextInput::builder()
            .parent(&upgrade.frame)
            .readonly(true)
            .build(&mut upgrade.text_input)
            .unwrap();
        nwg::Label::builder()
            .parent(&upgrade.frame)
            .text(upgrade_name)
            .build(&mut upgrade.label)
            .unwrap();
        nwg::FlexboxLayout::builder()
            .parent(&upgrade.frame)
            .child(&upgrade.label)
            .child_margin(Rect {
                start: Dimension::Points(8.0),
                end: Dimension::Undefined,
                top: Dimension::Undefined,
                bottom: Dimension::Undefined,
            })
            .child(&upgrade.text_input)
            .child_margin(Rect {
                start: Dimension::Undefined,
                end: Dimension::Points(8.0),
                top: Dimension::Undefined,
                bottom: Dimension::Undefined,
            })
            .build(&mut upgrade.layout)
            .unwrap();
        _app.upgrades.borrow_mut().push(upgrade);
    }

    let mut builder = nwg::FlexboxLayout::builder()
        .parent(&_app.powerups_frame)
        .flex_wrap(FlexWrap::Wrap)
        .flex_direction(FlexDirection::Column)
        .auto_spacing(Some(2));

    for upgrade in _app.upgrades.borrow().iter() {
        builder = builder.child(&upgrade.frame).child_min_size(Size {
            width: Dimension::Points(160.0),
            height: Dimension::Points(36.0),
        });
    }
    builder.build(&_app.powerups_layout).unwrap();

    nwg::dispatch_thread_events();
}
