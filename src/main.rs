// main.rs — Metatron Dynamics, Inc.
// ABR Language Structure — V1.0
// Bounded over D. No claim beyond D.
//
// Declaration:
//   Coherence is a relational property, not a linguistic one.
//   Two loci are coherent if their co-occurrence in the declared
//   sequence is stable across exposures.
//
//   M measures the state at each locus as the token identity.
//   No embedding. No borrowed structure.
//
//   ρ_n(i,j) = 1.0 if token j appears within declared proximity
//              of token i in this pass. 0.0 otherwise.
//
//   Accumulation rule (Origin-declared):
//     ρ_acc(i,j) ← ρ_acc(i,j) + η · ρ_n(i,j) · (1 − ρ_acc(i,j))
//
//   Admission: ρ_acc(i,j) >= θ_min
//
//   k per locus: count of j admitted from locus i.
//   Mean k: average across all loci.
//
// Open Conditions:
//   OC-COH-1: proximity window W is Origin-declared, not derived.
//             Its value is an open condition.
//   OC-COH-2: θ_min calibration — currently inherited from
//             abr-relational-attention (0.05). Not yet measured
//             from this domain.
//   OC-COH-3: single-pass ρ_n ∈ {0,1} is the simplest declaration.
//             A graded ρ_n (e.g. decaying with distance within W)
//             is a candidate for Phase 2.

mod coherence;
use coherence::{CoherenceConfig, measure_coherence, format_report};

fn main() {
    let config = CoherenceConfig {
        window: 4,
        theta_min: 0.05,
        eta: 0.1,
        passes: 20,
    };

    // Coherent passage — plain English, natural structure
    let coherent = "the cat sat on the mat the cat looked at the mat \
                    the mat was under the cat the cat sat still";

    // Incoherent passage — same words, order destroyed
    let incoherent = "mat the on cat sat the looked cat the mat the \
                      was mat under cat the still sat cat the";

    println!("ABR Language Structure — V1.0");
    println!("Metatron Dynamics, Inc. Bounded over D. No claim beyond D.");
    println!("Window W={}, θ_min={}, η={}, passes={}",
        config.window, config.theta_min, config.eta, config.passes);
    println!();

    let r_coherent   = measure_coherence(coherent,   &config);
    let r_incoherent = measure_coherence(incoherent, &config);

    println!("── Coherent passage ─────────────────────────────────────────");
    print!("{}", format_report(&r_coherent));
    println!();

    println!("── Incoherent passage ───────────────────────────────────────");
    print!("{}", format_report(&r_incoherent));
    println!();

    println!("── Comparison ───────────────────────────────────────────────");
    println!("Mean k (coherent):   {:.4}", r_coherent.mean_k);
    println!("Mean k (incoherent): {:.4}", r_incoherent.mean_k);
    if r_coherent.mean_k < r_incoherent.mean_k {
        println!("Result: coherent text admits narrower relational width.");
        println!("Structural reduction available: {:.2}x",
            r_incoherent.mean_k / r_coherent.mean_k);
    } else {
        println!("Result: no separation observed at these parameters.");
        println!("Open condition OC-COH-1 (window) or OC-COH-2 (θ_min) \
                  may require revision.");
    }
    println!();
    println!("OC-COH-1 OPEN: window W={} is declared, not derived.", config.window);
    println!("OC-COH-2 OPEN: θ_min={} inherited, not yet calibrated for this domain.",
        config.theta_min);
    println!("OC-COH-3 OPEN: binary ρ_n — graded decay is Phase 2 candidate.");
}
