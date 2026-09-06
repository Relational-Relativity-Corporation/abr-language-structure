// state.rs — Metatron Dynamics, Inc.
// ABR Language Structure — V3.0 (relational state vector)
// Bounded over D. No claim beyond D.
//
// ── Declaration ──────────────────────────────────────────────────────────────
//
// Primitive operator: Δ — change in relational state across a boundary.
// Δ is not redefined here. What changes from V2.x is the state vector
// it acts on.
//
// Relational state X_i:
//   The set of relations still active at locus i.
//   A relation r(a, b) is a directed pair of token identities.
//   X_i ⊆ { r(a,b) | a, b ∈ observed tokens }.
//
// Relation lifecycle (Origin-declared):
//   ENTER:   r(a, b) enters X when the adjacent pair (a, b) is first observed.
//   PERSIST: r(a, b) remains in X while either a or b has appeared within
//            the last W positions (declared window, OC-ST-1).
//   CLOSE:   r(a, b) closes when neither a nor b has appeared within W positions.
//
// State transition at boundary i → i+1:
//   Given X_i and the new token t_{i+1}:
//
//   Connections: relations in X_i that involve t_{i+1}.
//     These are the relations the new locus connects to.
//     A locus with many connections enters a dense relational field.
//     A locus with zero connections arrives into an empty field — disruption.
//
//   Survivors: relations in X_i that involve neither t_i nor t_{i+1}.
//     These are still active but untouched by this transition.
//
//   Closures: relations that were in X_i but are now outside the window.
//     These have expired — neither party has appeared recently enough.
//
//   New: r(t_i, t_{i+1}) — the relation introduced by this boundary.
//
//   X_{i+1} = (X_i ∪ {new}) \ {closures}
//
// Δ at boundary i → i+1:
//   connections:  |{ r ∈ X_i | t_{i+1} ∈ r }|   — how many active relations
//                                                    the new locus connects to
//   closures:     |closed relations|               — how many relations expired
//   new_relation: 1 if r(t_i, t_{i+1}) is new to X, 0 if already present
//   delta_size:   |X_{i+1}| - |X_i|               — net change in active set
//
// Coherence signal (emergent — not declared as primitive):
//   A locus that connects to zero active relations while the state is
//   non-empty is a candidate disruption point.
//   Incoherence = failure of the new observation to connect to the
//   established relational field.
//
// Open Conditions:
//   OC-ST-1: Window W = 5 (declared). Closure condition depends on W.
//            Calibration of W is an open experimental question.
//   OC-ST-2: Relation identity is token-pair identity (same as prior modules).
//            No semantic content imported.
//   OC-ST-3: "Connects to" is defined as token identity match within
//            an active relation. No semantic similarity used.
//   OC-ST-4: The four Δ components (connections, closures, new_relation,
//            delta_size) are candidate observables, not established variables.
//            Variable discovery experiment: which subset discriminates
//            coherent from incoherent transformations?

use std::collections::{HashMap, HashSet, VecDeque};

pub struct StateConfig {
    /// Window W: a relation r(a,b) persists while either a or b
    /// has appeared within the last W positions. OC-ST-1.
    pub window: usize,
}

/// A directed relation between two token identities.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Relation {
    pub from: String,
    pub to:   String,
}

/// The observable state transition at one boundary.
#[derive(Debug, Clone)]
pub struct BoundaryDelta {
    pub position:     usize,
    pub token_left:   String,
    pub token_right:  String,
    /// Relations in X_i that involve token_right — connections made.
    pub connections:  usize,
    /// Relations that closed at this step (expired from window).
    pub closures:     usize,
    /// Whether r(token_left, token_right) is new to X.
    pub new_relation: bool,
    /// Net change in active relation set size: |X_{i+1}| - |X_i|.
    pub delta_size:   i64,
    /// Size of X after this transition.
    pub state_size:   usize,
    /// Zero connections into a non-empty field — candidate disruption.
    pub disconnected: bool,
}

/// Full measurement result for a passage.
pub struct StateResult {
    pub tokens:   Vec<String>,
    pub deltas:   Vec<BoundaryDelta>,
    /// Number of disconnected loci (connections=0, state non-empty).
    pub disconnected_count: usize,
    /// Total boundaries measured.
    pub boundary_count: usize,
    /// Disconnection ratio: disconnected / boundary_count.
    pub disconnection_ratio: f64,
}

/// Tokenize: lowercase, split on whitespace.
fn tokenize(text: &str) -> Vec<String> {
    text.split_whitespace()
        .map(|w| w.to_lowercase())
        .collect()
}

/// Run the relational state measurement over a passage.
pub fn measure_state(text: &str, config: &StateConfig) -> StateResult {
    let tokens = tokenize(text);
    let n = tokens.len();

    if n < 2 {
        return StateResult {
            tokens,
            deltas: vec![],
            disconnected_count: 0,
            boundary_count: 0,
            disconnection_ratio: 0.0,
        };
    }

    // Active relation set X.
    let mut active: HashSet<Relation> = HashSet::new();
    // Last-seen position for each token identity.
    let mut last_seen: HashMap<String, usize> = HashMap::new();
    // Recent token window (position queue).
    let mut recent: VecDeque<String> = VecDeque::new();

    let mut deltas: Vec<BoundaryDelta> = Vec::new();

    for i in 0..(n - 1) {
        let t_left  = &tokens[i];
        let t_right = &tokens[i + 1];

        // Update last_seen for t_left.
        last_seen.insert(t_left.clone(), i);
        recent.push_back(t_left.clone());
        if recent.len() > config.window {
            recent.pop_front();
        }

        // ── Step 1: expire relations outside window ───────────────────
        // A relation r(a,b) closes if neither a nor b appears in recent.
        let recent_set: HashSet<&String> = recent.iter().collect();
        let before_size = active.len();
        active.retain(|r| {
            recent_set.contains(&&r.from) || recent_set.contains(&&r.to)
        });
        let closures = before_size - active.len();

        // ── Step 2: count connections ─────────────────────────────────
        // Relations in X that involve t_right.
        let connections = active.iter()
            .filter(|r| r.from == *t_right || r.to == *t_right)
            .count();

        // ── Step 3: introduce new relation r(t_left, t_right) ─────────
        let new_rel = Relation {
            from: t_left.clone(),
            to:   t_right.clone(),
        };
        let new_relation = !active.contains(&new_rel);
        active.insert(new_rel);

        // ── Step 4: compute delta ─────────────────────────────────────
        let state_size  = active.len();
        let delta_size  = state_size as i64 - before_size as i64;
        // Disconnected: zero connections into a non-empty prior field.
        let disconnected = connections == 0 && before_size > 0;

        deltas.push(BoundaryDelta {
            position: i,
            token_left:  t_left.clone(),
            token_right: t_right.clone(),
            connections,
            closures,
            new_relation,
            delta_size,
            state_size,
            disconnected,
        });
    }

    let boundary_count      = deltas.len();
    let disconnected_count  = deltas.iter().filter(|d| d.disconnected).count();
    let disconnection_ratio = if boundary_count == 0 {
        0.0
    } else {
        disconnected_count as f64 / boundary_count as f64
    };

    StateResult {
        tokens,
        deltas,
        disconnected_count,
        boundary_count,
        disconnection_ratio,
    }
}

/// Format a state measurement report.
pub fn format_state_report(label: &str, r: &StateResult) -> String {
    let mut out = String::new();

    out.push_str(&format!(
        "── {} ─────────────────────────────────────────\n", label
    ));
    out.push_str(&format!("Tokens:              {}\n", r.tokens.len()));
    out.push_str(&format!("Boundaries:          {}\n", r.boundary_count));
    out.push_str(&format!("Disconnected loci:   {}\n", r.disconnected_count));
    out.push_str(&format!(
        "Disconnection ratio: {:.4}\n", r.disconnection_ratio
    ));
    out.push_str("\nBoundary-by-boundary Δ:\n");
    out.push_str(
        "  [pos] left         → right          conn  close  new  |X|  Δsize  status\n"
    );

    for d in &r.deltas {
        let status = if d.disconnected { "DISCON" } else { "ok    " };
        out.push_str(&format!(
            "  [{:>3}] {:<12} → {:<12}  {:>4}  {:>5}  {:>3}  {:>3}  {:>5}  {}\n",
            d.position,
            d.token_left,
            d.token_right,
            d.connections,
            d.closures,
            if d.new_relation { "Y" } else { "N" },
            d.state_size,
            d.delta_size,
            status,
        ));
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cfg() -> StateConfig { StateConfig { window: 5 } }

    #[test]
    fn empty_produces_no_deltas() {
        let r = measure_state("", &cfg());
        assert_eq!(r.boundary_count, 0);
    }

    #[test]
    fn single_token_produces_no_deltas() {
        let r = measure_state("dog", &cfg());
        assert_eq!(r.boundary_count, 0);
    }

    #[test]
    fn first_boundary_has_zero_connections() {
        // At the very first boundary, X is empty — connections must be 0.
        let r = measure_state("the dog chased", &cfg());
        assert_eq!(r.deltas[0].connections, 0,
            "first boundary: X empty, connections must be 0");
        assert!(!r.deltas[0].disconnected,
            "first boundary: disconnected requires non-empty prior X");
    }

    #[test]
    fn repeated_token_generates_connections() {
        // "cat sat cat" — at boundary 2 (sat → cat), "cat" was seen at
        // position 0 and is still in the recent window.
        // r(cat, sat) is active. token_right "cat" appears as `from` in it.
        // Connection count should be > 0.
        let r = measure_state("cat sat cat sat", &cfg());
        // At boundary 2 (cat → sat), token_right is "sat".
        // r(cat, sat) is active. "sat" appears as `to` in it.
        let d2 = &r.deltas[2];
        assert!(d2.connections > 0,
            "returning token 'sat' should connect to active r(cat,sat): \
             got connections={}", d2.connections);
    }

    #[test]
    fn coherent_passage_has_low_disconnection() {
        let text = "the cat sat on the mat the cat sat on the mat";
        let r = measure_state(text, &cfg());
        assert!(r.disconnection_ratio < 0.3,
            "repetitive coherent text should have low disconnection: {:.4}",
            r.disconnection_ratio);
    }

    #[test]
    fn incoherent_passage_has_higher_disconnection() {
        let coherent   = "the cat sat on the mat the cat sat on the mat";
        let incoherent = "purple longitude decided fork sleeping eleven \
                          clouds argued wednesday carpet telephoned silence \
                          gravity forgot umbrella opinions melted thursday \
                          window invented democracy";
        let cfg = cfg();
        let r_c = measure_state(coherent,   &cfg);
        let r_i = measure_state(incoherent, &cfg);
        assert!(r_i.disconnection_ratio >= r_c.disconnection_ratio,
            "incoherent should have >= disconnection ratio: \
             coherent={:.4} incoherent={:.4}",
            r_c.disconnection_ratio, r_i.disconnection_ratio);
    }

    #[test]
    fn window_closure_expires_old_relations() {
        // With W=2, relations close quickly.
        // "a b c d e" — by position 4, r(a,b) should have expired.
        let cfg = StateConfig { window: 2 };
        let r = measure_state("a b c d e f", &cfg);
        // By boundary 4 (d→e), a and b are long gone from the window.
        // State size should reflect only recent relations.
        let d4 = &r.deltas[4];
        assert!(d4.state_size <= 4,
            "window W=2 should expire old relations: state_size={}", d4.state_size);
    }

    #[test]
    fn new_relation_flag_correct() {
        // "cat sat cat sat" — second (cat,sat) is not new.
        let r = measure_state("cat sat cat sat", &cfg());
        // Boundary 0: (cat,sat) — new.
        assert!(r.deltas[0].new_relation, "first (cat,sat) should be new");
        // Boundary 2: (cat,sat) again — not new.
        assert!(!r.deltas[2].new_relation,
            "second (cat,sat) should not be new");
    }
}
