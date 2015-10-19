extern crate getopts;
extern crate image;

use image::{GenericImage, Pixel};
use getopts::Options;
use std::option::Option;
use std::env;

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} FILE1 FILE2 [options]", program);
    print!("{}", opts.usage(&brief));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let mut opts = Options::new();
    opts.optflag("h", "help", "print this help menu");
    opts.optopt("s", "subimage", "only compare a subregion of the image", "x y width height");

    if args.len() < 3 {
        print_usage(&program, opts);
        return;
    }

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };
    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }

    let subimage = if matches.opt_present("s") {
        let dims = matches.opt_strs("s");
        println!("D: {:?}", dims.len());
        if dims.len() != 4 {
            print_usage(&program, opts);
            return;
        }
        Some(dims.iter().map(|s| s.parse::<u32>().unwrap()).collect::<Vec<u32>>())
    }
    else {
        None
    };

    let (file1, file2) = if matches.free.len() == 2 {
        (matches.free[0].clone(), matches.free[1].clone())
    } else {
        print_usage(&program, opts);
        return;
    };

    let mut img1 = image::open(file1).unwrap();
    let mut img2 = image::open(file2).unwrap();
    let sz1 = (img1.width(), img1.height());
    let sz2 = (img2.width(), img2.height());

    let (sub1, sub2) = match subimage {
            Some(v) => (img1.sub_image(v[0], v[1], v[2], v[3]),
                        img2.sub_image(v[0], v[1], v[2], v[3])),
            None    => (img1.sub_image(0, 0, sz1.0, sz1.1),
                        img2.sub_image(0, 0, sz2.0, sz2.1))
    };


    assert!(sub1.dimensions() == sub2.dimensions());

    let mut nom = 0usize;
    let mut sq1 = 0usize;
    let mut sq2 = 0usize;
    for (x, y, p1) in sub1.pixels() {
        let p2 = sub2.get_pixel(x, y);
        let a = p1.to_luma().data[0] as usize;
        let b = p2.to_luma().data[0] as usize;
        nom += a*b;
        sq1 += a*a;
        sq2 += b*b;
    }

    let ncc = nom as f64/(sq1 as f64 * sq2 as f64).sqrt();
    println!("NCC: {}", ncc);
}
