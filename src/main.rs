#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
use eframe::{
    egui::{self, TextStyle},
    epaint::FontId, Theme,
};
use std::{fs::{read_to_string}};
use std::{env, fs::OpenOptions, io::Write, path::Path};
use tinyfiledialogs;


fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(960.0, 540.0)),
        default_theme: Theme::Dark,
        ..Default::default()
    };
    eframe::run_native(
        "cool notepad",
        options,
        Box::new(|cc| Box::new(MyApp::new(cc))),
    )
}

struct MyApp {
    tmp_path: String,
    font_size: f32,
    text: String,
    fileex: bool,
}

fn configure_text_styles(ctx: &egui::Context) {
    use egui::FontFamily::{Monospace, Proportional};

    let mut style = (*ctx.style()).clone();
    style.text_styles = [
        (TextStyle::Heading, FontId::new(25.0, Proportional)),
        (TextStyle::Body, FontId::new(25.0, Proportional)),
        (TextStyle::Monospace, FontId::new(25.0, Monospace)),
        (TextStyle::Button, FontId::new(50.0, Proportional)),
        (TextStyle::Small, FontId::new(50.0, Proportional)),
    ]
    .into();
    ctx.set_style(style);
}

impl Default for MyApp {
    fn default() -> Self {        
        let args: Vec<String> = env::args().collect();
        match args.len() {
            2 => {
                MyApp {
                    tmp_path: "".to_string(),
                    font_size: 20.0,
                    text: read_to_string(Path::new(&args[1])).unwrap(),
                    fileex: true,
                }   
            },
            _ => {
                MyApp {
                    tmp_path: "".to_string(),
                    font_size: 20.0,
                    text: "".to_string(),
                    fileex: false,
                } 
            }
        }
    }
}

impl MyApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        configure_text_styles(&cc.egui_ctx);
        MyApp::default()
    }
}

impl eframe::App for MyApp {
    
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            
            egui::ScrollArea::vertical()
                .show(ui, |ui| {
                    ui.add(
                        
                        egui::TextEdit::multiline(&mut self.text)
                            .code_editor()
                            .desired_rows((ui.available_height() / 22.0) as usize)
                            .desired_width(ui.available_width())
                            .font(egui::FontId::monospace(self.font_size)),
                    );
                    ui.horizontal(|ui|
                        ui.label("this some info")

                    );
                    ctx.input(|input_state| {
                        if input_state.key_pressed(egui::Key::S) && input_state.modifiers.ctrl {
                            match self.fileex {
                                false => {
                                    let file_path = tinyfiledialogs::open_file_dialog(
                                        "В какой файл сохранить",
                                        "",
                                        None,
                                    );
                                    let _path: () = match file_path {
                                        Some(path) => {
                                            let mut file =
                                                match OpenOptions::new().write(true).open(&path) {
                                                    Ok(file) => file,
                                                    Err(_) => OpenOptions::new()
                                                        .write(true)
                                                        .open(env::current_dir().unwrap())
                                                        .unwrap(),
                                                };
                                            file.write_all(self.text.as_bytes()).unwrap();
                                        }
                                        None => (),
                                    };
                                }
                                true => {
                                    let x = &env::args().collect::<Vec<String>>();

                                    let path =  match  &env::args().collect::<Vec<String>>().len() {
                                        2 => &x[1],
                                        _ => &self.tmp_path
                                    };
                                    let mut file = match OpenOptions::new().write(true).open(&path)
                                    {
                                        Ok(file) => file,
                                        Err(_) => OpenOptions::new()
                                            .write(true)
                                            .open(env::current_dir().unwrap())
                                            .unwrap(),
                                    };
                                    file.write_all(
                                        &(self.text.chars().map(|c| c as u8).collect::<Vec<_>>()),
                                    )
                                    .unwrap();
                                }
                            }
                        }
                    });
                    ctx.input(|input_state| {
                        if input_state.key_pressed(egui::Key::Minus) && input_state.modifiers.ctrl {
                            self.font_size -= 2.0;
                        }
                        if input_state.key_pressed(egui::Key::PlusEquals) && input_state.modifiers.ctrl {
                            self.font_size += 2.0;
                        }
                    });
                    ctx.input(|input_state| {
                        if input_state.key_pressed(egui::Key::Q) && input_state.modifiers.ctrl {
                            let file_path = tinyfiledialogs::open_file_dialog(
                                "Какой файл открыть",
                                "",
                                None,
                            );
                            match file_path {
                                Some(path) => {
                                    self.fileex = true;
                                    self.tmp_path = path.clone();
                                    self.text = read_to_string(path).unwrap()
                                },
                                None => ()
                            }
                            
                        }
                    });

                });
        });
    }
}
