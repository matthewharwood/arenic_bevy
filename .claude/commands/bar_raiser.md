# Purpose & Guardrails

This playbook turns any submission into **decision-grade** research through a repeatable, surgical process. It doubles
the prior guidance by adding **pre-flight checks, artifact standards, per-rubric “good vs. red-flag” cues, richer task
specs,** and a **Decision Memo** template + quality gate.

**Principled Discretion.** This protocol is the default. For genuinely novel/exploratory work where a rubric may not
fully apply, you may adjust focus or scoring—**but you must explicitly document and defend**:

* *What* you deviated from,
* *Why* it’s justified (risk/benefit),
* *How* you still ensure decision-grade rigor.

---

# Review Workflow (expanded)

## 0) Pre-Flight Checklist (before reading)

* **Scope sanity:** What decision could this research inform *today*?
* **Artifacts present:** data sources named, methods overview, metric definitions, code/repro link (even if WIP).
* **Conflicts & ethics:** disclosures, IRB or privacy notes if applicable.

## 1) Pass 1 — Narrative Absorb (no notes)

Goal: Internalize the story and intent.
Output: 5 bullet “reader reactions”: one *excited*, one *confused*, one *incomplete*, two *hypotheses to test*.
Timebox: ≤15 minutes.

## 2) Pass 2 — Structural Mapping

Extract:

* **Thesis:** claim + target decision.
* **Evidence pillars:** data, methods, key figures/tables.
* **Scope boundaries:** what’s in/out; flag arbitrary limits.
* **Assumptions:** operational, statistical, business.
  Output: A one-page map with links to supporting artifacts.

## 3) Gap & Risk Pass

Actions:

* **Score the 12-D rubric** (Section 3) with one-line justification each.
* **Risk register:** failure modes, bias vectors, misuse/abuse, deployment pitfalls; severity × likelihood.
* **Discretion log (if used):** record any rubric adjustments and rationale.

## 4) Synthesize → 10× Task Matrix

* Select **≥10** task families (Section 4) relevant to gaps.
* **Deduplicate** overlapping asks; merge into one higher bar.
* For each task: **Goal, Method, Artifacts, Success Criteria, Priority (Must-Fix / Should-Do / Nice).**

## 5) Feedback Packet

Deliver:

* **Executive summary** (8–10 bullets, incl. pass-gate status).
* **Rubric table** with scores + one-liners.
* **10× Task Matrix** (full specs).
* **Must-Fix list** (top 5, ordered).
* **Decision Memo** request (use template in Section 5).
  Tone: specific, comparative, traceable.

---

# 12-Dimension Core Quality Rubric (0–3 each)

**Scoring key:**
0 Missing/incorrect · 1 Partial/major issues · 2 Adequate/replicable in principle · 3 Strong/anticipates criticism

For each dimension: *What good looks like* • *Red flags* • *Required artifact*

1. **Framing & Theory**

* Good: Non-tautological question; theory yields testable predictions.
* Red flags: Vague goals; post-hoc rationalization.
* Artifact: 1-pager with hypotheses + falsifiers.

2. **Literature & Baselines**

* Good: Implements SOTA and a deceptively simple baseline; positions against rivals.
* Red flags: Name-checking without replication; no baseline.
* Artifact: Repro leaderboard + citations.

3. **Data Fitness**

* Good: Representativeness checked; missingness + leakage audited; label fidelity measured (e.g., IRR ≥0.8).
* Red flags: Convenience sampling; silent label drift.
* Artifact: Data dictionary + bias/leakage audit.

4. **Identification/Validity**

* Good: Assumptions explicit; placebo/negatives; diagnostics reported.
* Red flags: Causal language from correlation; untested assumptions.
* Artifact: Spec file + validity checks.

5. **Metrics & Success**

* Good: Decision-relevant metrics with denominators and CIs; tie to costs/benefits.
* Red flags: Vanity metrics; no uncertainty.
* Artifact: Metric spec + backtest.

6. **Robustness & Sensitivity**

* Good: Ablations; alt specs; tornado/elasticities with flip points.
* Red flags: One-spec wonder; fragile results.
* Artifact: Sensitivity notebook.

7. **Generalization**

* Good: OOD cohorts (geo/segment/time); boundary conditions named.
* Red flags: Single cohort; “it probably generalizes.”
* Artifact: Cohort report.

8. **Reproducibility**

* Good: Container/lockfile, seeds, deterministic steps, “cold rerun” works.
* Red flags: Hidden preprocessing; “works on my machine.”
* Artifact: Repo with `README`, `Makefile`, provenance log.

9. **Ethics & Safety**

* Good: Harms/fairness/abuse analyzed; mitigations & monitors shipped.
* Red flags: “Out of scope”; privacy hand-waving.
* Artifact: Risk register + mitigation owners.

10. **Operationalization & Cost**

* Good: SLOs/SLIs, staffing, rollback, TCO by quarter.
* Red flags: “We’ll figure it out later.”
* Artifact: Ops one-pager + cost model.

11. **Communication**

* Good: 3 decision visuals; clear captions; executive TL; limitations stated.
* Red flags: Dense figures; no narrative.
* Artifact: Figure set + TL; limitations box.

12. **Competing Explanations**

* Good: Rival models/theories implemented and adjudicated.
* Red flags: Straw-man alternates or none.
* Artifact: Rival results + adjudication memo.

**Rules:** Any score **<2** ⇒ ≥1 **Must-Fix** task. Total **<24** ⇒ **Major Revision**.

---

# Task Generation (expanded)

Select **≥10** families per cycle; tailor and merge. Each task must include **Goal, Method, Artifacts, Success, Priority
**.

1. **Rival theories & predictions** — Pre-register divergent predictions; **Pass:** measurable divergence on ≥1 metric.
2. **Strong baselines & leaderboard** — ≥3 baselines incl. simple heuristic; **Pass:** reproducible ranking.
3. **Bias/leakage audit** — Source/label/temporal/proxy checks; **Pass:** mitigations + residual risk quantified.
4. **Identification stress tests** — Placebos/negatives/synthetic shocks; **Pass:** effects survive or tighten to honest
   bounds.
5. **Decision-metric upgrade** — Shift to cost-per-correct-decision (or equivalent); **Pass:** CI-backed improvement.
6. **Global sensitivity** — Tornado + elasticities; **Pass:** flip points and top-3 fragile assumptions called out.
7. **Out-of-domain cohorts** — 3 contrasting cohorts; **Pass:** generalization bounds reported.
8. **Full repro package** — Container/lockfile/seeds/`Makefile`/provenance; **Pass:** cold rerun matches ± tolerance.
9. **Safety & misuse red-team** — Enumerate misuse; **Pass:** ≥3 mitigations landed with owners & monitors.
10. **Ops-feasible pilot** — Guardrails/SLOs/rollback; **Pass:** approved runbook + staffing/costs.
11. **Decision visuals** — 3 annotated trade-off figures; **Pass:** ≥80% non-expert comprehension (blind test).
12. **Rival mechanism/model** — Implement serious rival; **Pass:** ruled out or conclusions revised.
13. **Historical causal map** — Tie inflections to practice; **Pass:** sourced causal timeline.
14. **Unit economics & incentives** — Unit P\&L + rent hot-spots; **Pass:** lever table with expected deltas.
15. **Stakeholder/dependency map** — Authority/info/resource flows; **Pass:** bottleneck fixes with trade-offs.
16. **Culture & norms fieldwork** — Ethnographic capture; **Pass:** codified norms impacting outcomes.
17. **Decision loop analysis** — Model cadence/uncertainty; **Pass:** predictions validated on logs/observation.
18. **Externalities & scenario tree** — 1st/2nd/3rd-order impacts; **Pass:** triggers + monitoring indicators.
19. **Pipeline & standards audit** — Entry paths/credential efficacy; **Pass:** policy/practice recommendations.
20. **Cross-country benchmark** — US/EU/Asia contrasts; **Pass:** universal vs. contingent practices labeled.
21. **Task decomposition & AI impact** — Automate-susceptibility matrix; **Pass:** post-AI role design + skill roadmap.
22. **Regulatory futures** — Laws/lobbying/crackdown triggers; **Pass:** compliance playbook + early warnings.
23. **Synthesis & stakeholder action** — One non-obvious insight + concrete action per stakeholder; **Pass:** Decision
    Memo delivered.

**Spec quality bar (for every task):** single-screen brief, unambiguous success criteria, direct link to the rubric
dimension it unblocks.

---

# Pass Gates & Stopping Rules

**Pass when all hold:**

* No dimension <2; **≥6 dimensions = 3**.
* **Reproducibility = 3**, **Communication = 3**.
* **Decision Memo** passes the quality gate (below).
* All **Must-Fix** tasks complete.
* No unresolved **critical** risks.

**Stopping rules:** Pass gate met *and* critical claims have quantified uncertainty; rivals adjudicated; a
decision-maker can act **today** with a pilot/ops plan—or deviation documented via **Principled Discretion**.

---

# Feedback Style (with examples)

* **Specific & surgical:** *“Fig 2: denominator excludes returns; recompute CPCD incl. returns; attach revised CI.”*
* **Actionable:** Every critique → a task with success metric.
* **Comparative:** Always “vs. what?” *Baseline A/B included.*
* **Traceable:** Link each ask to rubric/gate.
* **Respectful, terse, complete:** No theatrics.
  **Bad:** “Consider more cohorts.”
  **Better:** “Add SMB+EU+Q4 cohorts; report CI; flag flips vs. baseline; success = explicit bounds.”

---

# Anti-Patterns to Block (zero-tolerance)

* Results without baselines; metrics without denominators/uncertainty.
* P-hacking/spec fishing; cherry-picked cohorts; silent reruns after peeking.
* Overstated causality from observational designs.
* Missing repro artifacts; hidden preprocessing.
* Ethical/safety risks hand-waved.
* Volume over value (breadth without synthesis).

**Detector tips:** require pre-registration for confirmatory tests; diff-in-diff or negatives for identification;
leakage scanners; seed logs.

---

# Output Format (richer)

1. **Executive Summary (≤10 bullets):** outcome, top gaps, must-fixes, pass-gate status.
2. **Rubric Scores (table):** 12 dims with one-line justification + links.
3. **10× Task Matrix:** deduplicated tasks (≥10 axes) with full specs.
4. **Must-Fix Items:** top 5 blockers with owners & dates.
5. **Pass Gate Status:** checklist + delta to pass.
6. **Feedback Cover Note:** professional summary + next review date.
7. **Appendices:** figures, diagnostics, sensitivity, data dictionary links.

---

# Evidence & Deliverables (reviewer-enforced)

* **Repo layout:** `/data`, `/notebooks`, `/src`, `/reports`, `/figures`, `/env` (lockfile), `/provenance`.
* **Provenance log:** dataset version, transforms, seeds, timestamps.
* **Metrics ledger:** definitions, denominators, CI method, version.
* **Risk register:** severity × likelihood, mitigation owner, due date.
* **Cold-start script:** `make reproduce` end-to-end run succeeds.

---

# Decision Memo (required) — Standard Template (≤2 pages + appendix)

**Header**: Title • Owner/DRIs • Date/Version • Decision deadline/cadence • Stakeholders (A/C/I)

**0) TL;DR (≤5 bullets)**
Ask • Recommendation (Option # + why) • Expected impact (with CI) • Top risks + mitigations • Cost & timeline

**1) The Decision**
Exact question (binary/choice) • Why now (trigger/window; risk of waiting)

**2) Options Considered (incl. Do Nothing)**
Option 0 — Status Quo • Option 1 — … • Option 2 — … (only if materially different)

**Options Scorecard (weights sum to 100%)**

| Criterion (weight %)                              | Opt 0 | Opt 1 | Opt 2 | Notes            |
|---------------------------------------------------|------:|------:|------:|------------------|
| Decision metric (e.g., cost-per-correct-decision) |       |       |       | CI / sensitivity |
| Time to value                                     |       |       |       |                  |
| Risk & reversibility                              |       |       |       | rollback path    |
| Ethics/safety exposure                            |       |       |       | mitigations      |
| TCO (12–24 mo)                                    |       |       |       | capex/opex       |
| Org fit & staffing                                |       |       |       |                  |
| Externalities (2nd/3rd-order)                     |       |       |       |                  |
| **Weighted total**                                |       |       |       |                  |

**3) Recommendation**
Chosen option & rationale (tie to scorecard/strategy) • What we will measure (primary/secondary metrics, baselines,
CIs) • Reversal thresholds (what would change our mind)

**4) Evidence & Uncertainty**
Key results (effect sizes + CI) • Rival explanations tested • Sensitivity (top assumptions + flip points)

**5) Cost, Resourcing, Timeline**
TCO by quarter • Staffing plan (roles, % allocation) • Pilot → GA milestones (dates)

**6) Operational Plan (Pilot-first)**
Scope & guardrails (cohorts/geos/traffic share) • SLIs/SLOs & alerts • Rollback criteria (triggers/steps/owner)

**7) Ethics, Safety, Compliance**
Key risks (privacy/fairness/misuse/regulatory) • Mitigations & monitors (owners & dates)

**8) Open Questions & Next Steps**
Unknowns with owners/dates • Decision log (what we’re not doing and why)

**Decision Memo Quality Gate**

* [ ] Decision explicit (binary/choice) with **Do Nothing** baseline
* [ ] Impacts quantified with denominators + uncertainty; sensitivity summarized
* [ ] Pilot/SLOs/rollback concrete (owners named)
* [ ] Ethics/safety risks + shipped mitigations listed
* [ ] Reversal thresholds pre-committed
* [ ] ≤2 pages main; details in appendix

---

# Calibration & Consistency (for Bar Raisers)

* **Rubric calibration:** monthly cross-review of 3 anonymized memos; reconcile score drift.
* **Benchmarks library:** curated examples of “3-level” work per dimension.
* **Discretion registry:** log all deviations + outcomes; review quarterly.
* **Time standards:** Pass 1 ≤15m; Pass 2 ≤30m; Gap/Risk ≤30m; Task Matrix ≤45m; Packet ≤30m.

---

# Quick Reviewer Checklist

* [ ] Pre-flight complete; ethics/privacy noted.
* [ ] 12-D rubric scored with one-line evidence.
* [ ] ≥10 task families; tasks deduplicated; specs complete.
* [ ] Must-Fix list ordered with owners/dates.
* [ ] Repro contract verified by cold run.
* [ ] Decision Memo included and passes quality gate.
* [ ] Pass gate/stop rules evaluated; Principled Discretion (if any) documented.
* [ ] Feedback is specific, comparative, and traceable.
