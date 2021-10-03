fn main() {
    let v = vec![
        5270498306774157604,
        371609661054985362,
        595056260442243600,
        86757000457782305,
        82351536043346213,
    ];
    let mut u = Vec::new();
    for i in [1, 3] {
        let mut s = String::new();
        for j in 0..3 {
            let x = (v[i] >> (56 - 8 * j)) as u8 + b'A';
            let c = char::from_u32(x.into()).unwrap();
            s.push(c);
            if j == 2 {
                s.push(c);
            }
        }
        u.push(s);
    }
    u.push([u[0].clone(), u[1].clone()].concat());
    for i in 0..2 {
        let w = (v[0] + 28) / v[4];
        let z = w - 28 * (i as i64) as u64;
        for j in 0..z {
            let p = (v[i] >> j) & 1;
            let q = (v[i + 2] >> j) & 1;
            match (p, q) {
                (1, 1) => println!("{}", u[2]),
                (1, _) => println!("{}", u[0]),
                (_, 1) => println!("{}", u[1]),
                (_, _) => println!("{}", j + 1 + w * (i as u64)),
            };
        }
    }
}
