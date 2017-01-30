



use std::cmp::{min, max};


#[derive(Debug)]
/*struct Match {
    back: u32,
    copy: u32,
    char: Option<u8>,
}*/
enum Match {
    Repetition(i64, usize),
    Character(u8),
}



fn vec_eq2(a: Vec<&u8>, b: Vec<&u8>) -> bool {
    //for x in 0..min(a.len(), b.len()) {
    //print!("vec_eq: {:?} {:?}", a, b);
    if a.len() != b.len() {
        //println!(": false (size)");
        return false;
    }
        
    for x in 0..min(a.len(), b.len()) {
        if a[x] != b[x] {
            //println!(": false");
            return false;
        }
    }
    //println!(": true");
    return true;
}

fn vec_eq(v: &Vec<u8>, a: usize, b: usize, len: usize) -> bool {
    if (v.len() < a + len || v.len() < b + len) {
        return false;
    }
    
    for x in 0..len {
        if v[a + x] != v[b + x] {
            return false;
        }
    }

    return true;
}
    
    

struct Prs {
    //src: &'a Vec<u8>,
    result: Vec<u8>,
    pub result_index: usize,
    bitpos: u8,
    control_index: usize,
    
    
}

impl Prs {
    pub fn new(size: usize) -> Prs {
        let mut result = Vec::new();
        //result.resize((9*size)/8 + 2, 0); // some crazy formula, from kohle's code
        result.resize(size*2,  0); // some crazy formula, from kohle's code
        Prs {
            result: result,
            bitpos: 0,
            control_index: 0,
            result_index: 1,
        }
    }

    fn control_bit(&mut self, bit: u8) {
        /*self.control = self.control >> 1;
        self.control |= (bit << 7);
        self.bitpos += 1;
        if self.bitpos >= 8 {
            self.bitpos = 0;
            self.control = &result[self.result_index];
            self.result_index++;
    }*/
        self.control_bit_nosave(bit);
        self.control_bit_save();
    }

    fn control_bit_nosave(&mut self, bit: u8) {
        self.result[self.control_index] = self.result[self.control_index] >> 1;
        self.result[self.control_index] |= (bit << 7);
        self.bitpos += 1;
    }

    fn control_bit_save(&mut self) {
        if self.bitpos >= 8 {
            self.bitpos = 0;
            self.control_index = self.result_index;
            self.result_index += 1;
        }
    }

    fn static_data(&mut self, byte: u8)  {
        self.result[self.result_index] = byte;
        self.result_index += 1;
    }
    
    pub fn raw_byte(&mut self, byte: u8) {
        self.control_bit_nosave(1);
        self.static_data(byte);
        self.control_bit_save();
    }

    fn short_copy(&mut self, back2: i64, copy2: usize) {
        let back = back2 as u8;
        let copy = copy2 as u8 - 2;
        self.control_bit(0);
        self.control_bit(0);
        self.control_bit((copy >> 1) & 1);
        self.control_bit_nosave(copy & 1);
        self.static_data(back & 0xFF);
        self.control_bit_save();
    }

    fn long_copy(&mut self, back2: i64, copy2: usize) {
        //let back = back2 as u8;
        let copy = copy2 as u8 - 2;
        self.control_bit(0);
        self.control_bit_nosave(1);
        self.static_data(((back2 << 3) as u8 & 0xF8) | (copy & 0x07));
        self.static_data((back2 >> 5) as u8 & 0xFF);
        self.control_bit_save();
    }

    fn longer_copy(&mut self, back2: i64, copy2: usize) {
        //let back = back2 as u8;
        let copy = copy2 as u8;
        self.control_bit(0);
        self.control_bit_nosave(1);
        self.static_data((back2 << 3) as u8 & 0xF8);
        self.static_data((back2 >> 5) as u8 & 0xFF);
        self.static_data(copy - 1);
        self.control_bit_save();
    }

    pub fn copy_length(&mut self, back: i64, copy: usize) {
        if back > -0x100 && copy <= 5 {
            self.short_copy(back, copy);
        }
        else if copy <= 9 {
            self.long_copy(back, copy);
        }
        else {
            self.longer_copy(back, copy);
        }
        
    }

    #[allow(exceeding_bitshifts)]
    pub fn finalize(&mut self) {
        self.control_bit(0);
        self.control_bit(1);
        //if self.bitpos != 0 {
            //self.result[self.control_index] = (self.result[self.control_index] << self.bitpos) >> 8
        //}
        self.static_data(0);
        self.static_data(0);
    }
    
    pub fn as_bytes(&self) -> Vec<u8> {
        self.result.clone().into_iter().take(self.result_index+2).collect()
    }
}


pub fn compress(data: &Vec<u8>) -> Vec<u8> {
    let mut prs = Prs::new(data.len());
    
    let mut matches = Vec::new();
    let mut x = 0;
    while x < data.len() {
        let mut match_index = 0;
        let mut match_size = 0;
        let mut diff:i64 = 0;
        //let mut y = 0;
        let mut y = if x > 3 { x - 3 } else { 0 };
        let ymin = if x > 0x1ff0 { x - 0x1ff0 } else { 0 };
        //'outer: while y < ymax && y > ymin {
        'outer: while y > 0 && y > ymin {
        //while y < x {
            //for msize in (3..min(data.len() - x+1, 256)).rev() {
            for msize in (3..256).rev() {
                //if vec_eq(data.iter().skip(x).take(msize).collect(), data.iter().skip(y).take(msize).collect())
                if vec_eq(&data, x, y, msize)
                    && y + msize < x
                    && x + msize < data.len()
                {
                    
                    //diff = x - y;
                    //diff = y as i64 - x as i64;
                    diff = -(x as i64 - y as i64);
                    match_size = max(msize, match_size);
                    break 'outer;
                }
            }
            y -= 1;
        }
        

        //println!("match_size {}", match_size);
        let m = if match_size == 0 {
            /*Match {
                back:0,
                copy: 0,
                char: Some(data[x]),
        }*/
            Match::Character(data[x])
        }
        else {
            x += match_size -1;
            /*Match {
                back: diff as u32,
                copy: match_size as u32,
                char: None,
        }*/
            Match::Repetition(diff, match_size)
        };

        
        //println!("match: {:?}", m);
        
        match m {
            Match::Character(ch) => {
                println!("{:08X}->{:08X} byte",  x, prs.result_index);
                prs.raw_byte(ch);
            }
            Match::Repetition(back, copy) => {
                println!("{:08X}->{:08X} copy {:08X} {:08X} {:?}",  x-(match_size-1),
                         prs.result_index, diff as i32, match_size as i32,
                         data.iter().skip(x-(match_size-1)).take(match_size).collect::<Vec<&u8>>());
                prs.copy_length(back, copy);
            }
        }
        
        //println!("{:?}", prs.as_bytes());
        
        //println!("match: {:?}", m);
        //matches.push(m);

        x += 1;
    }

    println!("{:#?}", matches);


    //let mut prs = Vec::new();
    for m in matches.iter() {
        match m {
            &Match::Character(ch) => {
                prs.raw_byte(ch);
            }
            &Match::Repetition(back, copy) => {
                prs.copy_length(back, copy);
            }
        }
    }
    prs.finalize();

    let bytes = prs.as_bytes();
    println!("prs: {:?}", bytes);
    
    bytes
}


fn slice_eq(a: Vec<&u8>, b: Vec<&u8>, len: usize ) -> bool {
    //for x in 0..min(a.len(), b.len()) {
    println!("slice_eq: {:?} {:?}, {}", a, b, len);
    for x in 0..len {
        if a[x] != b[x] {
            return false;
        }
    }
    return true;
}


pub fn compress2(data: &Vec<u8>) -> Vec<u8> {
    //let mut matches = Vec::new();
    
    let mut x:i64 = 0;
    let size: i64 = data.len() as i64;
    while x < size {
        println!("while x({}) < size({})", x, size);
        let mut lsoffset:i64= 0;
        let mut lssize:i64 = 0;
        let mut xsize:i64 = 0;
        let mut y:i64 = x - 3;
        while (y > 0) && (y > (x - 0x1ff0) && (xsize < 255)) {
            xsize = 3;
            //if slice_eq(data.as_slice()[y..y+xsize], data.as_slice()[x..x+xsize], xsize) {
            if slice_eq(data.iter().skip(y as usize).collect(), data.iter().skip(x as usize).collect(), xsize as usize) {
                //while (data.as_slice()[y..y+xsize] == data.as_slice(x..x+xsize))
                //while (slice_eq(data.as_slice()[y..y+xsize], data.as_slice()[x..x+xsize]), xsize)
                while slice_eq(data.iter().skip(y as usize).collect(), data.iter().skip(x as usize).collect(), xsize as usize)
                    && (xsize < 256)
                    && ((y + xsize) < x)
                    && ((x + xsize) <= size)
                {
                    xsize += 1;
                }
                xsize -= 1;
                if (xsize > lssize) {
                    //lsoffset = -(x - y);
                    lsoffset = y - x;
                    lssize = xsize;
                }
            }
            
            
            y -= 1;
        }
        if (lssize == 0) {
            //rawbyte
            /*matches.push(Match {
                back: 0,
                copy: 0,
                char: Some(data[x as usize]),
            });*/
        }
        else {
            //prs-copy
            x += (lssize - 1);
        }
        
        x += 1;
        //println!("matches: {:?}", matches);
    }


    //prs-finish

    Vec::new()
}



// sylv
/*pub fn compress(data: &Vec<u8>) -> Vec<u8> {
}*/
