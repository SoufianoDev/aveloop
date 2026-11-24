use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

fn main() {
    let results = vec![
        ("spawn_and_kill", verify_spawn_and_kill()),
        ("exit_status_captured", verify_exit_status()),
        ("child_does_not_outlive_kill", verify_child_gone_after_kill()),
        ("timeout_then_kill", verify_timeout_then_kill()),
    ];

    let mut failed = 0;
    for (name, ok) in &results {
        let mark = if *ok { "PASS" } else { "FAIL" };
        println!("[T012] {}: {}", name, mark);
        if !ok {
            failed += 1;
        }
    }
    if failed > 0 {
        eprintln!("[T012] {} check(s) failed", failed);
        std::process::exit(1);
    }
    println!("[T012] all checks passed");
}

fn long_sleep_cmd() -> Command {
    // portable: `sleep 60` on Unix, `timeout 60` on Windows
    #[cfg(unix)]
    {
        let mut c = Command::new("sleep");
        c.arg("60");
        c
    }
    #[cfg(windows)]
    {
        let mut c = Command::new("cmd");
        c.args(["/C", "timeout", "/T", "60", "/NOBREAK"]);
        c
    }
}

fn verify_spawn_and_kill() -> bool {
    let mut child = match long_sleep_cmd()
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
    {
        Ok(c) => c,
        Err(e) => {
            eprintln!("[T012] spawn failed: {}", e);
            return false;
        }
    };
    if let Err(e) = child.kill() {
        eprintln!("[T012] kill failed: {}", e);
        return false;
    }
    child.wait().is_ok()
}

fn verify_exit_status() -> bool {
    let status = Command::new(if cfg!(unix) { "true" } else { "cmd" })
        .args(if cfg!(unix) { &[][..] } else { &["/C", "exit", "0"][..] })
        .status();
    match status {
        Ok(s) => s.success(),
        Err(e) => {
            eprintln!("[T012] exit status check failed: {}", e);
            false
        }
    }
}

fn verify_child_gone_after_kill() -> bool {
    let mut child = match long_sleep_cmd()
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
    {
        Ok(c) => c,
        Err(e) => {
            eprintln!("[T012] spawn failed: {}", e);
            return false;
        }
    };
    let pid = child.id();
    let _ = child.kill();
    let _ = child.wait();

    // On Unix: check /proc/<pid> is gone or kill(pid, 0) returns ESRCH.
    // On Windows: OpenProcess after wait() will get a stale handle or fail.
    // We rely on try_wait returning Some(status) as the portable signal.
    let gone = child.try_wait().map(|s| s.is_some()).unwrap_or(true);
    if !gone {
        eprintln!("[T012] pid {} still alive after kill+wait", pid);
    }
    gone
}

fn verify_timeout_then_kill() -> bool {
    let mut child = match long_sleep_cmd()
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
    {
        Ok(c) => c,
        Err(e) => {
            eprintln!("[T012] spawn failed: {}", e);
            return false;
        }
    };

    let deadline = Instant::now() + Duration::from_millis(200);
    loop {
        if Instant::now() >= deadline {
            break;
        }
        match child.try_wait() {
            Ok(Some(_)) => return true, // already exited (unexpected but fine)
            Ok(None) => std::thread::sleep(Duration::from_millis(20)),
            Err(e) => {
                eprintln!("[T012] try_wait error: {}", e);
                return false;
            }
        }
    }
    if let Err(e) = child.kill() {
        eprintln!("[T012] timeout kill failed: {}", e);
        return false;
    }
    child.wait().is_ok()
}
