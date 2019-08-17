# 200k+ RPS Hello World

- 239k RPS is the result on hello world with routing.
- Not bad actually. But Go made 1M RPS on this task.

- Fastest I could get with HTTP routing on Rust.
- Actix is the fastest of all Rust HTTP frameworks I've tested so far.
- It performs better than Hyper. And it's also based on Tokio.
- Maybe Tokio is the problem.
- Libuv can handle 1m RPS.


