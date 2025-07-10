use eframe::egui;
use rodio::{Decoder, OutputStream, Sink};  
use std::fs::File;
use std::io::BufReader;


struct MyApp
{
    is_playing: bool,
}


impl Default for MyApp
{
    fn default() -> Self {
        Self {is_playing: false}
    }
}

impl eframe::App for MyApp
{
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame)
    {
        egui::CentralPanel::default().show(ctx, |ui|
        {
            if ui.button("Play").clicked()
            {
                self.is_playing = true;
                //add stuff here to play music!
            }
        });
    }
}

fn main()
{
     let options = eframe::NativeOptions::default();
    eframe::run_native(
        "crittertunes",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    );
}