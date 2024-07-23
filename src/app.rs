use raylib::prelude::*;


pub struct App {
    rd: (RaylibHandle, RaylibThread),
    keyboard: crate::keyboard::Keyboard,
    input_handler: crate::input_handler::InputHandler,
    output_handler: crate::output_handler::OutputHandler,
}

impl App {
    pub fn new() -> Self {
        let app:Self = Self {
            rd: init()
                .size(1280, 720)
                .title("raycon")
                .build(),
            keyboard: crate::keyboard::Keyboard::new(),
            input_handler: crate::input_handler::InputHandler::new(),
            output_handler: crate::output_handler::OutputHandler::new(),
        };
        return app;
    }
    fn init(&mut self) {
        self.rd.0.set_target_fps(60);
        return;
    }
    fn update(&mut self) {
        let input:String = self.keyboard.get(&mut self.rd.0);
        let info:Vec<String> = self.input_handler.process(&input);
        let mut output = self.output_handler.process(&info);
        dbg!(info);
        return;
    }
    fn draw(&mut self) {
        let mut draw:RaylibDrawHandle = self.rd.0.begin_drawing(&self.rd.1);
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