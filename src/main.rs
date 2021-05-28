#[allow(unused_imports)]
use libspartan::{InputsAssignment, Instance, SNARKGens, VarsAssignment, SNARK};
#[allow(unused_imports)]
use merlin::Transcript;
#[allow(unused_imports)]
use ndarray::{Array1, Array2, Axis};
use ndarray_npy::read_npy;
use std::cmp;

fn to_bytes(n: i32) -> [u8; 32] {
    let mut a: [u8; 32] = [0; 32];
    let mut val = n;
    for i in 0..32 {
        a[i] = (val % 256) as u8;
        val = val / 256;
    }
    return a;
}

fn main() {
    let raw_a: Array2<i32> = read_npy("data/a.npy").unwrap();
    let raw_b: Array2<i32> = read_npy("data/b.npy").unwrap();
    let raw_c: Array2<i32> = read_npy("data/c.npy").unwrap();

    #[allow(non_snake_case)]
    let mut A: Vec<(usize, usize, [u8; 32])> = Vec::new();
    #[allow(non_snake_case)]
    let mut B: Vec<(usize, usize, [u8; 32])> = Vec::new();
    #[allow(non_snake_case)]
    let mut C: Vec<(usize, usize, [u8; 32])> = Vec::new();

    for row in raw_a.axis_iter(Axis(0)) {
        A.push((row[0] as usize, row[1] as usize, to_bytes(row[2])));
    }
    for row in raw_b.axis_iter(Axis(0)) {
        B.push((row[0] as usize, row[1] as usize, to_bytes(row[2])));
    }
    for row in raw_c.axis_iter(Axis(0)) {
        C.push((row[0] as usize, row[1] as usize, to_bytes(row[2])));
    }

    let num_cons = 3702739 + 1;
    let num_vars = 4105740;
    let num_inputs = 28 * 28 + 10;
    let num_nz = cmp::max(cmp::max(A.len(), B.len()), C.len()) + 10;

    let inst = Instance::new(num_cons, num_vars, num_inputs, &A, &B, &C).unwrap();
    println!("[+] inst");
    let gens = SNARKGens::new(num_cons, num_vars, num_inputs, num_nz);
    println!("[+] gens");
    let (comm, decomm) = SNARK::encode(&inst, &gens);
    println!("[+] encode")

    // let mut vars: Vec<[u8; 32]> = Vec::new();
    // let mut inps: Vec<[u8; 32]> = Vec::new();
    // let raw_w: Array1<i32> = read_npy("data/w.npy").unwrap();
    // for (i, &val) in raw_w.iter().enumerate() {
    //     if i == num_vars {
    //         continue;
    //     }
    //     if i < num_vars {
    //         vars.push(to_bytes(val));
    //     } else {
    //         inps.push(to_bytes(val));
    //     }
    // }

    // let assignment_vars = VarsAssignment::new(&vars).unwrap();
    // let assignment_inps = InputsAssignment::new(&inps).unwrap();

    // let mut prover_transcript = Transcript::new(b"snark_example");
    // let proof = SNARK::prove(
    //     &inst,
    //     &decomm,
    //     assignment_vars,
    //     &assignment_inps,
    //     &gens,
    //     &mut prover_transcript,
    // );

    // let mut verifier_transcript = Transcript::new(b"snark_example");
    // assert!(proof
    //     .verify(&comm, &assignment_inps, &mut verifier_transcript, &gens)
    //     .is_ok());
    // println!("proof verification successful!");
}
