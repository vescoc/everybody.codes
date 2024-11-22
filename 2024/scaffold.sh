#!/bin/bash

for d in 02 03 04 05 06 07 08 09 10 11 12 13 14 15 16 17 18 19 20;
do
    mkdir -p quest-$d
    (
        cd quest-$d &&
            cp -R ../quest-01/* . &&
            sed -i s/quest-01/quest-$d/g Cargo.toml &&
            sed -i s/quest_01/quest_$d/g benches/bench.rs src/main.rs
    )
done
