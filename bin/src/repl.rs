use std::borrow::Cow;
use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::io::Cursor;
use std::io::Error;
use std::io::Result;
use std::io::Write;
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
use serde_json::json;
use tiny_http::Header;
use tiny_http::Method;
use tiny_http::Request;
use tiny_http::Response;
use tiny_http::Server;
use url::Url;

use crate::cfg::comp::BinCompCfg;

pub struct WebRepl {
    sessions: SessionStore,
}

struct SessionStore {
    sessions: VecDeque<(String, Session)>,
}

struct Session {
    cfg: Cfg,
    ctx: Val,
    stdout_buf: Rc<RefCell<Vec<u8>>>,
    stderr_buf: Rc<RefCell<Vec<u8>>>,
}

static REPL_HTML: &str = include_str!("repl/repl.html");

const MAX_SESSIONS: usize = 32;

impl WebRepl {
    pub fn new() -> Self {
        Self { sessions: SessionStore::new() }
    }

    pub fn run(mut self) -> Result<()> {
        let server = Server::http("127.0.0.1:0").map_err(Error::other)?;
        let url = format!("http://{}", server.server_addr());
        println!("Air REPL running at {url}");
        if let Err(e) = webbrowser::open(&url) {
            eprintln!("Failed to open browser: {e}");
        }
        for request in server.incoming_requests() {
            let _ = self.handle_request(request);
        }
        Ok(())
    }

    fn handle_request(&mut self, mut request: Request) -> Result<()> {
        let base = Url::parse("http://127.0.0.1").unwrap();
        let Ok(url) = base.join(request.url()) else {
            return request.respond(not_found());
        };
        let queries: HashMap<Cow<str>, Cow<str>> = url.query_pairs().collect();
        let Ok(body) = read_request_body(&mut request) else {
            return request.respond(bad_request("expected utf-8 body"));
        };
        match url.path() {
            "/" | "/index.html" => {
                if *request.method() == Method::Get {
                    request.respond(index())
                } else {
                    request.respond(bad_request("expected Get"))
                }
            },
            "/eval" => {
                if *request.method() == Method::Post {
                    request.respond(self.handle_eval(&body, &queries))
                } else {
                    request.respond(bad_request("expected Post"))
                }
            },
            _ => request.respond(not_found()),
        }
    }

    fn handle_eval(
        &mut self, body: &str, queries: &HashMap<Cow<str>, Cow<str>>,
    ) -> Response<Cursor<Vec<u8>>> {
        let Some(session_id) = queries.get("session") else {
            return bad_request("missing session");
        };
        if session_id.is_empty() {
            return bad_request("missing session");
        }
        let session = self.sessions.get_session_mut(session_id);
        let result = match body.parse::<Val>() {
            Ok(source) => {
                let (output, stdout, stderr) = eval(session, source);
                if session.cfg.is_aborted() {
                    let error = get_abort_message(&session.cfg);
                    recover(&mut session.cfg);
                    json!({"ok": false, "output": "", "error": error, "stdout": stdout, "stderr": stderr})
                } else {
                    json!({"ok": true, "output": format!("{output:#}"), "error": "", "stdout": stdout, "stderr": stderr})
                }
            },
            Err(e) => {
                json!({"ok": false, "output": "", "error": e.to_string(), "stdout": "", "stderr": ""})
            },
        };
        Response::from_string(result.to_string()).with_header(json_header())
    }
}

fn index() -> Response<Cursor<Vec<u8>>> {
    Response::from_string(REPL_HTML).with_header(html_header())
}

fn eval(session: &mut Session, source: Val) -> (Val, String, String) {
    let output = Eval.call(&mut session.cfg, Ctx::new_mut(&mut session.ctx), source);
    let stdout = read_buffer(Rc::clone(&session.stdout_buf));
    let stderr = read_buffer(Rc::clone(&session.stderr_buf));
    (output, stdout, stderr)
}

fn get_abort_message(cfg: &Cfg) -> String {
    let type_ = cfg.import(Key::from_str_unchecked(ABORT_TYPE));
    let msg = cfg.import(Key::from_str_unchecked(ABORT_MSG));
    match (type_, msg) {
        (Some(type_), Some(msg)) => format!("aborted by {type_}: {msg}"),
        (None, Some(msg)) => format!("aborted: {msg}"),
        (Some(type_), None) => format!("aborted by {type_}"),
        (None, None) => "aborted".to_owned(),
    }
}

fn recover(cfg: &mut Cfg) {
    cfg.remove(&Key::from_str_unchecked(ABORT_TYPE));
    cfg.remove(&Key::from_str_unchecked(ABORT_MSG));
    cfg.recover();
}

fn html_header() -> Header {
    Header::from_bytes(b"Content-Type", b"text/html; charset=utf-8").expect("Header is valid")
}

fn json_header() -> Header {
    Header::from_bytes(b"Content-Type", b"application/json; charset=utf-8")
        .expect("Header is valid")
}

fn read_request_body(request: &mut Request) -> Result<String> {
    let mut str = String::new();
    request.as_reader().read_to_string(&mut str)?;
    Ok(str)
}

fn bad_request(reason: &str) -> Response<Cursor<Vec<u8>>> {
    Response::from_string(reason).with_status_code(400)
}

fn not_found() -> Response<Cursor<Vec<u8>>> {
    Response::from_string("Not Found").with_status_code(404)
}

impl SessionStore {
    fn new() -> Self {
        Self { sessions: VecDeque::new() }
    }

    fn get_session_mut(&mut self, id: &str) -> &mut Session {
        for (i, (session_id, _)) in self.sessions.iter().enumerate() {
            if session_id == id {
                return &mut self.sessions.get_mut(i).unwrap().1;
            }
        }
        if self.sessions.len() >= MAX_SESSIONS {
            self.sessions.pop_front();
        }
        let (_, session) = self.sessions.push_back_mut((id.to_owned(), Session::new()));
        session
    }
}

impl Session {
    fn new() -> Self {
        let mut cfg = BinCompCfg::generate();
        let ctx = prelude(&mut cfg);
        let stdout_buf = new_buffer(&mut cfg, STANDARD_OUTPUT);
        let stderr_buf = new_buffer(&mut cfg, STANDARD_ERROR);
        Self { cfg, ctx, stdout_buf, stderr_buf }
    }
}

fn read_buffer(buffer: Rc<RefCell<Vec<u8>>>) -> String {
    let s = String::from_utf8_lossy(&buffer.borrow()).into_owned();
    buffer.borrow_mut().clear();
    s
}

fn new_buffer(cfg: &mut Cfg, key: &str) -> Rc<RefCell<Vec<u8>>> {
    let buf = Rc::new(RefCell::new(Vec::new()));
    let writer = Rc::clone(&buf);
    let writer: Rc<RefCell<dyn Write>> = writer;
    let output = Val::Dyn(Box::new(Output::new(writer)));
    cfg.insert(Key::from_str_unchecked(key), output);
    buf
}
