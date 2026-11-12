# Architectural Audit Prompt: How Emotions Are Made (Lisa Feldman Barrett)

Copy and paste this codebase-agnostic prompt into your developer agent's session to conduct a systematic, phase-by-phase constructed system states, prediction control, and allostatic resource allocation audit.

---

## CONTEXT & OBJECTIVE
You are an expert Systems Neuroscientist, Predictive Design Architect, and Performance Optimization Specialist. Your task is to audit our codebase against the principles of constructed states, prediction error resolution, concepts, and allostatic "body budgets" established in Lisa Feldman Barrett's *How Emotions Are Made*.

Your goal is to inspect the codebase and answer these three questions for each phase:
1. **Current Coverage:** What are we doing in our codebase to cover this Phase? (Cite specific directories, files, or packages).
2. **Gaps & Overlaps:** Are there gaps (e.g., unthrottled loop ingestion, lack of predictive cache invalidation, body-budget resource bankruptcy, blind conceptual classification) or overlaps (e.g., redundant caching layers, duplicate validation, conflicting state-recovery mechanisms)?
3. **Actionable Recommendations:** How should we complete this Phase to make our system predictive, optimize memory budgets, and enforce clean conceptual boundaries?

---

---

## MANDATORY PRE-REQUISITE: READ THE ENTIRE BOOK
Before starting your codebase audit, you MUST read and analyze the full text of the sibling book PDF file to obtain complete context, grasp the subtle mathematical/neuromodulatory/logical models, and preserve absolute architectural fidelity.

*   **Target PDF File Name:** `how-emotions-are-made-the-secret-life-of-the-brain.pdf`
*   **Target PDF Path:** `/Users/devonshigaki/Desktop/Books/how-emotions-are-made-the-secret-life-of-the-brain.pdf`

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

## AUDIT ROADMAP (PHASE-BY-PHASE)

### Phase 1: The Search for "Fingerprints" (Dynamic Context-Dependent States)
*   *Audit Focus:* Moving away from rigid, hardcoded static error handlers and isolated trigger routes.
*   *Key Questions:* Do we assume that a specific error code or log always has a single static cause, or is the system designed to assess health based on dynamic, context-dependent execution states?

### Phase 2: System States Are Constructed (On-the-Fly Assembly)
*   *Audit Focus:* How runtime states and operational context are compiled on the fly from past caches and incoming signals.
*   *Key Questions:* Are states assembled dynamically based on histories and context-dependent predictions, or are we carrying massive, redundant, pre-allocated memory blocks that drift?

### Phase 3: The Myth of Universal Signatures (Highly Contextual Thresholds)
*   *Audit Focus:* Custom, dynamic thresholds for alerts, scaling, and timeouts instead of global hardcoded constants.
*   *Key Questions:* Are our timeout and alert limits globally hardcoded (risky under varying network environments)? Do we adjust thresholds based on current operational workloads?

### Phase 4: The Origin of Feeling (Allostasis & Resource "Body Budgets")
*   *Audit Focus:* Monitoring and managing the core resource budget of the system (memory, connection pools, CPU cycles, disk space, and power).
*   *Key Questions:* How does the system allocate and preserve its core resource "body budget"? Are we suffering from slow resource bankruptcy (e.g., memory leaks, socket exhaustion) because we are over-drawing our budget?

### Phase 5: Concepts, Goals, and Enums (Semantic Classification of Telemetry)
*   *Audit Focus:* How raw telemetry data, logs, and byte payloads are classified into logical, actionable concepts and enums.
*   *Key Questions:* Are raw bytes categorized cleanly into cohesive concepts (e.g., semantic logger, structured telemetry)? Do we have fuzzy classification boundaries that trigger false alerts?

### Phase 6: How the System Generates State (The Predictive Loop)
*   *Audit Focus:* The predictive loop: prediction, sensory input, prediction error calculation, and state resolution.
*   *Key Questions:* Does our code use predictive caches or pre-fetched queries? How does it resolve prediction errors when the cache is stale? Is the prediction correction loop optimized and lightweight?

### Phase 7: Inter-Service/Social Reality (Shared System Context)
*   *Audit Focus:* Constructing a shared logical reality across distributed services or concurrent threads from raw byte communications.
*   *Key Questions:* Do service boundaries or nodes maintain a synchronized logical state? Are communication contracts (APIs, events) designed to enforce a single, clean shared reality without contradictions?

### Phase 8: Mastering Your State / Self-Healing (Conscious Runtime Control)
*   *Audit Focus:* Implementing conscious runtime controls, dynamic resource tuning, memory cleanup loops, and self-healing boundaries.
*   *Key Questions:* Can the system actively tune its garbage collection, purge inactive caches, or reallocate memory pools under heavy stress, or is it helpless until a human intervenes?

### Phase 9: System Illness and Failure Cascades (Allostatic Collapse)
*   *Audit Focus:* When resource budgets go bankrupt, leading to systemic thread exhaustion or crash loops.
*   *Key Questions:* What are the systemic pathways when allostasis collapses? Are there circuit breakers, queue shedding, or fast-fail routes preventing a full system collapse?

### Phase 10: Compliance, Logging, and Law (Systemic Ethics)
*   *Audit Focus:* Auditing credentials, complying with data privacy, and keeping secrets out of execution paths.
*   *Key Questions:* Are we protecting user privacy and preventing secret leaks (API keys, credentials) in our execution path? Is there an automated compliance validation gate?

### Phase 11: External System Boundaries (Interpreting Unstructured Inputs)
*   *Audit Focus:* How the system translates and sanitizes raw, unpredictable, or unstructured inputs from external services.
*   *Key Questions:* Do we treat external APIs as trusted inputs, or do we construct a safe interface (Anti-Corruption Layer) to parse, sanitize, and validate them at the boundary?

### Phase 12: From Raw Data to Contextual Mind (High-Level Alert Synthesis)
*   *Audit Focus:* Synthesizing raw metrics and traces into high-level, human-actionable contextual alerts.
*   *Key Questions:* Do our alerts emit raw, cryptic stack traces, or do they synthesize raw data into clear, high-level business context that allows operators to act instantly?

---

## MANDATORY DELIVERABLE: CRITICAL PLAN & AUTOMATED CI/CD ENFORCEMENT
Perform your codebase discovery through the lens of this work and map how we worked to build on this knowledge. Once complete, you must compile and deliver:

1. **Executive Summary & Lens Review:** An evaluation of the codebase and application specifically through the lens of this work, detailing how we built on this knowledge to benefit the application and codebase.
2. **Phase Audit Matrix:** A table mapping *Phase* | *Citations/Files* | *Identified Gaps, Risks, or Overlaps* | *Priority (H/M/L)*.
3. **Critical Adjustment Plan:** If any critical gaps or boundary overlaps are identified, form a full, actionable plan to adjust and partition roles and responsibilities.
4. **Automated CI/CD Enforcement Script:** Write a fully functional automated validation script (e.g., Python validation check, Bash Git-hook, custom linter rule, or unit test assertions) that can be integrated directly into our current build scripts and CI/CD deployment pipelines to continuously maintain and enforce these rules!
