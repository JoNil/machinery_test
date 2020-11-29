#!/bin/bash
cargo build && cp target/debug/tm_code_rs.dll /c/dev/our_machinery/bin/plugins/ && the-machinery --hot-reload