#!/bin/bash

cargo build > /dev/null 2>&1

./target/debug/mini-scheme
