# nrv foundation primitives (file, dir, exec)

## Purpose
- Expose batteries for managing project files, directories, and command execution while keeping
  behaviour explicit and overridable.
- Provide enough capability for scripted LLM workflows to scaffold and maintain codebases.
- Maintain thin, common-sense guardrails: defaults fail safe, but callers can override when needed.

## nrv.file
### Surface
- `read(path)` → `Result<FileContent, FileError>`
- `write(options)` → `Result<WriteOutcome, FileError>`
- `stat(path)` → `Result<FileStat, FileError>`
- `remove(path)` → `Result<(), FileError>` (no recursive delete).
- `exists(path)` → `Result<bool, FileError>`

### Behaviour
- Paths are evaluated relative to caller-provided root; absolute paths require explicit opt-in.
- Writes use temp-file + atomic rename; caller may disable via `strategy: WriteStrategy`.
- Supports UTF-8 files by default; other encodings require explicit flag.
- `WriteStrategy`
  - `Overwrite` (default)
  - `Append`
  - `Create` (fails if file already exists)
  - `OverwriteWithBackup { suffix: String }`
- Returns `WriteOutcome { bytes_written, created: bool, warnings }`.
- Errors: `FileError::Io`, `FileError::Encoding`, `FileError::Permission`, `FileError::Guardrail`.

## nrv.dir
### Surface
- `list(path, options)` → `Result<Vec<DirEntry>, DirError>`
- `ensure(path)` → `Result<DirOutcome, DirError>`
- `remove_empty(path)` → `Result<(), DirError>`
- `walk(path, depth?)` → iterator/generator with streaming results.

### Behaviour
- Guardrails prevent recursive delete or hidden dotfile operations unless opted in.
- `list` supports filters (files only, extensions, glob) and returns deterministic ordering.
- `ensure` creates intermediate directories; returns whether created or pre-existing.

## nrv.exec
### Purpose
- Run external commands (formatters, tests, codegen) in a controlled environment.

### Surface
- `run(command)` → `Result<ExecOutcome, ExecError>`
- `CommandOptions { program, args, cwd?, env?, timeout? }`
- `ExecOutcome { status, stdout, stderr, duration_ms }`

### Behaviour
- Defaults to capturing stdout/stderr; streaming APIs may be added later.
- Implicit shell is disallowed; callers must specify command/args explicitly unless `use_shell` flag set.
- Timeout enforced by default (e.g., 10 minutes) with override.
- Exit codes propagate; non-zero returns `ExecError::NonZeroExit { code, stdout, stderr }`.
- Guardrails allow allowlisting commands, environment variables, or sandboxes when host requires.

## Interplay with `nrv.apply`
- Diff application lives in `.specs/12_nrv_apply.md`; file writes should integrate with diff outcomes for
  audit trails.

## Implementation Notes
- Rust/TS modules MUST share the same error enums and options where possible.
- Provide dry-run hooks (no-op writes) for plan/explain tooling.
- Allow callers to override root directory (e.g., repo checkout) while blocking accidental `/` operations.

## Testing Strategy
- Unit tests for guardrail enforcement, path validation, command timeout.
- Integration tests that operate on temp directories and ensure atomic behaviour.
- BDD coverage mirroring major flows (read/write/list/exec) with placeholders until implementation lands.
