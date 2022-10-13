use egui::{Button, Ui};

pub struct Tab {
    pub sizing: f32,
}

pub struct Tabs<'a> {
    index: usize,
    ui: &'a mut Ui,
    cnt: usize,
}

impl<'a> Tabs<'a> {
    pub fn new(ui: &'a mut Ui, index: usize) -> Self {
        Self { index, ui, cnt: 0 }
    }

    // pub fn add_header(self, header: impl FnOnce(&mut Ui)) {
    //     self.ui.horizontal(|ui| {
    //         header(ui);
    //     });
    // }

    pub fn add_header_section(self, header: impl FnOnce(&mut Ui)) -> Self {
        header(self.ui);
        self
    }

    pub fn add_body(mut self, body: impl FnOnce(&mut Ui)) -> Self {
        if self.cnt == self.index {
            body(self.ui);
        }
        self.cnt += 1;
        self
    }
}
