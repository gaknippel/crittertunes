use eframe::egui;
use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink};  
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use rfd::FileDialog;

struct MyApp
{
    is_playing: bool,
    file_path: Option<PathBuf>,
    #[allow(dead_code)]
    stream: Option<OutputStream>,
    sink: Option<Sink>,
}


impl Default for MyApp
{
    fn default() -> Self {
        Self {is_playing: false,
        file_path: None,
        stream: None,
        sink: None,
    }
    }
}

impl eframe::App for MyApp
{
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame)
    {
        egui::CentralPanel::default().show(ctx, |ui|
        {
            if ui.button("Load MP3").clicked()
            {
                if let Some(path) = FileDialog::new().add_filter("MP3", & ["mp3"]).pick_file()
                {
                    self.file_path = Some(path);
                    self.is_playing = false;
                    self.sink = None; //stop prev playback
                }
            }

            if let Some(path) = &self.file_path
            {
                ui.label(format!("Loaded: {}", path.display()));
                let available = ui.available_size();
            
            ui.add_space(available.y/ 2.0 - 20.0);

            ui.horizontal(|ui| 

                    {
                ui.add_space(available.x / 2.0 - 40.0);

                let mut play_button = egui::Button::new(egui::RichText::new("play!").size(32.0));

            if ui.add(play_button).clicked() && !self.is_playing
                {
                if let Ok((stream, stream_handle)) = OutputStream::try_default()
                {
                    let sink = Sink::try_new(&stream_handle).unwrap();
                    let file = BufReader::new(File::open(path).unwrap());
                    let source = Decoder::new(file).unwrap();
                    sink.append(source);
                    sink.play();
                    self.stream = Some(stream);
                    self.sink = Some(sink);
                    self.is_playing = true;
                }
                
                }
                    }
            );
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