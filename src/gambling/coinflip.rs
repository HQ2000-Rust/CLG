use crate::gambling::GambleResult;
use crate::gambling::ascii_art::numbers::*;
use crate::gambling::utils::{AnimStatus, LastChange};
use crossterm::event;
use crossterm::event::{Event, KeyEvent, KeyEventKind};
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::widgets::{Block, Paragraph, Widget};
use ratatui::{DefaultTerminal, Frame};

use crossterm::event::KeyCode;
use ratatui::prelude::*;
use ratatui::symbols::border;
use std::io;
use std::time::Duration;

pub fn machine() -> io::Result<GambleResult> {
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
const CHANGE_INTERVAL: Duration = Duration::from_millis(10);
const FINAL_INTERVAL: Duration = Duration::from_millis(1000);

#[derive(Debug, Default)]
struct App {
    state: State,
    success: Option<bool>,
    anim_status: AnimStatus,
    last_change: LastChange,
    anim_states: (AnimState, AnimState, AnimState),
    end_anim: bool,
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

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
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

    fn handle_key_event(&mut self, key_event: KeyEvent) -> () {
        match key_event.code {
            KeyCode::Esc | KeyCode::Char('q') => self.exit(),
            KeyCode::Enter => {
                if self.anim_status.is_uninit() {
                    self.anim_status.start();
                }
                if let Some(_) = self.success {
                    self.exit();
                }
            }
            _ => {}
        }
    }

    fn exit(&mut self) -> () {
        self.exit = true;
    }

    fn anim(&mut self) {
        fn rand() -> bool {
            fastrand::u8(..4) == 0
        }
        if self.last_change.inner.elapsed() >= CHANGE_INTERVAL
            && !self.anim_status.is_finished()
            && !self.anim_status.is_uninit()
        {
            if !self.end_anim {
                self.end_anim = fastrand::u8(..64) == 1;
            }
            //I literally can't avoid this code duplication...
            //except with an array/vec which would be an overkill
            if !self.end_anim {
                if rand() {
                    if let AnimState(0) = self.anim_states.0.advance() {
                        self.state.0 = advance(self.state.0);
                    }
                }
                if rand() {
                    if let AnimState(0) = self.anim_states.1.advance() {
                        self.state.1 = advance(self.state.1);
                    }
                }
                if rand() {
                    if let AnimState(0) = self.anim_states.2.advance() {
                        self.state.2 = advance(self.state.2);
                    }
                }
            } else {
                if (!matches!(self.anim_states.0, AnimState(0))) && rand() {
                    self.anim_states.0.advance();
                }
                if (!matches!(self.anim_states.1, AnimState(0))) && rand() {
                    self.anim_states.1.advance();
                }
                if (!matches!(self.anim_states.2, AnimState(0))) && rand() {
                    self.anim_states.2.advance();
                }
            }
            self.last_change = LastChange::default();
        } else if self.last_change.inner.elapsed() >= FINAL_INTERVAL {
            self.anim_status.finish();
        }
        if self.end_anim
            && self.anim_states.0.0 == 0
            && self.anim_states.1.0 == 0
            && self.anim_states.2.0 == 0
        {
            self.anim_status.finish();

            if self.state.0 == self.state.1 && self.state.1 == self.state.2 {
                self.success = Some(true);
            } else {
                self.success = Some(false);
            }
        }
    }

    fn render_numbers(&self) -> String {
        //clean that up later
        let binding = render_independent_third(self.state.0, self.anim_states.0);
        let mut first = binding.lines();

        let binding = render_independent_third(self.state.1, self.anim_states.1);
        let mut second = binding.lines();

        let binding = render_independent_third(self.state.2, self.anim_states.2);
        let mut third = binding.lines();
        let mut whole = String::new();
        for _ in 0..1 + HEIGHT + 1 {
            let mut line = String::new();
            line.push_str(&first.next().expect("The number of iteration equals the iterator's number of elements, so this returns always Some(_)"));
            line.push_str(&second.next().expect("The number of iteration equals the iterator's number of elements, so this returns always Some(_)"));
            line.push_str(&third.next().expect("The number of iteration equals the iterator's number of elements, so this returns always Some(_)"));
            whole.push_str(&[&line, "\n"].join(""));
        }
        whole
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let content = if self.last_change.inner.elapsed() >= FINAL_INTERVAL
            && self.anim_status.is_finished()
        {
            match self
                .success
                .expect("Success is determined after the animation ended")
            {
                true => "You win!",
                false => "You lose!",
            }
        } else {
            &self.render_numbers()
        };

        let enter_text = if self.anim_status.is_finished()
            && self.last_change.inner.elapsed() >= FINAL_INTERVAL
        {
            " Continue ".into()
        } else {
            " Start ".into()
        };

        let title = Line::from("Gambling Machine");
        /*let title = Line::from(format!(
            "{:?} {:?} {:?} {:?} {:?}",
            self.anim_status,
            self.anim_states,
            self.state,
            self.success,
            self.end_anim
        ));*/
        let bottom_text = Line::from(vec![
            enter_text,
            " <Enter> ".into(),
            " Exit ".into(),
            " <Q> ".into(),
            " <Esc> ".into(),
        ]);
        let block = Block::new()
            .title(title.centered())
            .title_bottom(bottom_text.centered())
            .border_style(Style::new().blue())
            .border_set(border::THICK);

        Paragraph::new(content)
            .bold()
            .block(block)
            .centered()
            .render(area, buf);
        /*Paragraph::new(
            self.render_numbers()
            /*vec![
            self.render_numbers().into(),
            format!("{:?}", self.state).into(),
            format!("{:?}", self.anim_states).into(),
        ]*/).render(area, buf);*/
    }
}

#[derive(Debug, Default, Copy, Clone)]
struct AnimState(usize);
impl AnimState {
    fn advance(&mut self) -> AnimState {
        self.0 = advance_anim(self.0);
        *self
    }
}

fn advance(num: usize) -> usize {
    if num < 9 {
        return num + 1;
    }
    1
}

fn advance_anim(num: usize) -> usize {
    if num < 6 {
        return num + 1;
    }
    0
}



fn reverse(num: usize) -> usize {
    if num > 1 {
        return num - 1;
    }
    9
}

fn render_independent_third(num: usize, state: AnimState) -> String {
    let mut result = String::new();

    let digits = (reverse(num), num, advance(num));
    let str_digits = (
        digit_to_ascii(digits.0),
        digit_to_ascii(digits.1),
        digit_to_ascii(digits.2),
    );

    let AnimState(step) = state;

    result.push_str(&take_from_bottom(str_digits.0, step + 1));

    //middle
    result.push_str(&take_from_top(
        str_digits.1,
        7usize.saturating_sub(step) + 1,
    ));
    //WHY?
    //WHY?
    //WHY?

    result.push_str(&take_from_top(
        str_digits.2,
        /*2usize.saturating_sub(step)*/ 9,
    ));

    result
}

//state of the machine
#[derive(Debug)]
struct State(usize, usize, usize);

impl Default for State {
    fn default() -> State {
        let rand = fastrand::usize;
        State(rand(1..10), rand(1..10), rand(1..10))
    }
}
