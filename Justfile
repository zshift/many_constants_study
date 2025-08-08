run_8478:
    cargo run --features consts_8478

run_8479:
    cargo run --features consts_8479

expand_8478:
    cargo expand --features consts_8478 > results/expanded_8478_main.rust

expand_8479:
    cargo expand --features consts_8479 > results/expanded_8479_main.rust

out_8478:
    cargo run --features consts_8478 > results/out_8478.txt

out_8479:
    cargo run --features consts_8479 > results/out_8479.txt