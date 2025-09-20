# ADR 003: Single `nrv` Object Injection

## Status

- Accepted  
- Supersedes ADR 001 (Standard Library in Global Scope)

## Context

Originally (ADR 001), Nerve defined a *global stdlib* of functions that were injected directly
into the language surface (e.g. `print()`, `now()`, `emit()`) across JS/TS/Rust syntaxes.  

This provided a zero-ceremony developer experience, but it created ambiguity:

- Hard to tell which symbols are “magic” vs. user-defined.
- Namespace collisions between user code and stdlib.
- Expansion of functionality would clutter the global scope.

A better approach is to **inject a single `nrv` object** into every runtime environment.
All core capabilities live under this namespace. This preserves **zero-ceremony access**
(no imports needed) while making the boundary between user code and Nerve capabilities explicit.

## Decision

- The CLI MUST inject exactly **one global object**: `nrv`.
- All built-in capabilities MUST be methods, fields, or sub-namespaces under `nrv`.
- No other functions are placed in the global scope by default.  
- Surface languages (JS, TS, Rust) MUST normalize access through `nrv`.

### Categories in `nrv`

1. **Filesystem / Repo State**  
   - `nrv.file.read(path)`, `nrv.file.write(path, content)`, `nrv.file.exists(path)`  
   - `nrv.dir.create(path)`, `nrv.dir.exists(path)`, `nrv.dir.tree(path, opts)`  
   - `nrv.dir.glob(pattern)`

2. **LLM Access**  
   - `nrv.llm(model).prompt([...])` → `.text() | .json() | .bool() | .edits()`  
   - No hidden prompts or roles; **all interactions are user-defined** (see ADR 002).

3. **Apply / Changesets**  
   - `nrv.apply.changeset(ops).dryRun()`  
   - `nrv.apply.changeset(ops).commit(msg)`  
   - Operations: `{ type: "file"|"dir"|"patch", path, content?, find?, replace? }`

4. **Specs / Contracts / Docs**  
   - `nrv.spec.lint(text)`  
   - `nrv.spec.align(text, opts)`  
   - `nrv.spec.detectGaps(specs[])`  
   - `nrv.spec.computeOverlap(specs[])`  
   - `nrv.doc.normalizeHeadings(md)`, `nrv.doc.syncFromSpecsToCode(targets)`

5. **Planning / Contracts**  
   - `nrv.plan.deriveWorkspaceFromIntent(nerveMd)`  
   - `nrv.contracts.deriveCliSurface(nerveMd|spec)`

6. **Quality / Validation**  
   - `nrv.qa.runTests(profile)`  
   - `nrv.qa.cargoMetadata(root)`  
   - `nrv.qa.validateScaffold(plan, tree)`  
   - `nrv.qa.genReadmeOutline(nerveMd)`  

7. **Version Control / Provenance**  
   - `nrv.vcs.savepoint(label)`  
   - `nrv.vcs.rollback(label)`  
   - `nrv.vcs.diff()`  

8. **Formatting**  
   - `nrv.fmt.markdown(text)`  
   - `nrv.fmt.rustfmtProject(root)`  
   - `nrv.fmt.prettierProject(root)`

9. **UI / Logging**  
   - `nrv.ui.step(name).start().ok().fail().warn()`  
   - `nrv.ui.diffPreview(patch)`  
   - `nrv.ui.table(data)`

10. **Time / Env / Config**  
    - `nrv.time.budget(ms)`  
    - `nrv.env.get(key)`  
    - `nrv.config.load(path)`  

### Language Surface Notes

- **JavaScript/TypeScript**  

  ```ts
  nrv.file.read("Nerve.md");
  nrv.llm("my-model").prompt(["Hello"]).text();
  ```

- **Rust (front-end syntax)**

  ```rust
  fn main() {
      let spec = nrv.file.read("Nerve.md");
      let out = nrv.llm("model").prompt(vec!["hello"]).text();
  }
  ```

- **TS Typings Example**

  ```ts
  declare const nrv: {
    file: { read(path:string):string; write(path:string,content:string):void; exists(path:string):boolean; },
    llm: (model:string) => { prompt(input:any[]): { text():string; json():any; bool():boolean; edits():any[] } },
    // ...
  }
  ```

## Consequences

**Pros**

- Single, explicit namespace (`nrv`) makes magic clear and auditable.
- Easier to expand without polluting globals.
- Aligns with ADR 002 (all prompts user-defined).
- Works consistently across JS/TS/Rust.

**Cons**

- Slightly more typing than bare globals (`nrv.file.read()` vs `read()`).
- Harder for beginners compared to `print("hello")`.
- Existing ADR 001 examples need updating.

**Notes**

- A sugar layer (`use nrv::*;`) MAY be added in the future for Rust, or `globalThis.print = nrv.ui.print` for JS, but only as **opt-in**.

## Alternatives Considered

- **Global stdlib functions (ADR 001)**: rejected due to clutter, hiddenness, collision risk.
- **Hybrid (few globals + `nrv` object)**: rejected for now; prefer consistency.

## References

- ADR 001: Standard Library in Global Scope (superseded)
- ADR 002: User-Defined LLM Interactions Only
- Internal `.specs` and `.plans` scaffolding design
