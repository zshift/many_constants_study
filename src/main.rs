use constants_proc_macro::constants;

// Works
#[cfg(feature = "consts_8478")]
constants!(8478);

// stack-overflow and rust-analyzer hangs
#[cfg(feature = "consts_8479")]
constants!(8479);

fn main() {
    println!("Holy constants, Batman!");

    let sum = all_consts
        .iter()
        .map(|c| c.value)
        .reduce(|a, b| a + b)
        .unwrap_or_default();
    println!("Sum of all constants: {sum}");

    for constant in all_consts {
        println!("{} = {}", constant.name, constant.value);
    }
}
