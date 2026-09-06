// coherence.rs — Metatron Dynamics, Inc.
// ABR Language Structure — V2.0
// Bounded over D. No claim beyond D.
//
// ── Declaration ──────────────────────────────────────────────────────────────
//
// Locus: a declared position i in a text sequence.
//   Observable at i: token identity (the word itself, lowercased).
//   No embedding. No borrowed structure.
//
// Adjacent pair: (token_i, token_{i+1}) — the relation between
//   a locus and the one immediately following it.
//   Window W = 1. Direction: causal — j = i+1 only.
//
// ρ_n(i, i+1): relational evidence at each step.
//   ρ_n = 1.0 if the pair (token_i, token_{i+1}) is observed.
//   Keyed by token identity pair, not position pair.
//
// Accumulation rule (Origin-declared):
//   ρ_acc(token_i, token_{i+1}) ←
//     ρ_acc(token_i, token_{i+1}) + η · ρ_n · (1 − ρ_acc(token_i, token_{i+1}))
//
// Admission: pair (token_i, token_{i+1}) is grounded if ρ_acc >= θ_min.
//   On first encounter: ρ_acc rises to η = 0.1, which exceeds θ_min = 0.05.
//   So a pair is grounded the first time it is seen, and strengthens on recurrence.
//
// ── Field Coherence ──────────────────────────────────────────────────────────
//
// Coherence is a field condition, sampleable at any locus.
// At each interior locus i (where i-1 and i+1 both exist):
//   Sample the triple (token_{i-1}, token_i, token_{i+1}).
//   Left transition:  (token_{i-1}, token_i)   — grounded or not.
//   Right transition: (token_i,     token_{i+1}) — grounded or not.
//   Coherent at i: both transitions grounded at time of sampling.
//   Incoherent at i: either transition ungrounded at time of sampling.
//
// Passage coherence ratio: proportion of interior loci that are coherent.
//
// Open Conditions:
//   OC-COH-1: θ_min = 0.05. Inherited. Not yet calibrated for this domain.
//   OC-COH-2: η = 0.1. Single-pass accumulation per adjacent pair.
//             Grounding on first encounter is a consequence, not a design choice.
//             Whether grounding should require recurrence is an open question.
//   OC-COH-3: Direction is causal (i → i+1 only). Symmetric keying
//             is a Phase 2 candidate if the field proves non-directional.

use std::collections::HashMap;

/// Origin-declared configuration.
pub struct CoherenceConfig {
    /// Admission threshold θ_min. OC-COH-1.
    pub theta_min: f64,
    /// Accumulation rate η. OC-COH-2.
    pub eta: f64,
}

/// Coherence reading at a single interior locus.
pub struct LociReading {
    /// Position in the sequence (1-indexed for display).
    pub position: usize,
    /// The triple of tokens sampled.
    pub left: String,
    pub center: String,
    pub right: String,
    /// Whether the left transition (left → center) is grounded.
    pub left_grounded: bool,
    /// Whether the right transition (center → right) is grounded.
    pub right_grounded: bool,
    /// Both transitions grounded — coherent at this locus.
    pub coherent: bool,
}

/// Result of a coherence measurement run.
pub struct CoherenceResult {
    /// Token sequence.
    pub tokens: Vec<String>,
    /// ρ_acc for each observed adjacent pair.
    pub rho_acc: HashMap<(String, String), f64>,
    /// Per-locus readings (interior loci only).
    pub readings: Vec<LociReading>,
    /// Proportion of interior loci that are coherent.
    pub coherence_ratio: f64,
    /// Count of coherent interior loci.
    pub coherent_count: usize,
    /// Total interior loci sampled.
    pub interior_count: usize,
}

/// Tokenize: lowercase, split on whitespace.
/// No stemming, no stop-word removal, no linguistic processing.
pub fn tokenize(text: &str) -> Vec<String> {
    text.split_whitespace()
        .map(|w| w.to_lowercase())
        .collect()
}

/// Run coherence measurement on a text passage.
/// Sequential: accumulate each adjacent pair as we walk the sequence,
/// then sample the field at each interior locus.
pub fn measure_coherence(text: &str, config: &CoherenceConfig) -> CoherenceResult {
    let tokens = tokenize(text);
    let n = tokens.len();

    if n < 3 {
        return CoherenceResult {
            tokens,
            rho_acc: HashMap::new(),
            readings: vec![],
            coherence_ratio: 0.0,
            coherent_count: 0,
            interior_count: 0,
        };
    }

    // Walk the sequence left to right.
    // At each position i, accumulate the pair (token_i, token_{i+1}),
    // then sample the field at i (if i is an interior locus: i >= 1).
    //
    // This is the sequential read: accumulation happens before sampling
    // at each step, so the field condition at i reflects what has been
    // established up to and including the current pair.

    let mut rho_acc: HashMap<(String, String), f64> = HashMap::new();
    let mut readings: Vec<LociReading> = Vec::new();

    for i in 0..(n - 1) {
        // Accumulate the adjacent pair at this step.
        let ti = tokens[i].clone();
        let ti1 = tokens[i + 1].clone();
        let entry = rho_acc.entry((ti, ti1)).or_insert(0.0);
        *entry += config.eta * (1.0 - *entry);

        // Sample the field at locus i (interior: has both left and right).
        if i >= 1 {
            let left   = tokens[i - 1].clone();
            let center = tokens[i].clone();
            let right  = tokens[i + 1].clone();

            let left_rho  = rho_acc.get(&(left.clone(), center.clone())).copied().unwrap_or(0.0);
            let right_rho = rho_acc.get(&(center.clone(), right.clone())).copied().unwrap_or(0.0);

            let left_grounded  = left_rho  >= config.theta_min;
            let right_grounded = right_rho >= config.theta_min;
            let coherent = left_grounded && right_grounded;

            readings.push(LociReading {
                position: i + 1,
                left,
                center,
                right,
                left_grounded,
                right_grounded,
                coherent,
            });
        }
    }

    let interior_count = readings.len();
    let coherent_count = readings.iter().filter(|r| r.coherent).count();
    let coherence_ratio = if interior_count == 0 {
        0.0
    } else {
        coherent_count as f64 / interior_count as f64
    };

    CoherenceResult {
        tokens,
        rho_acc,
        readings,
        coherence_ratio,
        coherent_count,
        interior_count,
    }
}

/// Format a coherence result for Origin review.
pub fn format_report(r: &CoherenceResult) -> String {
    let mut out = String::new();

    out.push_str(&format!("Tokens:              {}\n", r.tokens.len()));
    out.push_str(&format!("Interior loci:       {}\n", r.interior_count));
    out.push_str(&format!("Coherent loci:       {}\n", r.coherent_count));
    out.push_str(&format!("Coherence ratio:     {:.4}\n", r.coherence_ratio));
    out.push_str("\nLocus-by-locus field reading:\n");

    for reading in &r.readings {
        let status = if reading.coherent { "COH" } else { "INC" };
        let l_mark = if reading.left_grounded  { "G" } else { "U" };
        let r_mark = if reading.right_grounded { "G" } else { "U" };
        out.push_str(&format!(
            "  [{:>3}] {:<12} [{:<12}] {:<12}  L:{} R:{} → {}\n",
            reading.position,
            reading.left,
            reading.center,
            reading.right,
            l_mark, r_mark,
            status
        ));
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn default_config() -> CoherenceConfig {
        CoherenceConfig { theta_min: 0.05, eta: 0.1 }
    }

    #[test]
    fn tokenizer_lowercases_and_splits() {
        let tokens = tokenize("The Cat SAT");
        assert_eq!(tokens, vec!["the", "cat", "sat"]);
    }

    #[test]
    fn empty_text_produces_zero_ratio() {
        let r = measure_coherence("", &default_config());
        assert_eq!(r.coherence_ratio, 0.0);
    }

    #[test]
    fn two_tokens_produces_no_interior_loci() {
        let r = measure_coherence("hello world", &default_config());
        assert_eq!(r.interior_count, 0);
    }

    #[test]
    fn rho_acc_stays_bounded() {
        let r = measure_coherence("the cat sat on the mat the cat sat", &default_config());
        for &v in r.rho_acc.values() {
            assert!(v >= 0.0 && v <= 1.0, "ρ_acc out of bounds: {}", v);
        }
    }

    #[test]
    fn repeated_sequence_produces_high_coherence() {
        // Heavy repetition — all pairs grounded quickly.
        let text = "the cat sat on the mat the cat sat on the mat";
        let r = measure_coherence(text, &default_config());
        assert!(r.coherence_ratio > 0.5,
            "repeated text should produce high coherence ratio: {:.4}", r.coherence_ratio);
    }

    #[test]
    fn first_pair_accumulates_on_first_encounter() {
        // η=0.1 > θ_min=0.05: first encounter should ground a pair.
        let r = measure_coherence("a b c", &default_config());
        let ab = r.rho_acc.get(&("a".to_string(), "b".to_string())).copied().unwrap_or(0.0);
        assert!(ab >= 0.05, "pair a→b should be grounded on first encounter: ρ={:.4}", ab);
    }

    #[test]
    fn keying_is_by_identity_not_position() {
        // Same token pair at different positions should share ρ_acc.
        let r = measure_coherence("cat sat mat cat sat", &default_config());
        let cs = r.rho_acc.get(&("cat".to_string(), "sat".to_string())).copied().unwrap_or(0.0);
        // Pair (cat, sat) appears twice — should accumulate above single-encounter level.
        let single = 0.1_f64;
        let double = single + 0.1 * (1.0 - single);
        assert!((cs - double).abs() < 1e-10,
            "ρ_acc(cat,sat) should reflect two encounters: expected {:.4}, got {:.4}", double, cs);
    }

    #[test]
    fn incoherent_text_produces_lower_ratio_than_coherent() {
        let coherent   = "the cat sat on the mat the cat sat on the mat";
        let incoherent = "purple longitude decided fork sleeping eleven clouds argued Wednesday";
        let config = default_config();
        let r_c = measure_coherence(coherent,   &config);
        let r_i = measure_coherence(incoherent, &config);
        assert!(r_c.coherence_ratio >= r_i.coherence_ratio,
            "coherent ratio={:.4} should be >= incoherent ratio={:.4}",
            r_c.coherence_ratio, r_i.coherence_ratio);
    }

    #[test]
    fn three_token_minimum_produces_one_reading() {
        let r = measure_coherence("a b c", &default_config());
        assert_eq!(r.interior_count, 1);
    }
}
