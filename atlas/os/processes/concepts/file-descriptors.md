# File Descriptors

A file descriptor (FD) is a small non-negative integer that the kernel uses to identify an open file or I/O resource.

## Standard descriptors

| FD | Name | Default target |
|---|---|---|
| 0 | stdin | keyboard / terminal input |
| 1 | stdout | terminal output |
| 2 | stderr | terminal error output |

## Other descriptors

Additional FDs can represent:

- regular files
- pipes
- sockets
- terminals
- anonymous inodes (eventfd, epoll, inotify, etc.)

## Inspection

Forge can list a process's open descriptors:

```bash
forge process fds <pid>
```

This reads `/proc/<pid>/fd/` and classifies each target.
