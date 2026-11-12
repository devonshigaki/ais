# Architectural Audit Prompt: Unified Business Model & Go-To-Market (GTM)

Copy and paste this codebase-agnostic prompt into your developer agent's session to conduct a systematic, phase-by-phase business operations, Go-To-Market (GTM), and revenue telemetry audit.

---

## CONTEXT & OBJECTIVE
You are an expert Revenue Operations Architect, SaaS GTM Strategist, and Business Systems Engineer. Your task is to audit our codebase, application workflows, and data models against the comprehensive collection of business frameworks established in the research media in `Desktop/AIS/research/business/`. These frameworks cover the **SPICED methodology, The SaaS Sales Method, Metric-Based Leadership, the Operating Model for Recurring Revenue (the Bowtie Funnel), Product-Led Growth (PLG), and Customer Success mechanics.**

This audit drives critical backend and frontend architecture decisions to ensure our application structure directly mirrors and supports our business acquisition, conversion, and retention strategies.

Your goal is to inspect the codebase and answer these three questions for each phase:
1. **Current Coverage:** What are we doing in our codebase to cover this Phase? (Cite specific UI components, state stores, telemetry hooks, API routes, or database schemas).
2. **Gaps & Overlaps:** Are there gaps (e.g., lack of PLG telemetry, missing SPICED data capture, disjointed onboarding flows that ignore Critical Events) or overlaps (e.g., conflicting pricing models, duplicate billing synchronizations)?
3. **Actionable Recommendations:** How should we complete this Phase to build an intelligent, telemetry-driven business engine that optimizes the end-to-end customer journey?

---

## MANDATORY PRE-REQUISITE: READ THE ENTIRE BUSINESS MEDIA COLLECTION
Before starting your codebase audit, you MUST review and analyze the comprehensive business research PDFs located in the business directory to obtain complete context on the GTM mechanics, SPICED framework, and Recurring Revenue operating models:

*   **Target Directory Path:** `/Users/devonshigaki/Desktop/AIS/research/business/`
*   **Key Source Files to Analyze:** `SPICED Business Framework.pdf`, `The SaaS Sale Method.pdf`, `The Operating Model for Recurring Revenue.pdf`, `Customer Success as a Profit Center.pdf`, `How to add PLG to your existing GTM.pdf`, `Metric-Based Leadership.pdf`.

Ensure you load these files into your context (using your PDF rendering/reading tool) and analyze their structural models. Complete, deep understanding is critical.

---

## MANDATORY AUDIT SCOPE: CORE PRODUCT ONLY
To keep your audit focused on high-signal production systems and prevent wasting time or context tokens on non-production simulators or loosely-coupled marketing/CRM tools, you MUST adhere to the following scope boundaries:

*   **IN SCOPE (Audit these core production directories and domain crates):**
    *   `apps/engine`
    *   `apps/api`
    *   `apps/app` (Frontend UI, user settings, billing dashboards, usage telemetry, PLG upgrade hooks)
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

1.  **No Shallow Verification:** Finding a single Stripe webhook or a single telemetry event does NOT mean the codebase has "covered" that phase. You must inspect the *entire* in-scope codebase to determine if the business models (e.g., SPICED data capture, PLG upgrade paths) are applied systemically across the application.
2.  **The "Golden Blueprint" Principle:**
    *   If you discover a correct, highly robust implementation of a business pattern (e.g., an intelligent metric tracker that captures user Impact, a seamless Product-Led Growth upgrade boundary, or an elegant renewal auto-sync), declare that specific component as the **"Golden Blueprint"** for that pattern.
    *   You must then audit the rest of the application to identify every route or modal where this pattern *should* be applied but is completely missing.
    *   Use this "Golden Blueprint" as the exact template for your **Actionable Recommendations** and your **Automated CI/CD Enforcement Script** to enforce its systemic adoption across the application.
3.  **Full, Agnostic Event-Stream Auditing:**
    *   Audit revenue and customer journey event streams: signups, feature usage milestones, upgrade trigger points, renewal states, and churn indicators.

---

## AUDIT ROADMAP (PHASE-BY-PHASE)

### Phase 1: The SPICED Framework (Data Models & State Tracking)
*   *Framework Reference:* **S**ituation, **P**ain, **I**mpact, **C**ritical Event, **D**ecision.
*   *Audit Focus:* Does the application schema, user state, and onboarding logic capture and track SPICED criteria? 
    *   Are we capturing the user's *Impact* goal during onboarding and reflecting it in their dashboard?
    *   Are *Critical Events* (deadlines, key milestones) recorded in the database and used to trigger automated reminders or prioritized scheduling?

### Phase 2: Operating Model for Recurring Revenue (The Bowtie Funnel)
*   *Framework Reference:* Shifting from the traditional funnel to the Bowtie Funnel (Awareness $\rightarrow$ Education $\rightarrow$ Selection $\rightarrow$ Onboarding $\rightarrow$ Impact $\rightarrow$ Growth).
*   *Audit Focus:* Are the post-sale stages (Onboarding, Impact realization, and Growth/Upsell) instrumented with clear product milestones? Does the application track time-to-first-value (Impact) as a core backend metric?

### Phase 3: Product-Led Growth (PLG) & Sales-Led Motion Integration
*   *Framework Reference:* Adding PLG to existing GTM; Product Qualified Leads (PQLs).
*   *Audit Focus:* How does the product handle self-serve upgrades vs. sales handoffs? Are there automated "PQL" thresholds coded into the telemetry that flag high-usage free accounts for enterprise upgrades? Are there friction-free PLG upgrade gates in the UI?

### Phase 4: Customer Success as a Profit Center (Retention Architecture)
*   *Framework Reference:* Kickoff Calls, Joint Impact Plans, and Solving Underperformance.
*   *Audit Focus:* Does the product architecture support Customer Success (CS) operations? 
    *   Are Joint Impact Plans integrated into the user interface (e.g., shared progress trackers between the user and the system)?
    *   Does the backend emit alerts when usage drops (diagnosing underperformance), allowing CS to intervene before churn?

### Phase 5: Metric-Based Leadership & Forecasting Telemetry
*   *Framework Reference:* Growth Forecasting, Unit Economics, and Metric-Based Leadership.
*   *Audit Focus:* Are core financial metrics and unit economic indicators (LTV, CAC, NRR, GRR) supported by robust backend data pipelines? Is billing data cleanly isolated and aggregated to provide real-time forecasting visibility?

### Phase 6: Pricing, Discounts, and Deal Architecture
*   *Framework Reference:* The Relationship Between Discount and Win Rate; How to Trade.
*   *Audit Focus:* How are pricing plans and discounts hardcoded or configured in the system? Are discount codes and promotional trades handled securely via robust backend gateways (e.g., Stripe configurations) without leaking margin or allowing abuse?

### Phase 7: Automation & AI in Revenue Growth
*   *Framework Reference:* How AI Will Replace People in Business; Process and AI revolutionizing revenue.
*   *Audit Focus:* Does the system leverage AI or automated workflow orchestration (e.g., `apps/scheduler` background jobs) to automate repetitive business tasks (scheduling, follow-ups, diagnostic health checks) that traditionally required human intervention?

---

## MANDATORY DELIVERABLE: CRITICAL PLAN & AUTOMATED CI/CD ENFORCEMENT
Perform your codebase discovery through the lens of this work and map how we worked to build on this knowledge. Once complete, you must compile and deliver:

1. **Executive Summary & Lens Review:** An evaluation of the codebase and application specifically through the lens of our unified business model, detailing how we built on this knowledge to benefit the application and codebase.
2. **Phase Audit Matrix:** A table mapping *Phase* | *Citations/Files* | *Identified Gaps, Risks, or Overlaps* | *Priority (H/M/L)*.
3. **Critical Adjustment Plan:** If any critical gaps or boundary overlaps are identified, form a full, actionable plan to adjust and partition roles and responsibilities.
4. **Automated CI/CD Enforcement Script:** Write a fully functional automated validation script (e.g., Playwright E2E assertion, TypeScript type check, custom ESLint rule, or Python validation script) that can be integrated directly into our current build scripts and CI/CD deployment pipelines to continuously maintain and enforce these business rules!

---

**Begin your audit as an exhaustive, diagnostic fact-gathering and research expedition by scanning the workspace to map our active directories, configuration files, and dependencies—focusing exclusively on the core production scope: apps/engine, apps/api, apps/app, apps/adapters, apps/scheduler, and crates/, while ignoring sim-*, crm, meet, posthog, sign, website, auto, and cal. For every phase, look for full systemic implementations (such as complete event stream lifecycles) rather than checking the box with single examples. Identify any correct 'Golden Blueprints' in our codebase that we should enforce universally. Compile our [COMPLETED], [PARTIAL], or [NOT STARTED] coverage status for Phase 1 and Phase 2, citing exact file paths and lines of code before outlining any plans.**
