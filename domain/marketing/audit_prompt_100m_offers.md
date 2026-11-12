# Architectural Audit Prompt: $100M Offers (Alex Hormozi)

Copy and paste this codebase-agnostic prompt into your developer agent's session to conduct a systematic, phase-by-phase frontend value architecture, offer communication, and user onboarding audit.

---

## CONTEXT & OBJECTIVE
You are an expert Frontend Architect, Product Onboarding Engineer, and User Value Delivery Specialist. Your task is to audit our frontend user interface and application workflows against the foundational value creation, offer communication, and risk-reversal frameworks established in Alex Hormozi's *$100M Offers*.

This audit drives major frontend decisions around the user interface to ensure users immediately understand, navigate, and receive immense perceived and actual value from the application itself.

Your goal is to inspect the codebase and answer these three questions for each phase:
1. **Current Coverage:** What are we doing in our frontend/application code to cover this Phase? (Cite specific UI components, pages, routes, or state stores).
2. **Gaps & Overlaps:** Are there gaps (e.g., confusing value propositions, high user effort/sacrifice hurdles, long time delays to first "Aha!" moment, weak guarantees, friction in navigation) or overlaps (e.g., competing calls-to-action, redundant onboarding screens)?
3. **Actionable Recommendations:** How should we complete this Phase to optimize the frontend UI, maximize user clarity, and streamline rapid value realization?

---

## MANDATORY PRE-REQUISITE: READ THE REPO PLAYBOOK
Before starting your codebase audit, you MUST read and analyze the full text of the sibling playbook files in the founder-playbook repository to obtain complete context on the offer frameworks:

*   **Target Repo Path:** `/Users/devonshigaki/.gemini/tmp/devonshigaki/founder-playbook/100m-offers/`
*   **Key Files:** `frameworks.md`, `SKILL.md`, `integration.md`

Ensure you load these markdown playbooks into your context and analyze them end-to-end. Complete, deep understanding is critical.

---

## MANDATORY AUDIT SCOPE: CORE PRODUCT ONLY
To keep your audit focused on high-signal production systems and prevent wasting time or context tokens on non-production simulators or loosely-coupled marketing/CRM tools, you MUST adhere to the following scope boundaries:

*   **IN SCOPE (Audit these core production directories and domain crates):**
    *   `apps/engine`
    *   `apps/api`
    *   `apps/app` (Frontend UI, navigation, routes, user onboarding, checkout, dashboard)
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

1.  **No Shallow Verification:** Finding a single working modal or tooltip does NOT mean the codebase has "covered" that phase. You must inspect the *entire* in-scope frontend codebase to determine if the pattern is applied systemically across all screens, or if there are major usability gaps and inconsistent UX patterns elsewhere.
2.  **The "Golden Blueprint" Principle:**
    *   If you discover a correct, highly robust implementation of a frontend pattern (e.g., an intuitive onboarding flow, an unmistakable value-display card, or an elegant empty-state helper), declare that specific component as the **"Golden Blueprint"** for that pattern.
    *   You must then audit the rest of the application to identify every route or modal where this pattern *should* be applied but is completely missing, or where divergent, clunky structures are used instead.
    *   Use this "Golden Blueprint" as the exact template for your **Actionable Recommendations** and your **Automated CI/CD Enforcement Script** to enforce its systemic adoption across the frontend.
3.  **Full, Agnostic Event-Stream Auditing:**
    *   Audit frontend event streams and real-time UI synchronization: user interaction analytics, optimistic UI updates, error toast notifications, and WebSocket/SSE event listeners to ensure state remains reactive and seamless.

---

## AUDIT ROADMAP (PHASE-BY-PHASE)

### Phase 1: The Value Equation (Frontend Value Clarity)
*   *Framework Reference:* $\text{Value} = \frac{\text{Dream Outcome} \times \text{Perceived Likelihood of Achievement}}{\text{Time Delay} \times \text{Effort \& Sacrifice}}$
*   *Audit Focus:* How does the frontend communicate and deliver the 4 variables of the Value Equation?
    *   **Dream Outcome:** Does the user interface immediately display what specific transformation or goal the user will achieve upon login/onboarding?
    *   **Perceived Likelihood:** Are there clear social proof, verification badges, progress trackers, or cryptographic trust indicators boosting user confidence?
    *   **Time Delay:** How fast is the time-to-value? Does the application deliver a quick win in the first 60 seconds (low time delay), or does it force long setup configurations?
    *   **Effort & Sacrifice:** How much friction is in the UX? Can tasks be automated (Done-For-You/DFY) rather than forcing tedious manual entry?

### Phase 2: Solving All Obstacles (The How-To Reversal in UI)
*   *Framework Reference:* Identifying all 4 obstacle categories (Knowledge Gaps, Skill Deficiencies, Environmental Barriers, Psychological Blocks) and reversing them into "How-to" UI features.
*   *Audit Focus:* Review in-app guides, empty states, and error helpers. Does the UI proactively anticipate user confusion and provide instant solutions (e.g., inline help tooltips, automated defaults, templates) to prevent user drop-off?

### Phase 3: Delivery Vehicles & Modality Architecture
*   *Framework Reference:* Selecting the optimal delivery vehicle (Done-For-You vs. Done-With-You vs. DIY, 1-on-1 vs. 1-to-Many, Speed of Delivery).
*   *Audit Focus:* Are complex workflows automated for the user (DFY)? Does the application provide interactive wizards or guided checklists (DWY)? Are responsive response times and delivery speeds optimized across all views?

### Phase 4: Trim & Stack (Pruning Cognitive Feature Bloat)
*   *Framework Reference:* The Value-Cost Matrix (scoring components 1-10 on Value to Customer vs Cost/Complexity to Deliver; keeping high value, cutting low-value sludge).
*   *Audit Focus:* Feature bloat audit. Are there cluttered navigation bars, redundant buttons, or confusing secondary features on the main dashboard that distract from the primary user journey? Are non-essential tools trimmed or moved to advanced settings?

### Phase 5: Bundle & Feature Naming (Benefit-Driven UI Microcopy)
*   *Framework Reference:* Naming formula: `[Problem] -> [Solution Statement] -> [Sexy Bundle Name]`. Communicating benefits over technical jargon.
*   *Audit Focus:* Inspect buttons, tabs, headers, and tooltips. Do they use plain, benefit-driven language (e.g., "1-Click Credit Verification") or technical jargon (e.g., "Execute RPC Substrate Extrinsic")? Does the microcopy make the feature's value unmistakable?

### Phase 6: Guarantees & Risk Reversals in Critical Flows
*   *Framework Reference:* Unconditional, Conditional, and Anti-Guarantees. Removing perceived risk from high-friction actions.
*   *Audit Focus:* Review confirmation dialogs, payment screens, and high-impact transaction submissions. Is risk reversed through clear "Undo" actions, test-mode sandboxes, zero-fee trial periods, or transparent confirmation summaries that eliminate user hesitation?

### Phase 7: Scarcity, Urgency & Priority Access Signals
*   *Framework Reference:* Ethical scarcity (limited capacity/spots) and urgency (deadlines, countdowns, rolling windows).
*   *Audit Focus:* If the application includes limited-time promotions, priority queue tiers, or exclusive access gates, are they communicated clearly and ethically with real-time UI state synchronization?

---

## MANDATORY DELIVERABLE: CRITICAL PLAN & AUTOMATED CI/CD ENFORCEMENT
Perform your codebase discovery through the lens of this work and map how we worked to build on this knowledge. Once complete, you must compile and deliver:

1. **Executive Summary & Lens Review:** An evaluation of the codebase and application specifically through the lens of this work, detailing how we built on this knowledge to benefit the application and codebase.
2. **Phase Audit Matrix:** A table mapping *Phase* | *Citations/Files* | *Identified Gaps, Risks, or Overlaps* | *Priority (H/M/L)*.
3. **Critical Adjustment Plan:** If any critical gaps or boundary overlaps are identified, form a full, actionable plan to adjust and partition roles and responsibilities.
4. **Automated CI/CD Enforcement Script:** Write a fully functional automated validation script (e.g., Playwright E2E assertion, TypeScript type check, custom ESLint rule, or Python UI validator) that can be integrated directly into our current build scripts and CI/CD deployment pipelines to continuously maintain and enforce these rules!

---

**Begin your audit as an exhaustive, diagnostic fact-gathering and research expedition by scanning the workspace to map our active directories, configuration files, and dependencies—focusing exclusively on the core production scope: apps/engine, apps/api, apps/app, apps/adapters, apps/scheduler, and crates/, while ignoring sim-*, crm, meet, posthog, sign, website, auto, and cal. For every phase, look for full systemic implementations (such as complete event stream lifecycles) rather than checking the box with single examples. Identify any correct 'Golden Blueprints' in our codebase that we should enforce universally. Compile our [COMPLETED], [PARTIAL], or [NOT STARTED] coverage status for Phase 1 and Phase 2, citing exact file paths and lines of code before outlining any plans.**
