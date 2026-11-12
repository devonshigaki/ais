# Architectural Audit Prompt: Money Models (Alex Hormozi)

Copy and paste this codebase-agnostic prompt into your developer agent's session to conduct a systematic, phase-by-phase frontend pricing architecture, checkout flow, and monetization model audit.

---

## CONTEXT & OBJECTIVE
You are an expert Monetization Architect, Checkout Systems Engineer, and Revenue Operations Specialist. Your task is to audit our frontend user interface and application workflows against the foundational pricing, attraction, upsell, and continuity models established in Alex Hormozi's *Money Models*.

This audit drives major frontend decisions around pricing page clarity, checkout navigation, upsell/downsell modal sequencing, and recurring subscription retention so that users clearly understand pricing and experience maximum lifetime value.

Your goal is to inspect the codebase and answer these three questions for each phase:
1. **Current Coverage:** What are we doing in our frontend/application code to cover this Phase? (Cite specific pricing components, checkout routes, subscription hooks, or billing managers).
2. **Gaps & Overlaps:** Are there gaps (e.g., hidden pricing tiers, high-friction checkout forms, missing upsell moments of need, lack of downsell options on cancellation, high-churn billing UX) or overlaps (e.g., competing pricing plans, duplicate payment gateways)?
3. **Actionable Recommendations:** How should we complete this Phase to optimize the frontend UI, maximize customer lifetime value, and streamline seamless payments?

---

## MANDATORY PRE-REQUISITE: READ THE REPO PLAYBOOK
Before starting your codebase audit, you MUST read and analyze the full text of the sibling playbook files in the founder-playbook repository to obtain complete context on the monetization models:

*   **Target Repo Path:** `/Users/devonshigaki/.gemini/tmp/devonshigaki/founder-playbook/money-models/`
*   **Key Files:** `frameworks.md`, `SKILL.md`, `cases.md`, `integration.md`

Ensure you load these markdown playbooks into your context and analyze them end-to-end. Complete, deep understanding is critical.

---

## MANDATORY AUDIT SCOPE: CORE PRODUCT ONLY
To keep your audit focused on high-signal production systems and prevent wasting time or context tokens on non-production simulators or loosely-coupled marketing/CRM tools, you MUST adhere to the following scope boundaries:

*   **IN SCOPE (Audit these core production directories and domain crates):**
    *   `apps/engine`
    *   `apps/api`
    *   `apps/app` (Frontend UI, pricing tables, checkout flow, billing settings, upgrade modals, retention dashboard)
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

1.  **No Shallow Verification:** Finding a single Stripe button or a single pricing tier card does NOT mean the codebase has "covered" that phase. You must inspect the *entire* in-scope codebase to determine if the complete 4-step money model (Attraction -> Upsell -> Downsell -> Continuity) is implemented systemically across the application.
2.  **The "Golden Blueprint" Principle:**
    *   If you discover a correct, highly robust implementation of a monetization pattern (e.g., an effortless 1-click checkout flow, a smart Magnetic-Middle pricing tier comparison, or an empathetic cancellation-rescue downsell modal), declare that specific component as the **"Golden Blueprint"** for that pattern.
    *   You must then audit the rest of the application to identify every route or modal where this pattern *should* be applied but is completely missing.
    *   Use this "Golden Blueprint" as the exact template for your **Actionable Recommendations** and your **Automated CI/CD Enforcement Script** to enforce its systemic adoption across the application.
3.  **Full, Agnostic Event-Stream Auditing:**
    *   Audit subscription and billing event streams: Webhook handling (Stripe/crypto), invoice status updates, payment retry decay schedules, and cancellation event telemetry.

---

## AUDIT ROADMAP (PHASE-BY-PHASE)

### Phase 1: Attraction Offers & Entry Pricing (The Front-End Hook)
*   *Framework Reference:* The Attraction Offer (Win Your Money Back, Free/Freemium Decoys, Buy X Get Y Free, Pay Less Now or Pay More Later).
*   *Audit Focus:* Review the entry point pricing. Does the application offer a low-risk attraction tier (e.g., freemium with instant utility, free trial with card-on-file, or performance-based refund guarantee) that eliminates hesitation for new users?

### Phase 2: Upsell Sequencing (The Moment of Greatest Need)
*   *Framework Reference:* Classic Upsell, Menu Upsell, Anchor Upsell ("5-10x the main offer to get the gasp"), and the Magnetic Middle (pricing tiers designed to guide selection toward the optimal tier).
*   *Audit Focus:* Audit in-app upgrade triggers. Does the UI present contextual upsells at the customer's *moment of greatest need* (e.g., when reaching a capacity ceiling, rather than generic spam banners)? Is pricing tiered cleanly using the Magnetic Middle to make the core offering an obvious deal?

### Phase 3: Downsell Architecture (Converting the "No"s into a Different "Yes")
*   *Framework Reference:* The 6 Rules of Downselling (never drop price on the same thing; offer payment plans, feature restructuring, or trial extensions).
*   *Audit Focus:* Review the rejection/abandonment flows. When a user declines a premium upgrade or attempts to cancel a subscription, does the UI present an intelligent downsell (e.g., a flexible payment plan, pause subscription option, or lower-tier lightweight plan) to preserve the relationship?

### Phase 4: Continuity & Recurring Subscription Retention
*   *Framework Reference:* Continuity stacking (making customers pay for themselves and the next customer reliably; lifetime value maximization).
*   *Audit Focus:* Review subscription management and retention dashboards. Are recurring billing dates transparent? Are renewal receipts clean? Is cancellation self-serve but fortified with empathetic retention options (rescue offers)?

### Phase 5: Rollover Credits & Value Lock-In
*   *Framework Reference:* Rollover Upsells (crediting prior spend toward bigger/next offers rather than issuing refunds or letting credits expire).
*   *Audit Focus:* How does the system handle unused credits, tokens, or subscription balances? Are credits rolled forward dynamically to encourage ongoing usage and increase platform switching costs?

---

## MANDATORY DELIVERABLE: CRITICAL PLAN & AUTOMATED CI/CD ENFORCEMENT
Perform your codebase discovery through the lens of this work and map how we worked to build on this knowledge. Once complete, you must compile and deliver:

1. **Executive Summary & Lens Review:** An evaluation of the codebase and application specifically through the lens of this work, detailing how we built on this knowledge to benefit the application and codebase.
2. **Phase Audit Matrix:** A table mapping *Phase* | *Citations/Files* | *Identified Gaps, Risks, or Overlaps* | *Priority (H/M/L)*.
3. **Critical Adjustment Plan:** If any critical gaps or boundary overlaps are identified, form a full, actionable plan to adjust and partition roles and responsibilities.
4. **Automated CI/CD Enforcement Script:** Write a fully functional automated validation script (e.g., Playwright E2E assertion, TypeScript type check, custom ESLint rule, or Python UI validator) that can be integrated directly into our current build scripts and CI/CD deployment pipelines to continuously maintain and enforce these rules!

---

**Begin your audit as an exhaustive, diagnostic fact-gathering and research expedition by scanning the workspace to map our active directories, configuration files, and dependencies—focusing exclusively on the core production scope: apps/engine, apps/api, apps/app, apps/adapters, apps/scheduler, and crates/, while ignoring sim-*, crm, meet, posthog, sign, website, auto, and cal. For every phase, look for full systemic implementations (such as complete event stream lifecycles) rather than checking the box with single examples. Identify any correct 'Golden Blueprints' in our codebase that we should enforce universally. Compile our [COMPLETED], [PARTIAL], or [NOT STARTED] coverage status for Phase 1 and Phase 2, citing exact file paths and lines of code before outlining any plans.**
