# Architectural Audit Prompt: The DOCAS Framework

Copy and paste this codebase-agnostic prompt into your developer agent's session to conduct a systematic, phase-by-phase multi-timescale neuromodulatory and asynchronous execution audit.

---

## CONTEXT & OBJECTIVE
You are an expert Parallel Execution Architect, Neuromorphic Systems Engineer, and Asynchronous Performance Specialist. Your task is to audit our codebase against the computational model of trust dynamics established in Devon Shigaki's *The DOCAS Framework*.

Your goal is to inspect the codebase and answer these three questions for each phase:
1. **Current Coverage:** What are we doing in our codebase to cover this Phase? (Cite specific directories, files, or packages).
2. **Gaps & Overlaps:** Are there gaps (e.g., synchronous thread blockings, lack of timescale separation, raw uncalibrated feedback loops) or overlaps (e.g., competing lock boundaries, duplicate event dispatchers)?
3. **Actionable Recommendations:** How should we complete this Phase to align our execution with parallel neuromodulatory timescales?

---

---

## MANDATORY PRE-REQUISITE: READ THE ENTIRE PAPER
Before starting your codebase audit, you MUST read and analyze the full text of the sibling paper PDF file to obtain complete context, grasp the subtle mathematical/neuromodulatory/logical models, and preserve absolute architectural fidelity.

*   **Target PDF File Name:** `docas.pdf`
*   **Target PDF Path:** `/Users/devonshigaki/Desktop/Kimi_Agent_Orchestrated%20Paper%20Review6/osf_upload_v2/papers/pdf/docas.pdf`

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
The DOCAS Framework: A Computational Model of Trust Dynamics
as a Parallel Multi-Timescale Neuromodulatory Architecture
Devon Shigaki (ORCID: 0009-0003-8977-8010)
The FreshCredit Lab — Independent Research
Correspondence: freshcredit.xyz
Version 3.0 — September 2026
Abstract
The companion theory preprint defines trust as the measurable relation between expectation (E) and
validation (V), governed by the stability law ∆St = λ(Vt − Et) = λεt, with the stack-wide convention
εt = Vt − Et (validation-minus-expectation; positive ε is a confirmation). This paper specifies the the-
ory’s biological instantiation. Because the five neuromodulatory systems involved operate on timescales
spanning more than four orders of magnitude (0.1 s to ∼2,700 s; up to five if serotonergic tone is
included) — phasic dopamine bursts of ∼100–400 ms versus the cortisol awakening response peaking
30–45 minutes after awakening — no serial five-step cascade is physiologically coherent. DOCAS is
therefore framed here as a parallel, multi-timescale modulation architecture : five neuromodula-
tory channels — Dopamine, Oxytocin, Cortisol, Adrenergic/noradrenergic arousal, Serotonin — each
carrying, predominantly, one of five computational functions that together constitute one trust measure-
ment: Expectation, Validation, Recognition, Activation, Stabilization. The word “stagé́ is retained as
the name of a functional role, not a serial step; the five channels run concurrently and interact through
a closed loop in which the stabilized model becomes the next expectation. Each channel is defined
functionally, with explicit justification against common mislabelings (dopamine ̸= “reward́́; oxytocin ̸=
“trust moleculé́ or “bonding hormoné́; cortisol ̸= generic “stresś́; serotonin ̸= “status/contentment́́). We
formalize channel interactions through τ = -EFE, the negative of expected free energy as introduced
in active inference (Friston et al., 2015, 2017), with valence = sign(∆τ ), arousal = |∆τ |, dominan

   [PAGES 2]   
Contents
1. Introduction
2. The Five Carriers: Functional Definitions and Justi-
fications
3. Formalization: τ and the Affective Readout
4. Genetic Architecture of Trust-Sensitivity
5. The DOCAS Experiment (Pre-registered Design)
6. Discussion
7. Conclusion
1 Introduction
The companion theory preprint provides the formal
object — the trust vector T = [D, O, C, A, S] and
its stability law — but is silent about how a biologi-
cal observer computes it. That silence is by design:
the theory claims scale-invariance, and the biolog-
ical level is one coordinate system among several.
This paper supplies the biological coordinates.
Its central claim, revised under the timescale
constraint that forced this revision, is the follow-
ing. One trust measurement decomposes into five
computational functions — expectation, validation,
recognition, activation, stabilization — and each
function is carried predominantly (not exclusively)
by a distinct neuromodulatory system. These five
systems do not execute a serial sequence. Their
characteristic dynamics span ∼100–400 ms (phasic
dopamine, phasic locus-coeruleus norepinephrine),
seconds (serotonergic patience signals; peripheral
sympathetic drive), seconds-to-minutes (oxytocin
release tracking social events; tonic LC modes),
and tens of minutes (HPA-axis cortisol responses;
the cortisol awakening response) — a range of more
than four orders of magnitude (0.1 s to ∼2,700 s;
up to five if serotonergic tone is included) within
which a five-step serial cascade completing “one
measurement́́ in biological real time is physiolog-
ically incoherent (see Table 1, §2.8). What the
physiology supports is parallel, multi-timescale
modulation: five channels acting concurrently on
the same expectation–validation loop, each at its
own temporal grain, coupled through the loop’s
closure (the stabilized model becomes the next ex-
pectation). The framework is named DOCAS after
the five carriers; “stagé́ is retained through
```

---

## AUDIT ROADMAP (PHASE-BY-PHASE)

### Phase 1: Short-Term Timescale (Dopamine - Prediction Errors)
*   *Audit Focus:* Reviewing microsecond-to-second execution paths. Are prediction errors computed, processed, and updated instantly within lightweight event/reactor loops?

### Phase 2: Medium-Term Volatility recalibrations (Norepinephrine - Surprise)
*   *Audit Focus:* Reviewing mid-term recalibrations. How does the system handle sudden environmental volatility or unexpected payload surges? Are connection limits and retry decay rates recalculated dynamically?

### Phase 3: Long-Term Baseline Stability (Serotonin - Risk Aversion)
*   *Audit Focus:* Auditing baseline weights, deep state stores, and conservative risk-aversion margins. Are long-term metrics isolated and protected from short-term oscillations?

### Phase 4: Uncertainty & Precision Filtering (Acetylcholine - SNR)
*   *Audit Focus:* Analyzing signal-to-noise ratio (SNR) filters. Does the system filter out network noise, database flakiness, or duplicate events to maintain state precision?

---

## MANDATORY DELIVERABLE: CRITICAL PLAN & AUTOMATED CI/CD ENFORCEMENT
Perform your codebase discovery through the lens of this work and map how we worked to build on this knowledge. Once complete, you must compile and deliver:

1. **Executive Summary & Lens Review:** An evaluation of the codebase and application specifically through the lens of this work, detailing how we built on this knowledge to benefit the application and codebase.
2. **Phase Audit Matrix:** A table mapping *Phase* | *Citations/Files* | *Identified Gaps, Risks, or Overlaps* | *Priority (H/M/L)*.
3. **Critical Adjustment Plan:** If any critical gaps or boundary overlaps are identified, form a full, actionable plan to adjust and partition roles and responsibilities.
4. **Automated CI/CD Enforcement Script:** Write a fully functional automated validation script (e.g., Python validation check, Bash Git-hook, custom linter rule, or unit test assertions) that can be integrated directly into our current build scripts and CI/CD deployment pipelines to continuously maintain and enforce these rules!
