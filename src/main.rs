// main.rs — Metatron Dynamics, Inc.
// ABR Language Structure — V3.1
// Bounded over D. No claim beyond D.
//
// Controlled intervention experiment.
// Same operator at character, word, and sentence scale.
// Ground truth known — we performed the intervention.

mod coherence;
mod character;
mod state;
mod intervention;

use character::{CharConfig, edge_profile as char_edge_profile,
                compare_profiles, format_comparison};
use intervention::{
    Sequence, Intervention, apply, edge_profile, compare,
    format_intervention_report,
};

fn run_intervention(
    seq: &Sequence,
    iv:  &Intervention,
    scale: &str,
) {
    let variant  = apply(seq, iv);
    let ref_prof = edge_profile(seq);
    let var_prof = edge_profile(&variant);
    let delta    = compare(&ref_prof, &var_prof);
    print!("{}", format_intervention_report(
        seq, &variant, &ref_prof, &var_prof, &delta, iv
    ));
    println!("  Scale: {}", scale);
    println!();
}

fn main() {
    println!("ABR Language Structure — V3.1");
    println!("Metatron Dynamics, Inc. Bounded over D. No claim beyond D.");
    println!("Controlled intervention experiment.");
    println!("Observable: Δ(declared relations) under known interventions.");
    println!("Scale invariance test: same operator at all resolutions.");
    println!();

    // ── Character scale ───────────────────────────────────────────────
    println!("════════════════════════════════════════════════════════════");
    println!("SCALE: Character");
    println!("════════════════════════════════════════════════════════════");
    println!();

    let char_config = CharConfig { theta_min: 0.05, eta: 0.1 };

    // Retained from V2.2 — character edge profile comparison.
    let char_pairs = vec![
        ("clean",  "problem",   "Transposition(1,2)", "porblem"),
        ("clean",  "difficult", "disrupted",          "dificfult"),
    ];
    for (rl, rt, vl, vt) in &char_pairs {
        let rp    = char_edge_profile(rt, &char_config);
        let vp    = char_edge_profile(vt, &char_config);
        let delta = compare_profiles(&rp, &vp);
        print!("{}", format_comparison(rl, rt, vl, vt, &rp, &vp, &delta));
        println!();
    }

    // Also via intervention module at character scale.
    let problem = Sequence::from_chars("problem", "problem");
    run_intervention(
        &problem,
        &Intervention::Transposition(1, 2),
        "character",
    );

    // ── Word scale ────────────────────────────────────────────────────
    println!("════════════════════════════════════════════════════════════");
    println!("SCALE: Word");
    println!("Base: \"the dog chased the ball\"");
    println!("Same intervention types as character scale.");
    println!("════════════════════════════════════════════════════════════");
    println!();

    let base_word = Sequence::from_words(
        "the dog chased the ball", "the dog chased the ball"
    );

    // Transposition(1,2): dog ↔ chased
    run_intervention(&base_word, &Intervention::Transposition(1, 2), "word");

    // Transposition(3,4): the ↔ ball (end swap)
    run_intervention(&base_word, &Intervention::Transposition(3, 4), "word");

    // Displacement(2,4): move "chased" to end
    run_intervention(&base_word, &Intervention::Displacement(2, 4), "word");

    // Transposition(0,4): the ↔ ball (full inversion of ends)
    run_intervention(&base_word, &Intervention::Transposition(0, 4), "word");

    // ── Sentence scale ────────────────────────────────────────────────
    println!("════════════════════════════════════════════════════════════");
    println!("SCALE: Sentence");
    println!("Base: four-sentence causal progression.");
    println!("Same intervention types as word and character scale.");
    println!("════════════════════════════════════════════════════════════");
    println!();

    let base_sent = Sequence::from_sentences(vec![
        "John picked up the glass",
        "He carried it into the kitchen",
        "The glass slipped from his hand",
        "It shattered",
    ], "glass sequence");

    // Transposition(2,3): swap last two sentences — disrupts causal close
    run_intervention(&base_sent, &Intervention::Transposition(2, 3), "sentence");

    // Transposition(0,3): swap first and last — maximum disruption
    run_intervention(&base_sent, &Intervention::Transposition(0, 3), "sentence");

    // Displacement(3,1): move "It shattered" to position 1
    run_intervention(&base_sent, &Intervention::Displacement(3, 1), "sentence");

    // Identity check — no intervention
    let identical = base_sent.clone();
    let ref_prof  = edge_profile(&base_sent);
    let var_prof  = edge_profile(&identical);
    let delta     = compare(&ref_prof, &var_prof);
    println!("── Identity check (no intervention) ─────────────────────────────");
    println!("  Disrupted: {} / {}", delta.disruption_count, delta.compared);
    println!("  Expected: 0 disruptions.");
    println!();

    println!("── Open Conditions ──────────────────────────────────────────");
    println!("OC-INT-1: Adjacency-only edge profiles declared.");
    println!("          Long-range dependencies: Phase 2.");
    println!("OC-INT-2: Locus identity keys edges only — not semantic.");
    println!("OC-INT-3: Ground truth known — researcher performed intervention.");
    println!("OC-SCALE: Scale invariance hypothesis: same Δ quantity responds");
    println!("          to same intervention type at all resolutions.");
    println!("          Character result established. Word and sentence: this run.");
}
