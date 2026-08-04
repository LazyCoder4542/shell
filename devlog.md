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

### Day 3 — 2026-08-03
- **Goal today:** get a real REPL flow going and start adding builtins
- **What I did:**
  - implemented the read-eval-loop; added `exit`, `echo`, and `type`
  - hit the point where the eval branch was becoming an if/else chain, so I refactored into a real architecture:
    - `Command` trait with a default `exec` — each builtin (Exit, Echo, Type) is its own struct implementing it
    - `Registry` holding a `HashMap<String, Box<dyn Command>>` for name → command lookup
    - `Engine` owning a borrowed `&Registry` plus a `State` enum (Init / Running / Exiting), running the REPL and dispatching
    - split into modules: main / engine / parser / commands
    - `Parser` turning input into `Token::Word(..)` values instead of raw string splits
- **Design decisions:**
  - went with trait objects (`Box<dyn Command>`) so commands are registered dynamically and dispatched via the registry
  - `type` checks the registry directly, so it stays correct as commands are added (no hardcoded builtin list)
  - exit is modeled as a state transition (`engine.exit()` flips state to Exiting) rather than an inline `break`
- **What I learned:**
  - builtins vs executables — builtins are handled by the shell itself, in-process; executables are separate programs found on PATH. `type` reports which is which.
  - more on shadowing: the old value isn't necessarily dropped when shadowed — it may be moved out, borrowed by the new binding, or just linger in scope until the block ends.
  - applied Chapter 17 directly — trait objects + a registry is a clean answer to "dispatch over a growing set of commands." The enum-based State from Day 2 is doing real work here.
- **Next session starts with:** Locating and running executables

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

### Builtins vs executables
- Builtin: a command the shell runs itself, in-process (e.g. exit, echo, type, cd). No new process; some (like cd) *must* be builtins because they change the shell's own state.
- Executable: a standalone program on disk, found by searching PATH, run in a child process.
- `type <name>` reports the category — "shell builtin" vs a resolved path — which is why type has to know the builtin list.

### Shadowing: what happens to the old value
- Shadowing rebinds a name; it does NOT automatically drop the previous value.
- The old value's fate depends on how it was used: it may be moved out (ownership transferred), it may be borrowed by the new binding, or it may just linger in scope until the block ends.
- Refinement from Day 1's note: `.trim()` borrows the original String, so the original must stay alive — shadowing the name doesn't kill the value the borrow points at.

### Command dispatch: trait object + registry
- Each command = a struct implementing a shared `Command` trait. Register them by name in a HashMap<String, Box<dyn Command>>.
- Dispatch = look up the name, call `.exec()` on the trait object. Adding a command is just: define a struct, impl Command, register it — no touching the eval logic.
- This is the "heterogeneous collection" case trait objects exist for: many different command types stored and called through one interface.
- Contrast with the earlier if/else chain: that grew linearly and mixed parsing with dispatch. The registry separates "what commands exist" from "how the loop runs."

### Lifetime parameters & annotations
- Where it showed up: `Engine<'a>` holds `registry: &'a Registry`. The `'a` says "this Engine borrows a Registry and cannot outlive it." The Registry created in `main` must live at least as long as the Engine that borrows it — the compiler enforces that.
- A lifetime parameter on a struct isn't a value; it's a promise: any instance ties its own validity to the data it references. That's also why the impl block is `impl<'a> Engine<'a>` — the impl has to be generic over the same lifetime it uses.
- Storing a reference in a struct *requires* naming the lifetime — lifetime elision doesn't apply to struct definitions. Without `'a`, you couldn't hold `&Registry` as a field at all. The alternatives would be to own the Registry (no borrow, no lifetime) or wrap it (Rc, etc.).
- "Bound" specifically means an *outlives* constraint:
  - `'a: 'b` reads "'a outlives 'b" — 'a lasts at least as long as 'b.
  - `T: 'a` means every reference inside `T` stays valid for at least `'a`.
  - `T: 'static` means `T` holds no non-static borrows — it could live for the whole program.