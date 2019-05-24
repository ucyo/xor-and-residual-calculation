use bit_vec::BitVec;
use std::collections::HashMap;
use std::thread;
use std::io::{BufWriter, BufReader, Read, Write};
use compress::entropy::ari;
use pzip_bwt::{apply_bwt as abwt};

pub fn encode_bwt_range(data: &[u32]) -> FileContainer {
    let mut plzc = 0i32;
    let pfoc : i32;
    let bata = (*data).to_vec().clone();

    // split lzc calculation into separate method
    let h = thread::spawn(move || {
        let mut lzc = get_lzc(&bata);
        // debug!("L {:?} [encoded]", lzc);
        plzc = abwt(&mut lzc);
        apply_range_coding(&lzc) // bwt_range(lzc)
    });

    let mut foc = get_foc(&data);
    let cfoc = foc.clone();
    // debug!("F {:?} [encoded]", foc);
    pfoc = abwt(&mut foc);
    let efoc = apply_range_coding(&foc); // bwt_range(foc)

    let mut bv = BitVec::new();
    bv.push(true);
    'outer: for (i, &d) in data.iter().filter(|&&x| x != 0).enumerate() {
        let v = u32_to_bool(d);
        if v.len() == (cfoc[i] + 1) as usize {
            continue 'outer
        }
        for val in v.iter().skip(usize::from(cfoc[i] + 1)) {
            bv.push(*val)
        }
    }
    let residuals = bv.to_bytes();
    let lzc = h.join().unwrap();  // merge with lzc spawn
    // debug!("R: {:?}", residuals);
    FileContainer::new(0, data.len(), [plzc, pfoc],lzc, Vec::new(), efoc, residuals, HashMap::new(), HashMap::new())
}


pub struct FileContainer {
    start: u8,
    bwt: [i32;2],
    size : usize,
    huff_lzc: Vec<u8>,
    raw_sign: Vec<u8>,
    huff_6re: Vec<u8>,
    raw_res6: Vec<u8>,
    huff_lzc_codebook: HashMap<u8, BitVec>,
    huff_6re_codebook: HashMap<u8, BitVec>,
}

impl FileContainer {
    pub fn new(
        start: u8,
        size : usize,
        bwt : [i32;2],
        huff_lzc: Vec<u8>,
        raw_sign: Vec<u8>,
        huff_6re: Vec<u8>,
        raw_res6: Vec<u8>,
        huff_lzc_codebook: HashMap<u8, BitVec>,
        huff_6re_codebook: HashMap<u8, BitVec>,
    ) -> Self {
        FileContainer {
            start,
            bwt,
            size,
            huff_lzc,
            raw_sign,
            huff_6re,
            raw_res6,
            huff_lzc_codebook,
            huff_6re_codebook,
        }
    }
    pub fn nbytes(&self) -> usize {
        self.huff_lzc.len() + self.raw_sign.len() + self.huff_6re.len() + self.raw_res6.len() + 1 // +1 is for start
    }
}

impl std::fmt::Display for FileContainer {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "outbytes={}", self.nbytes())
    }
}

pub fn u32_to_bool(value: u32) -> Vec<bool> {
    let mut result: Vec<bool> = Vec::new();
    if value.is_power_of_two() {
        result.push(true)
    }
    let mut pow = value.next_power_of_two() >> 1;
    while pow > 0 {
        result.push(value & pow > 0);
        pow >>= 1;
    }
    result
}

pub fn get_lzc(data: &[u32]) -> Vec<u8> {
    data.iter()
        .map(|&x| x.leading_zeros() as u8)
        .collect::<Vec<u8>>()
}

pub fn get_foc(data: &[u32]) -> Vec<u8> {
    data.iter().filter(|&&x| x != 0).map(|&x| _foc(x)).collect::<Vec<u8>>()
}

fn _foc(val: u32) -> u8 {
    let mut result = 32 - val.leading_zeros();
    let mut ix = 0;
    while result > 0 && !((val >> ix) + 1).is_power_of_two() {
        ix += 1;
        result -= 1
    }
    result as u8
}

pub fn apply_range_coding(data: &[u8]) -> Vec<u8> {
    let mut e = ari::ByteEncoder::new(BufWriter::new(Vec::new()));
    e.write_all(data).unwrap();
    let (encoded, _) = e.finish();
    encoded.into_inner().unwrap()
}

pub fn reverse_range_coding(data: &[u8]) -> Vec<u8> {
    let mut d = ari::ByteDecoder::new(BufReader::new(&data[..]));
    let mut decoded = Vec::new();
    d.read_to_end(&mut decoded).unwrap();
    decoded
}
