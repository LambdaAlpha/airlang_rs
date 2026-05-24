use std::cell::RefCell;
use std::rc::Rc;

use airlang::cfg::error::ABORT_MSG;
use airlang::cfg::error::ABORT_TYPE;
use airlang::cfg::prelude;
use airlang::semantics::cfg::Cfg;
use airlang::semantics::core::Eval;
use airlang::semantics::ctx::Ctx;
use airlang::semantics::func::DynFunc;
use airlang::semantics::val::Val;
use airlang::type_::Key;
use airlang_ext::cfg::prim::lib::io::Output;
use airlang_ext::cfg::prim::lib::io::STANDARD_ERROR;
use airlang_ext::cfg::prim::lib::io::STANDARD_OUTPUT;
use ratatui::DefaultTerminal;
use ratatui::Frame;
use ratatui::crossterm::event::Event;
use ratatui::crossterm::event::KeyCode;
use ratatui::crossterm::event::KeyEvent;
use ratatui::crossterm::event::KeyEventKind;
use ratatui::crossterm::event::KeyModifiers;
use ratatui::crossterm::event::read;
use ratatui::layout::Constraint;
use ratatui::layout::Direction;
use ratatui::layout::Layout;
use ratatui::layout::Position;
use ratatui::layout::Rect;
use ratatui::layout::Size;
use ratatui::style::Color;
use ratatui::style::Modifier;
use ratatui::style::Style;
use ratatui::text::Line as TuiLine;
use ratatui::widgets::Block;
use ratatui::widgets::BorderType;
use ratatui::widgets::Borders;
use ratatui::widgets::Padding;
use ratatui::widgets::Paragraph;
use ratatui::widgets::Wrap;
use ratatui_textarea::CursorMove;
use ratatui_textarea::TextArea;
use tui_scrollview::ScrollView;
use tui_scrollview::ScrollViewState;
use tui_scrollview::ScrollbarVisibility;

use crate::cfg::comp::BinCompCfg;

pub struct Repl {
    cfg: Cfg,
    ctx: Val,
    version: String,
    stdout_buf: Rc<RefCell<Vec<u8>>>,
    stderr_buf: Rc<RefCell<Vec<u8>>>,
    histories: Vec<History>,
    history_nav: Option<usize>,
    scroll_state: ScrollViewState,
    textarea: TextArea<'static>,
}

struct History {
    input: String,
    paragraphs: Vec<Paragraph<'static>>,
}

const TIPS_TEXT: &str = "Ctrl+Q quit | Ctrl+S submit | Ctrl+PgUp/PgDn history | PgUp/PgDn scroll";

const CTRL_SHIFT: KeyModifiers =
    KeyModifiers::from_bits_truncate(KeyModifiers::CONTROL.bits() | KeyModifiers::SHIFT.bits());

impl Repl {
    pub fn new() -> Self {
        let mut cfg = BinCompCfg::generate();
        let ctx = prelude(&mut cfg);
        let version = format!("🜁  Air v{}", env!("CARGO_PKG_VERSION"));

        let stdout_buf = new_buffer(&mut cfg, STANDARD_OUTPUT);
        let stderr_buf = new_buffer(&mut cfg, STANDARD_ERROR);

        Self {
            cfg,
            ctx,
            version,
            stdout_buf,
            stderr_buf,
            histories: Vec::new(),
            history_nav: None,
            scroll_state: ScrollViewState::default(),
            textarea: new_textarea(),
        }
    }

    pub fn run(mut self) -> std::io::Result<()> {
        ratatui::run(|terminal| self.loop_(terminal))
    }

    fn loop_(&mut self, terminal: &mut DefaultTerminal) -> std::io::Result<()> {
        terminal.draw(|f| self.render(f))?;
        loop {
            match read()? {
                Event::Key(key) => {
                    if key.kind != KeyEventKind::Press {
                        continue;
                    }
                    if self.handle_key(key) {
                        break;
                    }
                },
                Event::Mouse(_) => continue,
                Event::Paste(text) => {
                    self.history_nav = None;
                    self.textarea.insert_str(&text);
                },
                Event::Resize(_, _) => {},
                Event::FocusGained => continue,
                Event::FocusLost => continue,
            }
            terminal.draw(|f| self.render(f))?;
        }
        Ok(())
    }

    fn handle_key(&mut self, key: KeyEvent) -> bool {
        let reset_nav = !matches!(
            (key.modifiers, key.code),
            (KeyModifiers::NONE | KeyModifiers::CONTROL, KeyCode::PageUp | KeyCode::PageDown)
        );
        if reset_nav {
            self.history_nav = None;
        }

        match (key.modifiers, key.code) {
            (KeyModifiers::CONTROL, KeyCode::Char('q')) => return true,

            (KeyModifiers::CONTROL, KeyCode::Char('s')) => self.submit(),

            (KeyModifiers::NONE, KeyCode::PageUp) => self.scroll_state.scroll_page_up(),
            (KeyModifiers::NONE, KeyCode::PageDown) => self.scroll_state.scroll_page_down(),
            (KeyModifiers::CONTROL, KeyCode::Up) => self.scroll_state.scroll_up(),
            (KeyModifiers::CONTROL, KeyCode::Down) => self.scroll_state.scroll_down(),
            (KeyModifiers::CONTROL, KeyCode::PageUp) => self.navigate_history(true),
            (KeyModifiers::CONTROL, KeyCode::PageDown) => self.navigate_history(false),

            (KeyModifiers::CONTROL, KeyCode::Char('a')) => self.textarea.select_all(),
            (KeyModifiers::CONTROL, KeyCode::Char('c')) => self.textarea.copy(),
            (KeyModifiers::CONTROL, KeyCode::Char('x')) => {
                self.textarea.cut();
            },
            (KeyModifiers::CONTROL, KeyCode::Char('v')) => {
                self.textarea.paste();
            },
            (KeyModifiers::CONTROL, KeyCode::Char('z')) => {
                self.textarea.undo();
            },
            (KeyModifiers::CONTROL, KeyCode::Char('y')) => {
                self.textarea.redo();
            },

            (KeyModifiers::NONE, KeyCode::Tab) => self.handle_tab(),
            (KeyModifiers::NONE, KeyCode::Up) => self.textarea.move_cursor(CursorMove::Up),
            (KeyModifiers::NONE, KeyCode::Down) => self.textarea.move_cursor(CursorMove::Down),
            (KeyModifiers::NONE, KeyCode::Backspace) => {
                self.textarea.delete_char();
            },
            (KeyModifiers::NONE, KeyCode::Delete) => {
                self.textarea.delete_next_char();
            },
            (KeyModifiers::NONE, KeyCode::Enter) => self.textarea.insert_newline(),
            (KeyModifiers::NONE, KeyCode::Esc) => self.textarea.cancel_selection(),
            (KeyModifiers::NONE | KeyModifiers::SHIFT, KeyCode::Char(c)) => {
                self.textarea.insert_char(c);
            },

            (modifiers, key_code) => Self::handle_move(&mut self.textarea, modifiers, key_code),
        }
        false
    }

    fn handle_move(textarea: &mut TextArea, modifiers: KeyModifiers, key_code: KeyCode) {
        match key_code {
            KeyCode::Home => match modifiers {
                KeyModifiers::NONE => textarea.move_cursor(CursorMove::Head),
                KeyModifiers::SHIFT => {
                    if !textarea.is_selecting() {
                        textarea.start_selection();
                    }
                    textarea.move_cursor(CursorMove::Head);
                },
                KeyModifiers::CONTROL => {
                    textarea.move_cursor(CursorMove::Top);
                    textarea.move_cursor(CursorMove::Head);
                },
                CTRL_SHIFT => {
                    if !textarea.is_selecting() {
                        textarea.start_selection();
                    }
                    textarea.move_cursor(CursorMove::Top);
                    textarea.move_cursor(CursorMove::Head);
                },
                _ => {},
            },
            KeyCode::End => match modifiers {
                KeyModifiers::NONE => textarea.move_cursor(CursorMove::End),
                KeyModifiers::SHIFT => {
                    if !textarea.is_selecting() {
                        textarea.start_selection();
                    }
                    textarea.move_cursor(CursorMove::End);
                },
                KeyModifiers::CONTROL => {
                    textarea.move_cursor(CursorMove::Bottom);
                    textarea.move_cursor(CursorMove::End);
                },
                CTRL_SHIFT => {
                    if !textarea.is_selecting() {
                        textarea.start_selection();
                    }
                    textarea.move_cursor(CursorMove::Bottom);
                    textarea.move_cursor(CursorMove::End);
                },
                _ => {},
            },
            KeyCode::Left => match modifiers {
                KeyModifiers::NONE => textarea.move_cursor(CursorMove::Back),
                KeyModifiers::SHIFT => {
                    if !textarea.is_selecting() {
                        textarea.start_selection();
                    }
                    textarea.move_cursor(CursorMove::Back);
                },
                KeyModifiers::CONTROL => textarea.move_cursor(CursorMove::WordBack),
                CTRL_SHIFT => {
                    if !textarea.is_selecting() {
                        textarea.start_selection();
                    }
                    textarea.move_cursor(CursorMove::WordBack);
                },
                _ => {},
            },
            KeyCode::Right => match modifiers {
                KeyModifiers::NONE => textarea.move_cursor(CursorMove::Forward),
                KeyModifiers::SHIFT => {
                    if !textarea.is_selecting() {
                        textarea.start_selection();
                    }
                    textarea.move_cursor(CursorMove::Forward);
                },
                KeyModifiers::CONTROL => textarea.move_cursor(CursorMove::WordForward),
                CTRL_SHIFT => {
                    if !textarea.is_selecting() {
                        textarea.start_selection();
                    }
                    textarea.move_cursor(CursorMove::WordForward);
                },
                _ => {},
            },
            _ => {},
        }
    }

    fn handle_tab(&mut self) {
        let cur = self.textarea.cursor();
        let line = &self.textarea.lines()[cur.0];
        let before = &line[.. cur.1.min(line.len())];
        if before.chars().all(|c| c == ' ') {
            self.textarea.insert_str("    ");
        } else {
            self.textarea.insert_str("\t");
        }
    }

    fn navigate_history(&mut self, up: bool) {
        if self.histories.is_empty() {
            return;
        }

        if up {
            let current = self.history_nav.unwrap_or(self.histories.len());
            if current == 0 {
                return;
            }
            self.history_nav = Some(current - 1);
        } else {
            let Some(n) = self.history_nav else {
                return;
            };
            if n + 1 >= self.histories.len() {
                self.history_nav = None;
                self.textarea.clear();
                return;
            }
            self.history_nav = Some(n + 1);
        }

        let idx = self.history_nav.unwrap();
        let input = &self.histories[idx].input;
        self.textarea.clear();
        self.textarea.insert_str(input);
    }

    fn submit(&mut self) {
        let input = self.textarea.lines().join("\n");
        if input.trim().is_empty() {
            return;
        }

        self.history_nav = None;
        let result = match input.parse::<Val>() {
            Ok(input_val) => {
                let output = Eval.call(&mut self.cfg, Ctx::new_mut(&mut self.ctx), input_val);
                let stdout = String::from_utf8_lossy(&self.stdout_buf.borrow()).into_owned();
                let stderr = String::from_utf8_lossy(&self.stderr_buf.borrow()).into_owned();
                self.stdout_buf.borrow_mut().clear();
                self.stderr_buf.borrow_mut().clear();

                if self.cfg.is_aborted() {
                    let error = self.get_abort_message();
                    self.recover();
                    new_history(input, String::new(), error, stdout, stderr)
                } else {
                    new_history(input, format!("{output:#}"), String::new(), stdout, stderr)
                }
            },
            Err(e) => {
                new_history(input, String::new(), e.to_string(), String::new(), String::new())
            },
        };

        self.histories.push(result);
        self.scroll_state.set_offset(Position::new(0, u16::MAX));

        self.textarea.clear();
    }

    fn get_abort_message(&self) -> String {
        let type_ = self.cfg.import(Key::from_str_unchecked(ABORT_TYPE));
        let msg = self.cfg.import(Key::from_str_unchecked(ABORT_MSG));
        match (type_, msg) {
            (Some(type_), Some(msg)) => format!("aborted by {type_}: {msg}"),
            (None, Some(msg)) => format!("aborted: {msg}"),
            (Some(type_), None) => format!("aborted by {type_}"),
            (None, None) => "aborted".to_string(),
        }
    }

    fn recover(&mut self) {
        self.cfg.remove(&Key::from_str_unchecked(ABORT_TYPE));
        self.cfg.remove(&Key::from_str_unchecked(ABORT_MSG));
        self.cfg.recover();
    }

    fn render(&mut self, f: &mut Frame) {
        let area = f.area();
        let tips_height = 1u16;
        let input_height = (self.textarea.lines().len() as u16)
            .saturating_add(2) // top bottom borders
            .min((area.height / 2).saturating_sub(tips_height));
        let title_height = 1u16;
        let history_height = area
            .height
            .saturating_sub(title_height)
            .saturating_sub(input_height)
            .saturating_sub(tips_height);

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(title_height),
                Constraint::Length(history_height),
                Constraint::Length(input_height),
                Constraint::Length(tips_height),
            ])
            .split(area);

        self.render_title(f, chunks[0]);
        if history_height > 0 {
            self.render_histories(f, chunks[1]);
        }
        f.render_widget(&self.textarea, chunks[2]);
        self.render_tips(f, chunks[3]);
    }

    fn render_title(&self, f: &mut Frame, area: Rect) {
        let title = TuiLine::styled(
            &self.version,
            Style::default().fg(Color::White).add_modifier(Modifier::BOLD),
        );
        f.render_widget(title, area);
    }

    fn render_tips(&self, f: &mut Frame, area: Rect) {
        let tips = TuiLine::styled(TIPS_TEXT, Style::default().fg(Color::DarkGray));
        f.render_widget(tips, area);
    }

    fn render_histories(&mut self, f: &mut Frame, area: Rect) {
        if self.histories.is_empty() {
            return;
        }

        let mut total_height = 0u16;
        for item in &self.histories {
            for p in &item.paragraphs {
                total_height += p.line_count(area.width) as u16;
            }
            total_height += 1; // spacer
        }

        let mut scroll_view = ScrollView::new(Size::new(area.width, total_height))
            .horizontal_scrollbar_visibility(ScrollbarVisibility::Never)
            .vertical_scrollbar_visibility(ScrollbarVisibility::Never);

        let mut y = 0u16;
        for item in &self.histories {
            for p in &item.paragraphs {
                let h = p.line_count(area.width) as u16;
                scroll_view.render_widget(p, Rect::new(0, y, area.width, h));
                y += h;
            }
            y += 1; // spacer
        }

        f.render_stateful_widget(scroll_view, area, &mut self.scroll_state);
    }
}

fn new_buffer(cfg: &mut Cfg, key: &str) -> Rc<RefCell<Vec<u8>>> {
    let stdout_buf = Rc::new(RefCell::new(Vec::new()));
    let stdout_writer = Rc::clone(&stdout_buf);
    let stdout: Rc<RefCell<dyn std::io::Write>> = stdout_writer;
    let stdout = Val::Dyn(Box::new(Output::new(stdout)));
    cfg.insert(Key::from_str_unchecked(key), stdout);
    stdout_buf
}

fn new_textarea() -> TextArea<'static> {
    let mut textarea = TextArea::default();
    textarea.set_block(
        Block::default().borders(Borders::ALL).border_style(Style::default().fg(Color::White)),
    );
    textarea.set_cursor_line_style(Style::default());
    textarea
}

fn new_history(
    input: String, output: String, error: String, stdout: String, stderr: String,
) -> History {
    let mut paragraphs: Vec<Paragraph<'static>> = Vec::new();
    paragraphs.push(history_paragraph(input.clone(), Color::White));
    if !error.is_empty() {
        paragraphs.push(history_paragraph(error, Color::Red));
    } else if !output.is_empty() {
        paragraphs.push(history_paragraph(output, Color::Green));
    }
    if !stdout.is_empty() {
        paragraphs.push(history_paragraph(stdout, Color::Blue));
    }
    if !stderr.is_empty() {
        paragraphs.push(history_paragraph(stderr, Color::Red));
    }
    History { input, paragraphs }
}

fn history_paragraph(text: String, border_color: Color) -> Paragraph<'static> {
    let block = Block::default()
        .borders(Borders::LEFT)
        .border_type(BorderType::Thick)
        .border_style(Style::default().fg(border_color))
        .padding(Padding::left(1));
    Paragraph::new(text).block(block).wrap(Wrap { trim: false })
}
