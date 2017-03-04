#[derive(Debug)]
pub struct Calibration {
    // Still need to consider signed-ness and endianness:
    pub t1 : u16,
    pub t2 : u16,
    pub t3 : u16,

    pub p1 : u16,
    pub p2 : i16,
    pub p3 : u16,
    pub p4 : u16,
    pub p5 : u16,
    pub p6 : i16,
    pub p7 : u16,
    pub p8 : i16,
    pub p9 : u16,

    pub h1 : u8,
    pub h2 : u16,
    pub h3 : u8,
    pub h4 : u16,
    pub h5 : u16,
    pub h6 : i8,
    pub h7 : u16    
}
