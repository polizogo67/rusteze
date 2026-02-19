### v0.0.1

Comments
- Struct enum variants — Command::Get { key: String } vs the simpler Value::Int(i64)
- Separation of concerns — parsing logic lives in Command::parse(), execution lives in main.rs
- pub — I need this to use things across modules

This pays off immediately: when I add new commands like INCR or KEYS, I just add a variant to the enum and a match arm in each place. And later when I add networking, the TCP server will use the exact same Command::parse() function — no rewriting needed.
