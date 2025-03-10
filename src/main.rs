use eframe::egui;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions { //setting options with defaul options except for outlined
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };
    eframe::run_native(
        "My Future Calculator App",
        options,
        Box::new(|_cc| {
           Ok(Box::<MyApp>::default())
        })
    )
}

struct MyApp {
    int1: usize,
    int2: usize,
}

impl Default for MyApp {
    fn default() -> Self {
        Self { 
            int1: 0,
            int2: 1,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
           
            ui.heading("My Future Calculator App");
            
            ui.horizontal(|ui| {
                if ui.button("7").clicked() {
                     println!("You clicked 7");}
                if ui.button("8").clicked() {
                     println!("You clicked 8");}
                if ui.button("9").clicked() {
                      println!("You clicked 9");}
            });
            ui.horizontal(|ui| {
                if ui.button("4").clicked() {
                     println!("You clicked 4");}
                if ui.button("5").clicked() {
                     println!("You clicked 5");}
                if ui.button("6").clicked() {
                      println!("You clicked 6");}
            });
            ui.horizontal(|ui| {
                if ui.button("1").clicked() {
                    println!("clicked 1");
                 }
                if ui.button("2").clicked() {
                    println!("clicked 2");
                }
                if ui.button("3").clicked() {
                    println!("Clicked 3");
                }
             });
         });
    }
}
