# abr-language-structure

**ABR Language Structure — V1.0**

Metatron Dynamics, Inc. · Bounded over D. No claim beyond D.

## Declaration

Coherence is a relational property, not a linguistic one.

Two loci are coherent if their co-occurrence in the declared sequence
is stable across exposures. M measures the state at each locus as the
token identity — the word itself, lowercased. No embedding. No borrowed
structure. No NLP heuristics.

**ρ_n(i,j)** = 1.0 if token j appears within declared proximity W of
token i in this pass. 0.0 otherwise.

**Accumulation rule** (Origin-declared):

    ρ_acc(token_i, token_j) ←
        ρ_acc(token_i, token_j) + η · ρ_n · (1 − ρ_acc(token_i, token_j))

EdgeStore is keyed by **token identity pair**, not position pair.
Coherence is a property of which tokens relate to which — not of
where they happened to sit in one particular sequence.

**Admission**: edge (token_i, token_j) admitted if ρ_acc ≥ θ_min.

**k per locus**: count of admitted targets from each source token type.

**Mean k**: mean admitted relational width across source token types.

## What This Measures

If coherence in language is real and bounded, coherent text should
produce small, stable k — the same token pairs consistently co-occur
within declared proximity.

Incoherent text — same words, order destroyed — lacks that pattern.
Pairs co-occur randomly. ρ_acc does not accumulate reliably. k stays
large or uneven.

The gap between k and n, at transformer scale, is what quadratic
attention wastes computation on. This is the first measurement of
that gap from declared relational structure alone.

## Open Conditions

- **OC-COH-1**: Window W is Origin-declared, not derived. Its exact
  value is an open condition. Current default: W=4.
- **OC-COH-2**: θ_min inherited from abr-relational-attention (0.05).
  Not yet calibrated for this domain.
- **OC-COH-3**: Binary ρ_n ∈ {0,1} is the simplest declaration.
  A graded ρ_n decaying with distance within W is a Phase 2 candidate.

## Build and Run

```
cargo test
cargo run --release
```

To test your own passages, edit the `coherent` and `incoherent`
strings in `src/main.rs`. Any text works — no preprocessing required.
