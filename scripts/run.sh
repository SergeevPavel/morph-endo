#!/bin/bash


/Users/Pavel.Sergeev/.cargo/bin/cargo run --release interpreter $1
/Users/Pavel.Sergeev/.cargo/bin/cargo run --release drawer $1
/Users/Pavel.Sergeev/.cargo/bin/cargo run --release gui $1