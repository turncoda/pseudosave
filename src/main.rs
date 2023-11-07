use native_windows_gui as nwg;
use native_windows_derive as nwd;

use lazy_static::lazy_static;

use nwd::NwgUi;
use nwg::NativeUi;
use std::env;
use std::cell::RefCell;

use std::fs::File;
use gvas::GvasFile;
use gvas::properties::str_property::StrProperty;
use gvas::properties::Property;

const PK_LEVEL_NAME: &str = "lastSavedZoneSpawnIn";
lazy_static!{
    static ref EMPTY_STR_PROP: Property = Property::from(StrProperty{value: None});
}

#[derive(Default, NwgUi)]
pub struct App {
    save_file: RefCell<Option<GvasFile>>,

    #[nwg_control(size: (720, 360), position: (120, 120), title: "Pseudoregalia Save File Editor")]
    #[nwg_events( OnWindowClose: [App::exit] )]
    window: nwg::Window,

    #[nwg_layout(parent: window, max_row: Some(9), max_column: Some(6) )]
    main_layout: nwg::GridLayout,

    #[nwg_resource(title: "Open File", action: nwg::FileDialogAction::Open, filters: "SAV(*.sav)")]
    dialog: nwg::FileDialog,

    #[nwg_control(text: "Open", focus: true)]
    #[nwg_layout_item(layout: main_layout, col: 0, row: 0)]
    #[nwg_events(OnButtonClick: [App::open_file])]
    open_btn: nwg::Button,

    #[nwg_control(text: "Write", enabled: false)]
    #[nwg_layout_item(layout: main_layout, col: 0, row: 4, col_span: 6)]
    #[nwg_events(OnButtonClick: [App::write_file])]
    write_btn: nwg::Button,

    #[nwg_control(readonly: true)]
    #[nwg_layout_item(layout: main_layout, col: 1, row: 0, col_span: 5)]
    file_name: nwg::TextInput,

    #[nwg_control(text: "Level name:", h_align: HTextAlign::Right)]
    #[nwg_layout_item(layout: main_layout, col: 0, row: 1)]
    level_name_label: nwg::Label,

    #[nwg_control]
    #[nwg_layout_item(layout: main_layout, col: 1, row: 1, col_span: 5)]
    level_name: nwg::TextInput,

    #[nwg_control(readonly: true, flags: "VISIBLE|VSCROLL|AUTOVSCROLL")]
    #[nwg_layout_item(layout: main_layout, col: 0, row: 5, col_span: 6, row_span: 4)]
    log_box: nwg::TextBox,
}

impl App {

    fn open_file(&self) {
        if let Ok(d) = env::current_dir() {
            if let Some(d) = d.to_str() {
                self.dialog.set_default_folder(d).expect("Failed to set default folder.");
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
        *self.save_file.borrow_mut() = Some(GvasFile::read(&mut file).unwrap());
        if let Property::StrProperty(prop) = self.save_file.borrow().as_ref().unwrap().properties.get(PK_LEVEL_NAME).unwrap_or(&EMPTY_STR_PROP) {
            self.level_name.set_text(prop.value.as_ref().unwrap_or(&String::new()));
        }
    }

    fn update_save_file(&self, sav: &mut GvasFile) -> bool {
        let mut is_changed = false;

        let mut level_name_prop = sav.properties.get(PK_LEVEL_NAME).unwrap_or(&EMPTY_STR_PROP).get_str().unwrap().clone();
        let old_level_name = level_name_prop.value.clone().unwrap_or(String::new());
        let new_level_name = self.level_name.text();
        if old_level_name != new_level_name {
            is_changed = true;
            level_name_prop.value.replace(new_level_name);
        }
        sav.properties.insert(PK_LEVEL_NAME.to_string(), Property::from(level_name_prop));

        is_changed
    }

    fn write_backup_file(&self) {
    }

    fn write_file(&self) {
        if let Some(sav) = self.save_file.borrow_mut().as_mut() {
            if !self.update_save_file(sav) {
                self.log_box.appendln("No change detected.");
                return;
            }
            let file_name = self.file_name.text();
            let mut file = File::create(file_name.as_str()).unwrap();
            let _ = sav.write(&mut file);
            self.log_box.appendln(&format!("Wrote to {}", file_name));
        }
    }

    fn exit(&self) {
        nwg::stop_thread_dispatch();
    }

}

fn main() {
    nwg::init().expect("Failed to init Native Windows GUI");
    nwg::Font::set_global_family("Segoe UI").expect("Failed to set default font");

    let _app = App::build_ui(Default::default()).expect("Failed to build UI");


    nwg::dispatch_thread_events();
}
