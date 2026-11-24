# T016–T019: gate closure evidence

Evidence for the three gaps recorded in the T015 feasibility report. All results come
from GitHub Actions on real runners (`windows-2022`, `macos-latest`) and are reproducible
from the commands in each section. No product code changed in these tasks.

| Gap (T015) | Task | Status |
|---|---|---|
| G1 Windows debugger backend undecided | T016, T017 | Closed: policy is user-confirmed and implemented; detected pair exercised live |
| G2 macOS debugger depth unverified | T018 | Closed: full LLDB session on a real runner |
| G3 macOS/Windows GPUI runtime unverified | T019 | Closed: window opened and exited on both runners |

## T016 — Windows compiler-debugger pair selection

**Policy (USER-CONFIRMED 2026-11-24, `spec.md` §7.2–§7.3, `plan.md` §3.3–§3.4):**

1. Only MinGW-w64 GCC/G++ with GDB complete and compatible → select that pair.
2. Only MSVC with CDB/DbgEng complete and compatible → select that pair.
3. Both complete → prefer MinGW-w64 GCC/G++ with GDB.
4. Neither complete → report that no supported combination was found; install nothing.

A compiler without its compatible debugger, or a debugger without its compatible
compiler, is not a valid pair. Explicit project or user configuration wins and is never
silently substituted.

**Implementation:** `verification/T016/t016_verify.rs`, registered as an explicit
`[[bin]]` target in `Cargo.toml` (commit `0f2d6e4`). `--detect` reports the real host
selection as a stable `selection=` line.

**CI:** `.github/workflows/t016-toolchain-pair.yml`, run
[`45304616166`](https://github.com/SoufianoDev/aveloop/actions/runs/45304616166) —
success, 8/8 checks:

| Check | Expected | Result |
|---|---|---|
| Scenario 1: only MinGW-w64 + GDB | `selection=mingw` | PASS |
| Scenario 2: only MSVC + CDB | `selection=msvc` | PASS |
| Scenario 3: both complete | `selection=mingw` | PASS |
| Scenario 4: neither complete | `selection=none` | PASS |
| Explicit MSVC with both present | `selection=msvc` | PASS |
| Explicit MSVC with only MinGW present | `selection=explicit-unavailable:msvc` | PASS |
| Compiler without debugger | `selection=none` | PASS |
| Debugger without compiler | `selection=none` | PASS |

**Real host detection** on `windows-2022`:

```text
selection=mingw
pair=mingw
compiler=C:\mingw64\bin\gcc.exe
debugger=C:\mingw64\bin\gdb.exe
```

The synthetic scenarios exercise selection logic; the detection run exercises real path
and extension handling on the target platform.

## T017 — Windows live debugger session with the detected pair

**CI:** `.github/workflows/t017-live-debugger.yml`, run
[`45305339035`](https://github.com/SoufianoDev/aveloop/actions/runs/45305339035) —
`Windows x86_64 live debugger` success. The job resolves the pair with the T016 probe,
then drives whichever pair the policy selects (MinGW-w64 + GDB on this runner).

```text
Breakpoint 1 at 0x140001460: file probe.c, line 4.
Thread 1 hit Breakpoint 1, main () at probe.c:4
4       int local_int = 42;
7       printf("STDOUT_MARK %d\n", local_int);
$1 = 42
$2 = {value = 7, pad = 0}
#0  main () at probe.c:7
STDERR_MARK 7
STDOUT_MARK 42
DONE 43
[Inferior 1 (process 7780) exited normally]
```

Verified: launch, breakpoint, three source-level steps, local variable, aggregate with
children, pointer dereference, stack frame, both output streams, normal termination.

**MSVC + CDB (explicit configuration):** a dedicated job was added, run, and then
removed. In runs `45306494934` and `45308438040` it verified that explicit
`AVELOOP_WINDOWS_TOOLCHAIN=msvc` is honoured without substitution, that `cl` builds the
fixture, and that CDB launches the probe with the child's streams appearing in the step
log. Scripted assertions could not be made reliable: CDB writes prompts and `dv`/`dx`/`k`
output to the console buffer, which shell redirection does not capture on `windows-2022`,
and the child's own output reached the captured file in one run but not the next. The
job was removed from CI rather than left flaky. **Residual:** the MSVC + CDB inspection
path is not verified end to end; MinGW-w64 + GDB is the verified Windows path and the
policy's default preference.

## T018 — macOS LLDB live session

**CI:** run
[`45305339035`](https://github.com/SoufianoDev/aveloop/actions/runs/45305339035) —
`macOS arm64 live debugger` success, Apple Clang build plus a scripted `lldb --batch`
session:

```text
frame #0: ... probe`main at probe.c:4
frame #0: ... probe`main at probe.c:5
frame #0: ... probe`main at probe.c:6
frame #0: ... probe`main at probe.c:7
(int) local_int = 42
(Child) (value = 7, pad = 0)
frame #0: ... probe`main at probe.c:7
STDOUT_MARK 42
STDERR_MARK 7
DONE 43
Process exited with status = 0
```

Verified: launch, breakpoint, four source-level steps, local variable, aggregate with
children, pointer dereference, backtrace, both output streams, clean exit.

## T019 — GPUI runtime launch on Windows and macOS

**CI:** `.github/workflows/t019-gpui-launch.yml`, run
[`45304616214`](https://github.com/SoufianoDev/aveloop/actions/runs/45304616214) and
re-run `45308437334` — both jobs success. The job runs the existing `t008_verify` probe
with `AVELOOP_PROBE_EXIT=1`, which opens a real GPUI window, prints
`[T008] Verification window opened successfully.`, and quits through the normal path.

This is a runtime launch, not `cargo check`: the window is created on the platform
backend and the event loop starts and stops cleanly.

## Reproduce

```bash
cargo run --bin t016_verify              # pair-selection checks
cargo run --bin t016_verify -- --detect  # real host selection

gh workflow run "T016 Windows toolchain pair detection"
gh workflow run "T017-T018 live debugger sessions"
gh workflow run "T019 GPUI runtime launch"
```

## Limitations

- Pair selection is validated with synthetic tool directories plus one real host per
  runner image; other Windows installations may place tools elsewhere. Discovery roots
  are `PATH`, the Windows Kits debugger directories, and `C:\msys64\mingw64\bin`.
- The MSVC + CDB scripted variable and stack assertions are not captured under
  redirection on `windows-2022`. MinGW-w64 + GDB is the verified Windows path and the
  policy's default preference.
- Debugger logs merge stdout and stderr; separation itself is proven by the T012
  pipe-based capture checks, not by these sessions.
- The Wayland clipboard read remains unverified (no `wl-clipboard` on the runner), as
  recorded in T008.
