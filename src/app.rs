use raylib::prelude::*;



pub struct App {
    pub rd: (RaylibHandle, RaylibThread),
}

impl App {
    pub fn new() -> Self {
        let app:Self = Self {
            rd: init()
                .size(1280, 720)
                .title("raycon")
                .build(),
        };
        return app;
    }
    fn init(&mut self) {
        return;
    }
    fn update(&mut self) {
        return;
    }
    fn draw(&mut self) {
        let mut draw = self.rd.0.begin_drawing(&self.rd.1);
        draw.clear_background(Color::BLACK);
        return;
    }
    pub fn run(&mut self) {
        self.init();
        while self.rd.0.window_should_close()==false {
            self.update();
            self.draw();
        }
        return;
    }
}