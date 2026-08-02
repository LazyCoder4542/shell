# Build Your Own Shell — Dev Log

## How I use this doc

- **Daily log** — append a dated entry every session. Cheap to write; bullets are fine. Do this *while* working.
- **Gotchas** — when a bug costs me more than ~15 min, I copy the problem + cause + fix here so future-me can grep it.
- **Concepts** — when something finally clicks, I write the one-paragraph version in my own words.

**Rule of thumb:** capture everything in the daily log while coding (low friction), then ~once a week promote the reusable stuff up into Gotchas / Concepts.

---

## Roadmap / progress

- [ ] REPL loop (print prompt → read line → loop)
- [ ] Tokenizer (split words, handle quotes and escapes)
- [ ] Execute external commands (fork / execvp / waitpid)
- [ ] PATH resolution
- [ ] Built-ins (cd, exit, echo, pwd, export)
- [ ] Redirection (`>`, `<`, `>>`, `2>`)
- [ ] Pipes (`cmd1 | cmd2`)
- [ ] Signal handling (Ctrl-C, Ctrl-\, Ctrl-Z)
- [ ] Job control (`&`, `jobs`, `fg`, `bg`)
- [ ] Env var expansion (`$VAR`, `$?`)
- [ ] Quality of life (history, tab completion)

---

## Daily log

### [Template — copy this block each session]

**Day N — YYYY-MM-DD**
- **Goal today:**
- **What I did:**
- **Problems hit:**
- **What I learned:**
- **Next session starts with:**

---

### Day 2 — 2026-08-02
- **Goal today:** read Rust Book ch.17 (OOP in Rust)
- **What I did:** worked through structs + traits as Rust's answer to OOP patterns; generics, trait bounds, trait objects; Box and heap data; the state-transition pattern with Option
- **Problems hit:** none — reading day, no code
- **What I learned:** (see Concepts entries below)
- **Next session starts with:** back to the shell — get the REPL loop + clean exit working

---

### Day 1 — 2026-08-01
- **Goal today:** print a prompt and read one line of input
- **What I did:** printed a `$ ` prompt; read a line with readline; trimmed the input by shadowing the binding
- **Problems hit:** none blocking — no loop yet, so it reads a single line then exits
- **What I learned:** shadowing lets me rebind the input variable to its trimmed value under the same name
- **Next session starts with:** Implementing the REPL architecture

---

## Gotchas (searchable bug + fix log)

### Ctrl-D causes an infinite loop
- **Symptom:** shell spins forever when I hit EOF
- **Cause:** didn't check `getline()`'s return value; it returns -1 on EOF/error
- **Fix:** break out of the loop when `getline()` returns -1
- **Tags:** #input #eof

---

## Concepts (distilled, in my own words)

### Generics vs trait objects (static vs dynamic dispatch)
- Generics (`fn f<T: Draw>(x: T)`): compiler monomorphizes — generates a concrete copy of the code per type used. Dispatch is resolved at compile time (static). Zero runtime cost, but each collection is one concrete type (a Vec<T> is all the same T).
- Trait objects (`Box<dyn Draw>`): one copy of the code; the actual method is looked up at runtime through a vtable (dynamic dispatch). Slight runtime cost + indirection, but lets me mix types — a Vec<Box<dyn Draw>> can hold buttons AND checkboxes together.
- Rule of thumb: reach for generics by default; reach for trait objects when I genuinely need a heterogeneous collection or want to avoid code bloat.

### Why Box, and the fat pointer
- `dyn Trait` is unsized → must be accessed behind a pointer. Box<dyn Trait> gives me an owned, fixed-size handle to it.
- That pointer is actually a "fat pointer": two words wide — one pointer to the data, one to the vtable (the table of method addresses). That's how dynamic dispatch knows which concrete method to call.
- Box<T> for a normal sized T is just a thin pointer to heap. It's specifically the *unsized* case where the extra vtable word shows up.

### State pattern + Option::take() placeholder
- Problem: a method wants to *replace* a state field (old state → new state), but the method only has `&mut self`. You can't move a value out of a borrow — that would leave the field holding nothing, which Rust forbids.
- Fix: make the field `Option<Box<dyn State>>`. Call `self.state.take()` — this swaps `None` in and hands you ownership of the old `Some(state)`. Now you own it, can consume it to compute the next state, and write the new `Some(next)` back.
- The `Option` is the placeholder that keeps the field valid during the swap. `take()` is the "give me the value, leave None behind" move.