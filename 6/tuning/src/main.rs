fn main() -> std::io::Result<()> {
    let bytes = include_bytes!("../input.txt");
    // let window_size = 4;
    let window_size = 14;
    let window = bytes.windows(window_size);
    let mut window_number: usize = window_size;
    for w in window {
        let mut accum: i32 = 0;
        for c in w {
            accum ^= 1 << (c % 32);
        }
        if format!("{:b}", accum).matches('1').count() == window_size {
            println!("{window_number}");
            return Ok(());
        } else {
            window_number += 1;
        }
    }

    Ok(())
}
