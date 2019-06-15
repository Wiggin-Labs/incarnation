extern crate memmap;

use memmap::MmapMut;

use std::io::Read;

fn main() {
    let mut args = std::env::args();
    if args.len() != 2 {
        eprintln!("Expected one file argument!");
        std::process::exit(-1);
    }
    let filename = args.nth(1).unwrap();
    let mut file = std::fs::File::open(filename).expect("unable to open file!");
    let mut code = Vec::new();
    file.read_to_end(&mut code).expect("unable to read file!");

    let mut m = MmapMut::map_anon(code.len()).unwrap();
    for (i, c) in code.iter().enumerate() {
        m[i] = *c;
    }
    let m = m.make_exec().unwrap();
    let exe = unsafe { std::mem::transmute::<*const u8, fn()>(m.as_ptr()) };
    exe();
}
