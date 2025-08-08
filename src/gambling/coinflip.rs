use super::utils::{AnimStatus, GambleResult, LastChange};
use ansi_to_tui::IntoText;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::style::Style;
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    symbols::border,
    text::Line,
    widgets::{Block, Paragraph, Widget},
};
use std::io;
use std::time::Duration;

pub fn coinflip() -> io::Result<GambleResult> {
    let mut terminal = ratatui::init();
    let mut app = App::default();
    let result = app.run(&mut terminal)?;
    ratatui::restore();
    if let Some(success) = result {
        return Ok(success.into());
    }
    std::process::exit(-1);
}

const TIMEOUT: Duration = Duration::from_millis(10);
const CHANGE_INTERVAL: Duration = Duration::from_millis(500);
const FINAL_INTERVAL: Duration = Duration::from_millis(1000);

#[derive(Debug, Eq, PartialEq)]
enum AnimFrame {
    Front,
    Side1,
    Back,
    Side2,
}

impl Default for AnimFrame {
    fn default() -> Self {
        match fastrand::bool() {
            false => AnimFrame::Back,
            true => AnimFrame::Front,
        }
    }
}

impl AnimFrame {
    fn advance(&mut self) {
        *self = match self {
            AnimFrame::Front => AnimFrame::Side1,
            AnimFrame::Side1 => AnimFrame::Back,
            AnimFrame::Back => AnimFrame::Side2,
            AnimFrame::Side2 => AnimFrame::Front,
        }
    }
    fn stringify(&self) -> ratatui::text::Text<'static> {
        use super::ascii_art::coin;
        use ansi_to_tui::IntoText;
        match self {
            AnimFrame::Side1 | AnimFrame::Side2 => {
                coin::SIDE.into_text().expect("correct ANSI color codes")
            }
            AnimFrame::Front => coin::TAILS.into_text().expect("correct ANSI color codes"),
            AnimFrame::Back => coin::HEAD.into_text().expect("correct ANSI color codes"),
        }
    }
}



#[derive(Debug, Default)]
struct App {
    success: Option<bool>,
    anim_status: AnimStatus,
    anim_frame: AnimFrame,
    last_change: LastChange,
    exit: bool,
}

impl App {
    fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<Option<bool>> {
        while !self.exit {
            self.handle_events()?;
            self.anim();
            terminal.draw(|mut frame| self.draw(&mut frame))?;
        }
        Ok(self.success)
    }

    fn anim(&mut self) {
        if self.last_change.inner.elapsed() >= CHANGE_INTERVAL && self.anim_status.is_ongoing() {
            self.anim_frame.advance();
            if self.anim_frame != AnimFrame::Side1
                && self.anim_frame != AnimFrame::Side2
                && fastrand::bool()
            {
                self.anim_status.finish();
                match self.anim_frame {
                    AnimFrame::Front => {
                        self.success = Some(true);
                    }
                    AnimFrame::Back => {
                        self.success = Some(false);
                    }
                    _ => unreachable!("just checked for the absence of these patterns"),
                }
            }

            self.last_change = LastChange::default()
        };
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area())
    }

    fn handle_events(&mut self) -> io::Result<()> {
        if event::poll(TIMEOUT)? {
            match event::read()? {
                Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                    self.handle_key_event(key_event);
                }
                _ => {}
            }
        }
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Esc | KeyCode::Char('q') => {
                self.success = None;
                self.exit();
            }
            KeyCode::Enter => {
                if self.anim_status.is_finished() {
                    self.exit();
                }
                self.anim_status.start();
            }
            _ => {}
        }
    }
    fn exit(&mut self) {
        self.exit = true;
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {

        let coin = if self.anim_status.is_finished()
            && self.last_change.inner.elapsed() >= FINAL_INTERVAL
        {
            match self.anim_frame {
                AnimFrame::Front => "You win!".into_text().expect("correct ANSI color codes"),
                AnimFrame::Back => "You lose!".into_text().expect("correct ANSI color codes"),
                _ => unreachable!(),
            }
        } else {
            self.anim_frame.stringify()
        };

        let enter_text = if self.anim_status.is_finished()
            && self.last_change.inner.elapsed() >= FINAL_INTERVAL
        {
            " Continue ".into()
        } else {
            " Flip the coin ".into()
        };

        let title = Line::from(" Coinflip ");
        let bottom_text = Line::from(vec![
            enter_text,
            " <Enter> ".bold().blue(),
            " Exit ".into(),
            " <Q> ".bold().blue(),
            " <Esc> ".bold().blue(),
        ]);
        let block = Block::new()
            .title(title.centered())
            .title_bottom(bottom_text.centered())
            .border_style(Style::new().blue())
            .border_set(border::THICK);

        Paragraph::new(coin)
            .bold()
            .block(block)
            .centered()
            .render(area, buf);
    }
}
