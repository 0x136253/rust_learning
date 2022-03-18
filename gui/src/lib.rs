pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

pub struct Button {
    pub width:i32,
    pub height:i32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("width:{}-height:{}-options:{}",self.width,self.height,self.label);
    }
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

