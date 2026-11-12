# Architectural Audit Prompt: General Theory of Trust (GTT)

Copy and paste this codebase-agnostic prompt into your developer agent's session to conduct a systematic, phase-by-phase mathematical trust-formalism and expectation-validation dynamic audit.

---

## CONTEXT & OBJECTIVE
You are an expert Systems Mathematician and Formal Trust Architect. Your task is to audit our codebase against the rigorous multiscale formalism of Devon Shigaki's *The General Theory of Trust (GTT)*.

Your goal is to inspect the codebase and answer these three questions for each phase:
1. **Current Coverage:** What are we doing in our codebase to cover this Phase? (Cite specific directories, files, or packages).
2. **Gaps & Overlaps:** Are there gaps (e.g., missing expectation updates, unvalidated inputs, lacking epistemic tags) or overlaps (e.g., redundant validation matches, duplicate state synchronization loops)?
3. **Actionable Recommendations:** How should we complete this Phase to mathematically enforce GTT laws and align our codebase with the formal Trust Stack?

---

---

## MANDATORY PRE-REQUISITE: READ THE ENTIRE PAPER
Before starting your codebase audit, you MUST read and analyze the full text of the sibling paper PDF file to obtain complete context, grasp the subtle mathematical/neuromodulatory/logical models, and preserve absolute architectural fidelity.

*   **Target PDF File Name:** `gtt.pdf`
*   **Target PDF Path:** `/Users/devonshigaki/Desktop/Kimi_Agent_Orchestrated%20Paper%20Review6/osf_upload_v2/papers/pdf/gtt.pdf`

Ensure you load this file into your context (using your PDF rendering/reading tool) and analyze it end-to-end. Do not rely solely on the summaries provided in this prompt. Complete, deep understanding is critical.

---

---

## MANDATORY AUDIT SCOPE: CORE PRODUCT ONLY
To keep your audit focused on high-signal production systems and prevent wasting time or context tokens on non-production simulators or loosely-coupled marketing/CRM tools, you MUST adhere to the following scope boundaries:

*   **IN SCOPE (Audit these core production directories and domain crates):**
    *   `apps/engine`
    *   `apps/api`
    *   `apps/app`
    *   `apps/adapters`
    *   `apps/scheduler`
    *   `crates/` (the core domain)

*   **OUT OF SCOPE (Prune and ignore these directories immediately from your search and context):**
    *   `sim-*` (including `sim-runner` or other non-production simulators)
    *   `crm` (Customer Relationship Management)
    *   `meet` (meeting/scheduler tools)
    *   `posthog` (telemetry/analytics integration)
    *   `sign` (signing tools)
    *   `website` (marketing static sites)
    *   `auto` (automation scripts)
    *   `cal` (calendar integrations)

Do not search, review, or analyze files outside the designated production scope. Focus 100% of your structural analyses on these core application components.

## THE METHODOLOGY: DIAGNOSTIC FACT-GATHERING & RESEARCH EXPEDITION
This audit is strictly structured as a **non-destructive, diagnostic fact-gathering and research expedition checklist**. Your primary objective is to discover, map, and document the actual current state of the codebase *before* proposing any plans or writing code.

For every single Phase in the roadmap below, you must systematically search the codebase and classify our current status into one of three distinct categories:
1.  **[COMPLETED]:** The concept/rule is fully implemented, operational, and tested. *You must cite the exact directories, file paths, classes, or lines of code as empirical proof.*
2.  **[PARTIAL]:** The concept/rule is partially implemented, but contains gaps, unhandled edge cases, or missing boundaries. *You must cite the existing partial code and detail exactly what remains incomplete.*
3.  **[NOT STARTED]:** The concept/rule is completely absent from the current codebase.

You must compile this exhaustive coverage audit across all phases *prior* to formulating any Critical Adjustment Plans or writing CI/CD enforcement scripts. This prevents premature modifications and honors our existing structural investments.

---

## MANDATORY SYSTEMIC COVERAGE: NO SHALLOW VERIFICATION ("CHECKING THE BOX")
To prevent the common analytical error of shallow verification, you MUST adhere to the following exhaustive, system-wide coverage and template rules:

1.  **No Shallow Verification:** Finding a single working example, class, or module that correctly implements a concept (such as a single event stream, state update, or validation check) does NOT mean the codebase has "covered" that phase. You must inspect the *entire* in-scope codebase to determine if the pattern is applied systemically, or if there are major gaps, omissions, and competing inconsistent patterns elsewhere.
2.  **The "Golden Blueprint" Principle:**
    *   If you discover a correct, highly robust implementation of a pattern (e.g., a perfect GTT state update, a clean singleton database connector, or a robust schema-enforced event handler), declare that specific implementation as the **"Golden Blueprint"** for that pattern.
    *   You must then audit the rest of the codebase to identify every location where this pattern *should* be applied but is completely missing, or where divergent, inferior structures are used instead (creating overlaps, vulnerabilities, and maintenance friction).
    *   Use this "Golden Blueprint" as the exact template for your **Actionable Recommendations** and your **Automated CI/CD Enforcement Script** to enforce its systemic adoption across all core repositories.
3.  **Full, Agnostic Event-Stream Auditing:**
    *   When auditing event streams, do not satisfy yourself with pointing to a single queue example. You must audit the *full, agnostic implementation of event streams* across their entire data lifecycle: ingestion buffers, deterministic execution ordering, retry/backpressure enforcements, schema validation at every consumer hop, and state synchronization.

## INTRODUCTORY & BODILY EXTRACTS (DIRECT PAPER READS)
```text
[PAGES 1]   
The General Theory of Trust: A Multiscale Formalism
for Expectation–Validation Dynamics
Devon Shigaki (ORCID: 0009-0003-8977-8010)
The FreshCredit Lab — Independent Research
Correspondence: freshcredit.xyz
Version 3.0 — September 2026
Abstract
Trust is typically treated as an attitude, a disposition, or a social lubricant — constructs that resist
measurement and resist unification across scales. This paper proposes the General Theory of Trust
(GTT): trust is a measurable dynamical quantity defined by the relationship between an agent’s expec-
tations (E) and the validations (V ) returned by its environment. We formalize trust as a five-component
vectorT = [D, O, C, A, S] (Expectation, Validation, Recognition, Activation, Stabilization), whose sta-
bilization component obeys the update law ∆St =λ(Vt−Et) — the Rescorla – Wagner delta rule applied
to trust as the state variable — with the alignment state E =V a Lyapunov-stable line of fixed points
whose transverse contraction rate is the corrective bandwidth µ (phase volumes contract uniformly at
rate µ; no divergence-based critical point is posited). A Mathematical Development section assembles
the theory’s machinery from established formalisms: the Fisher – Rao information metric on the sim-
plex of normalized stage-activity profiles (textbook result, derived for self-containment [STANDARD]);
Lyapunov stability of the alignment fixed point [STANDARD]; a stochastic Langevin/Fokker – Planck
extension with closed-form stationary distribution [STANDARD]; an observation-dependent dissipa-
tion model in which measurement salience modulates the mismatch dynamics, with stationary variance
σ2/(2(µ +ψ)) and a dissipation-floor slowing signature verified in silico [DERIVED + SIMULATION];
and a Kuramoto-type collective phase transition stated as a formal analogy whose critical coupling is a
mean-field reference value, not a numerical prediction (labeled [ANALOGY]). A measurement-theoretic
analysis separates four distinct senses 

   [PAGES 2]   
Contents
1. Introduction
2. Definitions and the Core Model
3. Mathematical Development
4. Multiscale Evidence Review
5. The Manifestation Chain
6. Proposed Empirical Program
7. Discussion
8. Conclusion
1. Introduction
Every cooperative act, market transaction, and
communicative exchange presupposes a quantity
that the social sciences name trust yet measure in-
consistently. Survey instruments measure stated
attitudes; economic games measure revealed behav-
ior; neuroimaging measures correlates of social eval-
uation. These instruments rarely agree, and the
disagreement is principled rather than accidental:
a recent meta-analysis of the behavioral-genetic lit-
erature finds that roughly 33% of the variance in
stated general trust is attributable to additive ge-
netic factors, but only 7 – 17% of the variance in
behavioral trust (Kettlewell & Tymula, 2024). The
two instruments are not measuring the same thing
at different resolutions; they are measuring differ-
ent projections of a latent process.
The General Theory of Trust (GTT) begins from
a definitional move intended to make that process
measurable: trust is not an attitude but a rela-
tion between expectation and validation, and ev-
ery observation of trust is a measurement. An
agent holds a predictive model of some counter-
party, institution, or environment (expectation, E);
the world returns outcomes (validation, V); the
signed mismatch ε =V −E (positive when reality
confirms or exceeds expectation) drives an update
to the agent’s stabilization state (S). This framing
is deliberately close to predictive-processing neuro-
science, in which perception is a recursive filter min-
imizing precision-weighted prediction error (Fris-
ton, 2010), and it is complementary to epistemic-
vigilance theories of communication, which model
hearers as equipped with dedicated mechanisms
of calibrated distrust — screening communicated
content for plausibility and sources for reliabil-
ity before acc
```

---

## AUDIT ROADMAP (PHASE-BY-PHASE)

### Phase 1: Formal Expectations ($E$)
*   *Audit Focus:* Identify where the codebase generates, models, or caches expectations ($E_t$) about the state, environment, or transactions. Are expectations explicitly typed, or are they fuzzy/implicit?

### Phase 2: Measured Validations ($V$)
*   *Audit Focus:* Review the validation mechanisms ($V_t$). Are sensory real-world inputs parsed, validated, and normalized cleanly before being compared to expectations?

### Phase 3: Expectation-Validation Errors ($\epsilon_t = V_t - E_t$)
*   *Audit Focus:* Auditing error calculation and logging. Does the system compute the validation-minus-expectation mismatch ($\epsilon_t$)? Are positive errors (confirmations) treated differently than negative errors?

### Phase 4: The Stability Law ($\Delta S_t = \lambda \epsilon_t$)
*   *Audit Focus:* How does the system update its stability or trust metrics over time based on learning rates ($\lambda$) and mismatches? Is state decay configured safely?

### Phase 5: Epistemic Status Tracking
*   *Audit Focus:* Auditing metadata tags. Are state parameters and equations labeled with their epistemic status: Derived, Standard, Analogy, Homology, Empirical, Hypothesized, Simulated?

---

## MANDATORY DELIVERABLE: CRITICAL PLAN & AUTOMATED CI/CD ENFORCEMENT
Perform your codebase discovery through the lens of this work and map how we worked to build on this knowledge. Once complete, you must compile and deliver:

1. **Executive Summary & Lens Review:** An evaluation of the codebase and application specifically through the lens of this work, detailing how we built on this knowledge to benefit the application and codebase.
2. **Phase Audit Matrix:** A table mapping *Phase* | *Citations/Files* | *Identified Gaps, Risks, or Overlaps* | *Priority (H/M/L)*.
3. **Critical Adjustment Plan:** If any critical gaps or boundary overlaps are identified, form a full, actionable plan to adjust and partition roles and responsibilities.
4. **Automated CI/CD Enforcement Script:** Write a fully functional automated validation script (e.g., Python validation check, Bash Git-hook, custom linter rule, or unit test assertions) that can be integrated directly into our current build scripts and CI/CD deployment pipelines to continuously maintain and enforce these rules!
