# TETRUST

![A game of Tetrust](/public/tetrust.gif)

A Rust implementation of Tetris – in your terminal!

Tetrust uses minimal dependencies, but I roped in [Ratatui](https://ratatui.rs/) and [crossterm](https://github.com/crossterm-rs/crossterm) to avoid getting bogged down in the specifics of different terminals.

## Gameplay Features

This isn't intended to be a one-for-one clone of any particular Tetris release. It implements my own take the game, often pulling details from memory or just busking it.

- [x] All seven classic block types
- [x] Blocks fall under gravity
- [x] Left, right and downward movement
- [x] Four rotation states for all blocks
- [x] Clearance of completed lines
- [x] Scoring
- [x] Acceleration of gravity as points accumulate
- [x] Next block preview
- [x] Game over screen

## Who (or What) Did What?

| Task                                                                    | Angus | Claude |
| ----------------------------------------------------------------------- | :---: | :----: |
| Project architecture                                                    |  ✅   |        |
| Game logic                                                              |  ✅   |        |
| Rotation system (including manual implementations of I, J and O blocks) |  ✅   |        |
| Rotations of L, S, T and Z blocks                                       |       |   ✅   |
| Unit tests                                                              |  ✅   |   ✅   |
| README (including en dash use)                                          |  ✅   |        |

## Installation

```
git clone https://github.com/AngusGMorrison/tetrust.git
cd tetrust
cargo run
```
