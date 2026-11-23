# T009 — Platform C/C++ Toolchain: C17 and C++20 Debug Builds

- **Task:** T009 (`specs/001-ave-loop-alpha/tasks.md` line 44)
- **Issue:** #9
- **Status:** VERIFIED on Linux x86_64 (GCC/G++ 14.2.0). Windows and macOS **NOT VERIFIED** — no such host in this environment; those rows record published vendor documentation only.
- **Date:** 2026-11-23
- **Branch:** `feat/gpui-kit-fast`
- **Governing requirements:** `spec.md` §7.1–§7.4, `plan.md` §3.3

## 1. Environment

| Component | Observed |
|---|---|
| `gcc --version` | `gcc (Ubuntu 14.2.0-19ubuntu2) 14.2.0` |
| `g++ --version` | `g++ (Ubuntu 14.2.0-19ubuntu2) 14.2.0` |
| Target triple (`gcc -dumpmachine`) | `x86_64-linux-gnu` |
| Linker (`ld --version`) | `GNU ld (GNU Binutils for Ubuntu) 2.44` |
| C runtime (`ldd --version`) | `ldd (Ubuntu GLIBC 2.41-6ubuntu1.2) 2.41` |
| Debugger (`gdb --version`) | `GNU gdb (Ubuntu 16.2-8ubuntu1) 16.2` |
| Kernel | Linux x86_64 (Ubuntu) |

No compiler, linker, runtime, or package was installed for this task. The probes were built in `/tmp/opencode/t009` and are not committed.

## 2. Standards selection

Both builds select the standard explicitly, as `spec.md` §7.1 requires, and use the debug profile from §7.4 (`-g -O0`):

```
gcc -std=c17  -g -O0 -Wall -Wextra -pedantic-errors -o c17_probe  c17_probe.c
g++ -std=c++20 -g -O0 -Wall -Wextra -pedantic-errors -o cxx20_probe cxx20_probe.cpp
```

Both compile with exit status 0 and no diagnostics.

C17 probe:

```c
#include <stdalign.h>
#include <stdatomic.h>
#include <stdio.h>
#include <stdlib.h>

_Static_assert(__STDC_VERSION__ == 201710L, "C17 required");

struct Point { int x, y; };

int main(void) {
    struct Point p = { .x = 1, .y = 2 };
    _Atomic int counter = 0;
    counter += p.x + p.y;
    void *mem = aligned_alloc(16, 32);
    int ok = mem != NULL;
    free(mem);
    printf("C17 ok: __STDC_VERSION__=%ld atomic=%d\n", (long)__STDC_VERSION__, (int)counter);
    return ok ? 0 : 1;
}
```

C++20 probe:

```cpp
#include <compare>
#include <concepts>
#include <cstdio>
#include <format>
#include <ranges>
#include <span>
#include <vector>

template <std::integral T>
constexpr T twice(T v) { return v * 2; }

struct Version {
    int major, minor;
    auto operator<=>(const Version &) const = default;
};

consteval int compile_time() { return 42; }

int main() {
    static_assert(__cplusplus == 202002L);
    std::vector<int> v{1, 2, 3, 4};
    auto even = v | std::views::filter([](int n) { return n % 2 == 0; });
    std::span<int> s{v};
    Version a{1, 2}, b{1, 3};
    std::printf("%s\n", std::format("C++20 ok: __cplusplus={} twice={} ct={} lt={} span={} first_even={}",
                                    (long)__cplusplus, twice(21), compile_time(), a < b, s.size(), *even.begin())
                            .c_str());
    return 0;
}
```

Observed output:

```
C17 ok: __STDC_VERSION__=201710 atomic=3
C++20 ok: __cplusplus=202002 twice=42 ct=42 lt=true span=4 first_even=2
```

`__STDC_VERSION__=201710` and `__cplusplus=202002` confirm the requested language modes rather than a compiler default. Both binaries exit 0.

## 3. Debug information

```
$ file c17_probe cxx20_probe
c17_probe:   ELF 64-bit LSB pie executable, x86-64, ... with debug_info, not stripped
cxx20_probe: ELF 64-bit LSB pie executable, x86-64, ... with debug_info, not stripped
```

`readelf -S` reports 6 debug sections in the C binary and 7 in the C++ binary. GDB 16.2 resolves source lines, sets breakpoints, and prints variables:

```
$ gdb -batch -ex "info line main" -ex "info sources" ./c17_probe
Line 10 of "c17_probe.c" starts at address 0x11a9 <main> and ends at 0x11b5 <main+12>.
/tmp/opencode/t009/c17_probe.c, /usr/include/stdio.h, /usr/include/stdlib.h

$ gdb -batch -ex "break c17_probe.c:14" -ex "run" -ex "print counter" -ex "print p" ./c17_probe
$1 = 3
$2 = {x = 1, y = 2}

$ gdb -batch -ex "break cxx20_probe.cpp:28" -ex "run" -ex "print v.size()" -ex "print a.major" ./cxx20_probe
$1 = 4
$2 = 1
```

Debug builds are therefore usable as debuggee input for T010.

## 4. GNU extensions are not enabled by default

`spec.md` §7.1 requires that GNU extensions are not silently enabled. With `-std=c17` and `-std=c++20` (not `gnu17`/`gnu++20`), the GNU `typeof` keyword is rejected:

```
$ gcc -std=c17 -pedantic-errors -o /dev/null gnu_ext.c
gnu_ext.c:1:18: error: implicit declaration of function 'typeof' [-Wimplicit-function-declaration]
$ g++ -std=c++20 -pedantic-errors -o /dev/null gnu_ext.cpp
gnu_ext.cpp:1:25: error: 'typeof' was not declared in this scope; did you mean 'typedef'?
```

Both commands exit 1. A project that needs an extension must pass `-std=gnu17`/`-std=gnu++20` or the specific flag explicitly, which matches the spec.

## 5. C++20 feature coverage on GCC 14.2

The probe exercises language features (`concepts`, `consteval`, three-way comparison, designated initializers, `std::span`) and library features (`<format>`, `<ranges>`). All compile and run.

This is a tested subset, not a conformance claim. Per `spec.md` §7.1, AveLoop must report compiler diagnostics rather than assert universal C++20 conformance, and it must maintain a tested feature matrix. GCC's own status page states that GCC has "almost full support" for C++20, that the ABI of C++20 features was not stable until GCC 16, and that modules remain experimental (see §7).

### 5.1 Modules

Modules work on this toolchain, but only behind the experimental flag and with the GCC 14 spelling `-fmodules-ts`:

```
$ g++ -std=c++20 -fmodules-ts -x c++ -c mod.cppm -o mod.o          # exit 0
$ g++ -std=c++20 -fmodules-ts -c mod_main.cpp -o mod_main.o        # exit 0
$ g++ -std=c++20 -fmodules-ts mod_main.o mod.o -o mod_probe        # exit 0
$ ./mod_probe
42
```

`-fmodules` (the spelling used by current GCC releases) is rejected by 14.2:

```
g++: error: unrecognized command-line option '-fmodules'; did you mean '-Mmodules'?
```

Consequence for AveLoop: module builds must be an explicit per-project configuration and must not be assumed portable across GCC versions.

## 6. Linker and runtime prerequisites (Linux)

`gcc`/`g++` drive `ld` and add the start files and default libraries automatically:

```
$ gcc -print-prog-name=ld
ld
$ gcc -print-file-name=crt1.o
/usr/lib/gcc/x86_64-linux-gnu/14/../../../x86_64-linux-gnu/crt1.o
$ g++ -print-file-name=libstdc++.so.6
/usr/lib/gcc/x86_64-linux-gnu/14/../../../x86_64-linux-gnu/libstdc++.so.6
```

Required at link time: `binutils` (`ld`), the glibc development files (`crt1.o`, `crti.o`, `crtn.o`, `libc.so`), and for C++ the GCC runtime (`libstdc++.so`, `libgcc_s`). Required at run time:

```
$ readelf -d ./cxx20_probe | grep NEEDED
 (NEEDED) Shared library: [libstdc++.so.6]
 (NEEDED) Shared library: [libm.so.6]
 (NEEDED) Shared library: [libgcc_s.so.1]
 (NEEDED) Shared library: [libc.so.6]

$ ldd ./c17_probe
 libc.so.6 => /lib/x86_64-linux-gnu/libc.so.6
 /lib64/ld-linux-x86-64.so.2
```

Both binaries are position-independent executables whose interpreter is `/lib64/ld-linux-x86-64.so.2`. A C++ binary will not start on a host without a compatible `libstdc++.so.6`; a C binary needs only the loader and glibc. `libstdc++` uses the Itanium C++ ABI with the dual-ABI `std::string`/`std::list` split, so objects built with different `-fabi-version` or `_GLIBCXX_USE_CXX11_ABI` values are not interchangeable.

## 7. External research (documentation, not local verification)

Sources read for the Windows and macOS rows and for the conformance caveats. Nothing in this section was executed on this host.

| Claim | Source |
|---|---|
| GCC has almost full C++20 support; C++20 ABI stable only from GCC 16; modules experimental, enabled by `-fmodules` | <https://gcc.gnu.org/projects/cxx-status.html> |
| GCC 14.2 released 2024-08-01; GCC 14 improved experimental C++20 library support (`std::chrono::parse`, Unicode in `std::format`) | <https://gcc.gnu.org/gcc-14/> and <https://gcc.gnu.org/gcc-14/changes.html> |
| `-std=c17`/`c18`/`iso9899:2017` is ISO C17, identical to C11 apart from defect corrections, supported to the same extent as C11 | <https://gcc.gnu.org/onlinedocs/gcc/C-Dialect-Options.html> |
| GDB implements the Debug Adapter Protocol with documented extensions (`launch`/`attach` parameters such as `program`) | <https://sourceware.org/gdb/current/onlinedocs/gdb/Debug-Adapter-Protocol.html> |
| libstdc++ uses the Itanium C++ ABI; `-fabi-version` and `_GLIBCXX_USE_CXX11_ABI` change binary compatibility | <https://gcc.gnu.org/onlinedocs/libstdc++/manual/abi.html> |
| MSVC C++20 core language features are listed as supported from VS 2019 16.7 and complete in VS 2022 17.x | <https://learn.microsoft.com/en-us/cpp/overview/visual-cpp-language-conformance> |
| Clang C++20 support is marked "Partial"; Apple Clang is derived from Clang | <https://clang.llvm.org/cxx_status.html> |

## 8. Platform verification status

| Platform | Toolchain | C17 | C++20 | Debug info | Linker/runtime prerequisites | Status |
|---|---|---|---|---|---|---|
| Linux x86_64 | GCC/G++ 14.2.0, ld 2.44, glibc 2.41 | PASS | PASS | PASS (GDB reads lines and variables) | Verified locally | **VERIFIED** |
| Windows x86_64 | MSVC `cl.exe` or MinGW-w64 GCC/G++ | Not run | Not run | Not run | Not run | **NOT VERIFIED** |
| macOS arm64 | Apple Clang via `xcrun --find clang` | Not run | Not run | Not run | Not run | **NOT VERIFIED** |

No Windows or macOS host is available in this environment, and installing a cross-toolchain would not verify the platform default toolchain that AveLoop actually uses. Those rows stay open.

## 9. Reproduce

```sh
mkdir -p /tmp/opencode/t009 && cd /tmp/opencode/t009
# write c17_probe.c and cxx20_probe.cpp from §2
gcc -std=c17  -g -O0 -Wall -Wextra -pedantic-errors -o c17_probe  c17_probe.c
g++ -std=c++20 -g -O0 -Wall -Wextra -pedantic-errors -o cxx20_probe cxx20_probe.cpp
./c17_probe && ./cxx20_probe
file c17_probe cxx20_probe
gdb -batch -ex "info line main" ./c17_probe
readelf -d ./cxx20_probe | grep NEEDED
```
