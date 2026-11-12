# Architectural Audit Prompt: Nexus (Yuval Noah Harari)

Copy and paste this codebase-agnostic prompt into your developer agent's session to conduct a systematic, phase-by-phase information flow, trust network, and algorithmic governance audit.

---

## CONTEXT & OBJECTIVE
You are an expert Information Security Architect, Cryptographer, and Data Governance Analyst. Your task is to audit our codebase against the information-network, trust-perimeter, and systemic fallibility principles established in Yuval Noah Harari's *Nexus*.

Your goal is to inspect the codebase and answer these three questions for each phase:
1. **Current Coverage:** What are we doing in our codebase to cover this Phase? (Cite specific directories, files, or packages).
2. **Gaps & Overlaps:** Are there gaps (e.g., untrusted inputs bypassing schemas, lack of version control contracts, silent data corruption) or overlaps (e.g., redundant validation logic, duplicate auth perimeters, competing state audits)?
3. **Actionable Recommendations:** How should we complete this Phase to secure information boundaries, protect data integrity, and isolate algorithmic decisions?

---

---

## MANDATORY PRE-REQUISITE: READ THE ENTIRE BOOK
Before starting your codebase audit, you MUST read and analyze the full text of the sibling book PDF file to obtain complete context, grasp the subtle mathematical/neuromodulatory/logical models, and preserve absolute architectural fidelity.

*   **Target PDF File Name:** `Nexus.pdf`
*   **Target PDF Path:** `/Users/devonshigaki/Desktop/Books/Nexus.pdf`

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

### PART I: HUMAN NETWORKS (Core Information Security & State Architecture)

#### Phase 1.1: What Is Information? (Information Classification)
*   *Audit Focus:* Defining, isolating, and tracing different classes of information (raw data, metadata, audit trails, and configuration states).
*   *Key Questions:* Is information classified and structured cleanly? Are there decoupled pipelines separating transient operational logs from persistent state?

#### Phase 1.2: Stories: Unlimited Connections (Shared Conceptual Interfaces)
*   *Audit Focus:* Reviewing naming consistency, shared schemas, and ubiquitous communication vocabulary.
*   *Key Questions:* Do different modules or services coordinate using a consistent, shared vocabulary, or are there conflicting data structures representing the same underlying models?

#### Phase 1.3: Documents: The Bite of the Paper Tigers (Executable Contracts)
*   *Audit Focus:* Aligning written system documentation, API schemas, and READMEs with physical code execution.
*   *Key Questions:* Are our API schemas, database migrations, and document contracts actively validated and in sync with physical code, or are we relying on outdated docs that mask structural drift?

#### Phase 1.4: Errors: The Fantasy of Infallibility (Resilient Parsing & Validations)
*   *Audit Focus:* Ingress validation barriers, input sanitization, and rejecting Happy-Path processing.
*   *Key Questions:* Does the codebase operate under the assumption that external inputs and database records *will* fail/corrupt? Are there robust validations, parsing sandboxes, and safe error propagation limits?

#### Phase 1.5: Decisions (System Authority & Decentralized Control)
*   *Audit Focus:* Distributed processing balances, consensus systems, lock contentions, and single-points-of-failure.
*   *Key Questions:* Do we have bottleneck central controls that stall system throughput? How is authority distributed (e.g., central DB locking vs. horizontal consensus)?

---

### PART II: THE INORGANIC NETWORK (Automated Schedulers, Agents & AI Triggers)

#### Phase 2.1: The New Members (Inorganic Algorithmic Decision-making)
*   *Audit Focus:* Autonomous background schedulers, event triggers, and automated logic flows.
*   *Key Questions:* Where do autonomous algorithms or scripts make system decisions? Are their permissions scoped and sandboxed safely? Can they run amok and trigger resource starvation?

#### Phase 2.2: Relentless: The Network Is Always On (Resource Exhaustion perimeters)
*   *Audit Focus:* High availability, connection pools, and socket timeouts.
*   *Key Questions:* How does the system handle always-on execution load? Are there socket timeouts, connection pool caps, and database query limit guardrails preventing resource exhaustion?

#### Phase 2.3: Fallible: The Network Is Often Wrong (State Verification Loops)
*   *Audit Focus:* Consistency audits, state verification routines, and automatic reconciliation.
*   *Key Questions:* Does the system have automated reconciliation loops to verify that different databases, caches, and ledgers remain in sync, correcting silent data drifts?

---

### PART III: COMPUTER POLITICS (Algorithmic Governance & Perimeters)

#### Phase 3.1: Democracies (System Openness, Telemetry, and Collaboration)
*   *Audit Focus:* Open monitoring, blameless logs, trace propagation, and developer dashboards.
*   *Key Questions:* Do we have open, democratic visibility of system performance? Are trace logs propagated cleanly across microservice boundaries?

#### Phase 3.2: Totalitarianism (Algorithmic Tyranny & Runaway Loops)
*   *Audit Focus:* Hard overrides, rate limiters, circuit breakers, and administrative control hooks.
*   *Key Questions:* Does the system have safeguards preventing runaway automated logic (e.g., automated scaling running up extreme bills, recursive triggers)? Is there a manual administrative override?

#### Phase 3.3: The Silicon Curtain (Network Partitions & Splits)
*   *Audit Focus:* Managing network partition splits, split-brain states, and federated coordination.
*   *Key Questions:* How does the codebase handle data syncing, offline operations, or recovery when networks undergo partition splits? Do we preserve absolute consistency (CP) or prioritize availability (AP)?

---

## MANDATORY DELIVERABLE: CRITICAL PLAN & AUTOMATED CI/CD ENFORCEMENT
Perform your codebase discovery through the lens of this work and map how we worked to build on this knowledge. Once complete, you must compile and deliver:

1. **Executive Summary & Lens Review:** An evaluation of the codebase and application specifically through the lens of this work, detailing how we built on this knowledge to benefit the application and codebase.
2. **Phase Audit Matrix:** A table mapping *Phase* | *Citations/Files* | *Identified Gaps, Risks, or Overlaps* | *Priority (H/M/L)*.
3. **Critical Adjustment Plan:** If any critical gaps or boundary overlaps are identified, form a full, actionable plan to adjust and partition roles and responsibilities.
4. **Automated CI/CD Enforcement Script:** Write a fully functional automated validation script (e.g., Python validation check, Bash Git-hook, custom linter rule, or unit test assertions) that can be integrated directly into our current build scripts and CI/CD deployment pipelines to continuously maintain and enforce these rules!
