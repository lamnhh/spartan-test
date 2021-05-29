extern crate curve25519_dalek;

use curve25519_dalek::scalar::Scalar;
#[allow(unused_imports)]
use libspartan::{InputsAssignment, Instance, SNARKGens, VarsAssignment, SNARK};
#[allow(unused_imports)]
use merlin::Transcript;
#[allow(unused_imports)]
use ndarray::{Array1, Array2, Axis};
use ndarray_npy::read_npy;
use std::cmp;

fn read_matrix(path: &str) -> Vec<(usize, usize, [u8; 32])> {
    let mut ans: Vec<(usize, usize, [u8; 32])> = Vec::new();
    let raw: Array2<i32> = read_npy(path).unwrap();
    let n_row = raw.shape()[0];
    for row in 0..n_row {
        let u = raw[(row, 0)] as usize;
        let v = raw[(row, 1)] as usize;
        let w = if raw[(row, 2)] < 0 {
            (-Scalar::from((-raw[(row, 2)]) as u32)).to_bytes()
        } else {
            Scalar::from(raw[(row, 2)] as u32).to_bytes()
        };
        ans.push((u, v, w))
    }
    return ans;
}

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
    #[allow(non_snake_case)]
    let A = read_matrix("data/a.npy");
    #[allow(non_snake_case)]
    let B = read_matrix("data/b.npy");
    #[allow(non_snake_case)]
    let C = read_matrix("data/c.npy");

    let num_cons = 3702739 + 1;
    let num_vars = 4105740;
    let num_inputs = 28 * 28 + 10;
    let num_nz = cmp::max(cmp::max(A.len(), B.len()), C.len()) + 10;

    let inst = Instance::new(num_cons, num_vars, num_inputs, &A, &B, &C).unwrap();
    println!("[+] inst");
    let gens = SNARKGens::new(num_cons, num_vars, num_inputs, num_nz);
    println!("[+] gens");
    let (comm, decomm) = SNARK::encode(&inst, &gens);
    println!("[+] encode");

    let mut vars: Vec<[u8; 32]> = Vec::new();
    let mut inps: Vec<[u8; 32]> = Vec::new();
    let raw_w: Array1<i32> = read_npy("data/w.npy").unwrap();
    for (i, &val) in raw_w.iter().enumerate() {
        if i == num_vars {
            continue;
        }
        if i < num_vars {
            vars.push(to_bytes(val));
        } else {
            inps.push(to_bytes(val));
        }
    }

    println!("[+] read w");

    let assignment_vars = VarsAssignment::new(&vars).unwrap();
    let assignment_inps = InputsAssignment::new(&inps).unwrap();

    let mut prover_transcript = Transcript::new(b"snark_example");
    println!("[+] prepare proof");
    let proof = SNARK::prove(
        &inst,
        &decomm,
        assignment_vars,
        &assignment_inps,
        &gens,
        &mut prover_transcript,
    );
    println!("[+] calculated proof");

    // let mut verifier_transcript = Transcript::new(b"snark_example");
    // assert!(proof
    //     .verify(&comm, &assignment_inps, &mut verifier_transcript, &gens)
    //     .is_ok());
    // println!("proof verification successful!");
}
