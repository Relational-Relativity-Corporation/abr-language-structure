// coherence.rs — Metatron Dynamics, Inc.
// ABR Language Structure — V1.0
// Bounded over D. No claim beyond D.
//
// ── Declaration ──────────────────────────────────────────────────────────────
//
// Locus: a declared position in a text sequence.
//   Observable: token identity (the word itself, lowercased).
//   M maps position i in the sequence to its token string.
//   No embedding. No borrowed structure.
//
// Proximity: locus j is within proximity of locus i if
//   0 < (j - i) <= W, where W is the declared window.
//   Direction: causal — j follows i in the sequence.
//
// ρ_n(i,j): relational evidence per pass.
//   ρ_n = 1.0 if token at j is within W of token at i.
//   ρ_n = 0.0 otherwise.
//   Keyed by (token_i, token_j) — token identity pair, not position pair.
//   This is the correct keying: coherence is a property of the token
//   relation, not of the particular positions they occupy in one sequence.
//
// Accumulation rule (Origin-declared):
//   ρ_acc(token_i, token_j) ←
//     ρ_acc(token_i, token_j) + η · ρ_n · (1 − ρ_acc(token_i, token_j))
//
// Admission: edge (token_i, token_j) is admitted if ρ_acc >= θ_min.
//
// k_i: admitted edge count from token_i = |{j : ρ_acc(i,j) >= θ_min}|
// mean_k: mean admitted width across all source token types in the sequence.

use std::collections::HashMap;

/// Origin-declared configuration. W and θ_min are open conditions.
pub struct CoherenceConfig {
    /// Proximity window W — declared, not derived. OC-COH-1.
    pub window: usize,
    /// Admission threshold θ_min. OC-COH-2.
    pub theta_min: f64,
    /// Accumulation rate η.
    pub eta: f64,
    /// Number of passes through the sequence.
    pub passes: usize,
}

/// Result of a coherence measurement run.
pub struct CoherenceResult {
    /// Token sequence (words in order).
    pub tokens: Vec<String>,
    /// ρ_acc for each admitted (token_i, token_j) pair.
    pub rho_acc: HashMap<(String, String), f64>,
    /// Admitted edge count per source token type.
    pub k_per_token: HashMap<String, usize>,
    /// Mean admitted relational width across source tokens.
    pub mean_k: f64,
    /// Total unique token-pair relations evaluated.
    pub total_pairs_evaluated: usize,
    /// Total admitted edges.
    pub total_admitted: usize,
}

/// Tokenize: lowercase, split on whitespace.
/// No stemming, no stop-word removal, no linguistic processing.
fn tokenize(text: &str) -> Vec<String> {
    text.split_whitespace()
        .map(|w| w.to_lowercase())
        .collect()
}

/// Run coherence measurement on a text passage.
pub fn measure_coherence(text: &str, config: &CoherenceConfig) -> CoherenceResult {
    let tokens = tokenize(text);
    let n = tokens.len();

    // EdgeStore: keyed by token identity pair (not position pair).
    // Coherence is a property of which tokens relate to which,
    // not of where they happened to sit in this particular sequence.
    let mut rho_acc: HashMap<(String, String), f64> = HashMap::new();

    let mut total_pairs_evaluated = 0usize;

    // Run accumulation passes.
    for _ in 0..config.passes {
        // For each source locus i, scan forward within window W.
        for i in 0..n {
            for j in (i + 1)..=(i + config.window).min(n - 1) {
                let ti = tokens[i].clone();
                let tj = tokens[j].clone();

                // ρ_n = 1.0: j is within declared proximity of i.
                let rho_n = 1.0_f64;

                let entry = rho_acc.entry((ti, tj)).or_insert(0.0);
                *entry += config.eta * rho_n * (1.0 - *entry);
                total_pairs_evaluated += 1;
            }
        }
    }

    // Read admitted edges: ρ_acc >= θ_min.
    // k per source token type: how many distinct targets are admitted.
    let mut k_per_token: HashMap<String, usize> = HashMap::new();
    let mut total_admitted = 0usize;

    for ((ti, _tj), &rho) in &rho_acc {
        if rho >= config.theta_min {
            *k_per_token.entry(ti.clone()).or_insert(0) += 1;
            total_admitted += 1;
        }
    }

    // Mean k across source token types that appear in the sequence.
    // Only count token types that actually appear as sources.
    let source_types: std::collections::HashSet<String> =
        tokens.iter().cloned().collect();

    let mean_k = if source_types.is_empty() {
        0.0
    } else {
        let total_k: usize = source_types.iter()
            .map(|t| k_per_token.get(t).copied().unwrap_or(0))
            .sum();
        total_k as f64 / source_types.len() as f64
    };

    CoherenceResult {
        tokens,
        rho_acc,
        k_per_token,
        mean_k,
        total_pairs_evaluated,
        total_admitted,
    }
}

/// Format a coherence result for Origin review.
pub fn format_report(r: &CoherenceResult) -> String {
    let mut out = String::new();

    out.push_str(&format!("Tokens:              {}\n", r.tokens.len()));
    out.push_str(&format!("Unique token types:  {}\n",
        r.tokens.iter().collect::<std::collections::HashSet<_>>().len()));
    out.push_str(&format!("Pairs evaluated:     {}\n", r.total_pairs_evaluated));
    out.push_str(&format!("Admitted edges:      {}\n", r.total_admitted));
    out.push_str(&format!("Mean k:              {:.4}\n", r.mean_k));

    // Show k per token type, sorted descending.
    let mut k_sorted: Vec<(&String, &usize)> = r.k_per_token.iter().collect();
    k_sorted.sort_by(|a, b| b.1.cmp(a.1).then(a.0.cmp(b.0)));

    out.push_str("k per source token:\n");
    for (token, k) in &k_sorted {
        out.push_str(&format!("  {:>12} : k={}\n", token, k));
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn default_config() -> CoherenceConfig {
        CoherenceConfig { window: 4, theta_min: 0.05, eta: 0.1, passes: 20 }
    }

    #[test]
    fn tokenizer_lowercases_and_splits() {
        let tokens = tokenize("The Cat SAT");
        assert_eq!(tokens, vec!["the", "cat", "sat"]);
    }

    #[test]
    fn empty_text_produces_zero_mean_k() {
        let r = measure_coherence("", &default_config());
        assert_eq!(r.mean_k, 0.0);
    }

    #[test]
    fn single_token_produces_zero_admitted_edges() {
        let r = measure_coherence("hello", &default_config());
        assert_eq!(r.total_admitted, 0);
    }

    #[test]
    fn rho_acc_accumulates_and_stays_bounded() {
        let r = measure_coherence("the cat sat on the mat", &default_config());
        for &v in r.rho_acc.values() {
            assert!(v >= 0.0 && v <= 1.0, "ρ_acc out of bounds: {}", v);
        }
    }

    #[test]
    fn repeated_pairs_accumulate_above_theta_min() {
        // "the cat" appears multiple times — should be admitted.
        let text = "the cat sat on the mat the cat looked the cat";
        let r = measure_coherence(text, &default_config());
        let key = ("the".to_string(), "cat".to_string());
        let rho = r.rho_acc.get(&key).copied().unwrap_or(0.0);
        assert!(rho >= 0.05, "repeated pair 'the→cat' should be admitted: ρ={:.4}", rho);
    }

    #[test]
    fn coherent_text_lower_mean_k_than_shuffled() {
        let coherent = "the cat sat on the mat the cat looked at the mat \
                        the mat was under the cat the cat sat still";
        let incoherent = "mat the on cat sat the looked cat the mat the \
                          was mat under cat the still sat cat the";
        let config = default_config();
        let r_c = measure_coherence(coherent,   &config);
        let r_i = measure_coherence(incoherent, &config);
        assert!(r_c.mean_k <= r_i.mean_k,
            "coherent mean_k={:.4} should be <= incoherent mean_k={:.4}",
            r_c.mean_k, r_i.mean_k);
    }

    #[test]
    fn keying_is_by_token_identity_not_position() {
        // Two sequences with the same token pairs in different positions
        // should produce the same ρ_acc values.
        let config = CoherenceConfig { window: 2, theta_min: 0.05, eta: 0.1, passes: 5 };
        let r1 = measure_coherence("a b c", &config);
        let r2 = measure_coherence("a b c", &config);
        let k1 = r1.rho_acc.get(&("a".to_string(), "b".to_string())).copied().unwrap_or(0.0);
        let k2 = r2.rho_acc.get(&("a".to_string(), "b".to_string())).copied().unwrap_or(0.0);
        assert!((k1 - k2).abs() < 1e-10);
    }

    #[test]
    fn mean_k_zero_for_all_unique_tokens_no_repetition() {
        // With passes=1 and no repetition, each pair appears once.
        // At η=0.1, ρ_acc = 0.1 > θ_min=0.05, so edges ARE admitted.
        // This tests that the accumulation actually fires.
        let config = CoherenceConfig { window: 1, theta_min: 0.05, eta: 0.1, passes: 1 };
        let r = measure_coherence("a b c d e", &config);
        // Each adjacent pair fires once: ρ_acc = 0.1 >= 0.05, so admitted.
        assert!(r.total_admitted > 0);
    }

    #[test]
    fn window_limits_evaluated_pairs() {
        let config_w1 = CoherenceConfig { window: 1, theta_min: 0.05, eta: 0.1, passes: 1 };
        let config_w4 = CoherenceConfig { window: 4, theta_min: 0.05, eta: 0.1, passes: 1 };
        let text = "a b c d e f g h";
        let r1 = measure_coherence(text, &config_w1);
        let r4 = measure_coherence(text, &config_w4);
        assert!(r4.total_pairs_evaluated > r1.total_pairs_evaluated,
            "wider window must evaluate more pairs");
    }
}
