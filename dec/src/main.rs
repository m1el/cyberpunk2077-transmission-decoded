extern crate image;
extern crate rayon;
fn imgdiff(d1: &[u8], d2: &[u8]) -> usize {
    let mut d = 0;
    for (p1, p2) in d1.iter().zip(d2.iter()) {
        let p1 = *p1 as isize;
        let p2 = *p2 as isize;
        d += ((p1-p2).abs() / 128) as usize;
    }
    return d;
}

use image::*;
use rayon::prelude::*;
use std::collections::HashMap;
fn main() {
    let mut alphabet =
        image::open("alphabet.png")
        .expect("no alphabet.png")
        .to_luma();
    let alpha_chars: Vec<char> = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz+/ ".chars().collect();
    let mut chars = Vec::<Vec<u8>>::new();
    for i in 0..65 {
        chars.push(alphabet.sub_image(i*20,0, 20, 40).to_image().into_raw());
    }

    let range = 30u64..3092+1;
    let files: HashMap<u64, (String, String)> =
    range.clone().into_par_iter().map(|nn| {
        let fname = format!("shots/{:04}.png", nn);
        eprintln!("new file!!!! {}", fname);
        let mut input =
            image::open(&fname)
            .expect("can't open input")
            .to_luma();
        let mut rows = String::new();
        for y in 0..=26 {
            let mut row = String::new();
            for x in 0..72 {
                let ch = input.sub_image(269+x*20,y*40,20,40).to_image().into_raw();

                /*
                let (minc, _) = chars
                    .par_iter().enumerate()
                    .map(|(i, alc)| (i, imgdiff(&ch, alc)))
                    .reduce_with_identity((0, 9999999), |p, c| {
                        if p.1 < c.1 { p }
                        else { c }
                    });
                */
                let mut minc = 0;
                let mut mind = 9999999;
                for (i, alc) in chars.iter().enumerate() {
                    let d = imgdiff(&ch, alc);
                    if d < mind {
                        minc = i;
                        mind = d;
                    }
                }

                let res = alpha_chars[minc];
                row.push(res);
            }
            rows += &row;
            rows.push('\n');
        }
        return (nn, (fname, rows));
    }).collect();
    for k in range {
        let &(ref fname, ref rows) = &files[&k];
        println!("new file!!!! {}", fname);
        print!("{}", rows);
    }
}
