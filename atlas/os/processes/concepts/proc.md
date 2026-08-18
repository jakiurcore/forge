# Linux `/proc`

`/proc` is a virtual filesystem that exposes kernel data structures as files. Each running process has a directory named after its PID.

## Common files

| File | Contents |
|---|---|
| `/proc/<pid>/status` | Human-readable status: PID, PPID, UID, GID, state, memory, threads. |
| `/proc/<pid>/stat` | One-line raw stat fields, including state and PPID. |
| `/proc/<pid>/cmdline` | Command line arguments separated by NUL bytes. |
| `/proc/<pid>/fd/` | Directory of symlinks to open file descriptors. |
| `/proc/<pid>/fdinfo/` | Metadata for each file descriptor. |
| `/proc/<pid>/maps` | Memory map. |
| `/proc/<pid>/smaps` | Detailed memory statistics. |

## Example

```bash
cat /proc/self/status
cat /proc/self/cmdline | tr '\0' ' '
ls -l /proc/self/fd
```

## Implementation

Forge parses these files in `crates/forge-process/src/info.rs` and `crates/forge-process/src/fds.rs`.
