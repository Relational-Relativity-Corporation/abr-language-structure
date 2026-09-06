# abr-language-structure
**ABR Language Structure — V3.2**
Metatron Dynamics, Inc. · Bounded over D. No claim beyond D.

## What This Establishes

Two experimental findings, formally tested:

**Finding 1 — Structural Δ can be content-independent.**
The same intervention applied to sequences with completely different locus
identities produces structurally identical Δ. The measurement depends on
the relational transformation, not what the loci contain.

**Finding 2 — Adjacency Δ is insufficient for coherence discrimination.**
Adjacency topology is content-blind. It cannot distinguish a causally
coherent sentence sequence from an accidentally ordered one. This closes
OC-INT-1 experimentally and declares exactly what additional relation
must be established next.

## Declaration

**Observable:** the change in declared adjacency relations produced by a
known intervention on an ordered sequence of loci.

**Locus:** any declared position in a sequence — character, word, or sentence.
The measurement does not depend on which.

**Edge profile:** for a sequence of n loci, the edge profile is a vector of
n-1 directed pairs: `profile[k] = (L_k, L_{k+1})`.

**Δ between reference and variant:**
```
disrupted[k] = true  if profile_ref[k] ≠ profile_var[k]
             = false otherwise
```

Locus identity is used only to key edge pairs. It is not a semantic variable.

**Scale invariance (observed, not confirmed):**
`Δ_T(1,2)^char ≅ Δ_T(1,2)^word ≅ Δ_T(1,2)^sentence`
for declared structural observables. Paragraph and above: pending.

## Intervention Types

```
Transposition(i, j)  — swap loci at positions i and j
Displacement(i, j)   — move locus at position i to position j,
                        shifting intervening loci
```

The intervention is performed by the researcher. Ground truth is known.
This is the experimental advantage over open passage analysis.

## Modules

| File | Contents |
|------|----------|
| `src/intervention.rs` | Resolution-independent intervention runner (V3.2) |
| `src/character.rs` | Character-resolution edge profile comparison (V2.2) |
| `src/state.rs` | Relational state vector experiment (V3.0, informative) |
| `src/coherence.rs` | Original accumulation-based coherence (V2.0, superseded) |

## Build and Run

```
cargo test        # 36 tests across all modules
cargo run --release
```

## Open Conditions

- **OC-INT-1**: CLOSED — adjacency topology is content-blind. Cannot
  distinguish causal from accidental sequence. Next declared relation
  required for that discrimination.
- **OC-INT-2**: Locus identity keys edges only — not a semantic variable.
- **OC-INT-3**: Interventions are researcher-declared. Ground truth known.
- **OC-SCALE**: Scale invariance observed for tested interventions across
  character, word, and sentence resolutions. Paragraph and above: pending.
- **OC-NEXT**: What relation distinguishes causal from accidental sequence
  under identical controlled intervention? That is the next experiment.

## Version History

| Version | Description |
|---------|-------------|
| V1.0 | Accumulation-based coherence, token co-occurrence, mean k |
| V2.0 | Field coherence — triple sampling, grounded/ungrounded |
| V2.2 | Character-resolution edge profile comparison |
| V3.0 | Relational state vector X_i — disconnection ratio |
| V3.1 | Controlled intervention runner — scale invariance observed |
| V3.2 | Content independence confirmed; OC-INT-1 closed experimentally |
