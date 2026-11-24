//! T016 — Windows compiler-debugger pair selection probe.
//!
//! Exercises the pair rule from `spec.md` §7.2 with synthetic tool directories
//! on any host, then optionally reports the real host selection.
//!
//! ```text
//! cargo run --bin t016_verify             # four scenarios + explicit-config checks
//! cargo run --bin t016_verify -- --detect # real host selection
//! ```

use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::time::{SystemTime, UNIX_EPOCH};

#[cfg(windows)]
const EXE: &str = ".exe";
#[cfg(not(windows))]
const EXE: &str = "";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Family {
    Mingw,
    Msvc,
}

impl Family {
    fn name(self) -> &'static str {
        match self {
            Family::Mingw => "mingw",
            Family::Msvc => "msvc",
        }
    }

    fn compiler(self) -> String {
        match self {
            Family::Mingw => format!("gcc{EXE}"),
            Family::Msvc => format!("cl{EXE}"),
        }
    }

    fn debugger(self) -> String {
        match self {
            Family::Mingw => format!("gdb{EXE}"),
            Family::Msvc => format!("cdb{EXE}"),
        }
    }

    fn parse(value: &str) -> Option<Self> {
        match value.trim().to_ascii_lowercase().as_str() {
            "mingw" | "mingw-w64" | "gcc" | "gdb" => Some(Family::Mingw),
            "msvc" | "cdb" | "dbgeng" => Some(Family::Msvc),
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Selection {
    Pair(Family, PathBuf, PathBuf),
    None,
    ExplicitUnavailable(Family),
}

impl Selection {
    fn label(&self) -> String {
        match self {
            Selection::Pair(family, _, _) => format!("selection={}", family.name()),
            Selection::None => "selection=none".to_string(),
            Selection::ExplicitUnavailable(family) => {
                format!("selection=explicit-unavailable:{}", family.name())
            }
        }
    }
}

fn locate(roots: &[PathBuf], exe: &str) -> Option<PathBuf> {
    roots
        .iter()
        .map(|root| root.join(exe))
        .find(|p| p.is_file())
}

/// A family is selectable only when compiler and debugger are both present.
fn pair(roots: &[PathBuf], family: Family) -> Option<(PathBuf, PathBuf)> {
    let compiler = locate(roots, &family.compiler())?;
    let debugger = locate(roots, &family.debugger())?;
    Some((compiler, debugger))
}

fn select(roots: &[PathBuf], explicit: Option<Family>) -> Selection {
    if let Some(family) = explicit {
        return match pair(roots, family) {
            Some((compiler, debugger)) => Selection::Pair(family, compiler, debugger),
            None => Selection::ExplicitUnavailable(family),
        };
    }
    for family in [Family::Mingw, Family::Msvc] {
        if let Some((compiler, debugger)) = pair(roots, family) {
            return Selection::Pair(family, compiler, debugger);
        }
    }
    Selection::None
}

fn split_roots(value: &str) -> Vec<PathBuf> {
    value
        .split(if cfg!(windows) { ';' } else { ':' })
        .filter(|s| !s.trim().is_empty())
        .map(PathBuf::from)
        .collect()
}

fn real_roots() -> Vec<PathBuf> {
    if let Ok(value) = env::var("AVELOOP_TOOLCHAIN_ROOTS") {
        return split_roots(&value);
    }
    #[allow(unused_mut)] // only Windows appends the Debuggers/MSYS2 roots
    let mut roots = env::var("PATH")
        .map(|p| split_roots(&p))
        .unwrap_or_default();
    #[cfg(windows)]
    for extra in [
        r"C:\Program Files (x86)\Windows Kits\10\Debuggers\x64",
        r"C:\Program Files\Windows Kits\10\Debuggers\x64",
        r"C:\msys64\mingw64\bin",
    ] {
        roots.push(PathBuf::from(extra));
    }
    roots
}

fn scenario_root(base: &Path, name: &str, files: &[&str]) -> PathBuf {
    let dir = base.join(name);
    fs::create_dir_all(&dir).expect("create scenario dir");
    for file in files {
        fs::write(dir.join(file), b"").expect("write stub tool");
    }
    dir
}

fn scratch_dir() -> PathBuf {
    let stamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock")
        .as_nanos();
    let dir = env::temp_dir().join(format!("aveloop-t016-{}-{stamp}", std::process::id()));
    fs::create_dir_all(&dir).expect("create scratch dir");
    dir
}

fn check(name: &str, roots: &[PathBuf], explicit: Option<Family>, expected: &str) -> bool {
    let actual = select(roots, explicit).label();
    let ok = actual == expected;
    println!(
        "{:<34} expect={:<32} actual={:<32} {}",
        name,
        expected,
        actual,
        if ok { "PASS" } else { "FAIL" }
    );
    ok
}

fn main() -> ExitCode {
    if env::args().any(|arg| arg == "--detect") {
        let explicit = env::var("AVELOOP_WINDOWS_TOOLCHAIN")
            .ok()
            .and_then(|v| Family::parse(&v));
        let selection = select(&real_roots(), explicit);
        println!("{}", selection.label());
        if let Selection::Pair(family, compiler, debugger) = &selection {
            println!("pair={}", family.name());
            println!("compiler={}", compiler.display());
            println!("debugger={}", debugger.display());
        }
        return ExitCode::SUCCESS;
    }

    let base = scratch_dir();
    let gcc = format!("gcc{EXE}");
    let gdb = format!("gdb{EXE}");
    let cl = format!("cl{EXE}");
    let cdb = format!("cdb{EXE}");

    let mingw_only = scenario_root(&base, "mingw-only", &[&gcc, &gdb]);
    let msvc_only = scenario_root(&base, "msvc-only", &[&cl, &cdb]);
    let both = scenario_root(&base, "both", &[&gcc, &gdb, &cl, &cdb]);
    let neither = scenario_root(&base, "neither", &[]);
    let compiler_only = scenario_root(&base, "compiler-without-debugger", &[&cl]);
    let debugger_only = scenario_root(&base, "debugger-without-compiler", &[&cdb]);

    let mut passed = 0;
    let mut total = 0;
    let mut run = |name: &str, roots: &[PathBuf], explicit: Option<Family>, expected: &str| {
        total += 1;
        if check(name, roots, explicit, expected) {
            passed += 1;
        }
    };

    run(
        "scenario 1 mingw only",
        std::slice::from_ref(&mingw_only),
        None,
        "selection=mingw",
    );
    run("scenario 2 msvc only", &[msvc_only], None, "selection=msvc");
    run(
        "scenario 3 both prefer mingw",
        std::slice::from_ref(&both),
        None,
        "selection=mingw",
    );
    run("scenario 4 neither", &[neither], None, "selection=none");
    run(
        "explicit msvc wins over default",
        std::slice::from_ref(&both),
        Some(Family::Msvc),
        "selection=msvc",
    );
    run(
        "explicit msvc never substituted",
        std::slice::from_ref(&mingw_only),
        Some(Family::Msvc),
        "selection=explicit-unavailable:msvc",
    );
    run(
        "compiler without debugger",
        &[compiler_only],
        None,
        "selection=none",
    );
    run(
        "debugger without compiler",
        &[debugger_only],
        None,
        "selection=none",
    );

    let _ = fs::remove_dir_all(&base);
    println!("\n{passed}/{total} checks passed");
    if passed == total {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}
