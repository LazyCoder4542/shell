# Rust Strings — A Complete Reference

## 1. `str` vs `String` — the core distinction

| | `String` | `&str` (string slice) |
|---|---|---|
| Ownership | **Owns** its data | **Borrows** — a view into data owned elsewhere |
| Location | Heap, growable | A fat pointer (ptr + len) to bytes somewhere |
| Mutable? | Yes (can grow/shrink) | No (fixed view) |
| `Copy`? | No | Yes |
| Analogy | `Vec<T>` | `&[T]` |

- Both are **always valid UTF-8**. This is why you can't index by byte freely.
- A string literal `"hello"` has type `&'static str` — baked into the binary, lives for the whole program.
- `String` **derefs to `&str`** (via `Deref`), so every `&str` method is callable on a `String` for free. That's why functions should take `&str` (accepts both) but return `String` when they own new data.

```rust
fn greet(name: &str) { println!("Hi {name}"); }

let owned = String::from("Ada");
greet(&owned);      // String -> &str automatically (deref coercion)
greet("Grace");     // literal is already &str
```

Rule of thumb: **take `&str`, store `String`.**

---

## 2. Creating strings

**Owned `String`:**
```rust
let a = String::new();                 // empty
let b = String::from("hi");            // from a literal
let c = "hi".to_string();              // via Display
let d = "hi".to_owned();               // clearest "borrow -> owned"
let e = String::with_capacity(32);     // pre-allocate, no reallocs while growing
let f = format!("{}-{}", 1, 2);        // "1-2", most flexible
let g: String = vec!['a','b'].into_iter().collect();  // from chars
```

`to_string`, `to_owned`, and `String::from` all produce the same result from a `&str`; pick by readability (`to_owned` signals intent best).

**Borrowed `&str`:**
```rust
let s = "literal";          // &'static str
let slice = &owned[0..2];   // sub-slice (byte range, must land on char boundaries)
let whole = owned.as_str(); // explicit String -> &str
```

---

## 3. Copying strings

- **`&str` is `Copy`** — copying it duplicates only the fat pointer (8+8 bytes), *not* the underlying text. Cheap.
- **`String` is NOT `Copy`** — it owns heap memory, so plain assignment **moves**:

```rust
let a = String::from("hi");
let b = a;          // MOVE — `a` is now invalid
// println!("{a}"); // ❌ borrow of moved value
```

To actually duplicate the text, **`.clone()`** (deep copy, new heap allocation):

```rust
let a = String::from("hi");
let b = a.clone();  // separate allocation; both usable
```

To turn a borrow into an owned copy: `.to_string()` / `.to_owned()` / `String::from(s)`.

---

## 4. Operations that allocate (return new) vs mutate in place

A running theme: many string ops come in two flavors — one **returns a new `String`** (allocates, needs no `mut`), one **mutates in place** (needs `mut`, no allocation).

### Case change

```rust
let s = "Grüße";

// Return a NEW String — Unicode-aware:
let up = s.to_uppercase();          // "GRÜSSE"
let lo = s.to_lowercase();

// Return a NEW String — ASCII only (leaves non-ASCII untouched):
let up = s.to_ascii_uppercase();

// Mutate IN PLACE — ASCII only, no allocation:
let mut t = String::from("hello");
t.make_ascii_uppercase();           // t == "HELLO"
```

Note: there is **no** in-place *Unicode* case change, because case mapping can change the byte length (e.g. `ß` → `SS`), which would require reallocation anyway.

### Sorting characters

A `String` can't be sorted in place (UTF-8 chars vary in byte width). Round-trip through `Vec<char>`:

```rust
let mut chars: Vec<char> = "dcba".chars().collect();
chars.sort();                        // or sort_unstable() — faster, no stability
let sorted: String = chars.into_iter().collect();  // "abcd"
```

### Appending (in place, needs `mut`)

```rust
let mut s = String::from("foo");
s.push('!');            // one char   -> "foo!"
s.push_str(" bar");     // a &str     -> "foo! bar"
s += " baz";            // AddAssign, takes &str -> "foo! bar baz"
```

### Concatenation

```rust
// `+` : LEFT side must be owned String (it's MOVED/consumed);
//        right side is &str (added by reference).
let a = String::from("foo");
let b = String::from("bar");
let c = a + &b;          // a is consumed; c == "foobar"

// format! : consumes nothing, most flexible, clearest for many pieces:
let c = format!("{a2}{b2}");

// concat() / join() on a slice of strings:
let c = ["foo", "bar"].concat();        // "foobar"
let c = ["foo", "bar"].join(", ");      // "foo, bar"
```

For building a string in a loop, prefer `push_str` into a `String::with_capacity(...)` over repeated `+` (avoids intermediate allocations).

---

## 5. Splitting

All splitters return **iterators of `&str`** that *borrow* into the original — no copying of substrings until you collect.

```rust
let s = "a,b,,c";

s.split(',')            // iter: "a","b","","c"  (pattern: char, &str, or closure)
s.split_whitespace()    // splits on any run of whitespace, no empties
s.lines()               // split into lines
s.splitn(2, ',')        // at most 2 pieces: "a", "b,,c"
s.rsplit(',')           // from the right
s.split_once(',')       // Option<(&str,&str)> — first split only: Some(("a","b,,c"))
s.chars()               // iterate characters
s.bytes()               // iterate raw bytes
```

Collect the pieces:

```rust
let parts: Vec<&str>   = s.split(',').collect();          // borrows
let owned: Vec<String> = s.split(',').map(String::from).collect(); // owns
```

---

## 6. Joining

```rust
let v = vec!["2024", "11", "05"];

v.join("-")     // "2024-11-05"   (separator between elements) -> String
v.concat()      // "20241105"     (no separator)               -> String

// From an iterator of chars/strings:
let s: String = vec!['h','i'].into_iter().collect();
```

`join` and `concat` work on slices/Vecs of `&str` **or** `String`.

---

## 7. Gotchas worth memorizing

- **No integer indexing:** `s[0]` does **not** compile. A byte index could land mid-character. Use:
  - `s.chars().nth(i)` — the i-th *character* (O(n), returns `Option<char>`)
  - `&s[0..4]` — a byte-range slice; **panics** if the range isn't on char boundaries
- **`.len()` is BYTES, not characters:** `"é".len()` is `2`. For character count use `s.chars().count()`.
- **Iterate the right unit:** `.chars()` for Unicode scalar values, `.bytes()` for raw bytes. Neither is "grapheme clusters" (user-perceived characters like emoji with modifiers) — that needs the `unicode-segmentation` crate.
- **Trimming:** `.trim()`, `.trim_start()`, `.trim_end()` return borrowed `&str` (no allocation).
- **Searching:** `.contains(pat)`, `.starts_with(pat)`, `.ends_with(pat)`, `.find(pat) -> Option<usize>` (byte offset), `.replace(from, to) -> String`.

---

## Quick decision guide

| I want to… | Use |
|---|---|
| Accept text in a function param | `&str` |
| Store/own/grow text in a struct | `String` |
| Duplicate a `String`'s text | `.clone()` |
| Borrow → owned | `.to_owned()` / `.to_string()` |
| Uppercase a copy | `.to_uppercase()` |
| Uppercase in place (ASCII) | `.make_ascii_uppercase()` |
| Add to the end | `.push` / `.push_str` |
| Combine, keeping originals | `format!` |
| Break apart | `.split(...)` (+ `.collect()`) |
| Stitch together | `.join(sep)` |