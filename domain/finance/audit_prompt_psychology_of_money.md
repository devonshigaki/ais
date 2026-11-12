# Architectural Audit Prompt: The Psychology of Money (Morgan Housel)

Copy and paste this codebase-agnostic prompt into your developer agent's session to conduct a systematic, phase-by-phase resource preservation, risk management, and system resilience audit.

---

## CONTEXT & OBJECTIVE
You are an expert Performance Optimizer, Risk Mitigation Engineer, and Distributed Systems Analyst. Your task is to audit our codebase against the lessons of risk stewardship, resource preservation, compounding growth, and margin-of-safety principles established in Morgan Housel's *The Psychology of Money*.

Your goal is to inspect the codebase and answer these three questions for each phase:
1. **Current Coverage:** What are we doing in our codebase to cover this Phase? (Cite specific directories, files, or packages).
2. **Gaps & Overlaps:** Are there gaps (e.g., nested dynamic allocations, O(N^2) loops, lack of margin-of-safety buffers, unprotected thread pools) or overlaps (e.g., redundant caches, duplicate backup procedures, competing lock boundaries)?
3. **Actionable Recommendations:** How should we complete this Phase to safeguard computing resources, optimize database queries, and design robust error recovery?

---

---

## MANDATORY PRE-REQUISITE: READ THE ENTIRE BOOK
Before starting your codebase audit, you MUST read and analyze the full text of the sibling book PDF file to obtain complete context, grasp the subtle mathematical/neuromodulatory/logical models, and preserve absolute architectural fidelity.

*   **Target PDF File Name:** `The Psychology of Money.pdf`
*   **Target PDF Path:** `/Users/devonshigaki/Desktop/Books/The Psychology of Money.pdf`

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

### Phase 1: Luck & Risk (Handling Unpredictability)
*   *Audit Focus:* Isolating the system from unpredictable environments: packet delays, garbage collection pauses, third-party API outages, and hardware crashes.
*   *Key Questions:* Does our system acknowledge runtime "bad luck"? Are timeouts, retries, fallback states, and data replicas designed to survive unexpected physical outages?

### Phase 2: Confounding Compounding (Algorithmic & Resource Complexity)
*   *Audit Focus:* Pruning O(N^2) complexity, recursive resource allocations, memory leaks, and compounding loop latencies.
*   *Key Questions:* Are we executing heavy database queries inside loops? Is there any cumulative latency, thread leakage, or memory drift that slowly poisons system throughput over time?

### Phase 3: Getting Wealthy vs. Staying Wealthy (Performant vs. Survivor Systems)
*   *Audit Focus:* Writing code that is not just fast (performant) but resilient and survival-focused under extreme failures.
*   *Key Questions:* Does our codebase focus heavily on the happy-path performance, while neglecting critical recovery scenarios, transaction rollbacks, or system-state restoration?

### Phase 4: Tails, You Win (Auditing Tail-End Percentiles)
*   *Audit Focus:* Tail-end performance metrics: p99 and p99.9 latencies, handling extreme edge cases, and massive payload exceptions.
*   *Key Questions:* How does the system handle the absolute worst 1% or 0.1% of scenarios? Do outliers trigger cascading crashes or block core queues?

### Phase 5: Room for Error (Enforcing Margins of Safety)
*   *Audit Focus:* Buffer capacities, thread queue safety ceilings, DB connection pool reserves, and backup states.
*   *Key Questions:* Does our codebase leave an adequate "margin of safety" under max load? Are queue limits fixed to prevent out-of-memory errors? Is there a safe backup plan for database failures?

### Phase 6: Reasonable > Rational (Pruning Academic Over-Engineering)
*   *Audit Focus:* Swapping fragile, over-engineered "highly optimized" code for simpler, robust, and maintainable pathways.
*   *Key Questions:* Are there overly complex academic algorithms or clever optimizations that are fragile, hard to test, and easy to break, when a simpler, highly readable approach is far more robust?

### Phase 7: Nothing is Free (Evaluating Architectural Trade-offs)
*   *Audit Focus:* Quantifying trade-offs: memory vs. CPU, network calls vs. cache storage, and development speed vs. safety.
*   *Key Questions:* Are we acknowledging the price of our architectural decisions? For example, are we using complex distributed transactions when local causal models are cheaper?

---

## MANDATORY DELIVERABLE: CRITICAL PLAN & AUTOMATED CI/CD ENFORCEMENT
Perform your codebase discovery through the lens of this work and map how we worked to build on this knowledge. Once complete, you must compile and deliver:

1. **Executive Summary & Lens Review:** An evaluation of the codebase and application specifically through the lens of this work, detailing how we built on this knowledge to benefit the application and codebase.
2. **Phase Audit Matrix:** A table mapping *Phase* | *Citations/Files* | *Identified Gaps, Risks, or Overlaps* | *Priority (H/M/L)*.
3. **Critical Adjustment Plan:** If any critical gaps or boundary overlaps are identified, form a full, actionable plan to adjust and partition roles and responsibilities.
4. **Automated CI/CD Enforcement Script:** Write a fully functional automated validation script (e.g., Python validation check, Bash Git-hook, custom linter rule, or unit test assertions) that can be integrated directly into our current build scripts and CI/CD deployment pipelines to continuously maintain and enforce these rules!
