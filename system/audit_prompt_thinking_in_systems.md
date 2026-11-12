# Architectural Audit Prompt: Thinking in Systems (Donella Meadows)

Copy and paste this codebase-agnostic prompt into your developer agent's session to conduct a systematic, phase-by-phase system dynamics, stocks and flows, and feedback loop audit.

---

## CONTEXT & OBJECTIVE
You are an expert Systems Dynamics Engineer and Control Loop Specialist. Your task is to audit our codebase against the principles of system structures, feedback loops, stocks and flows, and leverage-point interventions established in Donella Meadows' *Thinking in Systems*.

Your goal is to inspect the codebase and answer these three questions for each phase:
1. **Current Coverage:** What are we doing in our codebase to cover this Phase? (Cite specific directories, files, or packages).
2. **Gaps & Overlaps:** Are there gaps (e.g., unthrottled loop ingestion, lack of backpressure, positive feedback cascades, delay traps) or overlaps (e.g., redundant caching, competing rate-limiters, conflicting autoscaling triggers)?
3. **Actionable Recommendations:** How should we complete this Phase to control data dynamics, leverage high-impact optimization nodes, and prevent system traps?

---

---

## MANDATORY PRE-REQUISITE: READ THE ENTIRE BOOK
Before starting your codebase audit, you MUST read and analyze the full text of the sibling book PDF file to obtain complete context, grasp the subtle mathematical/neuromodulatory/logical models, and preserve absolute architectural fidelity.

*   **Target PDF File Name:** `Thinking in Systems.pdf`
*   **Target PDF Path:** `/Users/devonshigaki/Desktop/Books/Thinking in Systems.pdf`

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

### Phase 1: Stocks and Flows (Resource Capacities & Rates)
*   *Audit Focus:* Auditing buffers, in-memory caches, message queues, memory capacities (Stocks), and data ingestion, extraction, processing rates (Flows).
*   *Key Questions:* Are we mapping out our system's memory and queue "stocks" cleanly? Are incoming flow rates balanced with processing and egress flow rates?

### Phase 2: Feedback Loops (Control Systems & Stabilization)
*   *Audit Focus:* Balancing loops (rate limiters, retry decay limits, autoscaling, backpressure) vs. Destructive positive loops (cascading retries, cache stampedes, retry storms).
*   *Key Questions:* Are we using exponential backoff with jitter on retries to prevent retry storms? Do backpressure signals dynamically slow down incoming traffic flows?

### Phase 3: System Resilience, Self-Organization, and Hierarchy
*   *Audit Focus:* Component self-healing, decoupled fallback states, and subsystem separation boundaries.
*   *Key Questions:* Can subsystems operate independently if parent coordinators fail? Are our service hierarchies clearly defined, or are there messy circular dependency loops?

### Phase 4: Delays and Non-Linearity
*   *Audit Focus:* Network delay pools, queuing lag, database disk commit pauses, and sudden non-linear threshold collapses.
*   *Key Questions:* How do we handle system delays? Are there queuing delays where queues grow exponentially because ingestion exceeds service limits by a fraction?

### Phase 5: System Traps (Identifying Archetypes)
*   *Audit Focus:* Auditing for classic traps: Tragedy of the Commons (shared connection pool starvation), Policy Resistance (retries bypassing throttling limits), and Escalation (competing thread resource wars).
*   *Key Questions:* Identify where system modules compete for limited shared states, causing deadlock escalation or resource starvation.

### Phase 6: Leverage Points (High-Impact Interventions)
*   *Audit Focus:* Finding the 10% of code adjustments that yield 90% of system performance gains.
*   *Key Questions:* What are the leverage points in our codebase? (e.g., index optimizations, database query tuning, switching from synchronous polling to event-driven push, memory-allocation caching).

### Phase 7: Living in a World of Systems
*   *Audit Focus:* Simplicity, monitoring, and adapting to system-wide complexity.
*   *Key Questions:* Do we design for the unpredictable whole system? Are we writing clear, robust code, logging feedback loops clearly, and avoiding rigid over-engineering?

---

## MANDATORY DELIVERABLE: CRITICAL PLAN & AUTOMATED CI/CD ENFORCEMENT
Perform your codebase discovery through the lens of this work and map how we worked to build on this knowledge. Once complete, you must compile and deliver:

1. **Executive Summary & Lens Review:** An evaluation of the codebase and application specifically through the lens of this work, detailing how we built on this knowledge to benefit the application and codebase.
2. **Phase Audit Matrix:** A table mapping *Phase* | *Citations/Files* | *Identified Gaps, Risks, or Overlaps* | *Priority (H/M/L)*.
3. **Critical Adjustment Plan:** If any critical gaps or boundary overlaps are identified, form a full, actionable plan to adjust and partition roles and responsibilities.
4. **Automated CI/CD Enforcement Script:** Write a fully functional automated validation script (e.g., Python validation check, Bash Git-hook, custom linter rule, or unit test assertions) that can be integrated directly into our current build scripts and CI/CD deployment pipelines to continuously maintain and enforce these rules!
