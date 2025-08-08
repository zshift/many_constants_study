use constants_proc_macro::constants;

// Works
constants!(8479);
// stack-overflow and rust-analyzer hangs
// constants!(8479);

fn main() {
    println!("Holy constants, Batman!");

    for constant in all_consts {
        println!("{} = {}", constant.name, constant.value);
    }
}
