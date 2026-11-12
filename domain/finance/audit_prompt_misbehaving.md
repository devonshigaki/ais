# Architectural Audit Prompt: Misbehaving (Richard Thaler)

Copy and paste this codebase-agnostic prompt into your developer agent's session to conduct a systematic, phase-by-phase behavioral analysis of system biases, anomalies, and runtime "irrationalities".

---

## CONTEXT & OBJECTIVE
You are an expert Performance Optimizer, Code Quality Craftsman, and System Behavioral Analyst. Your task is to audit our codebase against the behavioral economics, systemic biases, and cognitive/execution anomalies identified in Richard Thaler's *Misbehaving*.

Your goal is to inspect the codebase and answer these three questions for each phase:
1. **Current Coverage:** What are we doing in our codebase to cover this Phase? (Cite specific directories, files, or packages).
2. **Gaps & Overlaps:** Are there gaps (e.g., hidden lock contention, unoptimized cache evictions, hardcoded timeouts, fragile error thresholds) or overlaps (e.g., redundant caching layers, competing state-flushing schedules, duplicate validations)?
3. **Actionable Recommendations:** How should we complete this Phase to eliminate code anomalies, reduce resource consumption, and establish safe default pathways?

---

---

## MANDATORY PRE-REQUISITE: READ THE ENTIRE BOOK
Before starting your codebase audit, you MUST read and analyze the full text of the sibling book PDF file to obtain complete context, grasp the subtle mathematical/neuromodulatory/logical models, and preserve absolute architectural fidelity.

*   **Target PDF File Name:** `Misbehaving.pdf`
*   **Target PDF Path:** `/Users/devonshigaki/Desktop/Books/Misbehaving.pdf`

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

### Phase 1: Part I SIFs: Supposedly Irrelevant Factors (Implicit Configurations)
*   *Chapter-Specific Concluding Summary:*
```text
more education, being taught in a smaller class or by a better teacher, being
given management consulting services, being given help to find a job, being
sentenced to jail, moving to a lower-poverty neighborhood, receiving health
insurance from Medicaid, and so forth. These studies show that one can learn
a lot about the world without imposing optimizing models, and in some cases
provide credible evidence against which to test such models and see if they
match actual human responses.
For much of economic theory, the assumption that all the agents are
optimizing is not a critical one, even if the people under study are not experts.
For example, the prediction that farmers use more fertilizer if the price falls is
safe enough, even if many farmers are slow to change their practices in
response to market conditions. The prediction is safe because it is imprecise:
all that is predicted is the direction of the effect. This is equivalent to a
prediction that when apples fall off the tree, they fall down rather than up.
The prediction is right as far as it goes, but it is not exactly the law of gravity.
Economists get in trouble when they make a highly specific prediction that
depends explicitly on everyone being economically sophisticated. Let’s go
back to the farming example. Say scientists learn that farmers would be better
off using more or less fertilizer than has been the tradition. If everyone can be
assumed to get things right as long as they have the proper information, then
there is no appropriate policy prescription other than making this information
freely available. Publish the findings, make them readily available to farmers,
and let the magic of markets take care of the rest.
Unless all farmers are Econs, this is bad advice. Perhaps multinational food
companies
```
*   *Audit Focus:* Compare our codebase behavior directly to the behavioral and performance constraints extracted from the actual PDF above. Review SIF configurations, custom vs. standard library dependencies, thread budgets, worker schedulers, queue fairness, and resource stewardship.

### Phase 2: Part I Bias: The Endowment Effect (Not Invented Here Syndrome)
*   *Chapter-Specific Concluding Summary:*
```text
3
The List
T
he discrepancy between buying and selling prices got my mind wandering.
What else do people do that is inconsistent with the economists’ model of
rational choice? Once I started paying attention, so many examples cropped
up that I started a list on the blackboard in my office. Here are a few that
describe the behavior of some of my friends:
•  
Jeffrey and I somehow get two free tickets to a professional basketball
game in Buffalo, normally an hour and a half drive from where we live in
Rochester. The day of the game there is a big snowstorm. We decide not to
go, but Jeffrey remarks that, had we bought the (expensive) tickets, we
would have braved the blizzard and attempted to drive to the game.
•  
Stanley mows his lawn every weekend and it gives him terrible hay fever. I
ask Stan why he doesn’t hire a kid to mow his lawn. Stan says he doesn’t
want to pay the $10. I ask Stan whether he would mow his neighbor’s lawn
for $20 and Stan says no, of course not.
•  
Linnea is shopping for a clock radio. She finds a model she likes at what
her research has suggested is a good price, $45. As she is about to buy it,
the clerk at the store mentions that the same radio is on sale for $35 at new
branch of the store, ten minutes away, that is holding a grand opening
sale. Does she drive to the other store to make the purchase?
On a separate shopping trip, Linnea is shopping for a television set and
finds one at the good price of $495. Again the clerk informs her that the
same model is on sale at another store ten minutes away for $485. Same
question . . . but likely different answer.
```
*   *Audit Focus:* Compare our codebase behavior directly to the behavioral and performance constraints extracted from the actual PDF above. Review SIF configurations, custom vs. standard library dependencies, thread budgets, worker schedulers, queue fairness, and resource stewardship.

### Phase 3: Part II Value: Bargains and Rip-offs (Resource Trade-offs)
*   *Chapter-Specific Concluding Summary:*
```text
(not a purchase), the annual “maintenance fee” is a nuisance, but future
vacations at the property are “free.” Whether such an investment makes sense
for a family will depend, in part, on how painful it is for them to spend
money on vacations. But such investments should be seen for what they are:
a way to disguise the cost of taking a vacation.
Costco, the discount retailer mentioned in the previous chapter, also uses a
version of this strategy. In order to shop at Costco a customer must become a
“member,” which currently costs a household $55 a year. It seems likely that
members view the annual fee as an “investment” and make no attempt to
allocate that cost over the various purchases they make during the year.
Rather, it serves as a sunk cost, offering up yet another reason to shop at
Costco. Similarly, Amazon charges customers $99 a year to become a “prime
member” which entitles them to “free” shipping. Again, the cost of the
membership may well be viewed as an investment that does not “count”
toward the cost of a particular purchase.
I
t is time for two confessions. Although I mostly advocate for thinking like
an Econ, when it comes to mental accounting I have some notably Human
tendencies. I am usually pretty good about ignoring sunk costs, especially if
the sunk costs are purely monetary in nature. But like most people, if I have
put a lot of work into some project I can find it difficult to give it up, even if
it is clearly the right thing to do. In writing this book, for instance, my
strategy for getting a first draft done was to keep writing and not worry about
whether a particular passage or section would make the final cut. This
process did produce a first draft, but one that was obviously too long. Some
bits were going to have to be cut, and I fielded sugge
```
*   *Audit Focus:* Compare our codebase behavior directly to the behavioral and performance constraints extracted from the actual PDF above. Review SIF configurations, custom vs. standard library dependencies, thread budgets, worker schedulers, queue fairness, and resource stewardship.

### Phase 4: Part II Sunk Costs: Letting Go of Bad Architectures
*   *Chapter-Specific Concluding Summary:*
```text
blatant a violation of the rule that money is fungible as one can find. The
money in either pocket will spend equally well.
Taking money off your colleagues is fun,
†
 but far from scientific. So Eric
Johnson, a marketing professor now at Columbia, and I started work on a real
paper. This is the one I mentioned in the preface that took a while to satisfy
Amos. Essentially, we wanted to replicate in the lab what I had seen at the
poker table. But first we had to address the problem that had originally
pushed Kahneman and Tversky to run experiments using hypothetical
questions. How can you run ethical experiments in which subjects can lose
money, and how can you get approval from the university review board that
oversees such experiments? We solved this problem by having subjects
answer a series of choices between sure things and gambles, some of which
involved gains and others losses, and truthfully told them that one of the
choices would be selected at random to “count” for the study. But not every
gamble was equally likely to be chosen, and by making the favorable
gambles the ones that were more likely to be played, we were able to assure
the subjects that the chance of losing money was tiny, although we made it
clear that we fully intended to collect from anyone who did lose money. If
they wished, they could pay off their debt by doing some research assistance.
In the end no one lost money, so we did not have to try to collect.
Here are three of the questions that were included in our study. The
numbers in brackets are the percentages of subjects who chose the selected
answer. In this example, an Econ who was risk averse would choose the sure
thing in each of these problems, since in every case the expected outcome of
the gamble is equal to the sure thing.
P
ROBLEM
 1
```
*   *Audit Focus:* Compare our codebase behavior directly to the behavioral and performance constraints extracted from the actual PDF above. Review SIF configurations, custom vs. standard library dependencies, thread budgets, worker schedulers, queue fairness, and resource stewardship.

### Phase 5: Part III Limits: Willpower (Execution Bottlenecks)
*   *Chapter-Specific Concluding Summary:*
```text
along and friends suggest going out to a new pizza place featuring craft beers,
we end up eating and drinking more than we would have predicted on
Sunday, or even on Wednesday before arriving at the restaurant with its
tempting aromas wafting from the wood-burning oven, not to mention an
intriguing list of special brews to sample. For such cases we may need a
planner to have established a rule—no midweek beer and pizza outings—and
then to think of a way of enforcing that rule.
In the time since I first removed that bowl of cashews, behavioral scientists
have learned a lot about self-control problems. This knowledge is proving
important in dealing with many of society’s biggest problems, as we will see
later on.
________________
*
   In truth, Odysseus was not clever enough to think up this plan himself. He got some good advice
from Circe, a goddess who specialized in herbs and drugs. Go figure.
†
   Some researchers have tried a version of the marshmallow/Oreo experiment on animals. Most go for
the immediate reward, but one particularly clever African gray parrot named Griffen was shown to
display better self-control than most preschoolers (Zielinski, 2014).
‡
   The two-system model articulated by Kahneman in 
Thinking, Fast and Slow
 was not the way he and
Tversky originally thought about their research. One of Danny’s main reasons for writing the book was
because he thought recasting their original work using the framework of a fast, automatic system and a
slow, reflexive system offered an insightful new perspective on their earlier findings.
§
   Tom Schelling started writing on this topic soon after I did. Our views were very much in the same
spirit, but he was less convinced than I was that the longsighted set of preferences is more likely to be
“right.” See, for example, Schelling (1984).
¶
   Amos always referred to the planner as a female. I will do likewise in his honor. And since men are
generally more like doers than women are, I will use the male pronoun for doers. Just call me a sexist.
#
   For the sake of simplicity, I will ignore the possibility that the one-energy-bar-per-day diet makes
the doers increasingly hungry as time passes.
**
   Such technology does exist. The Kitchen Safe (kitchensafe.com) is a plastic container than the user
can lock for any period of time. The manufacturers recommend it for anything tempting, from candy to
smartphones to car keys. I received one from a thoughtful student. Naturally it was filled with cashews.
In a world of Econs, there would be no demand for such a product.
```
*   *Audit Focus:* Compare our codebase behavior directly to the behavioral and performance constraints extracted from the actual PDF above. Review SIF configurations, custom vs. standard library dependencies, thread budgets, worker schedulers, queue fairness, and resource stewardship.

### Phase 6: Part III Schedulers: The Planner and the Doer
*   *Chapter-Specific Concluding Summary:*
```text
IV.
WORKING WITH DANNY:
1984–85
After our year in Stanford, Amos and Danny decided to immigrate to North
America. Amos stayed on at the Stanford psychology department and Danny
moved to the psychology department at the University of British Columbia in
Vancouver. Part of the allure of UBC was that Amos and Danny would be a
two-hour flight away from each other and in the same time zone. They
continued to work together, talking daily and visiting each other often.
Because we all started new jobs the same year, we were on the same
sabbatical schedule. In 1984–85 I would get my first one, and Amos and
Danny would be on leave too. Our year at Stanford had been so
transformative for me that when it came time to think about my research
leave, I naturally hoped to hook up with one or both of them. After various
machinations, I ended up in Vancouver with Danny. Amos, meanwhile,
headed off to Israel.
I got an office at the UBC business school, which was a good place for me
to hang out since it had an excellent finance department and I was in the
midst of trying to learn more about that field. But the main thing I did that
year was work with Danny and his collaborator, the environmental economist
Jack Knetsch, who taught at nearby Simon Frasier University. Like the year
at Stanford, this year in Vancouver offered me the rare opportunity for full
immersion in research. Apart from the year in Stanford, it would be the most
productive year of my life.
```
*   *Audit Focus:* Compare our codebase behavior directly to the behavioral and performance constraints extracted from the actual PDF above. Review SIF configurations, custom vs. standard library dependencies, thread budgets, worker schedulers, queue fairness, and resource stewardship.

### Phase 7: Part IV Fairness: What Seems Fair? (Fair Queue Allocation)
*   *Chapter-Specific Concluding Summary:*
```text
students were not wild about these pens, but the results were about the same.
The number of trades varied between four and five, and the ratio of selling to
buying prices was again in the neighborhood of 2:1.
We ran numerous versions of these experiments to answer the complaints
of various critics and journal referees, but the results always came out the
same. Buyers were willing to pay about half of what sellers would demand,
even with markets and learning. Again we see that losses are roughly twice as
painful as gains are pleasurable, a finding that has been replicated numerous
times over the years.
T
he endowment effect experiments show that people have a tendency to
stick with what they have, at least in part because of loss aversion. Once I
have that mug, I think of it as mine. Giving it up would be a loss. And the
endowment effect can kick in very fast. In our experiments, the subjects had
“owned” that mug for a few minutes before the trading started. Danny liked
to call this the “instant endowment effect.” And while loss aversion is
certainly part of the explanation for our findings, there is a related
phenomenon: inertia. In physics, an object in a state of rest stays that way,
unless something happens. People act the same way: they stick with what
they have unless there is some good reason to switch, or perhaps despite there
being a good reason to switch. Economists William Samuelson and Richard
Zeckhauser have dubbed this behavior “status quo bias.”
Loss aversion and status quo bias will often work together as forces that
inhibit change. Think of people who lose their jobs because a plant or a mine
closes down, and in order to find work, they would have to both take up
another line of work and give up the friends, family, and home to which they
have become att
```
*   *Audit Focus:* Compare our codebase behavior directly to the behavioral and performance constraints extracted from the actual PDF above. Review SIF configurations, custom vs. standard library dependencies, thread budgets, worker schedulers, queue fairness, and resource stewardship.

### Phase 8: Part VI Concurrency: The Beauty Contest (Predictive Lock Contentions)
*   *Chapter-Specific Concluding Summary:*
```text
conclusions, one of which was
gleefully heralded by critics as the “Shiller Killer.” (You may recall that one
of those papers, written by Allan Kleidon, was presented at the Chicago
conference discussed in 
chapter 17
.)
Academic economists still quibble about the right way to conduct Shiller’s
test. But I believe that the debate was effectively settled a few years later, on
Monday, October 19, 1987, and the days that surrounded it. That Monday,
stock prices fell dramatically all around the world. The carnage started in
Hong Kong and moved west, as markets opened in Europe and then the
United States. In New York, prices fell over 20%, after having already fallen
more than 5% the previous Friday. Crucial for our purposes, Monday the
19th was a day without any important news, financial or otherwise. No war
started, no political leader was assassinated, and nothing else of note
occurred. (For the sake of comparison, the U.S. stock market dropped 4.4%
the day after the Japanese bombed Pearl Harbor.) Yet prices were falling
precipitously all around the world. No one could say why. The volatility
continued for the next few days. In the United States, the S&P 500 index of
large company stocks rebounded a robust 5.3% on Tuesday, jumped another
9.1% on Wednesday, only to crash by 8.3% on Monday the 26th. The
headline in the 
Wall Street Journal
 at the end of that month should have been,
“Robert Shiller proven right: financial markets are too volatile.” In a rational
world, prices only change in reaction to news, and during that week, the only
news was that prices were moving crazily.
If prices are too variable, then they are in some sense “wrong.” It is hard to
argue that the price at the close of trading on Thursday, October 15, and the
price at the close of trading the follow
```
*   *Audit Focus:* Compare our codebase behavior directly to the behavioral and performance constraints extracted from the actual PDF above. Review SIF configurations, custom vs. standard library dependencies, thread budgets, worker schedulers, queue fairness, and resource stewardship.

### Phase 9: Part VIII Stewardship: Save More Tomorrow (Automatic Reclamation)
*   *Chapter-Specific Concluding Summary:*
```text
conclusion that presumed consent was not, in fact, the best
policy. Instead we liked a variant that had recently been adopted by the state
of Illinois and is also used in other U.S. states. When people renew their
driver’s license, they are asked whether they wish to be an organ donor.
Simply asking people and immediately recording their choices makes it easy
to sign up.
†
 In Alaska and Montana, this approach has achieved donation
rates exceeding 80%. In the organ donation literature this policy was dubbed
“mandated choice” and we adopted that term in the book.
This choice of terminology was unfortunate, as I learned later. Some time
after the book was published, I wrote a column in the 
New York Times
 about
organ donations and advocated the Illinois policy, which I continued to call
“mandated choice.” A few weeks later, someone on the editorial board of
USA Today
 called me to talk about the policy because their newspaper was
going to endorse it. A couple days later, I got an urgent phone call from the
editorial writer. It turns out she had called the state official in charge of this
policy, who has the title secretary of state, and he firmly denied that any such
policy existed. I was mystified. I had recently renewed my driver’s license
and was duly asked whether I wanted to be an organ donor. (I said yes.) A
few more phone calls solved the mystery. The secretary of state, Jesse White,
objected to the word “mandated.” He said that no one was required to do
anything, and technically he was right. When asked to be a donor, if someone
refuses to answer or remains mute, the official at the Department of Motor
Vehicles just takes that as a no.
It turns out that Jesse White is a smart politician, and being a smart
politician he realized that voters do not like mandates.
‡
```
*   *Audit Focus:* Compare our codebase behavior directly to the behavioral and performance constraints extracted from the actual PDF above. Review SIF configurations, custom vs. standard library dependencies, thread budgets, worker schedulers, queue fairness, and resource stewardship.

---

## MANDATORY DELIVERABLE: CRITICAL PLAN & AUTOMATED CI/CD ENFORCEMENT
Perform your codebase discovery through the lens of this work and map how we worked to build on this knowledge. Once complete, you must compile and deliver:

1. **Executive Summary & Lens Review:** An evaluation of the codebase and application specifically through the lens of this work, detailing how we built on this knowledge to benefit the application and codebase.
2. **Phase Audit Matrix:** A table mapping *Phase* | *Citations/Files* | *Identified Gaps, Risks, or Overlaps* | *Priority (H/M/L)*.
3. **Critical Adjustment Plan:** If any critical gaps or boundary overlaps are identified, form a full, actionable plan to adjust and partition roles and responsibilities.
4. **Automated CI/CD Enforcement Script:** Write a fully functional automated validation script (e.g., Python validation check, Bash Git-hook, custom linter rule, or unit test assertions) that can be integrated directly into our current build scripts and CI/CD deployment pipelines to continuously maintain and enforce these rules!
