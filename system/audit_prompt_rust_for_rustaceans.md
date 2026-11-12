# Architectural Audit Prompt: Rust for Rustaceans (Jon Gjengset)

Copy and paste this codebase-agnostic prompt into your developer agent's session to conduct a systematic, phase-by-phase advanced systems engineering, idiomatic interface design, and concurrency/unsafe audit.

---

## CONTEXT & OBJECTIVE
You are an expert Principal Rust Architect, Systems Performance Engineer, and Compiler Specialist. Your task is to audit our codebase against the advanced idiomatic patterns, memory layouts, interface designs, concurrency models, and unsafe invariants established in Jon Gjengset's *Rust for Rustaceans*.

Your goal is to inspect the codebase and answer these three questions for each phase:
1. **Current Coverage:** What are we doing in our codebase to cover this Phase? (Cite specific directories, files, or crates).
2. **Gaps & Overlaps:** Are there gaps (e.g., suboptimal memory layouts, leaky crate boundaries, poor error ergonomics, unsound unsafe blocks, non-pinned async state machines) or overlaps (e.g., duplicate macro definitions, competing error hierarchies, redundant traits)?
3. **Actionable Recommendations:** How should we complete this Phase to ensure idiomatic Rust mastery, optimal memory alignment, and bulletproof safety?

---

## MANDATORY PRE-REQUISITE: READ THE ENTIRE BOOK
Before starting your codebase audit, you MUST read and analyze the full text of the sibling book PDF file to obtain complete context, grasp the subtle memory layout and compiler-reasoning models, and preserve absolute architectural fidelity.

*   **Target PDF File Name:** `Rust for Rustaceans.pdf`
*   **Target PDF Path:** `/Users/devonshigaki/Desktop/Books/Rust for Rustaceans.pdf`

Ensure you load this file into your context (using your PDF rendering/reading tool) and analyze it end-to-end. Do not rely solely on the summaries provided in this prompt. Complete, deep understanding is critical.

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

---

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

1.  **No Shallow Verification:** Finding a single working example, class, or module that correctly implements a concept (such as a single custom error type or trait implementation) does NOT mean the codebase has "covered" that phase. You must inspect the *entire* in-scope codebase to determine if the pattern is applied systemically, or if there are major gaps, omissions, and competing inconsistent patterns elsewhere.
2.  **The "Golden Blueprint" Principle:**
    *   If you discover a correct, highly robust implementation of a pattern (e.g., an idiomatic trait boundary, a clean `thiserror` enum, or an optimal zero-cost async wrapper), declare that specific implementation as the **"Golden Blueprint"** for that pattern.
    *   You must then audit the rest of the codebase to identify every location where this pattern *should* be applied but is completely missing, or where divergent, inferior structures are used instead (creating overlaps, vulnerabilities, and maintenance friction).
    *   Use this "Golden Blueprint" as the exact template for your **Actionable Recommendations** and your **Automated CI/CD Enforcement Script** to enforce its systemic adoption across all core repositories.
3.  **Full, Agnostic Event-Stream Auditing:**
    *   When auditing event streams or async channels, do not satisfy yourself with pointing to a single queue example. You must audit the *full, agnostic implementation of event streams* across their entire data lifecycle: ingestion buffers, deterministic execution ordering, retry/backpressure enforcements, schema validation at every consumer hop, and state synchronization.

---

## AUDIT ROADMAP (PHASE-BY-PHASE)

### Phase 1: Foundations
*   *Chapter-Specific Concluding Summary:*
```text
Summary
The aim of this chapter has been to establish a solid, shared foundation 
that we can build on in the chapters to come. By now, I hope you feel that 
you have a firm grasp on Rust’s memory and ownership model, and that 
those errors you may have gotten from the borrow checker seem less mys -
terious. You might have known bits and pieces of what we covered here 
already, but hopefully the chapter has given you a more holistic image of 
how it all fits together. In the next chapter, we will do something similar for 
types. We’ll go over how types are represented in memory, see how gener-
ics and traits produce running code, and take a look at some of the special 
type and trait constructs Rust offers for more advanced use cases.
Intermediate Rust (Early Access) © 2022 by Jon Gjengset 
Intermediate Rust (Early Access) © 2022 by Jon Gjengset
```
*   *Audit Focus:* Compare our Rust codebase directly to the advanced idioms extracted above. Analyze memory alignment, trait bounds, type states, error hierarchies, crate visibility, macro hygiene, async pinning/runtimes, unsafe sound invariants, and concurrent synchronization primitives.

### Phase 2: Types
*   *Chapter-Specific Concluding Summary:*
```text
Summary
This chapter has provided a thorough review of the Rust type system. We’ve 
looked both at how the compiler manifests types in memory and how it 
reasons about the types themselves. This is important background material 
for writing unsafe code, complex application interfaces, and asynchronous 
code in later chapters. You’ll also find that much of the type reasoning from 
this chapter plays into how you design Rust code interfaces, which we’ll 
cover in the next chapter.
Intermediate Rust (Early Access) © 2022 by Jon Gjengset 
Intermediate Rust (Early Access) © 2022 by Jon Gjengset
```
*   *Audit Focus:* Compare our Rust codebase directly to the advanced idioms extracted above. Analyze memory alignment, trait bounds, type states, error hierarchies, crate visibility, macro hygiene, async pinning/runtimes, unsafe sound invariants, and concurrent synchronization primitives.

### Phase 3: Designing Interfaces
*   *Chapter-Specific Concluding Summary:*
```text
Summary
In this chapter we’ve explored the many facets of designing a Rust inter -
face, whether it’s intended for external use or just as an abstraction bound -
ary between the different modules within your crate. We covered a lot of 
specific pitfalls and tricks, but ultimately, the high-level principles are what 
should guide your thinking: your interfaces should be unsurprising, flex -
ible, obvious, and constrained. In the next chapter, we will dig into how to 
represent and handle errors in Rust code.
Intermediate Rust (Early Access) © 2022 by Jon Gjengset
```
*   *Audit Focus:* Compare our Rust codebase directly to the advanced idioms extracted above. Analyze memory alignment, trait bounds, type states, error hierarchies, crate visibility, macro hygiene, async pinning/runtimes, unsafe sound invariants, and concurrent synchronization primitives.

### Phase 4: Error Handling
*   *Chapter-Specific Concluding Summary:*
```text
Summary
This chapter covered the two primary ways to construct error types in Rust: 
enumeration and erasure. We looked at when you may want to use each one 
and the advantages and drawbacks of each. We also took a look at some of 
the behind-the-scenes aspects of the ? operator and considered how ? may 
become even more useful going forward. In the next chapter, we’ll take a 
step back from the code and look at how you structure a Rust project. We’ll 
look at feature flags, dependency management, and versioning as well as 
how to manage more complex crates using workspaces and subcrates. See 
you on the next page!
Intermediate Rust (Early Access) © 2022 by Jon Gjengset 
Intermediate Rust (Early Access) © 2022 by Jon Gjengset
```
*   *Audit Focus:* Compare our Rust codebase directly to the advanced idioms extracted above. Analyze memory alignment, trait bounds, type states, error hierarchies, crate visibility, macro hygiene, async pinning/runtimes, unsafe sound invariants, and concurrent synchronization primitives.

### Phase 5: Project Structure
*   *Chapter-Specific Concluding Summary:*
```text
Summary
In this chapter, we’ve looked at a number of mechanisms for configuring, 
organizing, and publishing crates, for both your own benefit and that of 
others. We’ve also gone over some common gotchas when working with 
dependencies and features in Cargo that now hopefully won’t catch you out 
in the future. In the next chapter we’ll turn to testing and dig into how you 
go beyond Rust’s simple #[test] functions that we know and love.
Intermediate Rust (Early Access) © 2022 by Jon Gjengset
```
*   *Audit Focus:* Compare our Rust codebase directly to the advanced idioms extracted above. Analyze memory alignment, trait bounds, type states, error hierarchies, crate visibility, macro hygiene, async pinning/runtimes, unsafe sound invariants, and concurrent synchronization primitives.

### Phase 6: Testing
*   *Chapter-Specific Concluding Summary:*
```text
Summary
In this chapter, we explored the built-in testing capabilities that Rust 
offers in great detail. We also looked at a number of testing facilities and 
techniques that are useful when testing Rust code. This is the last chapter 
that focuses on higher-level aspects of intermediate Rust use in this book. 
Starting with the next chapter on declarative and procedural macros, we 
will be focusing much more on Rust code. See you on the next page!
Intermediate Rust (Early Access) © 2022 by Jon Gjengset
```
*   *Audit Focus:* Compare our Rust codebase directly to the advanced idioms extracted above. Analyze memory alignment, trait bounds, type states, error hierarchies, crate visibility, macro hygiene, async pinning/runtimes, unsafe sound invariants, and concurrent synchronization primitives.

### Phase 7: Macros
*   *Chapter-Specific Concluding Summary:*
```text
Summary
In this chapter we covered both declarative and procedural macros, and 
looked at when you might find each of them useful in your own code. We 
also took a deeper dive into the mechanisms that underpin each type of 
macro and some of the features and gotchas to be aware of when you write 
your own macros. In the next chapter, we’ll start our journey into asyn -
chronous programming and the Future trait. I promise—it’s just on  
the next page.
Intermediate Rust (Early Access) © 2022 by Jon Gjengset 
Intermediate Rust (Early Access) © 2022 by Jon Gjengset
```
*   *Audit Focus:* Compare our Rust codebase directly to the advanced idioms extracted above. Analyze memory alignment, trait bounds, type states, error hierarchies, crate visibility, macro hygiene, async pinning/runtimes, unsafe sound invariants, and concurrent synchronization primitives.

### Phase 8: Asynchronous Programming
*   *Chapter-Specific Concluding Summary:*
```text
Summary
In this chapter, we’ve taken a look behind the scenes of the asynchronous 
constructs available in Rust. We’ve seen how the compiler implements gen-
erators and self-referential types, and why that work was necessary to sup -
port what we now know as async/await. We’ve also explored how futures are 
executed, and how wakers allow executors to multiplex among tasks when 
only some of them can make progress at any given moment. In the next 
chapter, we’ll tackle what is perhaps the deepest and most discussed area 
of Rust: unsafe code. Take a deep breath, and then turn the page.
Intermediate Rust (Early Access) © 2022 by Jon Gjengset
```
*   *Audit Focus:* Compare our Rust codebase directly to the advanced idioms extracted above. Analyze memory alignment, trait bounds, type states, error hierarchies, crate visibility, macro hygiene, async pinning/runtimes, unsafe sound invariants, and concurrent synchronization primitives.

### Phase 9: Unsafe Code
*   *Chapter-Specific Concluding Summary:*
```text
Summary
In this chapter, we’ve walked through the powers that come with the unsafe 
keyword, and the responsibilities we accept by leveraging those powers. We 
also talked about the consequences of writing unsafe unsafe code, and how 
you really should be thinking about unsafe as a way to swear to the compiler 
that you’ve manually checked that the indicated code is still safe. In the 
next chapter, we’ll jump into concurrency in Rust, and see how you can get 
all those cores on your shiny new computer to pull in the same direction!
Intermediate Rust (Early Access) © 2022 by Jon Gjengset
```
*   *Audit Focus:* Compare our Rust codebase directly to the advanced idioms extracted above. Analyze memory alignment, trait bounds, type states, error hierarchies, crate visibility, macro hygiene, async pinning/runtimes, unsafe sound invariants, and concurrent synchronization primitives.

### Phase 10: Concurrency (and Parallelism)
*   *Chapter-Specific Concluding Summary:*
```text
Summary
In this chapter, we first covered common correctness and performance pit -
falls in concurrent Rust, and some of the high-level concurrency patterns 
that successful concurrent applications tend to use to work around them. 
We also explored how asynchronous Rust enables concurrency without par -
allelism, and how to explicitly introduce parallelism in asynchronous Rust 
code. We then dove deeper into Rust’s many different lower-level concur-
rency primitives, including how they work, how they differ, and what they’re 
all for. Finally, we explored techniques for writing better concurrent code 
and looked at tools like Loom and TSan that can help you vet that code. In 
the next chapter we’ll continue our journey through the lower levels of Rust 
by digging into foreign function interfaces, which allow Rust code to link 
directly against code written in other languages.
Intermediate Rust (Early Access) © 2022 by Jon Gjengset
```
*   *Audit Focus:* Compare our Rust codebase directly to the advanced idioms extracted above. Analyze memory alignment, trait bounds, type states, error hierarchies, crate visibility, macro hygiene, async pinning/runtimes, unsafe sound invariants, and concurrent synchronization primitives.

### Phase 11: Foreign Function Interfaces
*   *Chapter-Specific Concluding Summary:*
```text
Summary
In this chapter, we’ve covered how to use the extern keyword to call out of 
Rust into external code, as well as how to use it to make Rust code accessible 
to external code. We’ve also discussed how to align Rust types with types on 
the other side of the FFI boundary, and some of the common pitfalls in try -
ing to get code written in two different languages to mesh well. Finally, we 
talked about the bindgen and cbindgen tools, which make the experience of 
keeping FFI bindings up to date much more pleasant. In the next chapter, 
we’ll look at how to use Rust in more restricted environments, like embed -
ded devices, where the standard library may not be available, and where 
even a simple operation like allocating memory may not be possible.
Intermediate Rust (Early Access) © 2022 by Jon Gjengset 
Intermediate Rust (Early Access) © 2022 by Jon Gjengset
```
*   *Audit Focus:* Compare our Rust codebase directly to the advanced idioms extracted above. Analyze memory alignment, trait bounds, type states, error hierarchies, crate visibility, macro hygiene, async pinning/runtimes, unsafe sound invariants, and concurrent synchronization primitives.

---

## MANDATORY DELIVERABLE: CRITICAL PLAN & AUTOMATED CI/CD ENFORCEMENT
Perform your codebase discovery through the lens of this work and map how we worked to build on this knowledge. Once complete, you must compile and deliver:

1. **Executive Summary & Lens Review:** An evaluation of the codebase and application specifically through the lens of this work, detailing how we built on this knowledge to benefit the application and codebase.
2. **Phase Audit Matrix:** A table mapping *Phase* | *Citations/Files* | *Identified Gaps, Risks, or Overlaps* | *Priority (H/M/L)*.
3. **Critical Adjustment Plan:** If any critical gaps or boundary overlaps are identified, form a full, actionable plan to adjust and partition roles and responsibilities.
4. **Automated CI/CD Enforcement Script:** Write a fully functional automated validation script (e.g., Python validation check, Bash Git-hook, custom `cargo clippy` or `cargo deny` rule, or unit test assertions) that can be integrated directly into our current build scripts and CI/CD deployment pipelines to continuously maintain and enforce these rules!

---

**Begin your audit as an exhaustive, diagnostic fact-gathering and research expedition by scanning the workspace to map our active directories, configuration files, and dependencies—focusing exclusively on the core production scope: apps/engine, apps/api, apps/app, apps/adapters, apps/scheduler, and crates/, while ignoring sim-*, crm, meet, posthog, sign, website, auto, and cal. For every phase, look for full systemic implementations (such as complete event stream lifecycles) rather than checking the box with single examples. Identify any correct 'Golden Blueprints' in our codebase that we should enforce universally. Compile our [COMPLETED], [PARTIAL], or [NOT STARTED] coverage status for Phase 1 and Phase 2, citing exact file paths and lines of code before outlining any plans.**
