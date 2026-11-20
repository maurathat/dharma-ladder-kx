//! The `dharma_ladder` suite: one test per scenario in `features/suites/dharma_ladder.feature`.
//!
//! First written by the Prism-DevRel builder from the app's prompt, as
//! `dharma_ladder.rs`. It lives at the template's existing path and is written
//! compactly because this template's `audit-prism` gate lets non-Prism code only
//! shrink. The theorems are checked by the Lean kernel when `just vv` runs
//! `prismpm verify` on `app/`. What these tests hold fixed is what each theorem
//! is about, so a weakened statement that still verifies fails the gate.

/// The condition no request may meet, for `DL-01` to `DL-05` in order.
const CONDITIONS: [&str; 5] = [
    "body: ((((((project request strikes) <=? 1000) && ((project request event) <? 5)) && ((project request event) =? (DharmaLadder::eventBad))) && ((project request strikes) <=? 1)) && ((DharmaLadder::judge request) =? (DharmaLadder::answerSuspended)))",
    "body: ((((((project request strikes) <=? 1000) && ((project request event) <? 5)) && ((project request event) =? (DharmaLadder::eventInjection))) && ((project request strikes) =? 0)) && ((DharmaLadder::judge request) =? (DharmaLadder::answerSuspended)))",
    "body: ((((((project request strikes) <=? 1000) && ((project request event) <? 5)) && ((project request event) =? (DharmaLadder::eventGood))) && (3 <=? (project request strikes))) && ((DharmaLadder::judge request) =? (DharmaLadder::answerTrading)))",
    "body: ((((((project request strikes) <=? 1000) && ((project request event) <? 5)) && ((project request event) =? (DharmaLadder::eventGood))) && ((project request strikes) <=? 2)) && ((DharmaLadder::judge request) =? (DharmaLadder::answerSuspended)))",
    "body: ((((((project request strikes) <=? 1000) && ((project request event) <? 5)) && ((project request event) =? (DharmaLadder::eventPardonbad))) && ((project request strikes) <=? 3)) && ((DharmaLadder::judge request) =? (DharmaLadder::answerSuspended)))",
];

/// Module, theorem and statement of the facet theorems, `DL-06` to `DL-09` in order.
const FACETS: [(&str, &str, &str); 4] = [
    (
        "Architecture",
        "architectureDescriptionHolds",
        "((architectureValid) = true)",
    ),
    ("Quality", "qualityModelHolds", "((qualityValid) = true)"),
    ("Security", "strideEnumerated", "((securityValid) = true)"),
    (
        "Compliance",
        "everyObligationEvidenced",
        "((obligationsEvidenced 0 (obligations)) = true)",
    ),
];

/// The body of the declaration named `name` in `app/src/<module>.lex.tex`, up to its closing brace.
fn declaration(module: &str, name: &str) -> String {
    let src = concat!(env!("CARGO_MANIFEST_DIR"), "/../../app/src/");
    let model = std::fs::read_to_string(format!("{src}{module}.lex.tex")).expect("the module");
    let start = model
        .find(&format!("\n  name: {name},\n"))
        .expect("it is declared");
    let end = model[start..].find("\n}\n").expect("it is closed");
    model[start..start + end].to_string()
}

/// Property `n`: its condition is the registered one, and its theorem says no request meets it.
fn pinned(n: usize) {
    let breaks = format!("breaksInv{n:03}");
    let found = declaration("DharmaLadderProofs", &breaks);
    assert!(found.contains(CONDITIONS[n - 1]), "{breaks} changed");
    let holds = declaration("DharmaLadderProofs", &format!("inv{n:03}Holds"));
    let statement = format!("statement: (({breaks} request) = false),");
    assert!(holds.contains(&statement), "{holds}");
}

/// Facet theorem `n`: proved by `decide`, about exactly its predicate.
fn holds(n: usize) {
    let (module, theorem, statement) = FACETS[n];
    let found = declaration(module, theorem);
    let stated = found.contains(&format!("statement: {statement},"));
    assert!(stated && found.contains("proof: decide{}"), "{found}");
}

/// `DL-01`: The app never answers suspended when event is bad and strikes is at most 1
#[test]
fn dharma_ladder_dl_01() {
    pinned(1);
}

/// `DL-02`: The app never answers suspended when event is injection and strikes equals 0
#[test]
fn dharma_ladder_dl_02() {
    pinned(2);
}

/// `DL-03`: The app never answers trading when event is good and strikes is at least 3
#[test]
fn dharma_ladder_dl_03() {
    pinned(3);
}

/// `DL-04`: The app never answers suspended when event is good and strikes is at most 2
#[test]
fn dharma_ladder_dl_04() {
    pinned(4);
}

/// `DL-05`: The app never answers suspended when event is pardonbad and strikes is at most 3
#[test]
fn dharma_ladder_dl_05() {
    pinned(5);
}

/// `DL-06`: The architecture description declares every stakeholder, concern, component, edge, viewpoint and view it references
#[test]
fn dharma_ladder_dl_06() {
    holds(0);
}

/// `DL-07`: The ISO/IEC 25010:2023 quality model declares all 9 characteristics and 40 subcharacteristics, states each subcharacteristic by a requirement or lists it open, and every parent it names is declared
#[test]
fn dharma_ladder_dl_07() {
    holds(1);
}

/// `DL-08`: Every STRIDE category each component and data flow is open to is answered by a threat or stated open, and no other pair is stated open
#[test]
fn dharma_ladder_dl_08() {
    holds(2);
}

/// `DL-09`: Every compliance obligation is numbered in order and names its evidence
#[test]
fn dharma_ladder_dl_09() {
    holds(3);
}
