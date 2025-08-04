use super::utils::{AnimStatus, GambleResult};
use color_eyre::owo_colors::StylePrefixFormatter;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::style::Style;
use ratatui::widgets::block::Title;
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
};
use std::io;
use std::time::{Duration, Instant};

pub fn coinflip(difficulty: u8) -> io::Result<GambleResult> {
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
    fn stringify(&self) -> &'static str {
        use super::ascii_art::coin;
        match self {
            AnimFrame::Side1 | AnimFrame::Side2 => coin::SIDE,
            AnimFrame::Front => coin::FRONT,
            AnimFrame::Back => coin::BACK,
        }
    }
}

#[derive(Debug, Default, Eq, PartialEq)]
enum CoinFlipStatus {
    #[default]
    Uninitialized,
    Ongoing,
    Finished,
}

#[derive(Debug)]
struct LastChange {
    inner: Instant,
}
impl Default for LastChange {
    fn default() -> Self {
        Self {
            inner: Instant::now(),
        }
    }
}

#[derive(Debug, Default)]
struct App {
    success: Option<bool>,
    //cf_status: CoinFlipStatus,
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
        if self.last_change.inner.elapsed() >= Duration::from_millis(500)
            && self.anim_status.is_ongoing()
        {
            self.anim_frame.advance();
            if self.anim_frame != AnimFrame::Side1
                && self.anim_frame != AnimFrame::Side2
                && fastrand::bool()
            {
                self.anim_status.finished();
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
                self.init_coinflip();
            }
            _ => {}
        }
    }
    fn exit(&mut self) {
        self.exit = true;
    }

    fn init_coinflip(&mut self) {
        self.anim_status.start();
        //self.cf_status = CoinFlipStatus::Ongoing;
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let coin = if self.anim_status.is_finished()
            && self.last_change.inner.elapsed() >= Duration::from_millis(1500)
        {
            match self.anim_frame {
                AnimFrame::Front => "You win!",
                AnimFrame::Back => "You lose!",
                _ => unreachable!(),
            }
        } else {
            self.anim_frame.stringify()
        };

        let enter_text = if self.anim_status.is_finished()
            && self.last_change.inner.elapsed() >= Duration::from_millis(1500)
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
        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(bottom_text.centered())
            .border_set(border::THICK);

        Paragraph::new(coin)
            .bold()
            .block(block)
            .centered()
            .render(area, buf);
    }
}
