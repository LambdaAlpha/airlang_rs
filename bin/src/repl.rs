use std::{
    fmt::Display,
    io::{
        stdin,
        Read,
        Result,
        Write,
    },
    mem::take,
    os::fd::AsRawFd,
};

use airlang::{
    generate,
    initial_ctx,
    interpret_mutable,
    parse,
    Ctx,
    MutableCtx,
    Val,
};
use crossterm::{
    cursor::{
        position,
        MoveToColumn,
        MoveToNextLine,
        MoveToPreviousLine,
        RestorePosition,
        SavePosition,
    },
    event::{
        read,
        DisableBracketedPaste,
        EnableBracketedPaste,
        Event,
        KeyCode,
        KeyEvent,
        KeyModifiers,
    },
    style::{
        Color,
        ContentStyle,
        Print,
        PrintStyledContent,
        StyledContent,
        Stylize,
    },
    terminal::{
        disable_raw_mode,
        enable_raw_mode,
        is_raw_mode_enabled,
        size,
        Clear,
        ClearType,
        ScrollDown,
        ScrollUp,
        SetTitle,
    },
    tty::IsTty,
    Command,
    ExecutableCommand,
    QueueableCommand,
};

use crate::init_ctx;

pub(crate) struct Repl<W: Write + AsRawFd> {
    ctx: Ctx,
    terminal: Terminal<W>,
    is_raw_mode_enabled: bool,

    multiline_mode: bool,
    previous_lines: Vec<String>,
    // last line, before cursor
    head_buffer: String,
    // last line, after cursor, backward
    tail_buffer: Vec<char>,
    histories: Vec<History>,
    // backward, index of last history is 1
    history_index: usize,
}

struct History {
    previous_lines: Vec<String>,
    last_line: String,
}

struct Terminal<W: Write + AsRawFd>(W);

enum CtrlFlow {
    None,
    Continue,
    Break,
}

impl<W: Write + AsRawFd> Repl<W> {
    pub(crate) fn new(out: W) -> Self {
        let mut ctx = initial_ctx();
        init_ctx(MutableCtx::new(&mut ctx));
        let terminal = Terminal(out);
        Self {
            ctx,
            terminal,
            is_raw_mode_enabled: false,
            multiline_mode: false,
            previous_lines: Vec::new(),
            head_buffer: String::new(),
            tail_buffer: Vec::new(),
            histories: Vec::new(),
            history_index: 0,
        }
    }

    pub(crate) fn run(&mut self) -> Result<()> {
        // is_terminal is stable since 1.70.0
        if !stdin().is_tty() || !self.terminal.is_tty() {
            return self.run_once();
        }

        self.is_raw_mode_enabled = is_raw_mode_enabled()?;

        self.setup()?;

        self.set_title()?;
        self.terminal.flush()?;

        loop {
            self.terminal.newline_default_prompt()?;
            self.terminal.flush()?;

            let should_break = self.handle_event()?;
            if should_break {
                break;
            }
        }
        Ok(())
    }

    fn setup(&mut self) -> Result<()> {
        enable_raw_mode()?;
        self.terminal.execute(EnableBracketedPaste)
    }

    fn cleanup(&mut self) {
        let _ = self.terminal.execute(DisableBracketedPaste);
        if self.is_raw_mode_enabled {
            let _ = enable_raw_mode();
        } else {
            let _ = disable_raw_mode();
        }
    }

    fn run_once(&mut self) -> Result<()> {
        let mut input = String::new();
        stdin().read_to_string(&mut input)?;
        if input.is_empty() {
            return Ok(());
        }
        self.eval(&input)?;
        self.terminal.new_line()?;
        self.terminal.flush()
    }

    fn handle_event(&mut self) -> Result<bool /* break */> {
        loop {
            let event = read()?;
            match event {
                Event::Key(key) => match self.handle_key(key)? {
                    CtrlFlow::None => {}
                    CtrlFlow::Continue => break Ok(false),
                    CtrlFlow::Break => break Ok(true),
                },
                Event::Mouse(_) => {}
                Event::Paste(text) => {
                    self.handle_paste(text)?;
                }
                Event::Resize(_, _) => {}
                Event::FocusGained => {}
                Event::FocusLost => {}
            }
        }
    }

    fn handle_paste(&mut self, text: String) -> Result<()> {
        for c in text.chars() {
            if c == '\n' {
                let new_line = take(&mut self.head_buffer);
                self.previous_lines.push(new_line);
            } else {
                self.head_buffer.push(c);
            }
        }

        self.terminal.queue(Clear(ClearType::FromCursorDown))?;
        self.terminal.print_texts(text.chars())?;
        self.print_tail_restore_position()?;
        self.update_default_prompt()?;
        self.terminal.flush()
    }

    fn handle_key(&mut self, key: KeyEvent) -> Result<CtrlFlow> {
        match key.code {
            KeyCode::Enter => {
                return self.handle_enter();
            }
            KeyCode::Char(c) => {
                return self.handle_char(c, key.modifiers);
            }
            KeyCode::Backspace => {
                self.handle_backspace()?;
            }
            KeyCode::Left => {
                self.handle_left()?;
            }
            KeyCode::Right => {
                self.handle_right()?;
            }
            KeyCode::Up => {
                self.handle_up_down(true)?;
            }
            KeyCode::Down => {
                self.handle_up_down(false)?;
            }
            KeyCode::Home => {
                self.handle_home()?;
            }
            KeyCode::End => {
                self.handle_end()?;
            }
            KeyCode::PageUp => {}
            KeyCode::PageDown => {}
            KeyCode::Tab => {}
            KeyCode::BackTab => {}
            KeyCode::Delete => {
                self.handle_delete()?;
            }
            KeyCode::Insert => {}
            KeyCode::F(_) => {}
            KeyCode::Null => {}
            KeyCode::Esc => {}
            KeyCode::CapsLock => {}
            KeyCode::ScrollLock => {}
            KeyCode::NumLock => {}
            KeyCode::PrintScreen => {}
            KeyCode::Pause => {}
            KeyCode::Menu => {}
            KeyCode::KeypadBegin => {}
            KeyCode::Media(_) => {}
            KeyCode::Modifier(_) => {}
        }
        Ok(CtrlFlow::None)
    }

    fn handle_enter(&mut self) -> Result<CtrlFlow> {
        if self.multiline_mode {
            let new_line = take(&mut self.head_buffer);
            self.previous_lines.push(new_line);

            self.terminal.queue(Clear(ClearType::UntilNewLine))?;
            self.terminal.newline_multiline_prompt()?;
            self.print_head()?;
            self.print_tail_restore_position()?;
            self.terminal.flush()?;
            Ok(CtrlFlow::None)
        } else {
            self.commit()?;
            self.terminal.flush()?;
            Ok(CtrlFlow::Continue)
        }
    }

    fn handle_char(&mut self, c: char, modifiers: KeyModifiers) -> Result<CtrlFlow> {
        if modifiers.is_empty() || modifiers == KeyModifiers::SHIFT {
            self.head_buffer.push(c);

            if c != '\r' {
                self.terminal.print(c)?;
            }
            self.print_tail_restore_position()?;
            self.terminal.flush()?;
            return Ok(CtrlFlow::None);
        }
        if modifiers == KeyModifiers::ALT && c == 'm' {
            self.handle_multiline_switch()?;
        } else if modifiers == KeyModifiers::CONTROL && c == 'c' {
            return Ok(CtrlFlow::Break);
        }
        Ok(CtrlFlow::None)
    }

    fn handle_multiline_switch(&mut self) -> Result<()> {
        self.multiline_mode = !self.multiline_mode;
        self.update_prompt()?;
        self.terminal.flush()
    }

    fn commit(&mut self) -> Result<()> {
        let input = self.get_input_buffer();
        let previous_lines = take(&mut self.previous_lines);
        let mut last_line = take(&mut self.head_buffer);
        for c in take(&mut self.tail_buffer) {
            last_line.push(c);
        }

        if !(previous_lines.is_empty() && last_line.is_empty()) {
            self.histories.push(History {
                previous_lines,
                last_line,
            });
        }
        self.history_index = 0;

        self.terminal.new_line()?;
        self.terminal.flush()?;

        if input.is_empty() {
            return Ok(());
        }

        self.cleanup();
        self.eval(&input)?;
        self.terminal.flush()?;
        self.setup()
    }

    fn handle_backspace(&mut self) -> Result<()> {
        if self.head_buffer.pop().is_some() {
            self.move_home_print_head()?;
            self.print_tail_clear_restore_position()?;
            self.terminal.flush()?;
            return Ok(());
        }
        let Some(previous_line) = self.previous_lines.pop() else {
            return Ok(());
        };
        self.head_buffer = previous_line;

        self.terminal.move_up(1)?;
        self.terminal.print_prompt(self.get_prompt())?;
        self.print_head()?;
        self.print_tail_clear_restore_position()?;
        self.terminal.flush()
    }

    fn handle_delete(&mut self) -> Result<()> {
        if self.tail_buffer.pop().is_none() {
            return Ok(());
        }
        self.print_tail_clear_restore_position()?;
        self.terminal.flush()
    }

    fn handle_up_down(&mut self, up: bool) -> Result<()> {
        if self.multiline_mode {
            return Ok(());
        }
        let len = self.histories.len();
        if up {
            if self.history_index >= len {
                return Ok(());
            }
            self.history_index += 1;
        } else {
            if self.history_index == 0 {
                return Ok(());
            }
            self.history_index -= 1;
        }

        let line_count = self.previous_lines.len();
        let move_up_line = u16::try_from(line_count).unwrap_or(u16::MAX);
        if self.history_index == 0 {
            self.previous_lines.clear();
            self.head_buffer.clear();
            self.tail_buffer.clear();
        } else {
            let history = &self.histories[len - self.history_index];
            self.previous_lines.clone_from(&history.previous_lines);
            self.head_buffer.clone_from(&history.last_line);
            self.tail_buffer.clear();
        }

        self.terminal.move_up(move_up_line)?;
        self.terminal.queue(Clear(ClearType::FromCursorDown))?;
        self.terminal.print_prompt(MULTILINE_PROMPT)?;
        for line in &self.previous_lines {
            self.terminal.print_texts(line.chars())?;
            self.terminal.newline_multiline_prompt()?;
        }
        self.print_head()?;
        self.print_tail_restore_position()?;
        self.update_default_prompt()?;
        self.terminal.flush()
    }

    fn handle_left(&mut self) -> Result<()> {
        let Some(last) = self.head_buffer.pop() else {
            return Ok(());
        };
        self.tail_buffer.push(last);
        self.move_home_print_head()?;
        self.terminal.flush()
    }

    fn handle_right(&mut self) -> Result<()> {
        let Some(last) = self.tail_buffer.pop() else {
            return Ok(());
        };
        self.head_buffer.push(last);
        self.move_home_print_head()?;
        self.terminal.flush()
    }

    fn handle_home(&mut self) -> Result<()> {
        self.tail_buffer.extend(self.head_buffer.chars().rev());
        self.head_buffer.clear();
        self.move_home()?;
        self.terminal.flush()
    }

    fn handle_end(&mut self) -> Result<()> {
        self.head_buffer.extend(self.tail_buffer.iter().rev());
        self.tail_buffer.clear();
        self.move_home_print_head()?;
        self.print_tail_restore_position()?;
        self.terminal.flush()
    }

    fn get_input_buffer(&self) -> String {
        let mut input = String::new();
        for s in &self.previous_lines {
            input.push_str(s);
            input.push('\n');
        }
        input.push_str(&self.head_buffer);
        for c in self.tail_buffer.iter().rev() {
            input.push(*c);
        }
        input
    }

    fn eval(&mut self, input: &str) -> Result<()> {
        match parse(input) {
            Ok(input) => {
                let output = interpret_mutable(MutableCtx::new(&mut self.ctx), input);
                match generate(&output) {
                    Ok(o) => self.terminal.print(o),
                    Err(e) => self.terminal.eprint(e.to_string()),
                }
            }
            Err(e) => self.terminal.eprint(e.to_string()),
        }
    }

    const TITLE: &'static str = "🜁 Air";

    fn set_title(&mut self) -> Result<()> {
        self.terminal.queue(SetTitle(Self::TITLE))?;
        self.terminal.print(Self::TITLE)?;
        self.terminal.print(" ")?;
        match parse(include_str!("air/version.air")) {
            Ok(repr) => match interpret_mutable(MutableCtx::new(&mut self.ctx), repr) {
                Val::String(s) => self.terminal.print(s),
                _ => self.terminal.eprint("unknown version"),
            },
            Err(err) => self.terminal.eprint(err.to_string()),
        }
    }

    fn move_home(&mut self) -> Result<()> {
        self.terminal.queue(MoveToColumn(0))?;
        self.terminal.print_prompt(self.get_prompt())?;
        Ok(())
    }

    fn move_home_print_head(&mut self) -> Result<()> {
        self.move_home()?;
        self.print_head()
    }

    fn print_head(&mut self) -> Result<()> {
        self.terminal.print(&self.head_buffer)
    }

    fn print_tail_clear_restore_position(&mut self) -> Result<()> {
        self.terminal.queue(SavePosition)?;
        self.print_tail()?;
        self.terminal.queue(Clear(ClearType::FromCursorDown))?;
        self.terminal.queue(RestorePosition)?;
        Ok(())
    }

    fn print_tail_restore_position(&mut self) -> Result<()> {
        self.terminal.queue(SavePosition)?;
        self.print_tail()?;
        self.terminal.queue(RestorePosition)?;
        Ok(())
    }

    fn print_tail(&mut self) -> Result<()> {
        for c in self.tail_buffer.iter().rev() {
            self.terminal.queue(Print(*c))?;
        }
        Ok(())
    }

    fn update_default_prompt(&mut self) -> Result<()> {
        if !self.multiline_mode {
            self.terminal.update_prompt(DEFAULT_PROMPT)?;
        }
        Ok(())
    }

    fn update_prompt(&mut self) -> Result<()> {
        self.terminal.update_prompt(self.get_prompt())
    }

    fn get_prompt(&self) -> &'static str {
        if self.multiline_mode {
            MULTILINE_PROMPT
        } else {
            DEFAULT_PROMPT
        }
    }
}

impl<W: Write + AsRawFd> Drop for Repl<W> {
    fn drop(&mut self) {
        self.cleanup();
    }
}

const DEFAULT_PROMPT: &str = "❯ ";
const MULTILINE_PROMPT: &str = "┃ ";

impl<W: Write + AsRawFd> Terminal<W> {
    fn update_prompt(&mut self, prompt: &str) -> Result<()> {
        self.0.queue(SavePosition)?;
        self.0.queue(MoveToColumn(0))?;
        self.print_prompt(prompt)?;
        self.0.queue(RestorePosition)?;
        Ok(())
    }

    fn print_texts<S: Iterator<Item = char>>(&mut self, text: S) -> Result<()> {
        for c in text {
            match c {
                '\n' => {
                    self.newline_multiline_prompt()?;
                }
                '\r' => {}
                c => {
                    self.0.queue(Print(c))?;
                }
            }
        }
        Ok(())
    }

    fn newline_default_prompt(&mut self) -> Result<()> {
        self.newline_prompt(DEFAULT_PROMPT)
    }

    fn newline_multiline_prompt(&mut self) -> Result<()> {
        self.newline_prompt(MULTILINE_PROMPT)
    }

    fn newline_prompt(&mut self, prompt: &str) -> Result<()> {
        self.new_line()?;
        self.print_prompt(prompt)
    }

    fn print_prompt(&mut self, prompt: &str) -> Result<()> {
        self.colored_print(prompt, Color::Green)
    }

    fn move_up(&mut self, lines: u16) -> Result<()> {
        let (_, row) = position()?;
        if row < lines {
            self.0.queue(ScrollDown(lines - row))?;
        }
        if lines > 0 {
            self.0.queue(MoveToPreviousLine(lines))?;
        } else {
            self.0.queue(MoveToColumn(0))?;
        }
        Ok(())
    }

    fn new_line(&mut self) -> Result<()> {
        let (_, row) = position()?;
        let (_, y) = size()?;
        let delta = y - row - 1;
        const LINES: u16 = 1;
        if LINES > delta {
            self.0.queue(ScrollUp(LINES - delta))?;
        }
        if LINES > 0 {
            self.0.queue(MoveToNextLine(LINES))?;
        } else {
            self.0.queue(MoveToColumn(0))?;
        }
        Ok(())
    }

    fn print(&mut self, d: impl Display) -> Result<()> {
        self.0.queue(Print(d))?;
        Ok(())
    }

    fn eprint(&mut self, d: impl Display) -> Result<()> {
        self.colored_print(d, Color::Red)
    }

    fn colored_print(&mut self, d: impl Display, color: Color) -> Result<()> {
        let colored = StyledContent::new(ContentStyle::new().with(color), d);
        self.0.queue(PrintStyledContent(colored))?;
        Ok(())
    }

    fn queue(&mut self, command: impl Command) -> Result<()> {
        self.0.queue(command)?;
        Ok(())
    }

    fn execute(&mut self, command: impl Command) -> Result<()> {
        self.0.execute(command)?;
        Ok(())
    }

    fn flush(&mut self) -> Result<()> {
        self.0.flush()
    }

    fn is_tty(&self) -> bool {
        self.0.is_tty()
    }
}
