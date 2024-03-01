use bitvec::prelude::*;

fn check_circuit(proc_id: usize, v: &BitSlice<u32>) -> bool {
   if ((v[0] || v[1]) && (!v[1] || !v[3]) && (v[2] || v[3])
           && (!v[3] || !v[4]) && (v[4] || !v[5])
           && (v[5] || !v[6]) && (v[5] || v[6])
           && (v[6] || !v[15]) && (v[7] || !v[8])
           && (!v[7] || !v[13]) && (v[8] || v[9])
           && (v[8] || !v[9]) && (!v[9] || !v[10])
           && (v[9] || v[11]) && (v[10] || v[11])
           && (v[12] || v[13]) && (v[13] || !v[14])
           && (v[14] || v[15]))
       &&
          ((v[16] || v[17]) && (!v[17] || !v[19]) && (v[18] || v[19])
           && (!v[19] || !v[20]) && (v[20] || !v[21])
           && (v[21] || !v[22]) && (v[21] || v[22])
           && (v[22] || !v[31]) && (v[23] || !v[24])
           && (!v[23] || !v[29]) && (v[24] || v[25])
           && (v[24] || !v[25]) && (!v[25] || !v[26])
           && (v[25] || v[27]) && (v[26] || v[27])
           && (v[28] || v[29]) && (v[29] || !v[30])
           && (v[30] || v[31]))
      {
        println!("Process {} found {}", proc_id, v);
        true
    } else {
        false
    }
}

fn main() {
    let total = (0..u32::MAX)
        .filter(|b| {
            if b % (1 << 24) == 0 {
                println!("Checking {:>3 }/256", 1 + b / (1 << 24))
            }
            check_circuit(0, b.view_bits::<Lsb0>())
        }).count();
    println!("A total of {} solutions were found.", total)
}
