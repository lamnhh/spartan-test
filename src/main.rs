extern crate curve25519_dalek;

use curve25519_dalek::scalar::Scalar;
use libspartan::{InputsAssignment, Instance, NIZKGens, VarsAssignment, NIZK};
use merlin::Transcript;
use ndarray::{Array1, Array2};
use ndarray_npy::read_npy;
use std::fs::File;
use std::io::Write;

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
    let a = read_matrix("data/a.npy");
    let b = read_matrix("data/b.npy");
    let c = read_matrix("data/c.npy");

    let num_cons = 3702739 + 1;
    let num_vars = 4105740;
    let num_inputs = 28 * 28 + 10;
    // let num_nz = cmp::max(cmp::max(A.len(), B.len()), C.len()) + 10;

    let inst = Instance::new(num_cons, num_vars, num_inputs, &a, &b, &c).unwrap();
    let gens = NIZKGens::new(num_cons, num_vars, num_inputs);

    println!("[+] Reading witness");
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
    let assignment_vars = VarsAssignment::new(&vars).unwrap();
    let assignment_inps = InputsAssignment::new(&inps).unwrap();

    println!("[+] Calculating proof");
    let mut prover_transcript = Transcript::new(b"nizk_example");
    let proof = NIZK::prove(
        &inst,
        assignment_vars,
        &assignment_inps,
        &gens,
        &mut prover_transcript,
    );

    let ser = serde_json::to_string(&proof).unwrap();
    println!("[+] Proof size: {}", ser.len());

    println!("[+] Writing proof to proof.dat");
    let mut w = File::create("proof.dat").unwrap();
    writeln!(&mut w, "{}", ser).unwrap();

    let mut verifier_transcript = Transcript::new(b"nizk_example");
    assert!(proof
        .verify(&inst, &assignment_inps, &mut verifier_transcript, &gens)
        .is_ok());

    println!("[+] Finished");
}
