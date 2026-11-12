# Architectural Audit Prompt: User Behavioral Archetypes (14 Personality Profiles)

Copy and paste this codebase-agnostic prompt into your developer agent's session to conduct a systematic, phase-by-phase user behavior classification, frontend persona adaptation, and interface ergonomics audit.

---

## CONTEXT & OBJECTIVE
You are an expert Behavioral UX Architect, User Profiling Engineer, and Interaction Design Specialist. Your task is to audit our frontend user interface, onboarding workflows, and interaction telemetry against the 14 personality archetypes and behavioral dynamics established in the research media in `Desktop/AIS/research/architypes/` (Analysts, Diplomats, Sentinels, Explorers).

This audit drives critical frontend architecture decisions to classify user behavior at runtime, detect user cognitive posture, and adapt the interface so that different psychological archetypes understand, navigate, and receive optimal value from the application.

Your goal is to inspect the codebase and answer these three questions for each phase:
1. **Current Coverage:** What are we doing in our frontend, telemetry, and routing code to cover this Phase? (Cite specific UI components, state stores, telemetry hooks, or route guards).
2. **Gaps & Overlaps:** Are there gaps (e.g., monolithic one-size-fits-all UI, lack of behavioral telemetry, alienation of analytical or sentinel users, confusing navigation) or overlaps (e.g., competing onboarding tours, duplicate persona tags)?
3. **Actionable Recommendations:** How should we complete this Phase to build an intelligent user classification engine and dynamically adapt the UI for diverse user archetypes?

---

## MANDATORY PRE-REQUISITE: READ THE ENTIRE ARCHETYPE MEDIA COLLECTION
Before starting your codebase audit, you MUST read and analyze the research profiles located in the archetypes directory to obtain complete context on each archetype's cognitive traits, communication preferences, problem areas, and behavioral drivers:

*   **Target Directory Path:** `/Users/devonshigaki/Desktop/AIS/research/architypes/`
*   **Key Archetype Profiles (14 PDFs):**
    *   **Analysts (_NT_):** `Architect.pdf` (INTJ), `Logician.pdf` (INTP), `Commander.pdf` (ENTJ), `Debater.pdf` (ENTP)
    *   **Diplomats (_NF_):** `Advocate.pdf` (INFJ), `Mediator.pdf` (INFP), `Protagonist.pdf` (ENFJ), `Campaigner.pdf` (ENFP)
    *   **Sentinels (_S_J):** `Logistician.pdf` (ISTJ), `Defender.pdf` (ISFJ), `Executive.pdf` (ESTJ), `Consul.pdf` (ESFJ)
    *   **Explorers (_S_P):** `Virtuoso.pdf` (ISTP), `Entertainer.pdf` (ESFP)

Ensure you load these files into your context (using your PDF rendering/reading tool) and analyze their structural models. Complete, deep understanding is critical.

---

## MANDATORY AUDIT SCOPE: CORE PRODUCT ONLY
To keep your audit focused on high-signal production systems and prevent wasting time or context tokens on non-production simulators or loosely-coupled marketing/CRM tools, you MUST adhere to the following scope boundaries:

*   **IN SCOPE (Audit these core production directories and domain crates):**
    *   `apps/engine`
    *   `apps/api`
    *   `apps/app` (Frontend UI, user settings, telemetry trackers, onboarding wizards, dashboards, views)
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

---

## THE METHODOLOGY: DIAGNOSTIC FACT-GATHERING & RESEARCH EXPEDITION
This audit is strictly structured as a **non-destructive, diagnostic fact-gathering and research expedition checklist**. Your primary objective is to discover, map, and document the actual current state of the codebase *before* proposing any plans or writing code.

For every single Phase in the roadmap below, you must systematically search the codebase and classify our current status into one of three distinct categories:
1.  **[COMPLETED]:** The concept/rule is fully implemented, operational, and tested. *You must cite the exact directories, file paths, components, or lines of code as empirical proof.*
2.  **[PARTIAL]:** The concept/rule is partially implemented, but contains gaps, unhandled edge cases, or missing boundaries. *You must cite the existing partial code and detail exactly what remains incomplete.*
3.  **[NOT STARTED]:** The concept/rule is completely absent from the current codebase.

You must compile this exhaustive coverage audit across all phases *prior* to formulating any Critical Adjustment Plans or writing CI/CD enforcement scripts. This prevents premature modifications and honors our existing structural investments.

---

## MANDATORY SYSTEMIC COVERAGE: NO SHALLOW VERIFICATION ("CHECKING THE BOX")
To prevent the common analytical error of shallow verification, you MUST adhere to the following exhaustive, system-wide coverage and template rules:

1.  **No Shallow Verification:** Finding a single theme toggle or a single user settings dropdown does NOT mean the codebase has "covered" that phase. You must inspect the *entire* in-scope frontend codebase to determine if user classification and adaptive UX are applied systemically across the application.
2.  **The "Golden Blueprint" Principle:**
    *   If you discover a correct, highly robust implementation of a behavioral pattern (e.g., an intelligent telemetry hook that infers user intent, an adaptive density switcher, or an elegant role-based dashboard view), declare that specific component as the **"Golden Blueprint"** for that pattern.
    *   You must then audit the rest of the application to identify every route or modal where this pattern *should* be applied but is completely missing.
    *   Use this "Golden Blueprint" as the exact template for your **Actionable Recommendations** and your **Automated CI/CD Enforcement Script** to enforce its systemic adoption across the application.
3.  **Full, Agnostic Event-Stream Auditing:**
    *   Audit user interaction event streams: clickstreams, scroll depths, dwell times, feature adoption metrics, and navigation drop-offs to verify how behavioral data is collected and transformed into user classification models.

---

## AUDIT ROADMAP (PHASE-BY-PHASE)

### Phase 1: The 5-Dimensional Behavioral Classifier (Mind, Energy, Nature, Tactics, Identity)
*   *Archetype Framework:* The 5 core personality aspects from NERIS Analytics:
    *   **Mind (Extraverted vs. Introverted):** Collaborative/interactive vs. Independent/focused task execution.
    *   **Energy (Intuitive vs. Observant):** Big-picture architectural views vs. Concrete, detailed, step-by-step data points.
    *   **Nature (Thinking vs. Feeling):** Objective logic, metrics, and efficiency vs. Empathy, values, and user experience harmony.
    *   **Tactics (Judging vs. Prospecting):** Structured checklists, deadlines, and strict workflows vs. Exploratory, flexible, and modular workspaces.
    *   **Identity (Assertive vs. Turbulent):** Confident, self-directed execution vs. Reassurance-seeking, perfectionist, validation-driven behavior.
*   *Audit Focus:* Does the application support these dimensions in its user model? Can the frontend infer or allow users to customize their working style across these 5 spectrums?

### Phase 2: The Analyst Archetypes (Architect, Logician, Commander, Debater) - Precision & Power-User UX
*   *Profiles Analyzed:* `Architect.pdf` (INTJ), `Logician.pdf` (INTP), `Commander.pdf` (ENTJ), `Debater.pdf` (ENTP).
*   *Archetype Needs:* High cognitive bandwidth, deep intellectual rigor, architectural clarity, macro-level visibility, API/terminal access, customizable density, and zero patronizing hand-holding.
*   *Audit Focus:* Review the developer/power-user views:
    *   Are raw data tables, JSON inspectors, terminal views, or advanced filters accessible?
    *   Can Analyst users bypass introductory wizards and jump directly into high-leverage controls?
    *   Is system logic transparent, predictable, and verifiable (e.g., displaying exact calculation formulas rather than opaque black boxes)?

### Phase 3: The Diplomat Archetypes (Advocate, Mediator, Protagonist, Campaigner) - Purpose, Ethics & Collaborative UX
*   *Profiles Analyzed:* `Advocate.pdf` (INFJ), `Mediator.pdf` (INFP), `Protagonist.pdf` (ENFJ), `Campaigner.pdf` (ENFP).
*   *Archetype Needs:* Value alignment, ethical transparency, human-centric design, clear mission impact, empathetic microcopy, community/sharing features, and meaningful progress tracking.
*   *Audit Focus:* Review collaborative and values-based views:
    *   Does the interface communicate *why* an action matters (impact on the broader mission or community)?
    *   Is microcopy respectful, warm, and encouraging rather than cold and mechanical?
    *   Are collaborative workflows (sharing, team invites, feedback channels) intuitive and non-adversarial?

### Phase 4: The Sentinel Archetypes (Logistician, Defender, Executive, Consul) - Stability, Compliance & Deterministic UX
*   *Profiles Analyzed:* `Logistician.pdf` (ISTJ), `Defender.pdf` (ISFJ), `Executive.pdf` (ESTJ), `Consul.pdf` (ESFJ).
*   *Archetype Needs:* Absolute reliability, clear procedural steps, complete audit trails, verifiable compliance badges, predictable schedules, and zero unexpected UI shifts.
*   *Audit Focus:* Review procedural, administrative, and compliance flows:
    *   Are transaction histories, audit logs, and exportable CSV/PDF reports easily accessible?
    *   Do forms provide clear validation rules *before* submission, preventing unexpected errors?
    *   Are critical actions accompanied by unambiguous confirmation dialogs and deterministic status feedback?

### Phase 5: The Explorer Archetypes (Virtuoso, Entertainer) - Hands-On Sandboxes & Tactical UX
*   *Profiles Analyzed:* `Virtuoso.pdf` (ISTP), `Entertainer.pdf` (ESFP).
*   *Archetype Needs:* Immediate tactile feedback, interactive playgrounds, visual sliders, minimal reading barriers, rapid prototyping, and real-time trial-and-error.
*   *Audit Focus:* Review interactive and playground interfaces:
    *   Can users manipulate parameters via interactive sliders, switches, and instant real-time previews?
    *   Is there an un-gated sandbox or simulation mode where users can test scenarios without committing permanent state?
    *   Is the interface snappy and tactile, delivering visual feedback in under 100 milliseconds?

### Phase 6: Dynamic Behavioral Telemetry & Classification State Machines
*   *Audit Focus:* In-app behavioral classification engine:
    *   Does the frontend capture behavioral signals (e.g., time spent on detailed logs vs. visual summaries, preferred view modes, click rates on advanced settings)?
    *   Is there a state machine or scoring hook that classifies user behavioral posture to tailor recommendations dynamically?
    *   Are telemetry signals collected with strict privacy guarantees (GDPR/compliance), ensuring classification is ethical and unobtrusive?

### Phase 7: Adaptive UI Rendering & Persona-Based Onboarding Routing
*   *Audit Focus:* Onboarding and layout adaptation:
    *   Does the First-Time User Experience (FTUX) ask lightweight preference questions (or infer intent) to branch users into their optimal initial view (e.g., Quick-Start Sandbox for Explorers vs. In-Depth Configuration Guide for Sentinels vs. Raw Dashboard for Analysts)?
    *   Can the application adapt layout density, notification frequency, and helper tips based on the user's observed archetype?

---

## MANDATORY DELIVERABLE: CRITICAL PLAN & AUTOMATED CI/CD ENFORCEMENT
Perform your codebase discovery through the lens of this work and map how we worked to build on this knowledge. Once complete, you must compile and deliver:

1. **Executive Summary & Lens Review:** An evaluation of the codebase and application specifically through the lens of user behavioral archetypes, detailing how we built on this knowledge to benefit the application and codebase.
2. **Phase Audit Matrix:** A table mapping *Phase* | *Citations/Files* | *Identified Gaps, Risks, or Overlaps* | *Priority (H/M/L)*.
3. **Critical Adjustment Plan:** If any critical gaps or boundary overlaps are identified, form a full, actionable plan to adjust and partition roles and responsibilities.
4. **Automated CI/CD Enforcement Script:** Write a fully functional automated validation script (e.g., Playwright E2E assertion, TypeScript type check, custom ESLint rule, or Python UI validator) that can be integrated directly into our current build scripts and CI/CD deployment pipelines to continuously maintain and enforce these rules!

---

**Begin your audit as an exhaustive, diagnostic fact-gathering and research expedition by scanning the workspace to map our active directories, configuration files, and dependencies—focusing exclusively on the core production scope: apps/engine, apps/api, apps/app, apps/adapters, apps/scheduler, and crates/, while ignoring sim-*, crm, meet, posthog, sign, website, auto, and cal. For every phase, look for full systemic implementations (such as complete event stream lifecycles) rather than checking the box with single examples. Identify any correct 'Golden Blueprints' in our codebase that we should enforce universally. Compile our [COMPLETED], [PARTIAL], or [NOT STARTED] coverage status for Phase 1 and Phase 2, citing exact file paths and lines of code before outlining any plans.**
