#!/bin/bash

cargo build > /dev/null 2>&1

rust-gdb ./target/debug/mini-scheme
