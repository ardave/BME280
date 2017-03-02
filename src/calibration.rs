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

    pub h1 : u16,
    pub h2 : u16,
    pub h3 : u16,
    pub h7 : u16    
}

// impl fmt::Display for Calibration {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "(t1:{}, t2:{}, t3:{}, p1:{}, p2:{}, p3:{}, p4:{}, p5:{}, p6:{}, p7:{}, p8:{}, p9:{}, h1:{}, h2:{}, h3:{}, h7:{})", 
//         self.t1, self.t2, self.t3, 
//         self.p2, self.p2, self.p3, self.p4, self.p5, self.p6, self.p7, self.p8, self.p9, 
//         self.h1, self.h2, self.h3, self.h7)
//     }
// }