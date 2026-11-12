# Architectural Audit Prompt: Gang-of-Four (GoF) Design Patterns

Copy and paste this codebase-agnostic prompt into your developer agent's session to conduct a systematic, phase-by-phase structural design and reusability pattern audit.

---

## CONTEXT & OBJECTIVE
You are an expert Software Design Pattern Analyst. Your task is to audit our codebase against the classic object-oriented and structural patterns established in the Gang-of-Four's *Design Patterns*.

Your goal is to inspect the codebase and answer these three questions for each phase:
1. **Current Coverage:** What are we doing in our codebase to cover this Phase? (Cite specific directories, files, or packages).
2. **Gaps & Overlaps:** Are there gaps (e.g., tight coupling, hardcoded instantiations, bloated conditionals, fragile structures) or overlaps (e.g., duplicate factories, competing wrappers, redundant structural adapters)?
3. **Actionable Recommendations:** How should we complete this Phase to make our code decoupled, supple, and highly reusable?

---

---

## MANDATORY PRE-REQUISITE: READ THE ENTIRE BOOK
Before starting your codebase audit, you MUST read and analyze the full text of the sibling book PDF file to obtain complete context, grasp the subtle mathematical/neuromodulatory/logical models, and preserve absolute architectural fidelity.

*   **Target PDF File Name:** `Design Patterns.pdf`
*   **Target PDF Path:** `/Users/devonshigaki/Desktop/Books/Design Patterns.pdf`

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

### Phase 1: Introduction (Pattern Foundations)
*   *Audit Focus:* Review core design principles: program to an interface (not an implementation), favor object composition over class inheritance, and encapsulate variation.
*   *Key Questions:* Does our codebase heavily rely on rigid class hierarchies or concrete types? Are there abstractions/interfaces decoupling our systems?

### Phase 2: Case Study: Designing a Document Editor
*   *Audit Focus:* Walk through a complex structural component to see how multiple patterns compose (e.g., Composite for hierarchy, Decorator for styling, Strategy for layouts, Command for history).
*   *Key Questions:* How do our primary services or modules compose? Are we solving complex structural problems by stacking single-purpose patterns cleanly?

### Phase 3: Creational Patterns
*   *Audit Focus:* Inspect object instantiation patterns: Abstract Factory (families of objects), Builder (step-by-step creation), Factory Method (subclass-based initialization), Prototype (cloning), and Singleton (sole instance).
*   *Key Questions:* Are constructors called directly everywhere (making mocking or swapping implementations difficult)? Are singletons implemented safely in multi-threaded environments? Do we leverage Builders for complex object construction?

### Phase 4: Structural Patterns
*   *Audit Focus:* Inspect class and object composition: Adapter (interface translation), Bridge (separating abstraction from implementation), Composite (recursive trees), Decorator (dynamic responsibilities), Facade (simplified sub-systems), Flyweight (shared fine-grained state), and Proxy (surrogate control).
*   *Key Questions:* Do we use Facades to simplify access to complex sub-systems? Are we using Adapters to wrap third-party libraries? Is there any recursive structure that could benefit from the Composite pattern?

### Phase 5: Behavioral Patterns
*   *Audit Focus:* Inspect communication, control flows, and state management: Chain of Responsibility (request piping), Command (encapsulating actions), Interpreter (language grammar), Iterator (sequential access), Mediator (coordination routing), Memento (state capture), Observer (event publishing), State (state-driven behavior), Strategy (swappable algorithms), Template Method (subclass skeleton), and Visitor (dynamic double-dispatch operations).
*   *Key Questions:* Are there massive, unmaintainable switch/if-else blocks managing state transitions (that could be refactored into the State pattern) or algorithmic variants (that could use the Strategy pattern)? Are we decoupling event listeners using the Observer pattern?

### Phase 6: Conclusion (Evolutionary Reusability)
*   *Audit Focus:* Evaluate design pattern synergy, maintenance overhead, and long-term codebase evolvability.
*   *Key Questions:* Are we over-engineering our codebase with "design pattern soup", or are we applying patterns surgically to solve real, documented maintenance pain points?

---

## MANDATORY DELIVERABLE: CRITICAL PLAN & AUTOMATED CI/CD ENFORCEMENT
Perform your codebase discovery through the lens of this work and map how we worked to build on this knowledge. Once complete, you must compile and deliver:

1. **Executive Summary & Lens Review:** An evaluation of the codebase and application specifically through the lens of this work, detailing how we built on this knowledge to benefit the application and codebase.
2. **Phase Audit Matrix:** A table mapping *Phase* | *Citations/Files* | *Identified Gaps, Risks, or Overlaps* | *Priority (H/M/L)*.
3. **Critical Adjustment Plan:** If any critical gaps or boundary overlaps are identified, form a full, actionable plan to adjust and partition roles and responsibilities.
4. **Automated CI/CD Enforcement Script:** Write a fully functional automated validation script (e.g., Python validation check, Bash Git-hook, custom linter rule, or unit test assertions) that can be integrated directly into our current build scripts and CI/CD deployment pipelines to continuously maintain and enforce these rules!
