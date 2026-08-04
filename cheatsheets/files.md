# Rust — Paths, Files & Directories Cheat-Sheet

Everything lives in `std::path` (`Path`, `PathBuf`) and `std::fs` (`read`, `write`, dirs…) plus `std::io` for the `Read`/`Write` traits.

```rust
use std::path::{Path, PathBuf};
use std::fs;
use std::io::{self, Read, Write, BufReader, BufWriter, BufRead};
```

---

## 1. `Path` vs `PathBuf` — same pattern you already know

| | `PathBuf` | `&Path` |
|---|---|---|
| Ownership | **Owns** | **Borrows** |
| Mutable / growable | Yes | No |
| Analogy | `String` / `Vec<T>` | `&str` / `&[T]` |

`PathBuf` **derefs to `Path`**, so every `&Path` method works on a `PathBuf` for free — exactly like `String` → `&str`. Rule of thumb: **take `&Path` (or `impl AsRef<Path>`), store `PathBuf`.**

```rust
fn load(p: &Path) { /* ... */ }     // accepts &Path, &PathBuf, and &str via coercion
```

Most `fs` functions take `impl AsRef<Path>`, which is why you can pass a `&str`, `String`, `&Path`, or `PathBuf` interchangeably:

```rust
fs::read_to_string("notes.txt")?;               // &str works
fs::read_to_string(Path::new("notes.txt"))?;    // so does &Path
```

---

## 2. Creating & building paths

```rust
let p = Path::new("/tmp/foo.txt");        // &Path
let p = PathBuf::from("/tmp/foo.txt");     // PathBuf
let p: PathBuf = ["a", "b", "c"].iter().collect();  // -> "a/b/c"

// Build up paths — DO THIS, not string concatenation:
let full = base.join("subdir").join("file.txt");  // returns PathBuf
let mut p = PathBuf::from("/tmp");
p.push("logs");        // in place -> /tmp/logs
p.push("today.log");   // -> /tmp/logs/today.log
p.pop();               // -> /tmp/logs  (drops last component)
```

**Never build paths with `format!("{}/{}", a, b)`** — `.join()` uses the correct separator per OS (`/` vs `\`) and handles edge cases. String concatenation is a cross-platform bug waiting to happen.

---

## 3. Inspecting a path (all borrow — no filesystem access)

```rust
let p = Path::new("/home/ada/report.txt");

p.file_name();    // Some("report.txt")   -> Option<&OsStr>
p.file_stem();    // Some("report")       name without extension
p.extension();    // Some("txt")
p.parent();       // Some("/home/ada")    -> Option<&Path>
p.components();   // iterator over path pieces
p.is_absolute();  // true
p.is_relative();  // false
p.starts_with("/home");   // true
p.ends_with("report.txt");// true

// Derive new paths without mutating:
p.with_extension("bak");        // /home/ada/report.bak
p.with_file_name("other.txt");  // /home/ada/other.txt
```

These are pure string-ish operations on the path value — they do **not** touch the disk or require the file to exist.

---

## 4. Paths aren't guaranteed UTF-8 (the `OsStr` gotcha)

Filesystems can contain bytes that aren't valid UTF-8, so paths are `OsStr`/`OsString`, not `str`/`String`. Consequences:

```rust
p.to_str();            // Option<&str>  — None if not valid UTF-8
p.to_string_lossy();   // Cow<str>      — replaces bad bytes with 
p.display();           // for printing: println!("{}", p.display());
```

Use `.display()` to print a path (you can't just `{}` a `Path`). Use `.to_str()` only when you truly need a `&str` and are ready to handle the `None` case.

---

## 5. Existence & metadata (these DO hit the disk)

```rust
p.exists();        // bool — but swallows errors (treats "permission denied" as false)
p.try_exists()?;   // Result<bool> — PREFER THIS: distinguishes missing vs error
p.is_file();       // bool
p.is_dir();        // bool

let md = fs::metadata(p)?;   // Result<Metadata> (follows symlinks)
md.len();          // size in bytes
md.is_dir();
md.modified()?;    // SystemTime
md.permissions();
```

**TOCTOU warning:** don't `if p.exists() { open(p) }` — the file can vanish between the check and the use. Just attempt the operation and handle the error (idiomatic Rust: try, don't pre-check).

---

## 6. Reading files

```rust
// Whole file, one shot:
let bytes: Vec<u8> = fs::read(p)?;              // raw bytes
let text:  String  = fs::read_to_string(p)?;   // UTF-8 (errors if not valid UTF-8)

// Manual, via File + Read trait:
let mut f = fs::File::open(p)?;
let mut s = String::new();
f.read_to_string(&mut s)?;

// Line by line, buffered (best for large files):
let f = fs::File::open(p)?;
for line in BufReader::new(f).lines() {
    let line = line?;          // each is io::Result<String>, newline stripped
    println!("{line}");
}
```

`fs::read_to_string` is the go-to for "slurp a small text file." Use `BufReader::lines()` when the file is large or you process line-by-line.

---

## 7. Writing files

```rust
// One shot — CREATES or TRUNCATES, accepts &str or &[u8]:
fs::write(p, "hello\n")?;
fs::write(p, &bytes)?;

// Manual — File::create also creates/truncates:
let mut f = fs::File::create(p)?;
f.write_all(b"hello\n")?;

// Buffered (best for many small writes):
let mut w = BufWriter::new(fs::File::create(p)?);
writeln!(w, "line {}", 1)?;
w.flush()?;                    // flush before it drops (or on drop)
```

### `OpenOptions` — full control (append, create-if-new, etc.)

```rust
use std::fs::OpenOptions;

let mut f = OpenOptions::new()
    .append(true)      // write to the end instead of truncating
    .create(true)      // create if missing
    .open(p)?;
writeln!(f, "appended line")?;
```

Key flags: `.read()`, `.write()`, `.append()`, `.truncate()`, `.create()`, and `.create_new()` (fails if the file already exists — good for "don't clobber").

---

## 8. Directories

```rust
fs::create_dir(p)?;        // ONE level; fails if parent missing
fs::create_dir_all(p)?;    // like `mkdir -p` — makes all missing parents

fs::remove_file(p)?;       // delete a file
fs::remove_dir(p)?;        // dir must be EMPTY
fs::remove_dir_all(p)?;    // recursive delete — careful!

fs::rename(from, to)?;     // move/rename (same filesystem)
fs::copy(from, to)?;       // returns bytes copied; Result<u64>
```

### Listing a directory

```rust
for entry in fs::read_dir(dir)? {   // read_dir -> Result<ReadDir>
    let entry = entry?;             // each item is io::Result<DirEntry>
    let path = entry.path();        // PathBuf
    let name = entry.file_name();   // OsString
    let ft   = entry.file_type()?;  // cheap; no extra syscall on most platforms
    if ft.is_dir() {
        println!("[dir]  {}", path.display());
    } else {
        println!("[file] {}", path.display());
    }
}
```

Entries come back in **arbitrary order** — collect and `.sort()` if you need them sorted.

---

## 9. Current dir, absolute paths, home

```rust
std::env::current_dir()?;          // Result<PathBuf> — the process CWD
std::env::set_current_dir(p)?;     // chdir

p.canonicalize()?;                 // absolute path, symlinks + `..` resolved
                                   //   (the file MUST exist)

// Home directory: env::home_dir() has a messy deprecation history on Windows.
// Portable-ish: std::env::var_os("HOME") (Unix). Robust cross-platform: the `dirs` crate.
```

---

## 10. Error handling — everything is `io::Result`

Every `fs`/`io` call returns `io::Result<T>` (= `Result<T, io::Error>`). Use `?` to propagate; match on `ErrorKind` when you care about *why*:

```rust
use std::io::ErrorKind;

match fs::read_to_string("maybe.txt") {
    Ok(text) => { /* ... */ }
    Err(e) if e.kind() == ErrorKind::NotFound => { /* create it, etc. */ }
    Err(e) => return Err(e),   // some other failure
}
```

A function that does file work typically returns `io::Result<T>` itself and lets `?` bubble errors up.

---

## Gotchas worth memorizing

- **Build paths with `.join()`, never `format!("{a}/{b}")`** — separators differ by OS.
- **Print with `.display()`** — `Path` doesn't implement `Display` directly (because it may not be UTF-8).
- **`.to_str()` returns `Option`** — paths aren't guaranteed valid UTF-8.
- **Don't check-then-use** (`exists()` then `open()`) — race condition; just try and handle the error. Prefer `try_exists()?` over `exists()` when you must check, so errors aren't hidden.
- **`create_dir` vs `create_dir_all`** — the plain one fails if a parent is missing.
- **`read_dir` order is unspecified** — sort yourself if needed.
- **`canonicalize()` requires the file to exist** — it hits the disk to resolve symlinks.
- **Buffer bulk I/O** — wrap in `BufReader`/`BufWriter`; flush writers before they drop.

---

## Quick decision guide

| I want to… | Use |
|---|---|
| Function param for a path | `&Path` or `impl AsRef<Path>` |
| Store/build a path | `PathBuf` |
| Combine path pieces | `.join()` / `.push()` |
| Read a small text file | `fs::read_to_string(p)?` |
| Read a big file by line | `BufReader::new(File::open(p)?).lines()` |
| Overwrite a file | `fs::write(p, data)?` |
| Append to a file | `OpenOptions::new().append(true).create(true).open(p)?` |
| Make nested dirs | `fs::create_dir_all(p)?` |
| List a directory | `for e in fs::read_dir(p)? { … }` |
| Delete a tree | `fs::remove_dir_all(p)?` (careful) |
| Print a path | `p.display()` |
| Absolute, resolved path | `p.canonicalize()?` |