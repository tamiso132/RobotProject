# Diff Details

Date : 2024-01-10 23:40:16

Directory /home/tom/projects/RobotProject

Total : 35 files,  2918 codes, 7 comments, 75 blanks, all 3000 lines

[Summary](results.md) / [Details](details.md) / [Diff Summary](diff.md) / Diff Details

## Files
| filename | language | code | comment | blank | total |
| :--- | :--- | ---: | ---: | ---: | ---: |
| [bind.sh](/bind.sh) | Shell Script | 1 | 0 | 0 | 1 |
| [build.rs](/build.rs) | Rust | 29 | 0 | 7 | 36 |
| [dobot/bluegen.h](/dobot/bluegen.h) | C++ | 3 | 0 | 0 | 3 |
| [dobot/camera/camera.c](/dobot/camera/camera.c) | C | 25 | 0 | 5 | 30 |
| [dobot/device/b_device.c](/dobot/device/b_device.c) | C | 70 | 0 | 19 | 89 |
| [dobot/device/device.c](/dobot/device/device.c) | C | 46 | 0 | 8 | 54 |
| [dobot/device/u_device.c](/dobot/device/u_device.c) | C | 103 | 14 | 20 | 137 |
| [dobot/device/u_s_device.c](/dobot/device/u_s_device.c) | C | 91 | 13 | 23 | 127 |
| [dobot/include/device.h](/dobot/include/device.h) | C | 25 | 9 | 17 | 51 |
| [dobot/include/utility.h](/dobot/include/utility.h) | C | 16 | 0 | 4 | 20 |
| [dobot/utility/utility.c](/dobot/utility/utility.c) | C | 43 | 0 | 5 | 48 |
| [src/cbinding/bindings.rs](/src/cbinding/bindings.rs) | Rust | 2,501 | 1 | 2 | 2,504 |
| [src/cbinding/mod.rs](/src/cbinding/mod.rs) | Rust | 18 | 1 | 5 | 24 |
| [src/image.rs](/src/image.rs) | Rust | 297 | 34 | 55 | 386 |
| [src/lib.rs](/src/lib.rs) | Rust | 2 | 0 | 1 | 3 |
| [src/main.rs](/src/main.rs) | Rust | 152 | 15 | 17 | 184 |
| [src/ordering.rs](/src/ordering.rs) | Rust | 85 | 2 | 10 | 97 |
| [src/position.rs](/src/position.rs) | Rust | 151 | 12 | 28 | 191 |
| [src/protocol/mod.rs](/src/protocol/mod.rs) | Rust | 400 | 61 | 102 | 563 |
| [/home/tomas/Projects/Rust/RobotProject/Cargo.lock](//home/tomas/Projects/Rust/RobotProject/Cargo.lock) | TOML | -384 | -2 | -53 | -439 |
| [/home/tomas/Projects/Rust/RobotProject/Cargo.toml](//home/tomas/Projects/Rust/RobotProject/Cargo.toml) | TOML | -13 | -1 | -7 | -21 |
| [/home/tomas/Projects/Rust/RobotProject/bind.sh](//home/tomas/Projects/Rust/RobotProject/bind.sh) | Shell Script | -1 | 0 | 0 | -1 |
| [/home/tomas/Projects/Rust/RobotProject/build.rs](//home/tomas/Projects/Rust/RobotProject/build.rs) | Rust | 0 | -29 | -7 | -36 |
| [/home/tomas/Projects/Rust/RobotProject/dobot/bluegen.h](//home/tomas/Projects/Rust/RobotProject/dobot/bluegen.h) | C++ | -3 | 0 | 0 | -3 |
| [/home/tomas/Projects/Rust/RobotProject/dobot/device/b_device.c](//home/tomas/Projects/Rust/RobotProject/dobot/device/b_device.c) | C | -70 | 0 | -19 | -89 |
| [/home/tomas/Projects/Rust/RobotProject/dobot/device/device.c](//home/tomas/Projects/Rust/RobotProject/dobot/device/device.c) | C | -46 | 0 | -8 | -54 |
| [/home/tomas/Projects/Rust/RobotProject/dobot/device/u_device.c](//home/tomas/Projects/Rust/RobotProject/dobot/device/u_device.c) | C | -103 | -14 | -20 | -137 |
| [/home/tomas/Projects/Rust/RobotProject/dobot/device/u_s_device.c](//home/tomas/Projects/Rust/RobotProject/dobot/device/u_s_device.c) | C | -96 | -33 | -27 | -156 |
| [/home/tomas/Projects/Rust/RobotProject/dobot/include/device.h](//home/tomas/Projects/Rust/RobotProject/dobot/include/device.h) | C | -24 | -9 | -16 | -49 |
| [/home/tomas/Projects/Rust/RobotProject/dobot/include/utility.h](//home/tomas/Projects/Rust/RobotProject/dobot/include/utility.h) | C | -16 | 0 | -4 | -20 |
| [/home/tomas/Projects/Rust/RobotProject/dobot/utility/utility.c](//home/tomas/Projects/Rust/RobotProject/dobot/utility/utility.c) | C | -43 | 0 | -5 | -48 |
| [/home/tomas/Projects/Rust/RobotProject/src/cbinding/mod.rs](//home/tomas/Projects/Rust/RobotProject/src/cbinding/mod.rs) | Rust | -18 | -1 | -4 | -23 |
| [/home/tomas/Projects/Rust/RobotProject/src/lib.rs](//home/tomas/Projects/Rust/RobotProject/src/lib.rs) | Rust | -2 | 0 | -1 | -3 |
| [/home/tomas/Projects/Rust/RobotProject/src/main.rs](//home/tomas/Projects/Rust/RobotProject/src/main.rs) | Rust | -17 | -12 | -7 | -36 |
| [/home/tomas/Projects/Rust/RobotProject/src/protocol/mod.rs](//home/tomas/Projects/Rust/RobotProject/src/protocol/mod.rs) | Rust | -304 | -54 | -75 | -433 |

[Summary](results.md) / [Details](details.md) / [Diff Summary](diff.md) / Diff Details