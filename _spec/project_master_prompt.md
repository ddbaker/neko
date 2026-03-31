# neko porting project

"neko" is a cat that chases the mouse cursor across the screen, an app written in the late 1980s and ported for many platforms.
This project aims to port existing "neko" program for Rust + Bevy platform.

## Programing Language and Gameing Engine to be used

1. Use Rust language 100%
2. Use Bevy Gaming engine, no other framework is used.

## MCP

* Always use serena MCP for read/edit files.
* Use context7 MCP proactively to make sure Codex uses the right Bevy APIs.

## Phased project management

phase-1: Analyze Go-language written "neko" source code repository [neko](D:\devel\inetsrc\neko) as our master reference.
 This project goal is to do "porting" Go-language version "neko" to Rust-language with Bevy game engine.

phase-2: Plan how bevy game engine can be used for our Rust version "neko".

phase-3: Write down detailed porting plan to a markdown file for verification.

phase-4: Map the porting plan (phase-3) into an implementation checklist.

phase-5: Proceed implementation with logs and tests

phase-6: Run `cargo build` and `cargo check`, make sure they are all error free.
