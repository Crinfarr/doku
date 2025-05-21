use std::{io};
use crossterm::{event, event::KeyEventKind};
use ratatui::{buffer::Buffer, layout::Rect, symbols, widgets::{Block, Widget}, DefaultTerminal, Frame};


fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    App::new().run(&mut terminal)
}

#[derive(Debug, Default)]
pub struct App {
    exit:bool,
}
impl App {
    pub fn new() -> Self {
        return Self::default();
    }
    pub fn run(&mut self, term:&mut DefaultTerminal) -> io::Result<()> {
        loop {
            if self.exit {
                break;
            }
            let _draw = term.draw(|frame| self.draw(frame));
            self.handle_events()?;
        }
        return Ok(());
    }
    fn draw(&self, frame:&mut Frame) {
        frame.render_widget(self, frame.area());
    }
    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            event::Event::Key(ev_key) if ev_key.kind == KeyEventKind::Press => {
                match ev_key.code {
                    event::KeyCode::Char('q') => {self.exit = true}
                    _=>{}
                }
            }
            _=>{}
        };
        return Ok(());
    }
}
impl Widget for &App {
    fn render(self, rect:Rect, buf:&mut Buffer) {
        Block::bordered().border_set(symbols::border::PLAIN).render(rect, buf);
    }
}