// character.rs — Metatron Dynamics, Inc.
// ABR Language Structure — V2.2 (character resolution)
// Bounded over D. No claim beyond D.
//
// ── Declaration ──────────────────────────────────────────────────────────────
//
// Scale invariance hypothesis (V7 kernel):
//   The operator F(L_i, L_{i+1}) is defined in terms of properties of the
//   relation — not in terms of what the loci contain.
//
// Edge profile:
//   For a sequence of n characters, the edge profile is a vector of n-1
//   values — one per adjacent pair. Each value is the ρ_acc of that pair
//   after a single sequential pass through the sequence.
//
//   profile[i] = ρ_acc(c_i, c_{i+1}) after observing c_i → c_{i+1}.
//
//   For a single-occurrence sequence (no repeated pairs), every pair
//   accumulates exactly once: profile[i] = η = 0.1 for all i.
//
//   This is the correct observable: not whether a pair was seen before,
//   but what the accumulated relational state looks like at each edge
//   after the sequence has been read.
//
// Δ between two profiles (reference vs. variant):
//   Given a reference sequence R and a variant sequence V of equal length:
//     Δ[i] = profile_R[i] − profile_V[i]
//   |Δ[i]| is the magnitude of relational change at boundary i.
//
//   A transposition changes the pair identities at the disruption boundary.
//   Those pairs will have different ρ_acc values because different pairs
//   were observed. The Δ at those boundaries will be non-zero.
//
//   For pairs that appear in both sequences (same identity, same position):
//     Δ[i] = 0.
//   For pairs that differ (disruption boundary):
//     profile_R[i] carries ρ_acc of the clean pair.
//     profile_V[i] carries ρ_acc of the disrupted pair.
//     If neither pair recurs, both = η = 0.1, Δ[i] = 0.
//     But the PAIR IDENTITY differs — and pair identity is what M tracks.
//
// Revised observable: pair identity change at boundary.
//   M observes whether the pair at boundary i in V matches the pair at
//   boundary i in R. If yes: no disruption. If no: disruption at i.
//   This is the direct comparison without accumulation ambiguity.
//
//   matched[i] = (c_R_i == c_V_i) AND (c_R_{i+1} == c_V_{i+1})
//
//   This is the minimal, clean declaration:
//     The relational field of R and V differ at boundary i if and only if
//     the pair at i is not the same pair in both sequences.
//
// Open Conditions:
//   OC-CHR-1: This declaration reduces to character identity comparison
//             at the pair level. It is the minimal observable that does
//             not import linguistic structure.
//   OC-CHR-2: For sequences of different lengths, comparison is bounded
//             by min(len_R, len_V). Length difference is itself a declared
//             disruption observable.
//   OC-CHR-3: Accumulation is retained for multi-occurrence sequences
//             where pairs recur. For single words, ρ_acc is uniform (η)
//             across all pairs — the pair identity comparison is the
//             meaningful discriminator.
//   OC-SCALE: This operation is scale-invariant: replace characters with
//             words, words with sentences — the same comparison applies.

use std::collections::HashMap;

pub struct CharConfig {
    pub theta_min: f64,
    pub eta: f64,
}

/// Edge profile: ρ_acc value at each adjacent boundary after one pass.
/// profile[i] = ρ_acc(c_i, c_{i+1}).
/// pairs[i]   = (c_i, c_{i+1}) — the identity of the pair at boundary i.
pub struct EdgeProfile {
    pub chars:   Vec<char>,
    pub pairs:   Vec<(char, char)>,
    pub rho_acc: Vec<f64>,
}

/// Build the edge profile for a character sequence.
pub fn edge_profile(text: &str, config: &CharConfig) -> EdgeProfile {
    let chars: Vec<char> = text.chars().collect();
    let n = chars.len();

    let mut store: HashMap<(char, char), f64> = HashMap::new();
    let mut pairs:   Vec<(char, char)> = Vec::with_capacity(n.saturating_sub(1));
    let mut rho_acc: Vec<f64>          = Vec::with_capacity(n.saturating_sub(1));

    for i in 0..(n.saturating_sub(1)) {
        let pair = (chars[i], chars[i + 1]);
        let entry = store.entry(pair).or_insert(0.0);
        *entry += config.eta * (1.0 - *entry);
        pairs.push(pair);
        rho_acc.push(*entry);
    }

    EdgeProfile { chars, pairs, rho_acc }
}

/// Comparison result between a reference and a variant profile.
pub struct ProfileDelta {
    /// Per-boundary disruption: true if the pair identity differs.
    pub disrupted: Vec<bool>,
    /// Per-boundary ρ_acc difference (reference − variant).
    pub rho_diff:  Vec<f64>,
    /// Number of boundaries compared.
    pub compared:  usize,
    /// Number of disrupted boundaries.
    pub disruption_count: usize,
    /// Whether the sequences have different lengths.
    pub length_mismatch: bool,
}

/// Compare reference profile to variant profile.
/// Δ[i]: pair identity mismatch at boundary i.
pub fn compare_profiles(reference: &EdgeProfile, variant: &EdgeProfile) -> ProfileDelta {
    let compared = reference.pairs.len().min(variant.pairs.len());
    let length_mismatch = reference.chars.len() != variant.chars.len();

    let mut disrupted = Vec::with_capacity(compared);
    let mut rho_diff  = Vec::with_capacity(compared);

    for i in 0..compared {
        let pair_match = reference.pairs[i] == variant.pairs[i];
        disrupted.push(!pair_match);
        rho_diff.push(reference.rho_acc[i] - variant.rho_acc[i]);
    }

    let disruption_count = disrupted.iter().filter(|&&d| d).count();

    ProfileDelta { disrupted, rho_diff, compared, disruption_count, length_mismatch }
}

/// Format a comparison report.
pub fn format_comparison(
    ref_label: &str, ref_text: &str,
    var_label: &str, var_text: &str,
    reference: &EdgeProfile,
    variant:   &EdgeProfile,
    delta:     &ProfileDelta,
) -> String {
    let mut out = String::new();

    out.push_str(&format!(
        "── {} vs {} ─────────────────────────────────────────\n",
        ref_label, var_label
    ));
    out.push_str(&format!("  Reference: \"{}\"\n", ref_text));
    out.push_str(&format!("  Variant:   \"{}\"\n", var_text));
    if delta.length_mismatch {
        out.push_str(&format!(
            "  Length: reference={} variant={} — mismatch declared.\n",
            reference.chars.len(), variant.chars.len()
        ));
    }
    out.push_str(&format!(
        "  Boundaries compared: {}\n", delta.compared
    ));
    out.push_str(&format!(
        "  Disrupted boundaries: {} / {}\n",
        delta.disruption_count, delta.compared
    ));
    out.push_str("\n  Boundary-by-boundary:\n");

    for i in 0..delta.compared {
        let rp = reference.pairs[i];
        let vp = variant.pairs[i];
        let status = if delta.disrupted[i] { "DISRUPT" } else { "match  " };
        out.push_str(&format!(
            "    [{:>2}] ref: '{}'→'{}'  var: '{}'→'{}'  {}\n",
            i, rp.0, rp.1, vp.0, vp.1, status
        ));
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cfg() -> CharConfig { CharConfig { theta_min: 0.05, eta: 0.1 } }

    #[test]
    fn clean_sequence_produces_no_disruptions() {
        let cfg = cfg();
        let r = edge_profile("problem", &cfg);
        let v = edge_profile("problem", &cfg);
        let d = compare_profiles(&r, &v);
        assert_eq!(d.disruption_count, 0,
            "identical sequences should have zero disruptions");
    }

    #[test]
    fn transposition_registers_disruptions() {
        let cfg = cfg();
        let r = edge_profile("problem", &cfg);
        let v = edge_profile("porblem", &cfg);
        let d = compare_profiles(&r, &v);
        assert!(d.disruption_count > 0,
            "transposition should register at least one disrupted boundary");
    }

    #[test]
    fn disruption_is_localized() {
        // "problem" vs "porblem": transposition of r and o.
        // Pairs: p-r, r-o, o-b, b-l, l-e, e-m (clean)
        //        p-o, o-r, r-b, b-l, l-e, e-m (transposed)
        // Boundaries 0,1,2 should differ; 3,4,5 should match.
        let cfg = cfg();
        let r = edge_profile("problem", &cfg);
        let v = edge_profile("porblem", &cfg);
        let d = compare_profiles(&r, &v);

        // First three boundaries disrupted.
        assert!(d.disrupted[0], "boundary 0 (p→r vs p→o) should be disrupted");
        assert!(d.disrupted[1], "boundary 1 (r→o vs o→r) should be disrupted");
        assert!(d.disrupted[2], "boundary 2 (o→b vs r→b) should be disrupted");
        // Remaining boundaries should match.
        assert!(!d.disrupted[3], "boundary 3 (b→l) should match");
        assert!(!d.disrupted[4], "boundary 4 (l→e) should match");
        assert!(!d.disrupted[5], "boundary 5 (e→m) should match");
    }

    #[test]
    fn word_order_disruption_registers() {
        // Scale invariance check at word level using same function on tokens.
        // "the dog chased the ball" vs "ball the dog chased the"
        // Treat each word as a 'character' — same operator.
        // Here we just verify the pair comparison works on longer sequences.
        let cfg = cfg();
        let r = edge_profile("abcde", &cfg);
        let v = edge_profile("eabcd", &cfg);
        let d = compare_profiles(&r, &v);
        assert!(d.disruption_count > 0,
            "order disruption should register");
    }

    #[test]
    fn rho_acc_accumulates_on_recurrence() {
        // In "aababc": pair (a,b) appears twice — should accumulate.
        let cfg = cfg();
        let p = edge_profile("aababc", &cfg);
        // Find the second (a,b) pair's rho_acc — should be > η.
        let ab_values: Vec<f64> = p.pairs.iter().zip(p.rho_acc.iter())
            .filter(|(&pair, _)| pair == ('a', 'b'))
            .map(|(_, &r)| r)
            .collect();
        assert!(ab_values.len() >= 2, "should see (a,b) at least twice");
        assert!(ab_values[1] > ab_values[0],
            "second encounter should accumulate higher ρ_acc");
    }
}
