#!/bin/bash

echo "Installing textual-geometry"
cd textual-geometry
cargo build

echo "Installing xIMP"
cd ..
cd ximp
cargo build

echo "TIM toolkit installed."