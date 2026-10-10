#!/usr/bin/env rust-script
//! Read-only, dependency-free structural OpenCode inventory. Never executes tools.
use std::{
    collections::BTreeMap,
    env, fs,
    path::{Path, PathBuf},
};

const MAX_BYTES: u64 = 1_048_576;
const MAX_DEPTH: usize = 64;
const MAX_ENTRIES: usize = 512;
const MAX_DIRS: usize = 128;
const MAX_SCAN_ENTRIES: usize = 8192;
const MAX_SCAN_DIRS: usize = 1024;

#[derive(Debug, PartialEq)]
enum Json {
    Null,
    Bool(bool),
    Number,
    String(String),
    Array(Vec<Json>),
    Object(BTreeMap<String, Json>),
}
impl Json {
    fn get(&self, key: &str) -> Option<&Json> {
        if let Self::Object(m) = self {
            m.get(key)
        } else {
            None
        }
    }
    fn text(&self) -> Option<&str> {
        if let Self::String(s) = self {
            Some(s)
        } else {
            None
        }
    }
    fn count(&self) -> usize {
        match self {
            Self::Array(v) => v.len(),
            Self::Object(v) => v.len(),
            _ => 0,
        }
    }
}
struct Parser<'a> {
    b: &'a [u8],
    i: usize,
}
impl<'a> Parser<'a> {
    fn parse(s: &'a str) -> Result<Json, ()> {
        if s.len() as u64 > MAX_BYTES {
            return Err(());
        }
        let mut p = Self {
            b: s.as_bytes(),
            i: 0,
        };
        let v = p.value(0)?;
        p.space()?;
        if p.i == p.b.len() {
            Ok(v)
        } else {
            Err(())
        }
    }
    fn space(&mut self) -> Result<(), ()> {
        loop {
            while self
                .b
                .get(self.i)
                .is_some_and(|b| matches!(b, b' ' | b'\n' | b'\r' | b'\t'))
            {
                self.i += 1;
            }
            if self.b.get(self.i..self.i + 2) == Some(b"//") {
                self.i += 2;
                while self.b.get(self.i).is_some_and(|b| *b != b'\n') {
                    self.i += 1;
                }
            } else if self.b.get(self.i..self.i + 2) == Some(b"/*") {
                self.i += 2;
                while self.b.get(self.i..self.i + 2) != Some(b"*/") {
                    if self.i >= self.b.len() {
                        return Err(());
                    }
                    self.i += 1;
                }
                self.i += 2;
            } else {
                return Ok(());
            }
        }
    }
    fn eat(&mut self, c: u8) -> bool {
        if self.b.get(self.i) == Some(&c) {
            self.i += 1;
            true
        } else {
            false
        }
    }
    fn hex(&mut self) -> Result<u32, ()> {
        let mut n = 0;
        for _ in 0..4 {
            let b = *self.b.get(self.i).ok_or(())?;
            self.i += 1;
            n = n * 16 + (b as char).to_digit(16).ok_or(())?;
        }
        Ok(n)
    }
    fn string(&mut self) -> Result<String, ()> {
        if !self.eat(b'"') {
            return Err(());
        }
        let mut out = String::new();
        loop {
            let b = *self.b.get(self.i).ok_or(())?;
            self.i += 1;
            match b {
                b'"' => return Ok(out),
                0..=31 => return Err(()),
                b'\\' => {
                    let e = *self.b.get(self.i).ok_or(())?;
                    self.i += 1;
                    out.push(match e {
                        b'"' => '"',
                        b'\\' => '\\',
                        b'/' => '/',
                        b'b' => '\u{8}',
                        b'f' => '\u{c}',
                        b'n' => '\n',
                        b'r' => '\r',
                        b't' => '\t',
                        b'u' => {
                            let mut n = self.hex()?;
                            if (0xd800..=0xdbff).contains(&n) {
                                if !self.eat(b'\\') || !self.eat(b'u') {
                                    return Err(());
                                }
                                let low = self.hex()?;
                                if !(0xdc00..=0xdfff).contains(&low) {
                                    return Err(());
                                }
                                n = 0x10000 + ((n - 0xd800) << 10) + low - 0xdc00;
                            }
                            char::from_u32(n).ok_or(())?
                        }
                        _ => return Err(()),
                    });
                }
                32..=127 => out.push(b as char),
                _ => {
                    self.i -= 1;
                    let width = match b {
                        0xc2..=0xdf => 2,
                        0xe0..=0xef => 3,
                        0xf0..=0xf4 => 4,
                        _ => return Err(()),
                    };
                    let s = std::str::from_utf8(self.b.get(self.i..self.i + width).ok_or(())?)
                        .map_err(|_| ())?;
                    let c = s.chars().next().ok_or(())?;
                    self.i += c.len_utf8();
                    out.push(c);
                }
            }
        }
    }
    fn value(&mut self, depth: usize) -> Result<Json, ()> {
        if depth > MAX_DEPTH {
            return Err(());
        }
        self.space()?;
        match self.b.get(self.i).copied().ok_or(())? {
            b'"' => self.string().map(Json::String),
            b'{' | b'[' => {
                let object = self.eat(b'{');
                if !object {
                    self.i += 1;
                }
                let end = if object { b'}' } else { b']' };
                let mut map = BTreeMap::new();
                let mut arr = Vec::new();
                self.space()?;
                if self.eat(end) {
                    return Ok(if object {
                        Json::Object(map)
                    } else {
                        Json::Array(arr)
                    });
                }
                loop {
                    self.space()?;
                    if object {
                        let k = self.string()?;
                        self.space()?;
                        if !self.eat(b':') {
                            return Err(());
                        }
                        let v = self.value(depth + 1)?;
                        if map.insert(k, v).is_some() {
                            return Err(());
                        }
                    } else {
                        arr.push(self.value(depth + 1)?);
                    }
                    self.space()?;
                    if self.eat(end) {
                        break;
                    }
                    if !self.eat(b',') {
                        return Err(());
                    }
                    self.space()?;
                    if self.eat(end) {
                        break;
                    }
                }
                Ok(if object {
                    Json::Object(map)
                } else {
                    Json::Array(arr)
                })
            }
            b't' | b'f' | b'n' => {
                let (word, val): (&[u8], Json) = match self.b[self.i] {
                    b't' => (b"true", Json::Bool(true)),
                    b'f' => (b"false", Json::Bool(false)),
                    _ => (b"null", Json::Null),
                };
                if self.b.get(self.i..self.i + word.len()) != Some(word) {
                    return Err(());
                }
                self.i += word.len();
                Ok(val)
            }
            b'-' | b'0'..=b'9' => {
                self.eat(b'-');
                if !self.eat(b'0') {
                    let start = self.i;
                    while self.b.get(self.i).is_some_and(u8::is_ascii_digit) {
                        self.i += 1;
                    }
                    if start == self.i {
                        return Err(());
                    }
                }
                if self.eat(b'.') {
                    let start = self.i;
                    while self.b.get(self.i).is_some_and(u8::is_ascii_digit) {
                        self.i += 1;
                    }
                    if start == self.i {
                        return Err(());
                    }
                }
                if self.eat(b'e') || self.eat(b'E') {
                    if !self.eat(b'+') {
                        self.eat(b'-');
                    }
                    let start = self.i;
                    while self.b.get(self.i).is_some_and(u8::is_ascii_digit) {
                        self.i += 1;
                    }
                    if start == self.i {
                        return Err(());
                    }
                }
                Ok(Json::Number)
            }
            _ => Err(()),
        }
    }
}

fn safe_id(s: &str) -> bool {
    let lower = s.to_ascii_lowercase();
    !s.is_empty()
        && s.len() <= 96
        && s.as_bytes()[0].is_ascii_alphanumeric()
        && s.bytes()
            .all(|b| b.is_ascii_alphanumeric() || b"-_.:/".contains(&b))
        && !s.contains("://")
        && !s.contains(':')
        && ![
            "sk-",
            "sk_",
            "ghp_",
            "github_pat_",
            "gho_",
            "xox",
            "akia",
            "asia",
            "bearer",
            "token",
            "secret",
            "password",
            "eyj",
        ]
        .iter()
        .any(|p| lower.split('/').any(|part| part.starts_with(p)))
}
fn escape(s: &str) -> String {
    s.chars()
        .take(160)
        .map(|c| {
            if c.is_control() || !c.is_ascii() {
                format!("&#{};", c as u32)
            } else if c.is_ascii_alphanumeric() || " ._-/".contains(c) {
                c.to_string()
            } else {
                format!("&#{};", c as u32)
            }
        })
        .collect()
}
// Check every existing component, including ancestor directories, without following links.
fn no_links(path: &Path) -> Result<(), &'static str> {
    let mut part = PathBuf::new();
    for c in path.components() {
        if matches!(c, std::path::Component::ParentDir) {
            return Err("parent components skipped");
        }
        part.push(c);
        match fs::symlink_metadata(&part) {
            Ok(m) if m.file_type().is_symlink() => return Err("symlink skipped"),
            Ok(_) => (),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Err("missing"),
            Err(_) => return Err("unreadable"),
        }
    }
    Ok(())
}
fn read_config(path: &Path) -> Result<Json, &'static str> {
    use std::io::Read;
    no_links(path)?;
    let m = fs::symlink_metadata(path).map_err(|_| "unreadable")?;
    if !m.is_file() {
        return Err("not a regular file");
    }
    if m.len() > MAX_BYTES {
        return Err("oversize");
    }
    let f = fs::File::open(path).map_err(|_| "unreadable")?;
    let mut data = Vec::new();
    f.take(MAX_BYTES + 1)
        .read_to_end(&mut data)
        .map_err(|_| "unreadable")?;
    if data.len() as u64 > MAX_BYTES {
        return Err("oversize");
    }
    let text = std::str::from_utf8(&data).map_err(|_| "invalid UTF-8")?;
    let v = Parser::parse(text).map_err(|_| "invalid JSON/JSONC or depth limit")?;
    if !matches!(v, Json::Object(_)) {
        return Err("config root is not an object");
    }
    Ok(v)
}
fn identifier(v: Option<&Json>) -> Option<&str> {
    v.and_then(Json::text).filter(|s| safe_id(s))
}
fn model(v: &Json) -> Option<String> {
    if let Some(s) = v.text() {
        let (base, variant) = s.split_once('#').map_or((s, None), |(b, v)| (b, Some(v)));
        if safe_id(base) && variant.is_none_or(safe_id) {
            return Some(s.into());
        }
        return None;
    }
    let p = identifier(v.get("providerID"))?;
    let m = identifier(v.get("model"))?;
    let mut result = format!("{p}/{m}");
    if let Some(var) = identifier(v.get("variant")) {
        result.push_str(&format!(" (variant {var})"));
    }
    Some(result)
}
fn summary(v: &Json, out: &mut String) {
    for key in ["default_agent", "update", "share"] {
        if let Some(value) = v.get(key) {
            match identifier(Some(value)) {
                Some(id) => out.push_str(&format!("  - {key}: {}.\n", escape(id))),
                None => out.push_str(&format!("  - {key}: configured; value omitted.\n")),
            }
        }
    }
    for key in ["snapshots", "formatter"] {
        if let Some(value) = v.get(key) {
            match value {
                Json::Bool(b) => out.push_str(&format!("  - {key}: {b}.\n")),
                _ => out.push_str(&format!("  - {key}: configured; details omitted.\n")),
            }
        }
    }
    match v.get("model") {
        None => out.push_str("  - Default model: not configured in this source.\n"),
        Some(v) => match model(v) {
            Some(m) => out.push_str(&format!("  - Default model: {}.\n", escape(&m))),
            None => out
                .push_str("  - Default model: configured; unsafe or unsupported value omitted.\n"),
        },
    }
    for key in ["providers", "agents"] {
        if let Some(Json::Object(items)) = v.get(key) {
            out.push_str(&format!("  - {key}: {} configured.\n", items.len()));
            if items.len() > MAX_ENTRIES {
                out.push_str("    - Identifier display limit reached; listing incomplete.\n");
            }
            for (id, item) in items.iter().take(MAX_ENTRIES) {
                if !safe_id(id) {
                    continue;
                }
                out.push_str(&format!("    - {}", escape(id)));
                if key == "agents" {
                    if let Some(mode) = identifier(item.get("mode")) {
                        out.push_str(&format!("; mode {}", escape(mode)));
                    }
                    match item.get("model") {
                        None => out.push_str("; model inherited/unspecified in this source"),
                        Some(m) => {
                            if let Some(m) = model(m) {
                                out.push_str(&format!("; model {}", escape(&m)));
                            } else {
                                out.push_str("; model value omitted");
                            }
                        }
                    }
                    if let Some(Json::Array(rules)) = item.get("permissions") {
                        out.push_str(&format!(
                            "; {} agent-specific permission rules (not evaluated)",
                            rules.len()
                        ));
                    }
                    for flag in ["disabled", "hidden"] {
                        if let Some(Json::Bool(b)) = item.get(flag) {
                            out.push_str(&format!("; {flag} {b}"));
                        }
                    }
                }
                out.push('\n');
                if key == "providers" {
                    if let Some(Json::Object(models)) = item.get("models") {
                        if models.len() > MAX_ENTRIES {
                            out.push_str(
                                "      - Model display limit reached; listing incomplete.\n",
                            );
                        }
                        for id in models.keys().take(MAX_ENTRIES).filter(|id| safe_id(id)) {
                            out.push_str(&format!("      - Model ID: {}\n", escape(id)));
                        }
                    }
                }
            }
        } else if v.get(key).is_some() {
            out.push_str(&format!("  - {key}: unsupported shape.\n"));
        }
    }
    for key in ["plugins", "commands", "skills"] {
        if let Some(value) = v.get(key) {
            out.push_str(&format!(
                "  - {key}: {} configured entries (static inventory).\n",
                value.count()
            ));
            if !matches!(value, Json::Array(_) | Json::Object(_)) {
                out.push_str("    - Unsupported shape; count is not authoritative.\n");
            }
            if key == "skills" {
                out.push_str("    - Explicit skill sources are not scanned.\n");
            }
        }
    }
    if let Some(mcp) = v.get("mcp") {
        match mcp.get("servers") {
            Some(Json::Object(s)) => out.push_str(&format!("  - MCP servers: {}.\n", s.len())),
            Some(_) => out.push_str("  - MCP servers: unsupported shape.\n"),
            None => out.push_str(&format!(
                "  - Legacy/unsupported MCP shape: {} entries; server count not verified.\n",
                mcp.count()
            )),
        }
    }
    if let Some(permissions) = v.get("permissions") {
        if let Json::Array(items) = permissions {
            let mut counts = [0usize; 4];
            let mut broad = false;
            for p in items {
                let effect = p.get("effect").and_then(Json::text);
                counts[match effect {
                    Some("allow") => 0,
                    Some("deny") => 1,
                    Some("ask") => 2,
                    _ => 3,
                }] += 1;
                if effect == Some("allow")
                    && ["action", "resource"].iter().any(|k| {
                        p.get(k)
                            .and_then(Json::text)
                            .is_some_and(|s| s.contains('*'))
                    })
                {
                    broad = true;
                }
            }
            out.push_str(&format!(
                "  - Permission effects: allow {}, deny {}, ask {}, other/unknown {}.\n",
                counts[0], counts[1], counts[2], counts[3]
            ));
            if broad {
                out.push_str(
                    "    - Warning: wildcard allow rule configured; scope not evaluated.\n",
                );
            }
        } else {
            out.push_str("  - Permissions: unsupported shape.\n");
        }
    }
    if let Some(w) = v.get("warming") {
        out.push_str(match w {
            Json::Bool(true) => "  - Warming: enabled in source.\n",
            Json::Bool(false) => "  - Warming: disabled in source.\n",
            Json::Object(_) => "  - Warming: configured object; details omitted.\n",
            _ => "  - Warming: unsupported shape.\n",
        });
    }
    for key in [
        "provider",
        "agent",
        "permission",
        "plugin",
        "command",
        "skill",
        "small_model",
    ] {
        if let Some(value) = v.get(key) {
            out.push_str(&format!(
                "  - Legacy key {key}: present ({} collection entries); not interpreted.\n",
                value.count()
            ));
        }
    }
}
fn config(path: &Path, label: &str, out: &mut String) {
    out.push_str(&format!("- {}: ", escape(label)));
    match read_config(path) {
        Ok(v) => {
            out.push_str("read successfully\n");
            summary(&v, out);
        }
        Err(reason) => out.push_str(&format!("{reason}\n")),
    }
}
// Byte-at-a-time reads deliberately avoid buffering any prompt body after the fence.
fn frontmatter(path: &Path) -> Result<Option<Vec<String>>, &'static str> {
    use std::io::Read;
    no_links(path)?;
    let metadata = fs::symlink_metadata(path).map_err(|_| "unreadable metadata")?;
    if !metadata.is_file() {
        return Err("not a regular file");
    }
    if metadata.len() > MAX_BYTES {
        return Err("oversize");
    }
    let mut file = fs::File::open(path).map_err(|_| "unreadable metadata")?;
    let mut total = 0;
    let mut lines = Vec::new();
    for number in 0..128 {
        let mut line = Vec::new();
        loop {
            if total >= 16 * 1024 {
                return Err("frontmatter byte limit reached");
            }
            let mut byte = [0];
            let n = file.read(&mut byte).map_err(|_| "unreadable metadata")?;
            if n == 0 {
                if number == 0 {
                    return Ok(None);
                }
                if line == b"---" || line == b"---\r" {
                    return Ok(Some(lines));
                }
                return Err("unclosed frontmatter");
            }
            total += 1;
            if byte[0] == b'\n' {
                break;
            }
            line.push(byte[0]);
        }
        let line = String::from_utf8(line).map_err(|_| "invalid frontmatter UTF-8")?;
        let line = line.trim_end_matches('\r');
        if number == 0 {
            if line != "---" {
                return Ok(None);
            }
        } else if line == "---" {
            return Ok(Some(lines));
        } else {
            lines.push(line.to_owned());
        }
    }
    Err("frontmatter line limit reached")
}

fn simple_scalar(value: &str) -> Option<&str> {
    let value = value.trim();
    let value = if value.starts_with('"') || value.starts_with('\'') {
        let quote = value.as_bytes()[0] as char;
        let inner = value.strip_prefix(quote)?.strip_suffix(quote)?;
        if inner.contains(quote) || inner.contains('\\') {
            return None;
        }
        inner
    } else {
        value
    };
    if value.is_empty()
        || value.chars().any(char::is_whitespace)
        || value.starts_with(['|', '>', '&', '*', '!', '{', '[', '#'])
        || value.contains(['{', '}', '[', ']', '\\', '"', '\''])
    {
        None
    } else {
        Some(value)
    }
}

fn markdown_metadata(path: &Path, kind: &str, out: &mut String) {
    let lines = match frontmatter(path) {
        Ok(Some(lines)) => lines,
        Ok(None) => {
            if kind == "skills" {
                out.push_str("    - Description present: false\n");
            }
            return;
        }
        Err(reason) => {
            out.push_str(&format!("    - Metadata: {reason}\n"));
            return;
        }
    };
    let mut values = BTreeMap::new();
    let mut description = false;
    let mut unsupported = false;
    for line in &lines {
        if line.starts_with(char::is_whitespace) || line.starts_with('#') {
            continue;
        }
        let Some((key, value)) = line.split_once(':') else {
            continue;
        };
        let key = key.trim();
        if key == "description" {
            description = true;
            continue;
        }
        if kind == "skills" || !["mode", "model", "agent", "subagent", "disabled"].contains(&key) {
            continue;
        }
        if values.insert(key, value).is_some() {
            out.push_str("    - Metadata: duplicate fields unsupported\n");
            return;
        }
    }
    if kind == "skills" {
        out.push_str(&format!("    - Description present: {description}\n"));
        return;
    }
    for (key, value) in values {
        let Some(value) = simple_scalar(value) else {
            unsupported = true;
            continue;
        };
        let safe = match key {
            "mode" if ["primary", "subagent", "all"].contains(&value) => Some(value.to_owned()),
            "agent" if safe_id(value) => Some(value.to_owned()),
            "subagent" | "disabled" if ["true", "false"].contains(&value) => Some(value.to_owned()),
            "model" => model(&Json::String(value.to_owned())),
            _ => None,
        };
        if let Some(safe) = safe {
            out.push_str(&format!("    - {key}: {}\n", escape(&safe)));
        } else {
            unsupported = true;
        }
    }
    if unsupported {
        out.push_str("    - Unsafe or unsupported metadata omitted (complex/block/anchor YAML unsupported).\n");
    }
}

fn inventory(path: &Path, label: &str, kind: &str, out: &mut String) {
    out.push_str(&format!("- {}:\n", escape(label)));
    if let Err(reason) = no_links(path) {
        out.push_str(&format!("  - {reason}\n"));
        return;
    }
    let mut stack = vec![(path.to_path_buf(), 0)];
    let mut seen = 0;
    let mut dirs = 0;
    let mut found = 0;
    while let Some((dir, depth)) = stack.pop() {
        dirs += 1;
        if dirs > MAX_SCAN_DIRS {
            out.push_str("  - Directory limit reached; inventory incomplete.\n");
            break;
        }
        let entries = match fs::read_dir(&dir) {
            Ok(e) => e,
            Err(_) => {
                out.push_str("  - Unreadable directory.\n");
                continue;
            }
        };
        let mut paths = Vec::new();
        for entry in entries {
            seen += 1;
            if seen > MAX_SCAN_ENTRIES {
                out.push_str("  - Entry limit reached; inventory incomplete.\n");
                return;
            }
            match entry {
                Ok(e) => paths.push(e.path()),
                Err(_) => out.push_str("  - Unreadable entry.\n"),
            }
        }
        paths.sort();
        for p in paths {
            let name = p.file_name().unwrap_or_default().to_string_lossy();
            if kind == "timers"
                && !(name.to_ascii_lowercase().contains("opencode") && name.ends_with(".timer"))
            {
                continue;
            }
            let relative = escape(&p.strip_prefix(path).unwrap_or(&p).to_string_lossy());
            let m = match fs::symlink_metadata(&p) {
                Ok(m) => m,
                Err(_) => {
                    out.push_str("  - Unreadable metadata.\n");
                    continue;
                }
            };
            if m.file_type().is_symlink() {
                out.push_str(&format!("  - {relative}: symlink skipped\n"));
                continue;
            }
            if m.is_dir() && kind != "timers" {
                if depth < 8 {
                    stack.push((p, depth + 1));
                } else {
                    out.push_str("  - Depth limit reached; inventory incomplete.\n");
                }
            } else if m.is_file()
                && ((kind == "skills"
                    && (name == "SKILL.md" || (depth == 0 && name.ends_with(".md"))))
                    || (kind == "markdown" && name.ends_with(".md"))
                    || kind == "timers")
            {
                found += 1;
                out.push_str(&format!("  - {relative}\n"));
                if kind == "skills" || kind == "markdown" {
                    markdown_metadata(&p, kind, out);
                }
            }
        }
    }
    if found == 0 {
        out.push_str("  - No matching regular files found.\n");
    }
}
struct Options {
    home: PathBuf,
    project: PathBuf,
    global: PathBuf,
}
fn options(args: &[String]) -> Result<Option<Options>, &'static str> {
    if args == ["--help"] {
        return Ok(None);
    }
    let mut home = None;
    let mut project = None;
    let mut i = 0;
    while i < args.len() {
        let dest = match args[i].as_str() {
            "--home" => &mut home,
            "--project" => &mut project,
            _ => return Err("unknown argument"),
        };
        if dest.is_some() {
            return Err("duplicate argument");
        }
        i += 1;
        let value = args
            .get(i)
            .filter(|s| !s.is_empty() && !s.starts_with("--"))
            .ok_or("missing path argument")?;
        *dest = Some(PathBuf::from(value));
        i += 1;
    }
    let isolated = home.is_some();
    let cwd = env::current_dir().map_err(|_| "current directory unavailable")?;
    let absolute = |p: PathBuf| if p.is_absolute() { p } else { cwd.join(p) };
    let home = absolute(
        home.or_else(|| env::var_os("HOME").map(PathBuf::from))
            .ok_or("HOME unavailable; use --home")?,
    );
    let global = if isolated {
        home.join(".config/opencode")
    } else {
        env::var_os("XDG_CONFIG_HOME")
            .map(PathBuf::from)
            .filter(|p| p.is_absolute())
            .unwrap_or_else(|| home.join(".config"))
            .join("opencode")
    };
    let project = absolute(project.unwrap_or_else(|| cwd.clone()));
    no_links(&project)
        .map_err(|_| "project path missing, unreadable, or contains symlinks/parent components")?;
    if !fs::metadata(&project).is_ok_and(|m| m.is_dir()) {
        return Err("project path is not a directory");
    }
    Ok(Some(Options {
        home,
        project,
        global,
    }))
}
fn executable_present(name: &str) -> bool {
    env::var_os("PATH").is_some_and(|p| {
        env::split_paths(&p).take(MAX_ENTRIES).any(|d| {
            let p = d.join(name);
            if no_links(&p).is_err() {
                return false;
            }
            fs::metadata(p).is_ok_and(|m| {
                #[cfg(unix)]
                {
                    use std::os::unix::fs::PermissionsExt;
                    m.is_file() && m.permissions().mode() & 0o111 != 0
                }
                #[cfg(not(unix))]
                {
                    m.is_file()
                }
            })
        })
    })
}
fn report(o: &Options) -> String {
    let mut out = format!("---\ntype: Reference\ntitle: OpenCode environment inventory\n---\n\n## Environment\n\n- OS: {}\n- Architecture: {}\n- OpenCode executable on PATH: {}\n- rust-script executable on PATH: {}\n- Executables are inspected by metadata only; symlinks are skipped.\n\n## Configuration sources\n\nDiscovery order below is an inventory, not an effective merged configuration.\nGlobal sources precede ALL direct ancestor sources, then ALL .opencode sources.\nWithin each group ancestors are ordered filesystem root to project.\nBoth JSON and JSONC candidates are listed; their coexistence precedence is not inferred.\nAncestor indices identify source positions, not literal directory names.\n\n", env::consts::OS, env::consts::ARCH, executable_present("opencode"), executable_present("rust-script"));
    for name in ["opencode.json", "opencode.jsonc"] {
        config(&o.global.join(name), &format!("global/{name}"), &mut out);
    }
    let mut ancestors: Vec<_> = o.project.ancestors().take(MAX_DIRS + 1).collect();
    if ancestors.len() > MAX_DIRS {
        out.push_str("- Ancestor limit reached; discovery incomplete.\n");
        ancestors.truncate(MAX_DIRS);
    }
    ancestors.reverse();
    for prefix in ["", ".opencode/"] {
        for (i, dir) in ancestors.iter().enumerate() {
            for name in ["opencode.json", "opencode.jsonc"] {
                config(
                    &dir.join(format!("{prefix}{name}")),
                    &format!("ancestor-{i}/{prefix}{name}"),
                    &mut out,
                );
            }
        }
    }
    out.push_str("\n## CLI configuration\n\nOnly global cli.json is a CLI configuration source. Values are omitted.\n\n");
    match read_config(&o.global.join("cli.json")) {
        Ok(v) => out.push_str(&format!(
            "- global/cli.json: read successfully; {} top-level fields.\n",
            v.count()
        )),
        Err(reason) => out.push_str(&format!("- global/cli.json: {reason}\n")),
    }
    out.push_str("\n## Skills, agents, commands and timer filenames\n\nFilenames and limited frontmatter are inspected; prompt bodies are not reported.\n\n");
    for (path, label) in [
        (o.global.clone(), "global/opencode"),
        (o.home.join(".agents"), "global/.agents"),
        (o.home.join(".claude"), "global/.claude"),
    ] {
        inventory(
            &path.join("skills"),
            &format!("{label}/skills"),
            "skills",
            &mut out,
        );
        if label == "global/opencode" {
            for name in ["agents", "commands", "agent", "command"] {
                inventory(
                    &path.join(name),
                    &format!("{label}/{name}"),
                    "markdown",
                    &mut out,
                );
            }
        }
    }
    for (i, dir) in o.project.ancestors().take(MAX_DIRS).enumerate() {
        for prefix in [".opencode", ".agents", ".claude"] {
            inventory(
                &dir.join(prefix).join("skills"),
                &format!("project-up-{i}/{prefix}/skills"),
                "skills",
                &mut out,
            );
            if prefix == ".opencode" {
                for name in ["agents", "commands", "agent", "command"] {
                    inventory(
                        &dir.join(prefix).join(name),
                        &format!("project-up-{i}/{prefix}/{name}"),
                        "markdown",
                        &mut out,
                    );
                }
            }
        }
        if fs::symlink_metadata(dir.join(".git")).is_ok() {
            break;
        }
    }
    inventory(
        &o.home.join(".config/systemd/user"),
        "global/systemd/user (opencode timers only)",
        "timers",
        &mut out,
    );
    out.push_str("\n## Inspection limits\n\n- Plugins, commands and warming are static configuration only.\n- Timer filenames do not establish enabled, active or scheduled automation.\n- Actual model usage, built-ins and effective inherited models are not inferred.\n- Session overrides, environment/remote configuration and schedules are unverified.\n- Explicit configured skill sources are not followed.\n- Prompts, command bodies, URLs, arbitrary settings and substitutions are never reported.\n- Unknown or unsafe identifiers are omitted; accepted IDs are syntax-filtered.\n- No auth, .env, service bodies, logs or databases are read.\n- No subprocesses, network access or writes are performed.\n- Reads are bounded to 1 MiB per config and 64 JSON nesting levels.\n- Frontmatter is bounded to 128 lines and 16 KiB; complex YAML is not interpreted.\n- Each inventory is bounded to 8192 entries, 1024 directories and depth 8.\n- Symlinks are skipped; concurrent filesystem changes are not snapshot-isolated.\n");
    out
}
fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    match options(&args) {
        Ok(Some(o)) => print!("{}", report(&o)),
        Ok(None) => println!("Usage: report.rs [--home PATH] [--project PATH]\nRead-only OpenCode structural inventory on stdout. --home isolates global discovery.\nNo tool execution, network or AI calls. Values are allowlisted, not a secrecy guarantee.\nSymlinks are skipped; inspection gaps appear in the report. Review before sharing."),
        Err(reason) => { eprintln!("report: {reason}; use --help"); std::process::exit(2); }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};
    static NEXT: AtomicUsize = AtomicUsize::new(0);
    #[test]
    fn model_variant_reference() {
        assert_eq!(
            model(&Json::String("openai/gpt-5#high".into())),
            Some("openai/gpt-5#high".into())
        );
        assert!(model(&Json::String("openai/gpt-5#sk-SECRET".into())).is_none());
    }
    #[test]
    fn config_order_and_global_only_cli() {
        let f = Fixture::new();
        f.put(".opencode/opencode.json", "{}");
        f.put("project/opencode.json", "{}");
        f.put("project/cli.json", "DO NOT READ");
        f.put("project/.git", "");
        let project = f.0.join("project");
        let index = project.ancestors().count() - 1;
        let s = report(&Options {
            home: f.0.clone(),
            global: f.0.join(".config/opencode"),
            project,
        });
        let direct = s
            .find(&format!(
                "ancestor-{index}/opencode.json: read successfully"
            ))
            .unwrap();
        let indirect = s
            .find(&format!(
                "ancestor-{}/.opencode/opencode.json: read successfully",
                index - 1
            ))
            .unwrap();
        assert!(direct < indirect);
        assert!(!s.contains(&format!("ancestor-{index}/cli.json")));
    }
    struct Fixture(PathBuf);
    impl Fixture {
        fn new() -> Self {
            let p = env::temp_dir().join(format!(
                "opencode-report-test-{}-{}",
                std::process::id(),
                NEXT.fetch_add(1, Ordering::Relaxed)
            ));
            fs::create_dir(&p).unwrap();
            Self(p)
        }
        fn put(&self, name: &str, s: &str) {
            let p = self.0.join(name);
            fs::create_dir_all(p.parent().unwrap()).unwrap();
            fs::write(p, s).unwrap();
        }
    }
    impl Drop for Fixture {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }
    #[test]
    fn jsonc_and_unicode() {
        let v = Parser::parse(
            r#"/*hi*/ {"a": [true, null, -1.2e+3,], "s": "https://x/*a*/\n\uD83D\uDE00é",} //end"#,
        )
        .unwrap();
        assert_eq!(v.get("s").unwrap().text(), Some("https://x/*a*/\n😀é"));
        assert_eq!(v.get("a").unwrap().count(), 3);
    }
    #[test]
    fn malformed_rejected() {
        for s in [
            "",
            "/*",
            "[1,,]",
            "01",
            "1.",
            "1e",
            "-",
            "true false",
            "{a:1}",
            "{\"a\":1,\"a\":2}",
            r#""\uD800""#,
            r#""\uDC00""#,
            r#""\q""#,
            "\"a\nb\"",
            "[",
            "{\"a\" 1}",
        ] {
            assert!(Parser::parse(s).is_err(), "{s}");
        }
    }
    #[test]
    fn parser_bounds() {
        assert!(Parser::parse(&format!("{}0{}", "[".repeat(66), "]".repeat(66))).is_err());
        assert!(Parser::parse(&" ".repeat(MAX_BYTES as usize + 1)).is_err());
    }
    #[test]
    fn safe_identifiers() {
        for s in [
            "sk-test",
            "openai/sk-SECRET",
            "ghp_abc",
            "${SECRET}",
            "https://x",
            "x\ny",
            "eyJabc",
            "token123",
            "<script>",
        ] {
            assert!(!safe_id(s));
        }
        assert!(safe_id("openai/gpt-5.2"));
    }
    #[test]
    fn summary_redacts() {
        let v = Parser::parse(r#"{"model":"sk-SECRET","providers":{"safe":{"apiKey":"HIDDEN","models":{"good":{},"sk-SECRET":{}}}},"agents":{"helper":{"mode":"subagent","prompt":"HIDDEN","model":{"providerID":"openai","model":"gpt-5","variant":"high"}}},"commands":{"x":"HIDDEN"},"plugins":["https://HIDDEN"],"mcp":{"servers":{"x":{"env":{"KEY":"HIDDEN"}}}},"permissions":[{"effect":"allow","action":"*","resource":"*"}],"skills":["HIDDEN"],"warming":true,"agent":{"legacy":{}}}"#).unwrap();
        let mut s = String::new();
        summary(&v, &mut s);
        assert!(!s.contains("SECRET"));
        assert!(!s.contains("HIDDEN"));
        assert!(s.contains("openai/gpt-5"));
        assert!(s.contains("wildcard"));
        assert!(s.contains("Legacy key agent"));
        assert!(s.contains("Explicit skill sources"));
    }
    #[test]
    fn invalid_and_oversize_no_payload() {
        let f = Fixture::new();
        f.put("bad.json", "SECRET invalid");
        let mut s = String::new();
        config(&f.0.join("bad.json"), "fixture", &mut s);
        assert!(s.contains("invalid JSON"));
        assert!(!s.contains("SECRET"));
        f.put("large", &"X".repeat(MAX_BYTES as usize + 1));
        assert_eq!(read_config(&f.0.join("large")), Err("oversize"));
    }
    #[test]
    fn fixture_inventory() {
        let f = Fixture::new();
        f.put(
            ".config/opencode/opencode.jsonc",
            r#"{"model":"provider/model","agents":{"helper":{}}}"#,
        );
        f.put("project/.git", "");
        f.put(".agents/skills/demo/SKILL.md", "SECRET PROMPT");
        f.put("project/.opencode/commands/a.md", "SECRET COMMAND");
        f.put(
            ".config/systemd/user/opencode-refresh.timer",
            "SECRET TIMER",
        );
        let s = report(&Options {
            home: f.0.clone(),
            project: f.0.join("project"),
            global: f.0.join(".config/opencode"),
        });
        assert!(s.starts_with("---\ntype: Reference"));
        assert!(s.contains("provider/model"));
        assert!(s.contains("inherited/unspecified"));
        assert!(s.contains("demo/SKILL.md"));
        assert!(s.contains("opencode-refresh.timer"));
        assert!(!s.contains("SECRET"));
        assert!(!s.contains(f.0.to_str().unwrap()));
    }
    #[test]
    fn traversal_bound() {
        let f = Fixture::new();
        for i in 0..=MAX_SCAN_ENTRIES {
            f.put(&format!("{i}.md"), "");
        }
        let mut s = String::new();
        inventory(&f.0, "fixture", "markdown", &mut s);
        assert!(s.contains("Entry limit reached"));
    }
    #[test]
    fn cli_validation() {
        assert!(options(&["--bad".into()]).is_err());
        assert!(options(&["--home".into()]).is_err());
        assert!(options(&["--help".into()]).unwrap().is_none());
        let o = options(&["--home".into(), "/fixture".into()])
            .unwrap()
            .unwrap();
        assert_eq!(o.global, PathBuf::from("/fixture/.config/opencode"));
        let f = Fixture::new();
        assert!(options(&[
            "--project".into(),
            f.0.join("missing").to_string_lossy().into_owned()
        ])
        .is_err());
        f.put("not-a-directory", "");
        assert!(options(&[
            "--project".into(),
            f.0.join("not-a-directory").to_string_lossy().into_owned()
        ])
        .is_err());
    }
    #[test]
    fn rendering() {
        assert_eq!(escape("a`[b]\n"), "a&#96;&#91;b&#93;&#10;");
    }
    #[cfg(unix)]
    #[test]
    fn symlinks_skipped() {
        use std::os::unix::fs::symlink;
        let f = Fixture::new();
        f.put("real/config", "{}");
        symlink(f.0.join("real"), f.0.join("link")).unwrap();
        assert_eq!(
            read_config(&f.0.join("link/config")),
            Err("symlink skipped")
        );
        let mut s = String::new();
        inventory(&f.0, "fixture", "skills", &mut s);
        assert!(s.contains("symlink skipped"));
    }

    #[test]
    fn flat_and_nested_skill_metadata() {
        let f = Fixture::new();
        f.put(
            "flat.md",
            "---\ndescription: |\n  PRIVATE DESCRIPTION\n---\nPRIVATE BODY",
        );
        f.put("nested/SKILL.md", "---\nname: demo\n---\nPRIVATE BODY");
        f.put("nested/ignored.md", "PRIVATE BODY");
        f.put(".env", "PRIVATE ENV");
        let mut s = String::new();
        inventory(&f.0, "skills", "skills", &mut s);
        assert!(s.contains("flat.md"));
        assert!(s.contains("nested/SKILL.md"));
        assert!(s.contains("Description present: true"));
        assert!(s.contains("Description present: false"));
        assert!(!s.contains("ignored.md"));
        assert!(!s.contains("PRIVATE"));
        assert!(!s.contains(".env"));
    }

    #[test]
    fn markdown_agent_and_command_metadata() {
        let f = Fixture::new();
        f.put("helper.md", "---\nmode: subagent\nmodel: provider/model#high\nagent: helper\nsubagent: true\ndisabled: false\n---\nPRIVATE BODY");
        let mut s = String::new();
        inventory(&f.0, "agents", "markdown", &mut s);
        assert!(s.contains("mode: subagent"));
        let expected = model(&Json::String("provider/model#high".into())).unwrap();
        assert!(s.contains(&format!("model: {}", escape(&expected))));
        assert!(s.contains("agent: helper"));
        assert!(s.contains("subagent: true"));
        assert!(s.contains("disabled: false"));
        assert!(!s.contains("PRIVATE"));
    }

    #[test]
    fn metadata_injections_and_complex_yaml_omitted() {
        let f = Fixture::new();
        for (i, value) in [
            "sk-PRIVATE",
            "https://PRIVATE",
            "${PRIVATE}",
            "&PRIVATE helper",
            "*PRIVATE",
            "|",
            ">",
            "[PRIVATE]",
            "{PRIVATE: x}",
            "\"helper\\nPRIVATE\"",
        ]
        .iter()
        .enumerate()
        {
            f.put(
                &format!("{i}.md"),
                &format!("---\nagent: {value}\n  PRIVATE\ndescription: PRIVATE\n---\nPRIVATE"),
            );
        }
        let mut s = String::new();
        inventory(&f.0, "commands", "markdown", &mut s);
        assert!(!s.contains("PRIVATE"));
        assert!(s.contains("unsupported metadata omitted"));
        f.put("duplicate.md", "---\nagent: helper\nagent: other\n---\n");
        let mut s = String::new();
        markdown_metadata(&f.0.join("duplicate.md"), "markdown", &mut s);
        assert!(s.contains("duplicate fields"));
        assert!(!s.contains("helper"));
    }

    #[test]
    fn frontmatter_reads_stop_and_bounds() {
        let f = Fixture::new();
        f.put(
            "plain.md",
            &format!("No frontmatter\n{}", "x".repeat(20_000)),
        );
        assert_eq!(frontmatter(&f.0.join("plain.md")), Ok(None));
        f.put(
            "closed.md",
            &format!("---\nmode: subagent\n---\n{}", "x".repeat(20_000)),
        );
        assert_eq!(
            frontmatter(&f.0.join("closed.md")),
            Ok(Some(vec!["mode: subagent".into()]))
        );
        f.put("lines.md", &format!("---\n{}---\n", "x: y\n".repeat(128)));
        assert_eq!(
            frontmatter(&f.0.join("lines.md")),
            Err("frontmatter line limit reached")
        );
        f.put(
            "bytes.md",
            &format!("---\n{}\n---\n", "x".repeat(16 * 1024)),
        );
        assert_eq!(
            frontmatter(&f.0.join("bytes.md")),
            Err("frontmatter byte limit reached")
        );
        f.put("unclosed.md", "---\nmode: subagent\n");
        assert_eq!(
            frontmatter(&f.0.join("unclosed.md")),
            Err("unclosed frontmatter")
        );
        f.put("large.md", &"x".repeat(MAX_BYTES as usize + 1));
        assert_eq!(frontmatter(&f.0.join("large.md")), Err("oversize"));
        assert_eq!(frontmatter(&f.0), Err("not a regular file"));
    }
}
