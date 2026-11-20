//! The `dharma_ladder` suite: one test per scenario in `features/suites/dharma_ladder.feature`.
//!
//! Written by the Prism-DevRel builder from the app's prompt. The theorems are
//! checked by the Lean kernel when `just vv` runs `prismpm verify` on `app/`.
//! What these tests hold fixed is what each theorem is about, so a weakened
//! statement that still verifies fails the gate.

use std::path::PathBuf;

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .ancestors()
        .nth(2)
        .expect("crates/conformance is two below the root")
        .to_path_buf()
}

/// The body of the top-level declaration named `name`, up to its closing brace.
fn declaration<'a>(model: &'a str, name: &str) -> &'a str {
    let start = model
        .find(&format!("\n  name: {name},\n"))
        .unwrap_or_else(|| panic!("{name} is not declared"));
    let end = model[start..]
        .find("\n}\n")
        .unwrap_or_else(|| panic!("{name} is not closed"));
    &model[start..start + end]
}

fn module(path: &str) -> String {
    std::fs::read_to_string(root().join(path)).expect("the module")
}

/// The condition no request may meet, and the theorem that says none does.
fn pinned(breaks: &str, condition: &str, theorem: &str) {
    let model = module("app/src/DharmaLadderProofs.lex.tex");
    let found = declaration(&model, breaks);
    assert!(
        found.contains(condition),
        "the condition of {breaks} changed:\n{found}"
    );
    let holds = declaration(&model, theorem);
    let statement = format!("statement: (({breaks} request) = false),");
    assert!(
        holds.contains(&statement),
        "{theorem} must state that no request meets {breaks}:\n{holds}"
    );
}

/// A facet theorem: proved by `decide`, about exactly this predicate.
fn holds(path: &str, theorem: &str, statement: &str) {
    let model = module(path);
    let found = declaration(&model, theorem);
    assert!(
        found.contains(&format!("statement: {statement},")),
        "{theorem} must state {statement}:\n{found}"
    );
    assert!(
        found.contains("proof: decide{}"),
        "{theorem} is proved by decide"
    );
}

/// `DL-01`: The app never answers suspended when event is bad and strikes is at most 1
#[test]
fn dharma_ladder_dl_01() {
    pinned(
        "breaksInv001",
        "body: ((((((project request strikes) <=? 1000) && ((project request event) <? 5)) && ((project request event) =? (DharmaLadder::eventBad))) && ((project request strikes) <=? 1)) && ((DharmaLadder::judge request) =? (DharmaLadder::answerSuspended)))",
        "inv001Holds",
    );
}

/// `DL-02`: The app never answers suspended when event is injection and strikes equals 0
#[test]
fn dharma_ladder_dl_02() {
    pinned(
        "breaksInv002",
        "body: ((((((project request strikes) <=? 1000) && ((project request event) <? 5)) && ((project request event) =? (DharmaLadder::eventInjection))) && ((project request strikes) =? 0)) && ((DharmaLadder::judge request) =? (DharmaLadder::answerSuspended)))",
        "inv002Holds",
    );
}

/// `DL-03`: The app never answers trading when event is good and strikes is at least 3
#[test]
fn dharma_ladder_dl_03() {
    pinned(
        "breaksInv003",
        "body: ((((((project request strikes) <=? 1000) && ((project request event) <? 5)) && ((project request event) =? (DharmaLadder::eventGood))) && (3 <=? (project request strikes))) && ((DharmaLadder::judge request) =? (DharmaLadder::answerTrading)))",
        "inv003Holds",
    );
}

/// `DL-04`: The app never answers suspended when event is good and strikes is at most 2
#[test]
fn dharma_ladder_dl_04() {
    pinned(
        "breaksInv004",
        "body: ((((((project request strikes) <=? 1000) && ((project request event) <? 5)) && ((project request event) =? (DharmaLadder::eventGood))) && ((project request strikes) <=? 2)) && ((DharmaLadder::judge request) =? (DharmaLadder::answerSuspended)))",
        "inv004Holds",
    );
}

/// `DL-05`: The app never answers suspended when event is pardonbad and strikes is at most 3
#[test]
fn dharma_ladder_dl_05() {
    pinned(
        "breaksInv005",
        "body: ((((((project request strikes) <=? 1000) && ((project request event) <? 5)) && ((project request event) =? (DharmaLadder::eventPardonbad))) && ((project request strikes) <=? 3)) && ((DharmaLadder::judge request) =? (DharmaLadder::answerSuspended)))",
        "inv005Holds",
    );
}

/// `DL-06`: The architecture description declares every stakeholder, concern, component, edge, viewpoint and view it references
#[test]
fn dharma_ladder_dl_06() {
    holds(
        "app/src/Architecture.lex.tex",
        "architectureDescriptionHolds",
        "((architectureValid) = true)",
    );
}

/// `DL-07`: The ISO/IEC 25010:2023 quality model declares all 9 characteristics and 40 subcharacteristics, states each subcharacteristic by a requirement or lists it open, and every parent it names is declared
#[test]
fn dharma_ladder_dl_07() {
    holds(
        "app/src/Quality.lex.tex",
        "qualityModelHolds",
        "((qualityValid) = true)",
    );
}

/// `DL-08`: Every STRIDE category each component and data flow is open to is answered by a threat or stated open, and no other pair is stated open
#[test]
fn dharma_ladder_dl_08() {
    holds(
        "app/src/Security.lex.tex",
        "strideEnumerated",
        "((securityValid) = true)",
    );
}

/// `DL-09`: Every compliance obligation is numbered in order and names its evidence
#[test]
fn dharma_ladder_dl_09() {
    holds(
        "app/src/Compliance.lex.tex",
        "everyObligationEvidenced",
        "((obligationsEvidenced 0 (obligations)) = true)",
    );
}
