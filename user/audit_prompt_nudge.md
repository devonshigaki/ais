# Architectural Audit Prompt: Nudge (Richard Thaler & Cass Sunstein)

Copy and paste this codebase-agnostic prompt into your developer agent's session to conduct a systematic, phase-by-phase Choice Architecture and Ergonomic API Usability audit.

---

## CONTEXT & OBJECTIVE
You are an expert API Designer, Developer Ergonomics Engineer, and Quality Assurance Architect. Your task is to audit our codebase against the principles of choice architecture, nudge designs, and cognitive/human-error prevention established in Thaler and Sunstein's *Nudge*.

Your goal is to inspect the codebase and answer these three questions for each phase:
1. **Current Coverage:** What are we doing in our codebase to cover this Phase? (Cite specific directories, files, or packages).
2. **Gaps & Overlaps:** Are there gaps (e.g., fragile API parameter defaults, complex option blocks, silent error tolerance, high-risk configurations) or overlaps (e.g., redundant validation patterns, duplicate default blocks, conflicting config schemas)?
3. **Actionable Recommendations:** How should we complete this Phase to simplify developer interfaces, eliminate error-prone entry points, and enforce safe-by-default execution?

---

---

## MANDATORY PRE-REQUISITE: READ THE ENTIRE BOOK
Before starting your codebase audit, you MUST read and analyze the full text of the sibling book PDF file to obtain complete context, grasp the subtle mathematical/neuromodulatory/logical models, and preserve absolute architectural fidelity.

*   **Target PDF File Name:** `Nudge.pdf`
*   **Target PDF Path:** `/Users/devonshigaki/Desktop/Books/Nudge.pdf`

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

### Phase 1: Part I Blunders: Biases and Blunders (Compile-Time Validation)
*   *Chapter-Specific Concluding Summary:*
```text
These two figures capture the key insight that behavioral economists
have borrowed from psychologists. Normally the human mind works re-
markably well. We can recognize people we have not seen in years, under-
stand the complexities of our native language, and run down a flight of
stairs without falling. Some of us can speak twelve languages, improve the
fanciest computers, and/or create the theory of relativity. However, even
Einstein would probably be fooled by those tables. That does not mean
something is wrong with us as humans, but it does mean that our under-
standing of human behavior can be improved by appreciating how people
systematically go wrong.
To obtain that understanding, we need to explore some aspects of hu-
man thinking. Knowing something about the visual system allowed Roger
Shepard (1990), a psychologist and artist, to draw those deceptive tables.
He knew what to draw to lead our mind astray. Knowing something about
the cognitive system has allowed others to discover systematic biases in the
way we think.
How We Think: Two Systems
The workings of the human brain are more than a bit befuddling.
How can we be so ingenious at some tasks and so clueless at others? Bee-
thoven wrote his incredible ninth symphony while he was deaf, but we
would not be at all surprised if we learned that he often misplaced his
house keys. How can people be simultaneously so smart and so dumb?
Many psychologists and neuroscientists have been converging on a de-
scription of the brain’s functioning that helps us make sense of these seem-
ing contradictions. The approach involves a distinction between two kinds
of thinking, one that is intuitive and automatic, and another that is reflec-
tive and rational.1 We will call the first the Automatic System and the sec-
ond the Refl
```
*   *Audit Focus:* Compare our codebase directly to the choice architecture guidelines, rules, and ergonomics extracted from the actual PDF above. Review compile-time type enforcements, connection thresholds, default function signatures, and resource stewardship.

### Phase 2: Part I Boundaries: Resisting Temptation (Resource Limits & Enforcements)
*   *Chapter-Specific Concluding Summary:*
```text
tactile information (they are not speed bumps) but rather just send a visual
signal to drivers. When the stripes first appear, they are evenly spaced, but
as drivers reach the most dangerous portion of the curve, the stripes get
closer together, giving the sensation that driving speed is increasing (see
Figure 1.5). One’s natural instinct is to slow down. When we drive on this
familiar stretch of road, we find that those lines are speaking to us, gently
urging us to touch the brake before the apex of the curve. We have been
nudged.
BIASES AND BLUNDERS 39
```
*   *Audit Focus:* Compare our codebase directly to the choice architecture guidelines, rules, and ergonomics extracted from the actual PDF above. Review compile-time type enforcements, connection thresholds, default function signatures, and resource stewardship.

### Phase 3: Part I Interface: Choice Architecture (Poka-Yoke Defaults)
*   *Chapter-Specific Concluding Summary:*
```text
81
Early in Thaler’s career, he was teaching a class on managerial de-
cision making to business school students. Students would sometimes
leave class early to go for job interviews (or a golf game) and would try to
sneak out of the room as surreptitiously as possible. Unfortunately for
them, the only way out of the room was through a large double door in the
front, in full view of the entire class (though not directly in Thaler’s line of
sight). The doors were equipped with large, handsome wood handles, ver-
tically mounted cylindrical pulls about two feet in length. When the stu-
dents came to these doors, they were faced with two competing instincts.
One instinct says that to leave a room you push the door. The other in-
stinct says, when faced with large wooden handles that are obviously de-
signed to be grabbed, you pull. It turns out that the latter instinct trumps
the former, and every student leaving the room began by pulling on the
handle. Alas, the door opened outward.
At one point in the semester, Thaler pointed this out to the class, as one
embarrassed student was pulling on the door handle while trying to escape
the classroom. Thereafter, as a student got up to leave, the rest of the class
would eagerly wait to see whether the student would push or pull. Amaz-
ingly, most still pulled! Their Automatic Systems triumphed; the signal
emitted by that big wooden handle simply could not be screened out.
(And when Thaler would leave that room on other occasions, he sheep-
ishly found himself pulling too.)
Those doors are bad architecture because they violate a simple psycho-
5
CHOICE ARCHITECTURE
```
*   *Audit Focus:* Compare our codebase directly to the choice architecture guidelines, rules, and ergonomics extracted from the actual PDF above. Review compile-time type enforcements, connection thresholds, default function signatures, and resource stewardship.

### Phase 4: Part II Savings: Save More Tomorrow (Automatic Reclamation)
*   *Chapter-Specific Concluding Summary:*
```text
PART
II
MONEY
Not surprisingly, Humans differ dramatically from Econs in how
they deal with money. Econs are sensible spenders and savers. They put
money away for a rainy day, and for retirement, and they invest that money
as if they had mbas. When they borrow, Econs have no trouble choosing
between ﬁxed- and variable-rate mortgages, and they pay their credit card
bills on time every month. If you are an Econ, you can skip this section of
the book, unless you want to understand the behavior of your spouse, kids,
and other Humans. A major goal of the next four chapters is to explore
how people can do a better job at the difﬁcult tasks of saving, investing,
and borrowing. We also offer some suggestions about how private and
public institutions might nudge people in directions that will make them a
bit wealthier and more secure.
```
*   *Audit Focus:* Compare our codebase directly to the choice architecture guidelines, rules, and ergonomics extracted from the actual PDF above. Review compile-time type enforcements, connection thresholds, default function signatures, and resource stewardship.

### Phase 5: Part II Allocation: Naïve Investing (Asset Allocations & Partitioning)
*   *Chapter-Specific Concluding Summary:*
```text
the next chapter. Here let’s see how we can help nudge the people who are
saving too little.
One indication that people need help in picking a savings rate and don’t
realize that they need the help is that most people spend very little time on
this important ﬁnancial decision. One survey found that 58 percent spent
less than one hour determining both their contribution rate and invest-
ment decisions.11 Most people spend more time than that picking a new
tennis racket or television set. Apparently, many people are using some
simple shortcuts. In many plans, participants are asked to state a desired
savings rate as a percentage of pay. Many people simply pick a “round num-
ber,” typically 5, 10, or 15 percent of income. Of course, there is no sensi-
ble reason why the correct percentage of your income to save would be an
exact multiple of 5.
Another common rule of thumb is to contribute to a retirement account
the minimum amount necessary to get the full employer match. If the em-
ployer matches employees’ contributions up to 6 percent of pay, then
many employees contribute 6 percent. If participants are behaving this
way, then ﬁrms wanting to encourage employee savings might alter their
matching formula to help workers. Changing the match formula from 50
percent on the ﬁrst 6 percent of pay to 30 percent on the ﬁrst 10 percent of
pay would probably increase contribution rates. Those who use the match
threshold as a rule of thumb would save more with a higher matching
threshold. And by picking a round number as the threshold, the company
would nudge those who use the “multiple of 5” heuristic.
Education
What else can employers do, if they want more employees to en-
roll in retirement plans, contribute an amount that will build a reasonable
retirement nest egg, and alloca
```
*   *Audit Focus:* Compare our codebase directly to the choice architecture guidelines, rules, and ergonomics extracted from the actual PDF above. Review compile-time type enforcements, connection thresholds, default function signatures, and resource stewardship.

---

## MANDATORY DELIVERABLE: CRITICAL PLAN & AUTOMATED CI/CD ENFORCEMENT
Perform your codebase discovery through the lens of this work and map how we worked to build on this knowledge. Once complete, you must compile and deliver:

1. **Executive Summary & Lens Review:** An evaluation of the codebase and application specifically through the lens of this work, detailing how we built on this knowledge to benefit the application and codebase.
2. **Phase Audit Matrix:** A table mapping *Phase* | *Citations/Files* | *Identified Gaps, Risks, or Overlaps* | *Priority (H/M/L)*.
3. **Critical Adjustment Plan:** If any critical gaps or boundary overlaps are identified, form a full, actionable plan to adjust and partition roles and responsibilities.
4. **Automated CI/CD Enforcement Script:** Write a fully functional automated validation script (e.g., Python validation check, Bash Git-hook, custom linter rule, or unit test assertions) that can be integrated directly into our current build scripts and CI/CD deployment pipelines to continuously maintain and enforce these rules!
