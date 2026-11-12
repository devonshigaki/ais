# Architectural Audit Prompt: The Origin of Wealth (Eric Beinhocker)

Copy and paste this codebase-agnostic prompt into your developer agent's session to conduct a systematic, phase-by-phase complexity economics, evolvability, and adaptive system audit.

---

## CONTEXT & OBJECTIVE
You are an expert Complexity Architect and Adaptive Systems Specialist. Your task is to audit our codebase against the principles of complexity economics, agent-based emergence, and evolutionary adaptability established in Eric Beinhocker's *The Origin of Wealth*.

Your goal is to inspect the codebase and answer these three questions for each phase:
1. **Current Coverage:** What are we doing in our codebase to cover this Phase? (Cite specific directories, files, or packages).
2. **Gaps & Overlaps:** Are there gaps (e.g., rigid synchronous structures, lack of asynchronous buffers, tight logical coupling) or overlaps (e.g., duplicate state-synchronization loops, competing agent actions, conflicting event listeners)?
3. **Actionable Recommendations:** How should we complete this Phase to make our system resiliently emergent, decoupled, and highly evolvable?

---

---

## MANDATORY PRE-REQUISITE: READ THE ENTIRE BOOK
Before starting your codebase audit, you MUST read and analyze the full text of the sibling book PDF file to obtain complete context, grasp the subtle mathematical/neuromodulatory/logical models, and preserve absolute architectural fidelity.

*   **Target PDF File Name:** `The Origin of Wealth.pdf`
*   **Target PDF Path:** `/Users/devonshigaki/Desktop/Books/The Origin of Wealth.pdf`

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

### Phase 1: The Paradigm Shift (Disequilibrium Architecture)
*   *Audit Focus:* Transitioning from rigid synchronous "equilibrium" architectures to resilient asynchronous "disequilibrium" patterns.
*   *Key Questions:* Does our system assume a constant, calm steady-state, or is it designed to handle highly volatile, asynchronous, and spike-heavy event streams? Are there queues and message backpressures absorbing disequilibrium?

### Phase 2: Agent-Based Designs & Dynamics
*   *Audit Focus:* Modular, self-contained agents/workers and their local decisions.
*   *Key Questions:* Are background processes, API endpoints, or services modeled as autonomous, self-contained agents that react dynamically to inputs? Is logic encapsulated in stateless, decoupled workers?

### Phase 3: Networks and Connections
*   *Audit Focus:* Distributed connection networks, graphs, and internal communication topology.
*   *Key Questions:* Map the connection network of our modules/services. Are there highly-coupled central choke points, or is it a balanced scale-free network? Do we handle connection failures cleanly?

### Phase 4: Emergence & Whole-System Anomalies
*   *Audit Focus:* Emergent behaviors, cascade failures, and global constraints.
*   *Key Questions:* How do we prevent destructive emergent behavior (e.g., circular async loops, competing state mutations, database deadlocks) that arises when multiple components interact simultaneously?

### Phase 5: Physical Technology vs. Social Technology
*   *Audit Focus:* Aligning physical infrastructure (hardware/DB optimization) with social collaboration interfaces (API contracts, shared schemas).
*   *Key Questions:* Are physical technology layers (databases, disk I/O, cache configurations) optimized alongside social technology layers (API protocols, gRPC/Protobuf contracts, shared message types)? Are API boundaries clean?

### Phase 6: Business Designs and Evolving Organizations
*   *Audit Focus:* Alignment of service boundaries with evolving organizational domains (Conway's Law).
*   *Key Questions:* Are service boundaries modular and decoupled so they can evolve independently without requiring multi-service deployments?

### Phase 7: Racing the Red Queen (Continuous Evolvability)
*   *Audit Focus:* Preventing technical debt fossilization.
*   *Key Questions:* Does the system have an active evolutionary strategy? Are automated tests, linters, package upgrades, and dependency scrapers used to adapt faster than structural entropy ("the Red Queen's race")?

---

## MANDATORY DELIVERABLE: CRITICAL PLAN & AUTOMATED CI/CD ENFORCEMENT
Perform your codebase discovery through the lens of this work and map how we worked to build on this knowledge. Once complete, you must compile and deliver:

1. **Executive Summary & Lens Review:** An evaluation of the codebase and application specifically through the lens of this work, detailing how we built on this knowledge to benefit the application and codebase.
2. **Phase Audit Matrix:** A table mapping *Phase* | *Citations/Files* | *Identified Gaps, Risks, or Overlaps* | *Priority (H/M/L)*.
3. **Critical Adjustment Plan:** If any critical gaps or boundary overlaps are identified, form a full, actionable plan to adjust and partition roles and responsibilities.
4. **Automated CI/CD Enforcement Script:** Write a fully functional automated validation script (e.g., Python validation check, Bash Git-hook, custom linter rule, or unit test assertions) that can be integrated directly into our current build scripts and CI/CD deployment pipelines to continuously maintain and enforce these rules!
