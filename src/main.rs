fn main() {
    let mut pw_iter = PASSWORD.iter();
    let mut pw_counter_vec: Vec<i32> = Vec::new();
    for c in CHAR_CHECK.iter() {
        let mut counter: i32 = 0;
            let mut pw_char_iter = match pw_iter.next() {
                Some(x) => x.chars(),
                None => break
            };
            
            loop {
                match pw_char_iter.next() {
                    Some(p) => {
                       
                        if &p == c {
                           
                            counter += 1;
                            //println!("{}", p);
                        }
                    }
                    None => break
                }
            }
            pw_counter_vec.push(counter);
    }
    println!("{:?}", pw_counter_vec);


    let mut RANGE_iter = RANGE.iter();
    let mut pw_counter_vec_iter = pw_counter_vec.iter();
    let mut answer_counter = 0;


    loop {
        let i;
        match pw_counter_vec_iter.next() {
            Some(b) => {i = b}
            None => break,
        };
        //println!("{}", i);


        match RANGE_iter.next() {
            Some(a) => match RANGE_iter.next() {
                Some (b)=> if i >= a && i <= b {
                    answer_counter += 1;
                    println!("{} {}",a,b);
                }
                None => break
            }
            None => break
        }
            
    }
    println!("{}", answer_counter);
}
    
   



#[allow(dead_code)]
static RANGE: [i32; 2000] = [
    15, 16, 5, 6, 3, 4, 3, 6, 10, 11, 4, 7, 4, 7, 11, 16, 7, 12, 4, 5, 9, 17, 2, 5, 4, 7, 11, 13,
    4, 8, 3, 15, 4, 5, 3, 6, 5, 8, 1, 15, 7, 8, 1, 15, 1, 3, 5, 6, 2, 5, 5, 7, 5, 10, 6, 9, 2, 7,
    16, 17, 1, 5, 3, 7, 1, 4, 3, 6, 11, 14, 4, 7, 2, 4, 7, 11, 3, 4, 3, 5, 2, 4, 1, 2, 5, 7, 1, 3,
    6, 8, 5, 6, 5, 6, 2, 6, 1, 8, 10, 14, 5, 15, 4, 5, 1, 15, 1, 2, 2, 4, 5, 14, 7, 9, 2, 19, 6, 7,
    3, 15, 9, 13, 5, 6, 8, 9, 5, 7, 8, 17, 13, 14, 2, 3, 2, 4, 2, 7, 6, 9, 4, 8, 1, 6, 7, 10, 10,
    14, 9, 10, 5, 6, 6, 13, 1, 6, 8, 10, 10, 12, 9, 10, 2, 8, 6, 7, 3, 7, 4, 10, 4, 5, 14, 15, 4,
    6, 4, 6, 5, 13, 7, 8, 3, 4, 11, 13, 11, 13, 1, 2, 1, 3, 2, 10, 8, 10, 2, 10, 5, 16, 3, 7, 1, 4,
    6, 8, 3, 5, 1, 14, 4, 9, 7, 8, 1, 14, 2, 3, 3, 4, 3, 4, 1, 5, 6, 13, 1, 3, 11, 12, 4, 5, 8, 10,
    1, 2, 8, 12, 1, 19, 10, 11, 1, 10, 7, 12, 2, 11, 3, 4, 3, 12, 3, 7, 5, 6, 6, 18, 6, 11, 12, 14,
    5, 6, 7, 9, 4, 11, 10, 11, 2, 13, 10, 11, 7, 8, 5, 6, 16, 17, 7, 13, 14, 17, 1, 4, 14, 15, 8,
    14, 1, 4, 11, 14, 4, 6, 1, 2, 2, 7, 7, 11, 2, 7, 3, 4, 2, 6, 8, 15, 5, 6, 1, 4, 13, 14, 2, 4,
    4, 5, 5, 8, 4, 7, 4, 15, 2, 3, 7, 13, 3, 6, 3, 4, 5, 7, 4, 5, 2, 8, 11, 14, 1, 5, 16, 17, 5, 8,
    10, 12, 7, 9, 12, 16, 10, 12, 2, 8, 10, 11, 1, 3, 6, 9, 1, 4, 10, 13, 8, 9, 3, 5, 6, 7, 5, 7,
    2, 4, 6, 7, 3, 7, 3, 6, 1, 4, 4, 5, 4, 8, 8, 9, 4, 13, 3, 8, 11, 12, 6, 8, 4, 5, 4, 8, 3, 8, 7,
    9, 4, 7, 3, 6, 6, 17, 1, 8, 2, 5, 6, 10, 11, 15, 12, 14, 10, 11, 6, 7, 1, 2, 10, 15, 8, 16, 6,
    7, 11, 13, 8, 11, 12, 18, 4, 9, 1, 5, 2, 16, 4, 5, 14, 16, 4, 5, 13, 15, 16, 17, 14, 15, 3, 4,
    5, 10, 1, 4, 13, 17, 8, 12, 14, 19, 6, 19, 3, 9, 2, 17, 1, 4, 9, 17, 7, 10, 2, 3, 12, 15, 1, 7,
    1, 9, 1, 4, 12, 16, 3, 4, 4, 6, 3, 4, 3, 19, 7, 12, 14, 15, 2, 4, 14, 15, 3, 14, 2, 6, 1, 2, 4,
    7, 3, 8, 3, 7, 17, 18, 15, 16, 2, 4, 6, 15, 3, 4, 8, 10, 9, 12, 3, 4, 12, 13, 13, 19, 14, 18,
    4, 11, 3, 19, 6, 11, 2, 4, 5, 6, 7, 9, 10, 13, 8, 10, 9, 10, 2, 12, 4, 12, 3, 6, 5, 10, 1, 4,
    4, 5, 3, 10, 16, 17, 2, 11, 1, 4, 3, 4, 4, 7, 18, 19, 8, 10, 1, 3, 3, 4, 15, 19, 1, 7, 6, 14,
    12, 17, 5, 9, 8, 9, 5, 10, 2, 5, 3, 7, 2, 5, 6, 7, 9, 13, 4, 10, 2, 7, 4, 6, 4, 6, 6, 8, 3, 6,
    16, 17, 4, 11, 3, 4, 2, 7, 2, 9, 11, 14, 5, 6, 15, 17, 13, 17, 15, 17, 4, 8, 11, 16, 3, 4, 4,
    5, 6, 14, 4, 5, 2, 4, 8, 9, 2, 15, 2, 7, 6, 12, 3, 5, 2, 7, 9, 10, 1, 4, 2, 8, 3, 12, 2, 3, 1,
    2, 6, 10, 5, 10, 8, 16, 4, 16, 2, 9, 5, 7, 2, 4, 3, 15, 9, 10, 2, 3, 9, 10, 3, 4, 2, 6, 9, 10,
    2, 6, 18, 19, 3, 5, 5, 6, 5, 6, 12, 15, 1, 4, 3, 8, 1, 2, 5, 14, 9, 10, 4, 6, 10, 14, 8, 12,
    15, 16, 1, 10, 2, 7, 2, 9, 4, 6, 3, 5, 8, 12, 10, 11, 5, 6, 17, 18, 3, 8, 6, 12, 2, 13, 7, 13,
    9, 10, 12, 14, 17, 18, 1, 9, 6, 7, 13, 15, 6, 7, 1, 17, 3, 5, 7, 8, 2, 5, 5, 7, 8, 9, 2, 3, 11,
    12, 3, 14, 3, 6, 11, 12, 13, 17, 4, 5, 2, 3, 9, 12, 3, 5, 5, 7, 5, 7, 9, 10, 3, 4, 1, 10, 5,
    13, 2, 3, 6, 12, 3, 9, 6, 10, 3, 4, 2, 3, 7, 8, 7, 10, 4, 9, 6, 9, 7, 10, 4, 6, 2, 4, 2, 4, 2,
    16, 3, 5, 6, 9, 1, 5, 2, 10, 6, 8, 13, 14, 2, 4, 8, 14, 3, 6, 11, 13, 4, 5, 3, 4, 1, 4, 8, 9,
    5, 8, 4, 6, 10, 15, 4, 5, 13, 17, 1, 11, 15, 16, 2, 3, 16, 19, 11, 13, 10, 11, 5, 8, 9, 12, 16,
    17, 9, 10, 4, 5, 4, 12, 1, 3, 16, 17, 9, 14, 6, 11, 2, 8, 3, 4, 9, 12, 5, 8, 5, 7, 7, 10, 12,
    13, 10, 14, 7, 10, 1, 4, 8, 10, 5, 9, 11, 13, 14, 16, 9, 11, 8, 11, 12, 18, 4, 5, 6, 16, 4, 13,
    14, 15, 3, 5, 11, 13, 5, 6, 6, 8, 1, 5, 2, 4, 1, 2, 10, 19, 3, 4, 1, 8, 13, 15, 4, 6, 9, 13,
    19, 20, 9, 14, 7, 15, 4, 5, 7, 15, 15, 17, 1, 2, 3, 9, 14, 15, 5, 6, 12, 14, 1, 2, 14, 15, 7,
    8, 7, 12, 13, 18, 10, 11, 9, 17, 5, 12, 2, 6, 4, 6, 10, 11, 8, 16, 4, 7, 8, 9, 9, 10, 16, 18,
    2, 13, 12, 18, 3, 4, 6, 11, 16, 17, 6, 11, 10, 14, 10, 15, 3, 7, 3, 5, 10, 13, 6, 7, 3, 7, 4,
    9, 8, 10, 13, 16, 1, 4, 1, 2, 7, 8, 18, 20, 8, 13, 4, 5, 10, 11, 11, 16, 2, 9, 7, 11, 1, 4, 8,
    10, 2, 4, 6, 8, 5, 8, 3, 5, 7, 8, 2, 7, 14, 15, 9, 10, 13, 18, 1, 7, 9, 19, 12, 18, 4, 9, 6,
    13, 8, 9, 3, 6, 1, 3, 10, 11, 2, 4, 5, 6, 3, 5, 12, 14, 6, 8, 11, 12, 8, 9, 10, 11, 5, 8, 1, 4,
    2, 3, 5, 6, 14, 15, 7, 8, 16, 19, 2, 4, 7, 13, 16, 17, 1, 2, 5, 7, 3, 5, 1, 4, 5, 11, 9, 12, 3,
    4, 5, 11, 3, 7, 6, 14, 2, 4, 5, 7, 2, 8, 17, 18, 12, 15, 3, 4, 5, 11, 15, 16, 3, 10, 2, 5, 3,
    5, 3, 4, 14, 15, 4, 13, 1, 8, 3, 4, 5, 9, 1, 2, 5, 6, 10, 11, 4, 7, 12, 16, 7, 18, 2, 5, 2, 8,
    8, 9, 4, 5, 1, 9, 8, 12, 2, 4, 1, 4, 13, 17, 3, 5, 8, 9, 1, 6, 6, 8, 16, 18, 3, 8, 2, 3, 8, 17,
    19, 20, 1, 11, 6, 7, 5, 7, 5, 6, 3, 4, 4, 15, 2, 4, 4, 6, 4, 8, 14, 17, 3, 5, 6, 7, 1, 6, 4, 6,
    7, 10, 3, 4, 5, 12, 3, 5, 4, 7, 4, 6, 11, 13, 12, 18, 18, 19, 3, 4, 15, 17, 6, 9, 1, 2, 2, 9,
    5, 12, 9, 16, 11, 15, 3, 8, 5, 8, 7, 9, 6, 8, 12, 13, 16, 17, 2, 4, 2, 11, 5, 6, 2, 13, 4, 8,
    7, 17, 3, 6, 4, 10, 16, 17, 2, 14, 5, 7, 4, 6, 10, 11, 1, 2, 15, 16, 3, 4, 2, 4, 2, 3, 2, 12,
    5, 8, 5, 9, 6, 7, 2, 9, 9, 18, 5, 9, 8, 10, 4, 12, 6, 7, 2, 6, 11, 12, 8, 9, 6, 12, 1, 8, 4, 9,
    4, 5, 2, 3, 3, 5, 2, 6, 11, 14, 7, 8, 13, 15, 6, 7, 7, 8, 1, 3, 8, 12, 7, 10, 2, 5, 3, 5, 3,
    17, 8, 12, 7, 9, 4, 8, 6, 10, 1, 5, 3, 12, 1, 9, 2, 6, 11, 13, 2, 4, 11, 14, 15, 19, 9, 10, 1,
    14, 6, 8, 5, 7, 5, 7, 6, 12, 2, 14, 7, 14, 15, 17, 4, 5, 11, 13, 5, 6, 5, 6, 1, 3, 7, 8, 6, 11,
    11, 17, 8, 12, 7, 11, 11, 12, 12, 13, 2, 3, 8, 11, 6, 9, 10, 13, 8, 11, 3, 4, 11, 12, 13, 17,
    1, 5, 7, 8, 6, 12, 3, 5, 5, 8, 6, 11, 4, 5, 5, 6, 9, 14, 6, 10, 14, 17, 4, 5, 1, 7, 6, 11, 6,
    13, 12, 13, 8, 10, 6, 7, 6, 8, 3, 4, 1, 3, 7, 9, 7, 9, 2, 3, 18, 19, 4, 6, 2, 3, 1, 9, 8, 12,
    4, 10, 9, 16, 7, 8, 7, 10, 5, 7, 5, 10, 4, 6, 7, 9, 3, 4, 8, 12, 6, 8, 2, 3, 1, 2, 6, 8, 1, 9,
    3, 6, 5, 6, 12, 13, 9, 12, 3, 6, 4, 7, 8, 10, 7, 8, 4, 5, 6, 9, 1, 10, 9, 13, 2, 3, 6, 11, 5,
    9, 3, 14, 1, 7, 2, 5, 1, 5, 8, 14, 7, 8, 5, 6, 1, 8, 1, 4, 1, 9, 1, 5, 1, 4, 4, 5, 2, 8, 8, 16,
    8, 9, 10, 11, 8, 11, 4, 5, 1, 2, 1, 4, 4, 8, 4, 10, 4, 6, 9, 10, 3, 4, 7, 9, 10, 12, 1, 5, 10,
    11, 8, 10, 3, 5, 8, 14, 6, 11, 4, 6, 1, 2, 8, 20, 12, 15, 1, 14, 11, 14, 2, 5, 10, 12, 3, 4, 2,
    7, 6, 8, 4, 8, 1, 4, 3, 14, 3, 4, 13, 15, 3, 5, 1, 12, 8, 9, 7, 9, 10, 12, 9, 12, 12, 13, 2, 5,
    5, 6, 2, 3, 4, 13, 17, 19, 7, 11, 1, 5, 8, 12, 10, 11, 3, 5, 1, 4, 1, 9, 7, 9, 1, 2, 6, 16, 4,
    5, 7, 10, 1, 2, 12, 16, 7, 10, 6, 7, 1, 4, 9, 18, 2, 4, 3, 4, 5, 11, 1, 4, 4, 6, 13, 14, 9, 12,
    11, 13, 7, 11, 17, 19, 4, 5, 4, 6, 2, 3, 6, 11, 2, 15, 3, 4, 6, 7, 16, 17, 2, 4, 5, 7, 4, 10,
    5, 6, 4, 11, 10, 13, 11, 12, 4, 8, 7, 10, 2, 8, 7, 8, 8, 10, 1, 4, 6, 12, 8, 12, 1, 4, 1, 2,
    10, 14, 11, 15, 7, 9, 12, 15, 6, 7, 15, 19, 6, 8, 16, 20, 7, 10, 3, 4, 5, 6, 10, 12, 4, 7, 5,
    13, 1, 2, 1, 8, 3, 4, 16, 17, 10, 15, 8, 15, 4, 8, 16, 17, 1, 4, 1, 9, 1, 5, 2, 12, 12, 13, 9,
    10, 4, 5, 8, 14, 1, 4, 3, 4, 2, 5, 3, 9, 3, 4, 8, 11, 6, 10, 1, 16, 1, 7, 1, 5, 14, 16, 3, 8,
    1, 2, 3, 5, 10, 14, 2, 11, 1, 5, 15, 16, 15, 16, 11, 14, 8, 10, 1, 3, 7, 8, 6, 14, 9, 12, 4, 6,
    9, 16,
];
#[allow(dead_code)]
static CHAR_CHECK: [char; 1000] = [
    'm', 'd', 's', 'b', 'q', 'g', 'f', 'h', 'n', 'f', 'm', 'h', 'x', 'h', 'q', 'x', 'j', 'l', 's',
    'd', 'l', 'm', 'x', 't', 'x', 'l', 'g', 'j', 'b', 'j', 'p', 'q', 'k', 'l', 'x', 'k', 'l', 'q',
    'f', 'z', 'k', 'k', 'x', 'p', 'c', 's', 'w', 'w', 'z', 'b', 'v', 'j', 'q', 'w', 'c', 'r', 'n',
    'w', 'r', 'w', 's', 'j', 'f', 'n', 'd', 'v', 'h', 'g', 'r', 'n', 'k', 'v', 'w', 'k', 's', 'z',
    'j', 'k', 'v', 'm', 'k', 'b', 'x', 'l', 'j', 'g', 'n', 'h', 'b', 'm', 'r', 'q', 's', 'x', 'f',
    'd', 'r', 'w', 'r', 'c', 'w', 's', 'j', 'c', 'j', 'w', 'p', 'd', 'r', 'j', 'q', 'k', 't', 'x',
    'f', 'l', 'n', 'x', 'w', 'p', 'h', 'c', 'v', 'b', 'n', 's', 'b', 'r', 'n', 'v', 's', 't', 'p',
    's', 'n', 'r', 'k', 's', 'd', 'q', 'd', 'v', 'c', 't', 'c', 'n', 'p', 'd', 'b', 'f', 'q', 'g',
    'w', 'j', 'w', 'f', 'n', 'k', 'd', 'g', 'x', 'h', 'q', 'j', 'k', 'g', 'b', 'j', 'x', 'r', 'c',
    'f', 'f', 'n', 'm', 'l', 'v', 'w', 'x', 'd', 'h', 'c', 'c', 'm', 'k', 'x', 'q', 'g', 'n', 'k',
    'n', 'm', 'q', 'n', 's', 'b', 'h', 'n', 'f', 'p', 'd', 'n', 'r', 'z', 'f', 'd', 'w', 'b', 'z',
    'w', 's', 's', 'x', 'z', 't', 'd', 'w', 'x', 'h', 'm', 'w', 'f', 'n', 'l', 'r', 'd', 'j', 's',
    'f', 'l', 'g', 'x', 'q', 'x', 'b', 'x', 'd', 'c', 'z', 'm', 'l', 'r', 'f', 't', 'c', 't', 'c',
    'n', 'q', 't', 'j', 'w', 'z', 'c', 'k', 'k', 's', 'r', 'g', 'z', 't', 'q', 'p', 'b', 'x', 'm',
    'b', 'k', 'j', 'w', 's', 'n', 'z', 'b', 'x', 'x', 'g', 'b', 't', 'q', 'j', 'v', 'q', 'z', 'n',
    'd', 'l', 'd', 'v', 'x', 'k', 'd', 'b', 'b', 'h', 'p', 'w', 'l', 'j', 'd', 'c', 'f', 'b', 'v',
    'l', 't', 'f', 'q', 'c', 'g', 'n', 'x', 's', 'm', 'g', 'q', 'd', 'b', 'p', 'n', 'f', 'l', 'g',
    'r', 'f', 's', 'f', 'w', 'j', 'r', 'w', 's', 'd', 'w', 'd', 'h', 'q', 'g', 'p', 'm', 'z', 'l',
    'w', 's', 'm', 'q', 'x', 'k', 'b', 'm', 'k', 'm', 'z', 'n', 'w', 'j', 'd', 'r', 'v', 'x', 'k',
    'j', 'j', 'k', 't', 'd', 'f', 'z', 'n', 'g', 'f', 'm', 'n', 'l', 't', 't', 'w', 'h', 'v', 'd',
    'x', 'f', 'b', 'r', 's', 'z', 'j', 'l', 'm', 'l', 'd', 'm', 't', 'j', 'm', 'l', 'x', 'k', 'v',
    'd', 'p', 'm', 'm', 'n', 'j', 'r', 'c', 'j', 'z', 'j', 'w', 'p', 'j', 'm', 'k', 'd', 's', 'x',
    't', 'j', 't', 'v', 'r', 'g', 'x', 'r', 'v', 's', 'w', 'b', 'v', 'f', 'q', 'w', 'w', 't', 'x',
    'n', 'f', 'n', 'l', 's', 'z', 'x', 'c', 'v', 'r', 'b', 's', 'k', 'x', 'd', 'p', 's', 's', 's',
    'j', 's', 'k', 'g', 'n', 'n', 'm', 'b', 'm', 'z', 'q', 'f', 't', 'w', 'z', 'z', 'v', 't', 'l',
    'j', 'k', 'w', 'b', 'c', 'l', 'q', 'z', 'x', 'z', 'l', 'b', 'x', 'f', 'd', 'g', 's', 'b', 'h',
    'b', 'p', 'q', 's', 'q', 'z', 'r', 'q', 'g', 'w', 'h', 'v', 'j', 'c', 'c', 'p', 'h', 'q', 'f',
    'j', 'p', 'w', 'f', 'l', 's', 'x', 'n', 'g', 'n', 'n', 'q', 'r', 'z', 'g', 'v', 's', 'h', 't',
    'f', 'p', 'b', 'n', 'r', 'r', 'f', 'r', 'b', 'p', 'q', 'r', 'g', 'j', 't', 'f', 'k', 'm', 'q',
    'n', 'd', 'f', 'p', 'm', 'h', 's', 'x', 'h', 'v', 'n', 'l', 'j', 'h', 'z', 's', 'z', 'd', 'q',
    'm', 'l', 'b', 'b', 'c', 'k', 'b', 'q', 'g', 'h', 'w', 'x', 'f', 'f', 'f', 'r', 't', 'r', 'b',
    'm', 'g', 'g', 'l', 'n', 'v', 's', 'x', 'f', 'b', 's', 'p', 'h', 's', 'k', 'm', 'w', 't', 'f',
    'd', 'l', 'z', 'v', 'b', 'd', 'z', 'b', 't', 'v', 'n', 'h', 'j', 't', 'z', 'l', 'j', 'n', 'z',
    'z', 't', 'n', 'b', 'd', 'w', 'q', 'z', 'n', 'q', 'r', 'l', 'g', 'w', 't', 'x', 'g', 'l', 'p',
    'x', 'l', 'b', 'k', 'p', 'n', 'm', 'k', 'r', 'h', 'k', 'g', 'x', 'x', 'f', 'd', 'f', 'r', 'q',
    'b', 's', 'd', 's', 's', 'j', 'g', 'q', 'd', 'p', 'm', 't', 'k', 't', 'm', 'l', 'j', 'x', 's',
    'v', 'h', 'n', 'v', 'f', 'w', 'n', 'd', 'd', 'v', 'l', 'p', 'b', 'm', 'j', 'g', 'p', 'k', 'b',
    't', 'd', 'r', 's', 'm', 'h', 'q', 'd', 'f', 'v', 'p', 'w', 'm', 't', 'v', 'z', 'k', 'm', 'm',
    'p', 's', 'q', 'x', 'b', 'c', 'd', 'h', 'l', 'q', 'w', 'd', 'k', 'd', 'n', 'k', 'g', 'v', 's',
    'm', 't', 'k', 'x', 'd', 't', 'r', 'd', 'x', 'w', 'j', 'm', 'b', 'v', 'f', 'm', 'h', 'w', 'f',
    'f', 'v', 'd', 'w', 'l', 'v', 'h', 'b', 'l', 'l', 'c', 'b', 'z', 'j', 'z', 'm', 'q', 'p', 'w',
    'm', 'v', 'q', 'z', 'g', 'd', 'n', 'z', 'n', 'b', 'f', 'h', 'x', 'v', 'z', 'x', 'x', 's', 'j',
    's', 'h', 'h', 'm', 'h', 'c', 'q', 'r', 't', 'g', 'd', 'g', 'm', 'z', 'x', 'z', 'k', 'q', 'l',
    'v', 'v', 'h', 't', 'g', 'q', 'w', 'b', 'g', 'w', 'p', 'z', 'v', 'q', 'r', 'd', 'p', 's', 'q',
    'k', 'z', 'b', 'c', 'x', 'm', 'h', 'x', 'l', 'd', 'j', 'd', 'j', 'm', 'k', 'l', 'h', 'q', 'l',
    'l', 'k', 'w', 'g', 'z', 'p', 'g', 'b', 'q', 'w', 'm', 'x', 'g', 'r', 'q', 'c', 'w', 'r', 'd',
    'c', 'n', 'n', 'h', 'l', 'c', 'l', 'w', 'k', 'r', 'g', 'g', 'c', 't', 'q', 'r', 't', 'l', 'x',
    'd', 'f', 'f', 's', 'p', 'p', 'h', 'b', 'n', 'r', 'l', 'p', 'g', 'q', 'x', 'h', 'q', 'g', 'r',
    'm', 'p', 'v', 'r', 'z', 'd', 'c', 'x', 'g', 'm', 'w', 's', 'l', 'l', 'h', 'v', 'k', 'f', 'd',
    'q', 'w', 'l', 's', 'c', 'n', 'c', 'n', 't', 'b', 'z', 'k', 'x', 'v', 's', 'l', 'q', 'n', 's',
    'r', 'q', 'w', 'g', 'l', 'v', 'r', 'n', 'x', 'k', 'k', 'n', 'm', 'k', 'h', 'p', 'g', 'f', 'd',
    'w', 'z', 'f', 'x', 'r', 'w', 'k', 'r', 't', 'z', 'p', 'd', 'c', 'q', 'f', 'v', 'p', 'v', 'r',
    'f', 'p', 'x', 'b', 't', 'f', 'r', 'n', 's', 't', 'g', 'd',
];

#[allow(dead_code)]
const PASSWORD: [&'static str; 1000] = [
    "mhmjmzrmmlmmmmmm",
    "dcdddhzld",
    "vqssdcbl",
    "bbhbbbqbbb",
    "cqqntqhqwwh",
    "ggkgggw",
    "fdfffflfvn",
    "thhrhrwhbshshsdhhhhr",
    "nnnnnnvnnnnn",
    "ffxfhldh",
    "mmmmmmmmjmmmmmmmw",
    "hqrfzhhh",
    "vxxfxxjxwvnlx",
    "dhhhhhhhdhhhrh",
    "qqphvphfrlqqkztgslzb",
    "htblqcbdxdzgvhszxz",
    "jjhnn",
    "bwkjllgcjrzhr",
    "ssmsspsss",
    "ddhmhmvlkppkbbdxbc",
    "ltllllllhgzlv",
    "mtmmmvmmbmmfmmkmmmm",
    "xrxzcxcx",
    "tttttdtv",
    "xxxxs",
    "gclqlml",
    "gggggggggn",
    "xwjjfngjgjsjsr",
    "hbqtchkcmblppvqp",
    "jjjjjjjjjjjjjjmjjj",
    "wpppdlpphppp",
    "mspqqqqq",
    "hkjk",
    "llbllldzf",
    "xxnxvxxxxlxxxkv",
    "zkkksmjmrzxftx",
    "lllwq",
    "tbqqmmvkrsz",
    "gwfs",
    "ztszz",
    "kkpzl",
    "kkwckkzwskgbdc",
    "mxxxdxf",
    "pppp",
    "rcqrmcqmpcc",
    "srgvwsc",
    "wwwwwww",
    "xwwzfwr",
    "djzzzcmz",
    "dbwbbbbbldtbbbrbbq",
    "wvzftvfvzkcvjdvvzcj",
    "lrjdmjj",
    "qqqqqqqqqqqqqqkqqq",
    "vwkw",
    "lccsfmkrnrldzbrc",
    "rcpghlsspmvwvzmpvl",
    "nnnnnnnnx",
    "wwwwwwwwwwwwwwwhwbww",
    "rrrrrzc",
    "mnxmfqgklqddfww",
    "tsslqtsszsqsssqssss",
    "jjjtjrb",
    "fffffffwfff",
    "nnnnnnhn",
    "dnfdqvwngbddvbndbsf",
    "vvvvvvvvtvvvvt",
    "hsbnhhhs",
    "ggcgm",
    "rvrrrrrvr",
    "qpxchngsbhxrgqdgbqs",
    "kkkkkkkkd",
    "mrvlldzv",
    "vvwkwrbnwczwww",
    "kkkknkkkkpkkkdmkh",
    "qqbwmssnssjs",
    "bzvdjz",
    "jjjjrzbrjzjmjzjsk",
    "kkkkkrsskk",
    "vvvvvvwvvzvb",
    "mmmmmmmmmdmm",
    "kkkkkkkkjkk",
    "pbcsbbprc",
    "xxxxnpxxxk",
    "llllllml",
    "jrjzjjjcjx",
    "gggmgg",
    "nnnndnnxnnnnntnn",
    "hzqpgbhwh",
    "plzbkbrpscqbxl",
    "hptczmkkmplmw",
    "vjhrhrkdrrrk",
    "qqqq",
    "sssnssssssmsss",
    "xxbhkwvvxgxjh",
    "ftlrdz",
    "mdslnl",
    "xrzbqcxlvtlrsznplk",
    "wwwwmwwtww",
    "zbzplcvnvrr",
    "lcccccclcccgcccccc",
    "wwpwwbww",
    "svsss",
    "jjllkwnjjjgcjpg",
    "cccbb",
    "jpkjjqjjgqmzbjjhjljj",
    "dwwlwwdmkww",
    "pppppppjhqp",
    "djmfmdddkhfqdzfdddd",
    "zrbjhpg",
    "jjkj",
    "qqqq",
    "kkkkwk",
    "ttzxtjtdjtmtttmtt",
    "jxvx",
    "fffffffffffkffffffff",
    "lllhlm",
    "hnnnnnkdfrnntdhnk",
    "gxxx",
    "pwwwwwwfwwww",
    "ppnxfswpxfpbfcpnnpq",
    "hhhphhhhhhgv",
    "dccbvcccccc",
    "vvvvvvrvdvvvvvvvvvvl",
    "jbncfdbbbbcb",
    "nnrn",
    "btnfjtszszssdrsd",
    "bznkfbb",
    "rhprrhrrrhnzjhr",
    "xznnnwwmwnvfnfnlhn",
    "smsvpcjvvgvtvkmvv",
    "sssssssssssssc",
    "ttttfx",
    "pppppnvppp",
    "fbvfqsswshlgr",
    "thnnnnvnnnnnvbnn",
    "dqnvjskcqhmrrtn",
    "kvhkskkfrkr",
    "sssssssm",
    "fddddvv",
    "qqmkfqvvqdqqckqrw",
    "lddrdgdddzddz",
    "vvvvvvvvvqrvvvvvbnqx",
    "ccqn",
    "tbtrrqgtldttmktrt",
    "csccctccrrcwcqc",
    "nnnbnnn",
    "pppppppppppppppppp",
    "hcrkddqddm",
    "xbbbb",
    "cfwqnzbtfnt",
    "qqqqkwqqlqqqq",
    "gtgsgwg",
    "wwks",
    "rjjstgwskhwc",
    "wwwwsgrwwhwjvgqw",
    "ffffhr",
    "kxcn",
    "kkkrkzkkkkkrkhq",
    "fdtpdlbp",
    "ggggg",
    "skzrxxsxxhknxzxxxpnf",
    "ddhhbxnlhfb",
    "wqqlqqqrgqqqwqqqqqq",
    "jjwlnjb",
    "kkxkknjkkkkkksk",
    "ggmgggggggg",
    "bbgbbfd",
    "jjjjwjjj",
    "hvnhxhxxkzxzxc",
    "rlrrwrrrc",
    "ccgccrcccccbhccccm",
    "vfffffffffff",
    "fffffffffffffffff",
    "nwnnngppnn",
    "mmmmmmmmmvmmm",
    "llllllgll",
    "vvvvvvvvvvvlvvvvv",
    "wwwwwxvwwjwwdwwwq",
    "cxjlkdwctx",
    "mdddlndmsdd",
    "bhkfwhjls",
    "cccpcncccc",
    "tzxzctxhpt",
    "mmmsmwmmmsmmcmh",
    "kkkdkkkrb",
    "xkxjxx",
    "cqmxqqqlhtqpgqsqwqqd",
    "ggzggggqg",
    "ncnvzvrk",
    "kkkkkzk",
    "nnnjcmpgtgxqdwjz",
    "zzzvmmmmlmq",
    "nqqqqq",
    "cnnzvn",
    "bscfsnss",
    "pqbtmxfbt",
    "shqjphgfvkrrj",
    "cnphjnhbtndtcn",
    "ffkfzfffbftfffff",
    "ppwppxspppbp",
    "ddddnddd",
    "qnbhwnzxd",
    "csflkrvrq",
    "zzwtzzmzzzzzxdb",
    "wgfffkw",
    "wgmxddvdwtdtknvsz",
    "mzmwwwwzkxwwcvpwrs",
    "bbnltzsmbbfp",
    "tpztzz",
    "rwtwrvwpwwwwpx",
    "sdsssssdsxlpsss",
    "sssvsssssssssr",
    "dqxfvxxxxlx",
    "zqzzzlzz",
    "ftvbp",
    "dddddddddmwdddshdd",
    "wwwfbwwfqwzwvwthjw",
    "xxxxxxpx",
    "hchhhhhhmhfhp",
    "vmmmkmmmxvmmmt",
    "jwmpwwwdwwwwkwswdwz",
    "hwfftvxmpfffff",
    "nhchnnkpn",
    "nlxtpcdlzdkhnmqsvqfg",
    "rrrrr",
    "dddddpgdrddddcdbddd",
    "gskjzxjjz",
    "qsmccrpnsrrvwsk",
    "ffffffflfffffffff",
    "nllcllllvnlwltll",
    "gxghgggdggg",
    "xfxxqljxxkxxx",
    "lmzzq",
    "xxxxxxxxxxxxxxxxt",
    "bbbmxbbbjbbbb",
    "glxxxxhxxxxwxjxxxxc",
    "ndflnddcxksdzdbwdddt",
    "ccccccccl",
    "zrxzbzwzzzzzfzzcz",
    "fmmmml",
    "lllllllllllllbllllll",
    "wgrnrkxrrmrprxrr",
    "vcfff",
    "tttttttttttwtttt",
    "xcccccc",
    "kftttbttchttttttttt",
    "cpkcwmdwxnwvjzfbj",
    "nnnnnnnnlnnnnnnnn",
    "qcjq",
    "tdttkshpmlgqstpttfcc",
    "jjmcxlb",
    "qwdwwhwwjqmwwwwgqwfw",
    "zzgzzzwzkzzb",
    "nccccbdqwdccccd",
    "kkkkrv",
    "kkkkkkkkkkkkkkrkkkk",
    "sspsssssssssstsssss",
    "tdrrrwcrrcrr",
    "gfgg",
    "zczhzkzzccmpg",
    "ttvtscttzcxtt",
    "svkxfrxrvqqpmxzsbk",
    "ppnppppppppppppppb",
    "brbbbbbbbbbbbbpbb",
    "shxx",
    "mmmmmmmmmmmmmmvm",
    "bbbbb",
    "kgkmkkkjkkgkkmk",
    "jtjlwjjjjjml",
    "whwdpqfwghhj",
    "ssssssssssssg",
    "nwdnntnnnqnnqnrbmnmn",
    "zzzzzzzzzzhzzzzzzl",
    "bbbbbbbbbbhbbb",
    "jwxjftcjnkmlgcfgxzmj",
    "xxxxxxxzxmxx",
    "vqgg",
    "bbtbbvhpvbbm",
    "csthttrtttt",
    "qqqqnqgcqnqqqlqcq",
    "jjjjjjjdjw",
    "vrpnvvvvsvqvcvv",
    "jqqqqqwqqqkqkqqq",
    "zrcznzmzzwzzfs",
    "ntnptpnvnns",
    "ddddhddqdd",
    "pcdsl",
    "ddddz",
    "kgnvtcvfvvqkvgwkvj",
    "pdpxctpsxgpjshbxvvq",
    "kkkskkkkkkz",
    "ddddm",
    "bbqm",
    "bbrbbbb",
    "hhhhhhhhhhhhhhhhbhwh",
    "pvlxpppbppppptqbwgp",
    "sfwgwnvghzrn",
    "llwlln",
    "jjjjjjjjjjjjjjbjjjjj",
    "ddddndddd",
    "ccccchcdcccccc",
    "ffxffjffhffzsffffffv",
    "pnbglbbbrdn",
    "vvvvvvfwvvqgc",
    "llklllgllzlm",
    "tttttt",
    "fsfhzcfffvffnrfv",
    "gqqqq",
    "ccccccj",
    "vrgjgkggvpggggggg",
    "nrqnnknbvntszznzmgbn",
    "wslrxrx",
    "ssszbwst",
    "mzmcgnm",
    "gdggggggg",
    "qqqqqknrq",
    "dddddddddddddddzd",
    "bbbpbbbbbbvb",
    "jrppp",
    "wgznngfndc",
    "kcfrnfcpknxfgj",
    "nvdlmzgpllqzllvlqlt",
    "ggggdsgg",
    "kqflshskjhrcgrfcr",
    "gpfwfffffffbfffffff",
    "srssfssshssgsstss",
    "fvfpfbffffb",
    "wwwwwwwwwwwwwwwr",
    "jhjvhwd",
    "rrqtrr",
    "dkwwqwdrxrwwbxwqgww",
    "sssgk",
    "ddtmxd",
    "wwwwwwwwn",
    "rhkgdbbzdszjmbm",
    "jvqmhwbhfqkgzw",
    "qtqqqfqqqqqd",
    "gsxcggzfwgggsl",
    "dprmlmpwpw",
    "mmmmmmmmmkmm",
    "zzzqkfzt",
    "llgllllb",
    "hwwcspbckppcgwb",
    "tnsxtstsstb",
    "fmsm",
    "qnqfqqqqqv",
    "tlgxxwcxgx",
    "kmkknmkgkkkkbkkkkkr",
    "nwbbfcfngbbspnxbj",
    "mmmnmvmvjxmmdmr",
    "fkkkjfbwfkk",
    "tmkfjfqqjmgsjmtz",
    "zzzzzzzzzzzqzzwzzzd",
    "dnnnnnxtnnnnnn",
    "wwwlcwzl",
    "mzbjvzjjvjjgrjj",
    "dhdd",
    "jrcrwjr",
    "bkvvvvvvvjvv",
    "xnwnnx",
    "kkkkkkkkkkkkkkkkkpk",
    "gtjrr",
    "jjjjjj",
    "kkzknk",
    "tttttttttbdlvtq",
    "ddwvz",
    "xlfffrpf",
    "zzdscgwbxtxrd",
    "nsnnnlnnznnxsnnnnnd",
    "gwslkgdgwqdbgws",
    "sfcgkffmfc",
    "dmmwlqmmmmmmmmnm",
    "nnnnnnnnvnnv",
    "lllllllllllllvqlf",
    "tftpttjtnt",
    "ljzdkxtwvbmjtff",
    "stwwlqcbtws",
    "hjwhhvvtpg",
    "vvgvv",
    "ddhcdpdjdddhddmr",
    "kxxllzjrdxlsxx",
    "fffjfrff",
    "bbbbbbbbbbbbhbbbbbbb",
    "znrntdqtmrg",
    "srsvsmsssssssssssss",
    "dzfvpthfxnpnz",
    "jjjjjjrjjcjjw",
    "tllllllljl",
    "mmmmmmmmmmmmmm",
    "llllllllqlllllllggl",
    "dkmqddddzdnrfckdz",
    "gmmmmmlh",
    "ttttxtttttttggbt",
    "jmzvwkjjnjsnjkjxj",
    "gmmnmdvfmmtmmqmmzcj",
    "hvxsjt",
    "kxqczpdxfpjp",
    "kgkkhkdkk",
    "nbjvcgsvf",
    "cxddddnsxd",
    "mmqnjbtbpnmp",
    "jmmmmwvmmhtmmmmlmm",
    "zmmjsqfmqrnzmlmwrjm",
    "vnnnnjn",
    "jjjjjjjcjjjs",
    "rrrrrrrrrrrrrrrrrr",
    "bcncn",
    "bjftxcnzpnjrzxhxlp",
    "tzrlxlzdxzsz",
    "sjtltj",
    "wwwwwww",
    "ppzppcwkvwmwpscfpp",
    "jcjjjjjjjj",
    "mmrm",
    "zkkktkkkkk",
    "bpfldtnfrbdfk",
    "zsskw",
    "xxxxxkxxbxjxlzx",
    "ntgdtbtwnntjbcsfz",
    "sndjqjjdjbfjc",
    "ckqtjtffctwtcp",
    "wvhffpnngzv",
    "tlrncrsg",
    "gjggkfvffg",
    "xpxlxxxmpr",
    "srrrprrrr",
    "gvvnnvhvvvvvdjjvv",
    "psnsgsnfxlscwss",
    "vwwpwx",
    "bbbb",
    "vvvvvvvvvmdvvvvv",
    "ffkffjffff",
    "lqqqqjqqxn",
    "qcvwwwhw",
    "wwwwwwwkjz",
    "tttttttz",
    "xxdxxxhbxxxgcdx",
    "svkl",
    "fffffcfffffffnffw",
    "lsnnnpnnn",
    "lllllllblllll",
    "sssss",
    "zbzt",
    "xxxsx",
    "wlxctcccrqccsccr",
    "vvnvndfqq",
    "rrrtrc",
    "nbbbpfbbbcbzzxbf",
    "sglpschlsgsqbskrd",
    "bvfzktkkkzkkjkkwkf",
    "zkxxxgxxkwxvxgx",
    "dddddddddddddgdwd",
    "pzpprsp",
    "sssbssscjsshwmmszst",
    "stsssssssgmssp",
    "sssssssssgs",
    "jjjjttkj",
    "ssssgsssswsv",
    "kkkkkkkkkkkkkkkwb",
    "bzcdgvrvqg",
    "nnnrn",
    "knnrnnxnnnnjndwn",
    "mtmspm",
    "bbbbbbbbbbbbbbbxb",
    "mmdmqwwbmmlxmjljmm",
    "wzzzzzzzzzzzzszxzvz",
    "qqqqrqqqq",
    "ffhf",
    "ttlqqthvttcttttm",
    "zwwwwlmwwlww",
    "zjjwzkkkjlwzvkp",
    "zkzgzzzxtzz",
    "vvmvvklvvvvgv",
    "ttttttqttttsrttktv",
    "llhlllzllv",
    "blwjjmj",
    "qpffpqskqk",
    "wkdjxcwwswwwwwh",
    "bbbbbbbbbbgtjb",
    "ccccfcbccccccccccc",
    "lflnpwbcmld",
    "qqqqqqflqhtpqq",
    "zzzzzzzzzzzmzzzzzl",
    "xxfhxx",
    "tfrhztxxzzqczznwzz",
    "vlrlmlllldllq",
    "bbwbvbbhbbbbkvnbbbb",
    "frxxx",
    "ffbfffnfffjlffffff",
    "dkrdrgdd",
    "gggwglggvtkglg",
    "sqsnpp",
    "bxjnb",
    "hhfr",
    "tzbpbsbbbtbbbbbbbbbb",
    "pkmcpm",
    "qqqqtqcnqxkqb",
    "ssmssqssssssssts",
    "qqdsqrqq",
    "zzbzzzzpzzzzz",
    "xlrrrgrrrcrktrrhrrrc",
    "qzsrqqqjxwqqhqd",
    "ggggggsggggggggp",
    "wwwkq",
    "ghgpkphhzpmhjpwxq",
    "vcvvvdvvvvvvvpvvhv",
    "qjqbfwbj",
    "nvcwkcndt",
    "cccccccccpccgkclcc",
    "pppppp",
    "hxtxhthhshkrmh",
    "wqjqzqqk",
    "fffffffffffffbqf",
    "vjjjjjfh",
    "trzzppxpptpfp",
    "wwwwwwwmrwwwwwwkwwww",
    "rfffszjfsvn",
    "llllllmlllllllllkl",
    "xssssqssshsss",
    "fxxxsxbnx",
    "nnnnxhnn",
    "gggggggggqg",
    "nknsfcnnnqgnnnxsnfnj",
    "ndnnnnnn",
    "shqphcskmqjqvqqqq",
    "rrrrrjrrrrrr",
    "zfzzzrzzzzzzzzzzzhzz",
    "gkgggqggggggrgg",
    "vvvvvvvvvvvvvvvvvvv",
    "hssq",
    "kdwftsxqcnhph",
    "ttttttttttttttttt",
    "fffffdffffsfffffffff",
    "pppppjvpppppph",
    "bbpbbbbbbtbbbbb",
    "nnnnnnx",
    "rrffrsr",
    "brrrrrrrxrrrrr",
    "ffffkjf",
    "srdrrsrrfrcr",
    "gflbcxtvknsbpjbwrk",
    "pttpkfppcppdznpgpp",
    "qqqqqqqqqqqqqqtqs",
    "rrrwr",
    "zggggggggggg",
    "jjjjjjwb",
    "pwnrvbdzxntvgjjjltqn",
    "fffxffflffffff",
    "cklkk",
    "mmnmmmmmmnm",
    "qqqqqqsqqfsqzqqqq",
    "nnnnnnbnnc",
    "sddnmgfkdddzdd",
    "qfdlftk",
    "pzpppxphphpp",
    "mlwf",
    "jxhhhfztzh",
    "ssnnnssssstsssssd",
    "hlxsx",
    "qhhhhhhwdhph",
    "gxdvcvw",
    "nnnnnkhnfnnnfnnnnnn",
    "lllllllhlnklx",
    "jjjdjcljjjjjjjjjjhj",
    "jkhhhhhhj",
    "tzznzzztcbxkzvssrzzz",
    "sswssscsstnssjssss",
    "zzzxmzgkzspdmq",
    "ndldddpddddddkmdd",
    "zgqrkhxqq",
    "mmmgfdw",
    "dlls",
    "kbbbsbbdbkbqbbbbs",
    "bbbp",
    "xctccckq",
    "kkkkk",
    "bbbbbbzbbnqbbb",
    "jmmhqqlqqkgqcjvqq",
    "ggggpggggghsgg",
    "hhhhchhhbh",
    "zwwcfgwwwwc",
    "xxxxnxlxxx",
    "fffhf",
    "fbwfxr",
    "vrfffd",
    "lrrrrrrrrrrrrfc",
    "tbnptvtt",
    "rrgrrrrrrrrljrrrrrrr",
    "bbpsh",
    "gmmsmmlmmbmmmmmmmgkx",
    "ggggggggggggggrfgg",
    "fcfmr",
    "lclllll",
    "nnnccgvn",
    "npvv",
    "sshsxsswshss",
    "mxxkxxxxsxpkxsxhnktx",
    "fsff",
    "bqzzbcbbbxbk",
    "ssxsssv",
    "pxppjppppppvjp",
    "hhkch",
    "sdshspsssqs",
    "vwkkkkwkskjr",
    "mmmmmmmmmmmrmmmmzm",
    "nwrgwwwjpwwwwpv",
    "ttft",
    "kfffzffzfffk",
    "ddhddddddddcddddd",
    "wplpvlrcqwlblvlcqm",
    "zpzzz",
    "vvgvv",
    "jbgbb",
    "ddddddndddddddd",
    "zszfktzrzjtzzbmn",
    "xbbxrbbbbbb",
    "tttftt",
    "ptvvvcvvvvvnvvvv",
    "tcnnnnln",
    "hhhhhh",
    "jvjjhfjbxjj",
    "twtdttttttttt",
    "zzzzzzzzzwzzzwzzzzzz",
    "hnhcvlnxglxlldlfgvll",
    "jjjjjj",
    "nrknnntn",
    "zzzzzzzqz",
    "zzzzr",
    "bztttgtllwq",
    "ckjnntmkxxcnwkqznp",
    "xbrvvhbbb",
    "dddzqd",
    "wwlwvlwpwwdwwwwqb",
    "lqzmlq",
    "bpzzzczzpzz",
    "nbjqmnhxwh",
    "qqqqqqqzq",
    "rrrrrvrrrrnrrrrrrb",
    "lbfdwlpzmkl",
    "srwcwmgvzjjxj",
    "wwwwwwwjwwwwwwwwww",
    "tttttttttdttttttttvf",
    "pwnwxxfxnkxxtzpglx",
    "lgpglng",
    "llllfll",
    "ppppppp",
    "xxdf",
    "fgkzwrrpmvmhzplsqp",
    "bbqjb",
    "kbkkkwkkk",
    "mppzpprpp",
    "ttfvdtwxnnfdsnxbn",
    "mmzmmm",
    "kkkkndk",
    "hkkrln",
    "hhhhhmhhhhhhh",
    "kkkkkkkkksk",
    "fggggg",
    "qhgrxxhrxjjxxhxgb",
    "lxpxxb",
    "frffsfjff",
    "dpddjfxdrt",
    "fffpxfcfffdfnfzf",
    "rrrrwrrrrrwrrrcfrcr",
    "kqqqqqqqqqqqqnqqqkqq",
    "bgbrf",
    "ssssssssssssssbdss",
    "bnvlpxlctvdd",
    "smsss",
    "cpgrfdfmm",
    "jjjxdqjctjjtnzcjq",
    "xqzgsrdtcvblfpmg",
    "qqqsqqqqwdqqmqqpqh",
    "xddzddtdnk",
    "blppgpppppppx",
    "mqwlxbbcmqf",
    "ltttbttsbtrtts",
    "kqmfkcvkwsnddkk",
    "hrkwqdtckqdktgctj",
    "mmmm",
    "tlflzkxlvlq",
    "jjjjjl",
    "wzfjxsbqznqlx",
    "mssssfzpsh",
    "vvxvvvllvxvvvhrvvv",
    "qchchw",
    "nllnnbnrvnnnmgnzb",
    "vvvfvvvvvvvvpvvvjv",
    "lpbfwlffxhlxfffkf",
    "cfwkwnl",
    "nnnnnn",
    "dcddddddddd",
    "nvrdpnvnlxccjrd",
    "vvjvvvvvvvvvvvhzv",
    "wwlpllqk",
    "pvpp",
    "bmbf",
    "snpdndwgtfqjlzdmmth",
    "rpjjjjlpsj",
    "gdmvgsgjrhg",
    "pppppdspp",
    "dktlkrkwlnd",
    "bbxbbbbbrbbbzbbbbvb",
    "ttqdcwdjtwtqttttbc",
    "njdddddtddhd",
    "fkrfrrrrrrrgr",
    "ssskgxssbs",
    "mtmmmkmm",
    "hhhhjhhhhhgh",
    "kfqkscqnq",
    "sdpddmvdvdxdl",
    "cffzgfffkfff",
    "vvvvcnljw",
    "wplvpp",
    "kxwwww",
    "nmmmmm",
    "ttntttjhctkztt",
    "vvvvvvvvvvqvvf",
    "zzgjqzzzzzdzzkzw",
    "kkkkkkkkfkkkkkk",
    "mmmmmfmgm",
    "fgxsdqmnmmszv",
    "pqppp",
    "sssxqstsssnfsxss",
    "qqqfqqxqqqq",
    "djhvxxm",
    "bjbbbrjgbbxkbgpqd",
    "ccscccjccmclccccccc",
    "djrdgddwxddxskt",
    "hhhhhhzhmh",
    "jwlplsct",
    "qvqhqxnqqmz",
    "wwpwqw",
    "pltzkcsdhphdmdxkb",
    "kkkkkkkkx",
    "dbddqddd",
    "njnrjnnnnnqzn",
    "kkkkkk",
    "gggggggggggcgngg",
    "vxqvvvvvvvxvvvbvvvdv",
    "ssscsssssf",
    "mmmmfmpqmmmmml",
    "sttqtbtjt",
    "vkkshkw",
    "xscxxxm",
    "pddxdddqkdddzmddpd",
    "gtmxtcttqtttxtv",
    "rrrrrrrrrrrrrpr",
    "ddddfdddddddddddmd",
    "xqxxx",
    "wwwwwwwwwwfww",
    "jjjjjj",
    "mmmmmm",
    "fbblbbgbx",
    "vvvvvxvrvd",
    "ffffpwpfqfhfzxdc",
    "mtmdkbvmjmxmmdmmgb",
    "nhbtsbhjhgdh",
    "wwwpwwdwwwd",
    "hffrfffffffqqf",
    "flfcffffffffqdff",
    "hbbv",
    "drdlbdhwdsddd",
    "swrwnbwmkxwt",
    "llflsllllntlll",
    "xvvvvvtkmcvvvxh",
    "hzhbh",
    "bzbbbbbbbgzxbbbbbm",
    "lqwxqmqmjlsmfwltkzl",
    "mwllx",
    "cccnccxcc",
    "tbbsjbkprrdg",
    "zmpzc",
    "jjjltjjj",
    "xzlzgzqhqwgz",
    "mmmmmm",
    "pqwfqh",
    "pppppptfhpzjtct",
    "wdwwwlwwdww",
    "mmmmmsmmmmqmmwmmm",
    "vvhpfnvv",
    "qfqqbxqvqf",
    "zzzzzzzzzzd",
    "gggggfggggggg",
    "ddpdjqdddddmd",
    "nnndnnnnnn",
    "zzgdzzdvz",
    "vdnvnsnnnlnn",
    "bblbl",
    "zpfjkfr",
    "fhtxkhwhgq",
    "xxdxxxpxx",
    "vmxv",
    "zzzzzzzzzzzzzzzzzhrz",
    "hjlsdv",
    "jzxxvkhsxxck",
    "sfjvsjqvss",
    "xxcmpzjjtmkswwr",
    "ssjtvssslwszss",
    "rhhphhhhwwhhhgrhhh",
    "hvhhhhhzh",
    "mmmdqbtssvmfmmmgr",
    "hhbhzhh",
    "ccccqcwxcccc",
    "bmhqqbpfrqfdkq",
    "rcmrdrrrjlrscrrhkl",
    "ttgz",
    "mgggtggggzgzcphhx",
    "dvddfctd",
    "gmglf",
    "dmqwpbhmmktcvc",
    "bzwzzkxqpz",
    "xxxxbwvxcxxcxwznw",
    "zzzzzz",
    "kkkkhk",
    "zqqnrcqqqplqqndfg",
    "jdvxtbqblmllfbnlff",
    "vvkvvv",
    "xbtzjdvp",
    "hhmhhhhhhnmd",
    "ttktgqfzxrdxqf",
    "gggjf",
    "qqqqmmqqqqq",
    "rwjwwwwwrcw",
    "ssbbbcbbbbbbd",
    "bggggv",
    "wwwwfwwxwwww",
    "ppjppppzt",
    "gpzzzdzzjzzpzzzz",
    "vbbfvjz",
    "qdwczqqq",
    "rrrrhwtf",
    "dddddddtdddddddd",
    "ppppppzlp",
    "lbssnssss",
    "lqnpqqqqq",
    "rkqkkk",
    "zgkxmkmgqzg",
    "cbbbbbf",
    "dctcc",
    "sxxtl",
    "qmfqbfmlkkkjzhqjbxpd",
    "lhlghlcbqgsmjhbh",
    "xxxxxxxxx",
    "cvndhntlghk",
    "dddddddcddz",
    "jvjpjj",
    "bmhdpdr",
    "jnjcjnjtd",
    "djsvzshmxmgb",
    "bhpkgzksrkkqk",
    "lllllwlvwlglxqll",
    "hhhhhhhhhm",
    "qqsbq",
    "dlllllgllllszg",
    "lllllllllvlzl",
    "kkkkr",
    "wdwfkwwwwww",
    "rzswrffrqgdhgzm",
    "zzzzz",
    "pplxppppzprpdpjpppdp",
    "mkghrgcmhxggcpddvx",
    "sjwbqv",
    "qvpqlsqqqrnbp",
    "wwwwwwwwwwwwwwwwwwwg",
    "mmmmlmmmmmfwmmm",
    "xxxxvxsxxxxwcjxvxxx",
    "ggdggggggggggx",
    "xrqtf",
    "qqqvqcqjqjfqqxqqkqj",
    "cvszc",
    "mwwwwwvwx",
    "hrgrckfb",
    "ddddgxcznlnwdddd",
    "drhvzzc",
    "pfnjnnhpnppfnrbhz",
    "bnnnb",
    "hhhhhphhhhhthhgh",
    "lfllpmknmplx",
    "lcchctpccccccx",
    "lznsllwlbllmllzl",
    "wwwwqshnwkwwww",
    "zxkknkxkrkckkr",
    "rdrkbkgrrrrznrr",
    "ggggggvgggggr",
    "vgxggr",
    "cccthsc",
    "nptdqmfglpzdvwkspt",
    "qqsxdqqfngqqkqq",
    "rrrrrrrrrrrrrrrrxrf",
    "ttttmthttttt",
    "lllwl",
    "xxxxxxxbxxxwxxx",
    "dddddbfddddcdd",
    "lfffb",
    "ffff",
    "swdckstss",
    "pbfzppbmp",
    "xpqssm",
    "hlhhqdhhmhhhhhphhrh",
    "pbbgxbhqbzvnwxxb",
    "nrbdnnnwnnnlnn",
    "mspcrbgfqs",
    "rlvmtklhllrvljlllx",
    "mpppppprpmjs",
    "gpggwmggggv",
    "qqdqlqq",
    "xxxxxxxxtxxxxxfxxxx",
    "hhhshh",
    "qllq",
    "ggggggggggsg",
    "mrwbr",
    "knhmnm",
    "pppppppppppppp",
    "vvtvvvvfvztvvvxwqlg",
    "rrrrprrrrrzrr",
    "zzzzzzjzzzzz",
    "ddddddddddddddddjdd",
    "ccncdc",
    "gxxvqx",
    "ggpg",
    "blgvnmbxhpmxmb",
    "wvdwwfqjwqwvmfrzw",
    "ssdw",
    "mmllqlcllklxcml",
    "lllplllllllllllllnl",
    "qtwhwz",
    "xvvwfvfvv",
    "kkkkkkkkqkkk",
    "qzvvpf",
    "whhddttshzds",
    "qjlqqqqqpwqqgqq",
    "nwwwdmvwwwwk",
    "rlljllhljl",
    "sssssssssd",
    "qcgrtxrfccgc",
    "pnnngnnnvnwdnn",
    "cscckctxcrhgc",
    "ngnncrnn",
    "ztzqttbmtztpsnrnt",
    "bbbbbbbbbbbb",
    "czzzzzzz",
    "kktmkf",
    "xxxjsxxnxxxxxqxxpx",
    "vxvvrvvvdvvvvvbv",
    "nsslmtsdxbxfsfssswfx",
    "lllllllllllfcltllll",
    "qqqqtqdqq",
    "nnnnnnnnnnnnnnbnnnn",
    "ssttshsks",
    "rrrrprrrjrlqrrsrrrrb",
    "qqqqqqqqqq",
    "wcbw",
    "tcggrxgrgvl",
    "kllswlmlglps",
    "qkvvvvqxjktkvvvjv",
    "rrrrqrrrrrrrrrr",
    "nnnnmb",
    "xktqbxgkkjwlt",
    "kskk",
    "kkdkkkkkkkkkkkkwk",
    "vnnnnnnnnqjnnknnnr",
    "mfmmgmmqmmmmmkmmmmmm",
    "kkkgkkkjkkk",
    "hhhhhhhhhhhlhhhqh",
    "ppprpppppp",
    "gbdxggmggrgw",
    "vfffff",
    "jzvwdmsqpdnh",
    "wwcwgzwwwwwhwwwwww",
    "zzdzzlzvgzzztzzz",
    "ffftff",
    "xxxxxxxrxxxxxx",
    "kgrrvrrvrr",
    "zkpwxkk",
    "tkckkfcvxkxk",
    "rbrrrrgrrrrkbrrr",
    "ttxx",
    "zzzzzzzzzzz",
    "pnwhxpmxpfskq",
    "dddddddddddddddgdddd",
    "kctcccg",
    "sqqqrqqq",
    "fjfffffqfkfffjffff",
    "knvrrqvtv",
    "ptgswlvpdnmr",
    "vvkmnvdwk",
    "rrrrrrrrrhlrwrrrd",
    "xffffxjfffzfffpf",
    "nppppppp",
    "xxxxxxxxxxxxxxsx",
    "prhbmdvwcmtzpvbb",
    "rngdttnzqtjtcttdvbmt",
    "ffvldffqfqffgf",
    "rzqrmz",
    "tntnnnntn",
    "qpvlfbsgswsnwsmpz",
    "tvwstttttttktwt",
    "vslqbgg",
    "dsdddddddrdddddhdbdd",
];
