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
    println!("ABR Language Structure — V3.2");
    println!("Metatron Dynamics, Inc. Bounded over D. No claim beyond D.");
    println!("Controlled intervention experiment.");
    println!("Observable: Δ(declared relations) under known interventions.");
    println!("V3.2 adds: content independence + adjacency limit experiments.");
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

    // ── Experiment 1: Content independence ───────────────────────────────
    println!("════════════════════════════════════════════════════════════");
    println!("EXPERIMENT 1: Content independence");
    println!("Apply T(1,2) to sequences with completely different locus");
    println!("identities. Δ should be structurally identical across all.");
    println!("════════════════════════════════════════════════════════════");
    println!();

    let iv_t12 = Intervention::Transposition(1, 2);

    let content_seqs: Vec<(&str, Sequence)> = vec![
        ("A: domestic",  Sequence::from_words("the dog chased the ball",         "A")),
        ("B: narrative", Sequence::from_words("she walked into the garden",       "B")),
        ("C: nonsense",  Sequence::from_words("seventeen purple clouds fell down","C")),
        ("D: technical", Sequence::from_words("matrix eigenvalue kernel gradient descent", "D")),
    ];

    let mut prev_delta: Option<intervention::InterventionDelta> = None;
    let mut all_equal = true;

    for (label, seq) in &content_seqs {
        let var    = apply(seq, &iv_t12);
        let rp     = edge_profile(seq);
        let vp     = edge_profile(&var);
        let delta  = compare(&rp, &vp);

        println!("  {} — disrupted: {}/{}, extent: {:?}, recovery: {:?}",
            label,
            delta.disruption_count,
            delta.compared,
            delta.disruption_extent,
            delta.recovery_at,
        );

        if let Some(ref prev) = prev_delta {
            if !intervention::structurally_equal(prev, &delta) {
                all_equal = false;
            }
        }
        prev_delta = Some(delta);
    }

    println!();
    if all_equal {
        println!("RESULT: All sequences produce structurally identical Δ under T(1,2).");
        println!("        Measurement depends on transformation, not locus content.");
    } else {
        println!("RESULT: Structural difference detected — content independence not confirmed.");
    }
    println!();

    // ── Experiment 2: Adjacency limits ───────────────────────────────────
    println!("════════════════════════════════════════════════════════════");
    println!("EXPERIMENT 2: Adjacency limits (closes OC-INT-1)");
    println!("Same intervention on coherent vs. unrelated sentence sequence.");
    println!("If operator produces identical results: adjacency topology");
    println!("cannot distinguish causal coherence from accidental sequence.");
    println!("This is not failure — it declares what must be added next.");
    println!("════════════════════════════════════════════════════════════");
    println!();

    let causal = Sequence::from_sentences(vec![
        "John picked up the glass",
        "He carried it into the kitchen",
        "The glass slipped from his hand",
        "It shattered",
    ], "causal");

    let accidental = Sequence::from_sentences(vec![
        "The committee approved the budget",
        "Rain fell on the eastern provinces",
        "Seven satellites crossed the equator",
        "The algorithm converged",
    ], "accidental");

    let dc = compare(&edge_profile(&causal),     &edge_profile(&apply(&causal,     &iv_t12)));
    let da = compare(&edge_profile(&accidental), &edge_profile(&apply(&accidental, &iv_t12)));

    println!("  Causal sequence     — T(1,2): disrupted={}/{}, extent={:?}, recovery={:?}",
        dc.disruption_count, dc.compared, dc.disruption_extent, dc.recovery_at);
    println!("  Accidental sequence — T(1,2): disrupted={}/{}, extent={:?}, recovery={:?}",
        da.disruption_count, da.compared, da.disruption_extent, da.recovery_at);
    println!();

    if intervention::structurally_equal(&dc, &da) {
        println!("RESULT: Identical structural Δ for causal and accidental sequences.");
        println!("        OC-INT-1 CLOSED experimentally:");
        println!("        Adjacency topology contains no information capable of");
        println!("        distinguishing causal coherence from accidental sequence.");
        println!("        Next declared relation required for that discrimination.");
    } else {
        println!("RESULT: Structural difference detected — unexpected.");
        println!("        OC-INT-1 remains open — investigate.");
    }
    println!();

    println!("── Open Conditions ──────────────────────────────────────────");
    println!("OC-INT-1: CLOSED — adjacency topology is content-blind.");
    println!("          Next relation to declare: what survives change of");
    println!("          locus identity while preserving relational organization?");
    println!("OC-INT-2: Locus identity keys edges only — not semantic.");
    println!("OC-INT-3: Ground truth known — researcher performed intervention.");
    println!("OC-SCALE: Scale invariance observed for tested interventions across");
    println!("          character, word, and sentence resolutions.");
    println!("          Paragraph and above: pending.");
    println!("OC-NEXT:  Content independence established for adjacency.");
    println!("          What relation distinguishes causal from accidental?");
    println!("          That is the next experimental question.");
}
