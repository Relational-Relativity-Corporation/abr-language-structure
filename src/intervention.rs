// intervention.rs — Metatron Dynamics, Inc.
// ABR Language Structure — V3.1 (controlled intervention runner)
// Bounded over D. No claim beyond D.
//
// ── Declaration ──────────────────────────────────────────────────────────────
//
// Resolution-independent relational observable (V7 kernel, corrected):
//   At every declared resolution, the observable is the change in relations
//   produced by an intervention on the ordered loci — not the identities
//   of the loci themselves.
//
// Controlled intervention:
//   Given a reference sequence S = (L_0, L_1, ..., L_{n-1}),
//   an intervention I produces a variant S' by modifying the order
//   of declared loci. The content of each locus is unchanged.
//   Only the relational organization changes.
//
// Supported interventions (Origin-declared):
//   Transposition(i, j): swap loci at positions i and j.
//   Displacement(i, j):  move locus at position i to position j,
//                        shifting intervening loci.
//
// Edge profile (same as character.rs, now at word resolution):
//   For a sequence of n loci, the edge profile is a vector of n-1 pairs.
//   profile[k] = (L_k, L_{k+1}) — the directed pair at boundary k.
//
// Δ between reference and variant:
//   disrupted[k] = true if profile_S[k] ≠ profile_S'[k]
//   disruption_count = |{ k | disrupted[k] }|
//   disruption_extent = span from first to last disrupted boundary
//   recovery_at = first boundary after which all remaining are undisrupted
//
// Scale invariance test:
//   Apply the identical intervention type at:
//     (a) character scale — loci are characters
//     (b) word scale      — loci are words
//     (c) sentence scale  — loci are sentences
//   The boundary disruption pattern should reflect the intervention
//   regardless of what constitutes a locus.
//
// Open Conditions:
//   OC-INT-1: Only adjacency-based edge profiles are declared here.
//             Non-adjacent relations (long-range dependencies) are
//             a Phase 2 candidate.
//   OC-INT-2: Locus identity is used only to construct edge pairs.
//             It is not a semantic variable.
//   OC-INT-3: The intervention is performed by the researcher, not
//             derived from the sequence. Ground truth is known.
//             This is the experimental advantage over open passages.

/// A locus is an ordered element of a declared sequence.
/// At word scale: a word string.
/// At sentence scale: a sentence string.
/// The measurement does not depend on which.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Locus {
    pub content: String,
}

impl Locus {
    pub fn new(s: &str) -> Self {
        Locus { content: s.to_string() }
    }
}

/// A declared sequence of loci.
#[derive(Debug, Clone)]
pub struct Sequence {
    pub loci: Vec<Locus>,
    pub label: String,
}

impl Sequence {
    pub fn from_words(text: &str, label: &str) -> Self {
        let loci = text.split_whitespace()
            .map(|w| Locus::new(&w.to_lowercase()))
            .collect();
        Sequence { loci, label: label.to_string() }
    }

    pub fn from_sentences(sentences: Vec<&str>, label: &str) -> Self {
        let loci = sentences.iter()
            .map(|s| Locus::new(s.trim()))
            .collect();
        Sequence { loci, label: label.to_string() }
    }

    pub fn from_chars(text: &str, label: &str) -> Self {
        let loci = text.chars()
            .map(|c| Locus::new(&c.to_string()))
            .collect();
        Sequence { loci, label: label.to_string() }
    }
}

/// Declared intervention types.
#[derive(Debug, Clone)]
pub enum Intervention {
    /// Swap loci at positions i and j.
    Transposition(usize, usize),
    /// Move locus at position i to position j, shifting intervening loci.
    Displacement(usize, usize),
}

impl Intervention {
    pub fn describe(&self) -> String {
        match self {
            Intervention::Transposition(i, j) =>
                format!("Transposition({}, {}): swap positions {} and {}", i, j, i, j),
            Intervention::Displacement(i, j) =>
                format!("Displacement({}, {}): move position {} to {}", i, j, i, j),
        }
    }
}

/// Apply an intervention to a sequence, producing a variant.
pub fn apply(seq: &Sequence, intervention: &Intervention) -> Sequence {
    let mut loci = seq.loci.clone();
    match intervention {
        Intervention::Transposition(i, j) => {
            loci.swap(*i, *j);
        }
        Intervention::Displacement(i, j) => {
            let locus = loci.remove(*i);
            let insert_at = if *j > *i { *j - 1 } else { *j };
            loci.insert(insert_at.min(loci.len()), locus);
        }
    }
    let label = format!("{} + {}", seq.label, intervention.describe());
    Sequence { loci, label }
}

/// Edge profile: directed pairs at each boundary.
pub fn edge_profile(seq: &Sequence) -> Vec<(String, String)> {
    let n = seq.loci.len();
    (0..n.saturating_sub(1))
        .map(|i| (
            seq.loci[i].content.clone(),
            seq.loci[i + 1].content.clone(),
        ))
        .collect()
}

/// Comparison result between reference and variant profiles.
pub struct InterventionDelta {
    pub disrupted:         Vec<bool>,
    pub disruption_count:  usize,
    /// Span from first to last disrupted boundary (inclusive).
    pub disruption_extent: Option<(usize, usize)>,
    /// First boundary index after which all remaining are undisrupted.
    pub recovery_at:       Option<usize>,
    pub compared:          usize,
}

/// Compare reference and variant edge profiles.
pub fn compare(
    ref_profile: &[(String, String)],
    var_profile: &[(String, String)],
) -> InterventionDelta {
    let compared = ref_profile.len().min(var_profile.len());
    let disrupted: Vec<bool> = (0..compared)
        .map(|i| ref_profile[i] != var_profile[i])
        .collect();

    let disruption_count = disrupted.iter().filter(|&&d| d).count();

    let first_disrupted = disrupted.iter().position(|&d| d);
    let last_disrupted  = disrupted.iter().rposition(|&d| d);

    let disruption_extent = match (first_disrupted, last_disrupted) {
        (Some(f), Some(l)) => Some((f, l)),
        _ => None,
    };

    // Recovery: first index after which no more disruptions.
    let recovery_at = last_disrupted.map(|l| l + 1)
        .filter(|&r| r < compared);

    InterventionDelta {
        disrupted,
        disruption_count,
        disruption_extent,
        recovery_at,
        compared,
    }
}

/// Format a full intervention report.
pub fn format_intervention_report(
    reference: &Sequence,
    variant:   &Sequence,
    ref_prof:  &[(String, String)],
    var_prof:  &[(String, String)],
    delta:     &InterventionDelta,
    intervention: &Intervention,
) -> String {
    let mut out = String::new();

    out.push_str(&format!(
        "── Intervention: {} ─────────────────────────────────────────\n",
        intervention.describe()
    ));
    out.push_str(&format!(
        "  Reference: {}\n", reference.label
    ));
    out.push_str(&format!(
        "  Loci: {}\n",
        reference.loci.iter().map(|l| l.content.as_str()).collect::<Vec<_>>().join(" | ")
    ));
    out.push_str(&format!(
        "  Variant loci: {}\n",
        variant.loci.iter().map(|l| l.content.as_str()).collect::<Vec<_>>().join(" | ")
    ));
    out.push_str(&format!(
        "  Boundaries compared: {}\n", delta.compared
    ));
    out.push_str(&format!(
        "  Disrupted: {} / {}\n", delta.disruption_count, delta.compared
    ));

    if let Some((f, l)) = delta.disruption_extent {
        out.push_str(&format!(
            "  Disruption extent: boundaries {} → {} (span {})\n",
            f, l, l - f + 1
        ));
    }
    if let Some(r) = delta.recovery_at {
        out.push_str(&format!("  Recovery at boundary: {}\n", r));
    } else if delta.disruption_count == 0 {
        out.push_str("  No disruption — sequences are relationally identical.\n");
    } else {
        out.push_str("  No recovery — disruption extends to end of sequence.\n");
    }

    out.push_str("\n  Boundary-by-boundary:\n");
    for i in 0..delta.compared {
        let status = if delta.disrupted[i] { "DISRUPT" } else { "match  " };
        let rp = &ref_prof[i];
        let vp = &var_prof[i];
        if delta.disrupted[i] {
            out.push_str(&format!(
                "    [{:>2}] ref: {:<14}→ {:<14}  var: {:<14}→ {:<14}  {}\n",
                i, rp.0, rp.1, vp.0, vp.1, status
            ));
        } else {
            out.push_str(&format!(
                "    [{:>2}] {:<14}→ {:<14}  {}\n",
                i, rp.0, rp.1, status
            ));
        }
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── Character scale ───────────────────────────────────────────────

    #[test]
    fn char_transposition_localizes_disruption() {
        // "problem" → "porblem": transposition(1, 2) — swap r and o.
        let s = Sequence::from_chars("problem", "problem");
        let v = apply(&s, &Intervention::Transposition(1, 2));
        let rp = edge_profile(&s);
        let vp = edge_profile(&v);
        let d  = compare(&rp, &vp);

        assert!(d.disruption_count > 0,
            "transposition should produce disruption");
        // Disruption should be localized — recovery should exist.
        assert!(d.recovery_at.is_some(),
            "disruption should recover before end of sequence");
        // Boundaries after extent should all match.
        if let Some((_, last)) = d.disruption_extent {
            for i in (last + 1)..d.compared {
                assert!(!d.disrupted[i],
                    "boundary {} after disruption extent should match", i);
            }
        }
    }

    #[test]
    fn char_identical_produces_no_disruption() {
        let s = Sequence::from_chars("problem", "problem");
        let v = s.clone();
        let d = compare(&edge_profile(&s), &edge_profile(&v));
        assert_eq!(d.disruption_count, 0);
    }

    // ── Word scale ────────────────────────────────────────────────────

    #[test]
    fn word_transposition_localizes_disruption() {
        // "the dog chased the ball" — transpose positions 1 and 2 (dog ↔ chased).
        let s = Sequence::from_words("the dog chased the ball", "base");
        let v = apply(&s, &Intervention::Transposition(1, 2));
        let rp = edge_profile(&s);
        let vp = edge_profile(&v);
        let d  = compare(&rp, &vp);

        assert!(d.disruption_count > 0,
            "word transposition should produce disruption");
        assert!(d.recovery_at.is_some(),
            "word transposition should recover");
    }

    #[test]
    fn word_displacement_produces_disruption() {
        // Move "chased" (pos 2) to end (pos 4).
        let s = Sequence::from_words("the dog chased the ball", "base");
        let v = apply(&s, &Intervention::Displacement(2, 4));
        let rp = edge_profile(&s);
        let vp = edge_profile(&v);
        let d  = compare(&rp, &vp);

        assert!(d.disruption_count > 0,
            "displacement should produce disruption");
    }

    #[test]
    fn word_identical_produces_no_disruption() {
        let s = Sequence::from_words("the dog chased the ball", "base");
        let v = s.clone();
        let d = compare(&edge_profile(&s), &edge_profile(&v));
        assert_eq!(d.disruption_count, 0);
    }

    // ── Sentence scale ────────────────────────────────────────────────

    #[test]
    fn sentence_transposition_produces_disruption() {
        let sentences = vec![
            "John picked up the glass",
            "He carried it into the kitchen",
            "The glass slipped from his hand",
            "It shattered",
        ];
        let s = Sequence::from_sentences(sentences, "base");
        let v = apply(&s, &Intervention::Transposition(2, 3));
        let rp = edge_profile(&s);
        let vp = edge_profile(&v);
        let d  = compare(&rp, &vp);

        assert!(d.disruption_count > 0,
            "sentence transposition should produce disruption");
    }

    #[test]
    fn sentence_identical_produces_no_disruption() {
        let sentences = vec![
            "John picked up the glass",
            "He carried it into the kitchen",
            "The glass slipped from his hand",
            "It shattered",
        ];
        let s = Sequence::from_sentences(sentences, "base");
        let v = s.clone();
        let d = compare(&edge_profile(&s), &edge_profile(&v));
        assert_eq!(d.disruption_count, 0);
    }

    // ── Scale invariance ──────────────────────────────────────────────

    #[test]
    fn transposition_produces_disruption_at_all_scales() {
        // Same intervention type (Transposition(1,2)) at three scales.
        // All three should produce non-zero disruption.
        let char_s  = Sequence::from_chars("abcde", "chars");
        let word_s  = Sequence::from_words("a b c d e", "words");
        let sent_s  = Sequence::from_sentences(
            vec!["a", "b", "c", "d", "e"], "sentences"
        );

        for (label, seq) in &[
            ("char",     &char_s),
            ("word",     &word_s),
            ("sentence", &sent_s),
        ] {
            let v  = apply(seq, &Intervention::Transposition(1, 2));
            let rp = edge_profile(seq);
            let vp = edge_profile(&v);
            let d  = compare(&rp, &vp);
            assert!(d.disruption_count > 0,
                "{} scale: Transposition(1,2) should produce disruption", label);
        }
    }

    #[test]
    fn disruption_extent_matches_intervention_scope() {
        // Transposition(1,2) on a 5-element sequence.
        // Expected disrupted boundaries: 0 (A→B vs A→C), 1 (B→C vs C→B), 2 (C→D vs B→D).
        // Boundaries 3 (D→E) should match.
        let s = Sequence::from_words("a b c d e", "base");
        let v = apply(&s, &Intervention::Transposition(1, 2));
        let d = compare(&edge_profile(&s), &edge_profile(&v));

        assert!(!d.disrupted[3], "boundary 3 (d→e) should match after recovery");
        if let Some((f, _)) = d.disruption_extent {
            assert_eq!(f, 0, "disruption should start at boundary 0");
        }
    }
}
