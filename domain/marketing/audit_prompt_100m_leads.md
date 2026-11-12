# Architectural Audit Prompt: $100M Leads (Alex Hormozi)

Copy and paste this codebase-agnostic prompt into your developer agent's session to conduct a systematic, phase-by-phase user ingress, lead magnet, and onboarding conversion audit.

---

## CONTEXT & OBJECTIVE
You are an expert Frontend Growth Engineer, User Acquisition Architect, and First-Time User Experience (FTUX) Specialist. Your task is to audit our frontend user interface and application workflows against the foundational lead-generation and conversion engines established in Alex Hormozi's *$100M Leads*.

This audit drives major frontend decisions around the initial user ingress, onboarding velocity, viral referral loops, and seamless navigation so that new users convert rapidly into active, engaged participants.

Your goal is to inspect the codebase and answer these three questions for each phase:
1. **Current Coverage:** What are we doing in our frontend/application code to cover this Phase? (Cite specific UI components, routes, auth flows, or state hooks).
2. **Gaps & Overlaps:** Are there gaps (e.g., high-friction signup barriers, lack of instant-value lead magnets, weak viral sharing hooks, confusing first-run tours) or overlaps (e.g., duplicate onboarding questionnaires, conflicting call-to-action buttons)?
3. **Actionable Recommendations:** How should we complete this Phase to optimize the frontend UI, maximize user engagement, and streamline viral referral pathways?

---

## MANDATORY PRE-REQUISITE: READ THE REPO PLAYBOOK
Before starting your codebase audit, you MUST read and analyze the full text of the sibling playbook files in the founder-playbook repository to obtain complete context on the lead engines:

*   **Target Repo Path:** `/Users/devonshigaki/.gemini/tmp/devonshigaki/founder-playbook/100m-leads/`
*   **Key Files:** `frameworks.md`, `SKILL.md`, `lead-getters.md`, `paid-ads.md`

Ensure you load these markdown playbooks into your context and analyze them end-to-end. Complete, deep understanding is critical.

---

## MANDATORY AUDIT SCOPE: CORE PRODUCT ONLY
To keep your audit focused on high-signal production systems and prevent wasting time or context tokens on non-production simulators or loosely-coupled marketing/CRM tools, you MUST adhere to the following scope boundaries:

*   **IN SCOPE (Audit these core production directories and domain crates):**
    *   `apps/engine`
    *   `apps/api`
    *   `apps/app` (Frontend UI, routes, auth flows, onboarding, invite modals, user dashboard)
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

1.  **No Shallow Verification:** Finding a single invite button or a single demo banner does NOT mean the codebase has "covered" that phase. You must inspect the *entire* in-scope frontend codebase to determine if the lead-generation and conversion engines are applied systemically across the entire user journey.
2.  **The "Golden Blueprint" Principle:**
    *   If you discover a correct, highly robust implementation of a growth pattern (e.g., an effortless 1-click Google/wallet auth flow, an irresistible instant-value dashboard preview, or a seamless share-link generator), declare that specific component as the **"Golden Blueprint"** for that pattern.
    *   You must then audit the rest of the application to identify every route or modal where this pattern *should* be applied but is completely missing.
    *   Use this "Golden Blueprint" as the exact template for your **Actionable Recommendations** and your **Automated CI/CD Enforcement Script** to enforce its systemic adoption across the application.
3.  **Full, Agnostic Event-Stream Auditing:**
    *   Audit conversion telemetry event streams: tracking funnel drop-offs, registration events, referral attribution tracking, and in-app activation checkpoints.

---

## AUDIT ROADMAP (PHASE-BY-PHASE)

### Phase 1: In-App Lead Magnets & Immediate Value Ingress
*   *Framework Reference:* Lead Magnet Naming & "Give Away What Others Charge For" (delivering instant, massive value in the first 30 seconds).
*   *Audit Focus:* Does the frontend offer an irresistible "Lead Magnet" experience upon first arrival (e.g., a free instant credit score preview, a free simulation run, an un-gated playground)? Does the user experience real value *before* facing high-friction signup walls?

### Phase 2: Hook-Retain-Reward Framework in UI Onboarding
*   *Framework Reference:* The Hook (first 3 seconds: stopping the bounce), Retain (interactive progression), and Reward (delivering high-value outcome).
*   *Audit Focus:* Walk through the First-Time User Experience (FTUX). Does the landing view hook the user immediately? Is the onboarding progress clearly tracked (e.g., interactive step-by-step progress bar)? Does the first completed action immediately reward the user with tangible feedback?

### Phase 3: The ACA Framework (Acknowledge, Compliment, Ask) in User Prompts
*   *Framework Reference:* The ACA Framework in communication (Acknowledge context, Compliment user progress, Ask for action).
*   *Audit Focus:* Inspect in-app notification dialogs, permission request modals, and upgrade prompts. Do they bluntly demand action (bad), or do they acknowledge the user's progress and frame the request constructively (good)?

### Phase 4: Customer Referral Loops & Viral "Lead-Getters"
*   *Framework Reference:* Referral Magic Words ("Do you know anyone who..."), Two-sided incentives (referrer and referee both get rewarded).
*   *Audit Focus:* Audit referral and sharing mechanisms. Is there a clean, 1-click referral link generator? Are two-sided incentives (e.g., both parties earn tokens or credit bonuses) displayed prominently on the dashboard? Can users invite team members effortlessly?

### Phase 5: Affiliates & Partner Integration Boundaries
*   *Framework Reference:* Integrating external partners, agencies, and affiliates as automated lead engines.
*   *Audit Focus:* Review partner integration endpoints and affiliate tracking links. Are referral tracking cookies/tokens persisted reliably across authentication boundaries? Is attribution transparent and tamper-resistant?

---

## MANDATORY DELIVERABLE: CRITICAL PLAN & AUTOMATED CI/CD ENFORCEMENT
Perform your codebase discovery through the lens of this work and map how we worked to build on this knowledge. Once complete, you must compile and deliver:

1. **Executive Summary & Lens Review:** An evaluation of the codebase and application specifically through the lens of this work, detailing how we built on this knowledge to benefit the application and codebase.
2. **Phase Audit Matrix:** A table mapping *Phase* | *Citations/Files* | *Identified Gaps, Risks, or Overlaps* | *Priority (H/M/L)*.
3. **Critical Adjustment Plan:** If any critical gaps or boundary overlaps are identified, form a full, actionable plan to adjust and partition roles and responsibilities.
4. **Automated CI/CD Enforcement Script:** Write a fully functional automated validation script (e.g., Playwright E2E assertion, TypeScript type check, custom ESLint rule, or Python UI validator) that can be integrated directly into our current build scripts and CI/CD deployment pipelines to continuously maintain and enforce these rules!

---

**Begin your audit as an exhaustive, diagnostic fact-gathering and research expedition by scanning the workspace to map our active directories, configuration files, and dependencies—focusing exclusively on the core production scope: apps/engine, apps/api, apps/app, apps/adapters, apps/scheduler, and crates/, while ignoring sim-*, crm, meet, posthog, sign, website, auto, and cal. For every phase, look for full systemic implementations (such as complete event stream lifecycles) rather than checking the box with single examples. Identify any correct 'Golden Blueprints' in our codebase that we should enforce universally. Compile our [COMPLETED], [PARTIAL], or [NOT STARTED] coverage status for Phase 1 and Phase 2, citing exact file paths and lines of code before outlining any plans.**
