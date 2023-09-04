fn main() {
    println!("Hello, world!");
}

// dbg!(&args); // Pass args as reference to not consume them so the can be used again

// // iter consults the Vec
// for (i, a) in args.iter().enumerate() {
//     println!("ARGS: {} is element {} in args", a, i);
// }

// println!("{}", args.len()); // first argument (0) is lazuli program location

// into_iter consumes the Vec
// for (i, a) in args.into_iter().enumerate() {
//     println!("ARGS: {} is element {} in args", a, i);
// }
// into_mut alows modification of the Vec arguments
