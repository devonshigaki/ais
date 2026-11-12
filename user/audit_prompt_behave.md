# Architectural Audit Prompt: Systems Sociology & Human Factors (Reference: Behave by Robert Sapolsky)

Copy and paste this codebase-agnostic prompt into your developer agent's session to conduct a systematic, phase-by-phase human-factors, operability, and systems sociology audit.

---

## CONTEXT & OBJECTIVE
You are an expert Systems Sociologist, Human Factors Engineer, and Conway's Law Analyst. Your task is to audit our codebase and development workflow against the physiological, neurological, and evolutionary patterns of human behavior described in Robert Sapolsky's *Behave: The Biology of Humans at Our Best and Worst*, mapping them systematically to **Conway's Law, Developer Cognitive Load, and Human-Fault Prevention**.

Your goal is to inspect the codebase and answer these three questions for each phase:
1. **Current Coverage:** What are we doing in our codebase to cover this Phase? (Cite specific directories, files, or packages).
2. **Gaps & Overlaps:** Are there gaps (e.g., highly complex paths causing developer fatigue, hidden single-points-of-failure, poor system visibility) or overlaps (e.g., duplicate validation, multiple teams editing the same file, conflicting deployment configurations)?
3. **Actionable Recommendations:** How should we complete this Phase to reduce developer friction, mitigate the probability of human errors during outages, and cleanly partition roles and responsibilities?

---

---

## MANDATORY PRE-REQUISITE: READ THE ENTIRE BOOK
Before starting your codebase audit, you MUST read and analyze the full text of the sibling book PDF file to obtain complete context, grasp the subtle mathematical/neuromodulatory/logical models, and preserve absolute architectural fidelity.

*   **Target PDF File Name:** `Behave.pdf`
*   **Target PDF Path:** `/Users/devonshigaki/Desktop/Books/Behave.pdf`

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

### Phase 1: The Behavior
*   *Chapter-Specific Concluding Summary:*
```text
49
I
THE FRONTAL CORTEX
’ve spent decades studying the hippocampus. It’s been good to me;
I’d like to think I’ve been the same in return. Yet I think I might have
made the wrong choice back then—maybe I should have studied the
frontal cortex all these years. Because it’s the most interesting part of the
brain.
What does the frontal cortex do? Its list of expertise includes
working memory, executive function (organizing knowledge
strategically, and then initiating an action based on an executive
decision), gratification postponement, long-term planning, regulation of
emotions, and reining in impulsivity.35
This is a sprawling portfolio. I will group these varied functions
under a single definition, pertinent to every page of this book: the frontal
cortex makes you do the harder thing when it’s the right thing to do.
To start, here are some important features of the frontal cortex:
It’s the most recently evolved brain region, not approaching full
splendor until the emergence of primates; a disproportionate
percentage of genes unique to primates are active in the frontal
cortex. Moreover, such gene expression patterns are highly
individuated, with greater interindividual variability than average
levels of whole-brain differences between humans and chimps.
The human frontal cortex is more complexly wired than in other
apes and, by some definitions as to its boundaries,
proportionately bigger as well.36
The frontal cortex is the last brain region to fully mature, with
the most evolutionarily recent subparts the very last. Amazingly,
it’s not fully online until people are in their midtwenties. You’d
better bet this factoid will be relevant to the chapter about
adolescence.
```
*   *Audit Focus:* Compare our system metrics, workflows, alert patterns, boundaries, and operability directly to the sociological and biological constraints extracted from the actual PDF pages above.

### Phase 2: One Second Before
*   *Chapter-Specific Concluding Summary:*
```text
CONCLUSIONS
his completes our introduction to the nervous system and its role in
pro- and antisocial behaviors. It was organized around three
themes: the hub of fear, aggression, and arousal centered in the
amygdala; the hub of reward, anticipation, and motivation of the
dopaminergic system; and the hub of frontal cortical regulation and
restraint of behavior. Additional brain regions and neurotransmitters will
be introduced in subsequent chapters. Amid this mountain of
information, be assured that the key brain regions, circuits, and
neurotransmitters will become familiar as the book progresses.
Hang on. So what does this all mean? It’s useful to start with three
things that this information doesn’t mean:
1. First, there’s the lure of needing neurobiology to
confirm the obvious. Someone claims that, for example,
their crappy, violent neighborhood leaves them so
anxious that they can’t function effectively. Toss them in
a brain scanner and flash pictures of various
neighborhoods; when their own appears, the amygdala
explodes into activity. “Ah,” it is tempting to conclude,
“we’ve now proven that the person really does feel
frightened.”
It shouldn’t require neuroscience to validate
someone’s internal state. An example of this
fallacy was reports of atrophy of the hippocampus in
combat vets suffering from PTSD; this was in
accord with basic research (including from my lab)
showing that stress can damage the hippocampus.
The hippocampal atrophy in PTSD got a lot of play
in Washington, helping to convince skeptics that
PTSD is an organic disorder rather than neurotic
malingering. It struck me that if it took brain scans
to convince legislators that there’s something
```
*   *Audit Focus:* Compare our system metrics, workflows, alert patterns, boundaries, and operability directly to the sociological and biological constraints extracted from the actual PDF pages above.

### Phase 3: Seconds to Minutes Before
*   *Chapter-Specific Concluding Summary:*
```text
109
force studies. First they engineered the brains of male mice to express
the prairie vole version of the vasopressin receptor in their brains, and
they groomed and huddled more with familiar females (but not with
strangers). Then the scientists engineered the brains of male montane
voles to have more vasopressin receptors in the nucleus accumbens; the
males became more socially affiliative with individual females.*
What about versions of vasopressin receptor genes in other species?
When compared with chimps, bonobos have a variant associated with
more receptor expression and far more social bonding between females
and males (although, in contrast to prairie voles, bonobos are anything
but monogamous).27
How about humans? This is tough to study, because you can’t
measure these neuropeptides in tiny brain regions in humans and instead
have to examine levels in the circulation, a fairly indirect measure.
Nevertheless, these neuropeptides appear to play a role in human
pair-bonding.28 For starters, circulating oxytocin levels are elevated in
couples when they’ve first hooked up. Furthermore, the higher the levels,
the more physical affection, the more behaviors are synchronized, the
more long-lasting the relationship, and the happier interviewers rate
couples to be.
Even more interesting were studies where oxytocin (or a control
spray) was administered intranasally. In one fun study, couples had to
discuss one of their conflicts; oxytocin up their noses, and they’d be
rated as communicating more positively and would secrete less stress
hormones. Another study suggests that oxytocin unconsciously
strengthens the pair-bond. Heterosexual male volunteers, with or without
an oxytocin spritz, interacted with an attractive female researcher, doing
some nonsense task. Among men
```
*   *Audit Focus:* Compare our system metrics, workflows, alert patterns, boundaries, and operability directly to the sociological and biological constraints extracted from the actual PDF pages above.

### Phase 4: Hours to Days Before
*   *Chapter-Specific Concluding Summary:*
```text
139
Let’s add hormones to this, translating some of our stress concepts
into the language of neural plasticity. Moderate, transient stress (i.e., the
good, stimulatory stress) promotes hippocampal LTP, while prolonged
stress disrupts it and promotes LTD—one reason why cognition tanks at
such times. This is the inverted-U concept of stress writ synaptic.4
Moreover, sustained stress and glucocorticoid exposure enhance LTP
and suppress LTD in the amygdala, boosting fear conditioning, and
suppress LTP in the frontal cortex. Combining these effects—more
excitable synapses in the amygdala, fewer ones in the frontal cortex—
helps explain stress-induced impulsivity and poor emotional regulation.5
Rescued from the Trash
The notion of memory resting on the strengthening of preexisting
synapses dominates the field. But ironically, the discarded idea that
memory requires the formation of new synapses has been resuscitated.
Techniques for counting all of a neuron’s synapses show that housing
rats in a rich, stimulatory environment increases their number of
hippocampal synapses.
Profoundly fancy techniques let you follow one dendritic branch of a
neuron over time as a rat learns something. Astonishingly, over minutes
to hours a new dendritic spine emerges, followed by an axon terminal
hovering nearby; over the next weeks, they form a functioning synapse
that stabilizes the new memory (and in other circumstances, dendritic
spines retract, eliminating synapses).
Such “activity-dependent synaptogenesis” is coupled to LTP—when
a synapse undergoes LTP, the tsunami of calcium rushing into the spine
can diffuse and trigger the formation of a new spine in the adjacent
stretch of the dendritic branch.
New synapses form throughout the brain—in motor-cortex neurons
when you learn a motoric task
```
*   *Audit Focus:* Compare our system metrics, workflows, alert patterns, boundaries, and operability directly to the sociological and biological constraints extracted from the actual PDF pages above.

### Phase 5: Days to Months Before
*   *Chapter-Specific Concluding Summary:*
```text
179
in a “cognition” bucket, ignoring the impact of social and emotional
factors.
One example to be discussed in chapter 12 concerns preverbal
infants, who sure don’t grasp transitivity (if A > B, and B > C, then A >
C). Show a violation of transitivity in interactions between shapes on a
screen (shape A should knock over shape C, but the opposite occurs),
and the kid is unbothered, doesn’t look for long. But personify the shapes
with eyes and a mouth, and now heart rate increases, the kid looks longer
—“Whoa, character C is supposed to move out of character A’s way,
not the reverse.” Humans understand logical operations between
individuals earlier than between objects.10
Social and motivational state can shift cognitive stage as well.
Rudiments of ToM are more demonstrable in chimps who are interacting
with another chimp (versus a human) and if there is something
motivating—food—involved.*11
Emotion and affect can alter cognitive stage in remarkably local
ways. I saw a wonderful example of this when my daughter displayed
both ToM and failure of ToM in the same breath. She had changed
preschools and was visiting her old class. She told everyone about life in
her new school: “Then, after lunch, we play on the swings. There are
swings at my new school. And then, after that, we go inside and Carolee
reads us a story. Then, after that . . .” ToM: “play on the swings”—wait,
they don’t know that my school has swings; I need to tell them. Failure
of ToM: “Carolee reads us a story.” Carolee, the teacher at her new
school. The same logic should apply—tell them who Carolee is. But
because Carolee was the most wonderful teacher alive, ToM failed.
Afterward I asked her, “Hey, why didn’t you tell everyone that Carolee is
your teacher?” “Oh, everyone knows Carolee.” How could everyon
```
*   *Audit Focus:* Compare our system metrics, workflows, alert patterns, boundaries, and operability directly to the sociological and biological constraints extracted from the actual PDF pages above.

### Phase 6: Adolescence
*   *Chapter-Specific Concluding Summary:*
```text
conclusion is reached by an inverse of CAH, namely
androgen insensitivity syndrome (AIS, historically called “testicular
feminization syndrome”).69 A fetus is male—XY chromosomes, testes
that secrete testosterone. But a mutation in the androgen receptor makes
it insensitive to testosterone. Thus the testes can secrete testosterone till
the cows come home but there won’t be any masculinization. And often
the individual is born with a female external phenotype and is raised as a
girl. Along comes puberty, she’s not getting periods, and a trip to the
doctor reveals that the “girl” is actually a “boy” (with testes typically
near the stomach, plus a shortened vagina that dead-ends). The
individual usually continues with a female identity but is infertile as an
adult. In other words, when human males don’t experience the
organizational prenatal effects of testosterone, you get female-typical
behaviors and identification.
Between CAH and AIS, the issue seems settled—prenatal
testosterone plays a major role in explaining sex differences in
aggression and various affiliative prosocial behaviors in humans.
Careful readers may have spotted two whopping big problems with
this conclusion:70
Remember that CAH girls are born with a “something’s
very different” Post-it—the ambiguous genitalia,
typically requiring multiple reconstructive surgeries.
CAH females are not merely prenatally androgenized.
They’re also raised by parents who know something is
different, have slews of doctors mighty interested in
their privates, and are treated with all sorts of hormones.
It’s impossible to attribute the behavioral profile solely
to the prenatal androgens.
Testosterone has no effects in AIS individuals because
of the androgen receptor mutation. But doesn’t
219
testosterone have most of its fetal
```
*   *Audit Focus:* Compare our system metrics, workflows, alert patterns, boundaries, and operability directly to the sociological and biological constraints extracted from the actual PDF pages above.

### Phase 7: Back to the Crib
*   *Chapter-Specific Concluding Summary:*
```text
259
“masculinization” of the cortex. An interaction between receptor variant
and testosterone levels occurs. High basal testosterone levels do not
predict elevated levels of aggressive mood or of amygdaloid reactivity to
threatening faces in males—except in those with that variant.
Interestingly, the equivalent variant predicts aggressiveness in Akita
dogs.
How important are these findings? A key theme in chapter 4 was
how little individual differences in testosterone levels in the normal
range predict individual differences in behavior. How much more
predictability is there when combining knowledge of testosterone levels
and of receptor sensitivity? Not much. How about hormone levels and
receptor sensitivity and number of receptors? Still not much. But
definitely an improvement in predictive power.
Similar themes concern the genetics of the estrogen receptor.56 For
example, different receptor variants are associated with higher rates of
anxiety among women, but not men, and higher rates of antisocial
behavior and conduct disorder in men, but not women. Meanwhile, in
genetically manipulated mice, the presence or absence of the receptor
gene influences aggression in females . . . depending on how many
brothers there were in the litter in utero—gene/environment again. Once
again, the magnitude of these genetic influences is tiny.
Finally, there is work on genes related to glucocorticoids, particularly
regarding gene/environment interactions.57 For example, there is an
interaction between one variant of the gene for a type of receptor for
glucocorticoids (for mavens: it’s the MR receptor) and childhood abuse
in producing an amygdala that is hyperreactive to threat. Then there is a
protein called FKBP5, which modifies the activity of another type of
receptor for glucocortic
```
*   *Audit Focus:* Compare our system metrics, workflows, alert patterns, boundaries, and operability directly to the sociological and biological constraints extracted from the actual PDF pages above.

### Phase 8: Back to the Fertilized Egg
*   *Chapter-Specific Concluding Summary:*
```text
conclusions about rats don’t even hold for
humans. In some cities—Chicago, for example, circa 1970—higher
population density in neighborhoods does indeed predict more violence.
Nevertheless, some of the highest-density places on earth—Hong Kong,
Singapore, and Tokyo—have miniscule rates of violence. High-density
living is not synonymous with aggression in rats or humans.
—
The preceding sections examined the effects of living with lots of people,
and in close quarters. How about the effects of living with different kinds
of people? Diversity. Heterogeneity. Admixture. Mosaicism.
Two opposite narratives come to mind:
Mister Rogers’ neighborhood: When people of differing
ethnicities, races, or religions live together, they experience the
similarities rather than the differences and view one another as
individuals, transcending stereotypes. Trade flows, fostering
fairness and mutuality. Inevitably, dichotomies dissolve with
intermarriage, and soon you’re happily watching your grandkid
in the school play on “their” side of town. Just visualize whirled
peas.
299
Sharks versus the Jets: Differing sorts of people living in close
proximity rub, and thus abrade, elbows regularly. One side’s act
of proud cultural identification feels like a hostile dig to the other
side, public spaces become proving grounds for turf battles,
commons become tragedies.
Surprise: both outcomes occur; the final chapter explores
circumstances where intergroup contact leads to one rather than the
other. Most interesting at this juncture is the importance of the spatial
qualities of the heterogeneity. Consider a region filled with people from
Elbonia and Kerplakistan, two hostile groups, each providing half the
population. At one extreme, the land is split down the middle, each
group occupying one side,
```
*   *Audit Focus:* Compare our system metrics, workflows, alert patterns, boundaries, and operability directly to the sociological and biological constraints extracted from the actual PDF pages above.

### Phase 9: Centuries to Millennia
*   *Chapter-Specific Concluding Summary:*
```text
349
defect—that’s two years. But maybe defect, in case he cooperates . . .”
Round and round.*
If you play PD once, there is a rational solution. If you, prisoner A,
defect, your sentence averages out to one year (zero years if B
cooperates, two years if B defects); if you cooperate, the average is two
years (one year if B cooperates, three years if B defects). Thus you
should defect. In single-round versions of PD, it’s always optimal to
defect. Not very encouraging for the state of the world.
Suppose there are two rounds of PD. The optimal strategy for the
second round is just like in a single-round version—always defect. Given
that, the first-round defaults into being like a single-round game—and
thus, defect during it also.
What about a three-round game? Defect in the third, meaning that
things default into a two-round game. In which case, defect in the
second, meaning defect in the first.
It’s always optimal to defect in round Z, the final round. And thus it’s
always optimal to defect in round Z−1, and thus round Z−2. . . . In other
words, when two individuals play for a known number of rounds, the
optimal strategy precludes cooperation.
But what if the number of rounds is unknown (an “iterated” PD)?
Things get interesting. Which is when the game theorists and biologists
met.
The catalyst was political scientist Robert Axelrod of the University
of Michigan. He explained to his colleagues how PD works and asked
them what strategy they’d use in a game with an unknown number of
rounds. The strategies offered varied enormously, with some being hair-
raisingly complicated. Axelrod then programmed the various strategies
and pitted them against each other in a simulated massive round-robin
tournament. Which strategy won, was most optimal?
It was provided by a mathematician
```
*   *Audit Focus:* Compare our system metrics, workflows, alert patterns, boundaries, and operability directly to the sociological and biological constraints extracted from the actual PDF pages above.

### Phase 10: The Evolution of Behavior
*   *Chapter-Specific Concluding Summary:*
```text
399
being in a group means that someone else’s behaviors can make you look
bad.*24
This raises a larger issue, namely our sense of obligation and loyalty
to Us as a whole. At one extreme it can be contractual. This can be
literal, with professional athletes in team sports. It is expected that when
jocks sign contracts, they will play their hardest, putting the team’s
fortune above showboating. But the obligations are finite—they’re not
expected to sacrifice their lives for their team. And when athletes are
traded, they don’t serve as a fifth column, throwing games in their new
uniform to benefit their old team. The core of such a contractual
relationship is the fungibility of both employer and employee.
At the other extreme, of course, are Us memberships that are not
fungible and transcend negotiation. People aren’t traded from the Shiites
to the Sunnis, or from the Iraqi Kurds to the Sami herders in Finland. It
would be a rare Kurd who would want to be Sami, and his ancestors
would likely turn over in their graves when he nuzzled his first reindeer.
Converts are often subject to ferocious retribution by those they left—
consider Meriam Ibrahim, sentenced to death in Sudan in 2014 for
converting to Christianity—and suspicion from those they’ve joined.
With the sense of one’s lot being permanent come distinctive elements of
Us-ness. You don’t sign a faith-based baseball contract with vague
promises of a salary. But Us-ness based on sacred values, with wholes
bigger than sums of parts, where unenforceable obligations stretch across
generations, millennia, even into afterlives, where it’s Us, right or wrong,
is the essence of faith-based relationships.
Naturally, things are more complicated. Sometimes an athlete
choosing to switch teams is viewed as betraying a sacred trus
```
*   *Audit Focus:* Compare our system metrics, workflows, alert patterns, boundaries, and operability directly to the sociological and biological constraints extracted from the actual PDF pages above.

### Phase 11: Us Versus Them
*   *Chapter-Specific Concluding Summary:*
```text
conclusion. More consistent
has been a link between lower intelligence and a subtype of
conservatism, namely right-wing authoritarianism (RWA, a fondness for
hierarchy). One particularly thorough demonstration of this involved
more than fifteen thousand subjects in the UK and United States;
importantly, the links among low IQ, RWA, and intergroup prejudice
were there after controlling for education and socioeconomic status. The
standard, convincing explanation for the link is that RWA provides
simple answers, ideal for people with poor abstract reasoning skills.
INTELLECTUAL STYLE
This literature has two broad themes. One is that rightists are
relatively uncomfortable intellectually with ambiguity; this is covered
below. The other is that leftists, well, think harder, have a greater
capacity for what the political scientist Philip Tetlock of the University
of Pennsylvania calls “integrative complexity.”
In one study conservatives and liberals, when asked about the causes
of poverty, both tended toward personal attributions (“They’re poor
because they’re lazy”). But only if they had to make snap judgments.
Give people more time, and liberals shifted toward situational
explanations (“Wait, things are stacked against the poor”). In other
words, conservatives start gut and stay gut; liberals go from gut to
head.34
This differing attributional style extends way beyond politics. Tell
liberals or conservatives about a guy who trips over someone’s feet while
learning a dance, ask for a snap assessment, and everyone does personal
attribution—the guy’s clumsy. It’s only with time that the liberals go
situational—maybe the dance is really difficult.
Obviously this dichotomy isn’t perfect. Rightists did personal
attribution for Lewinsky-gate (Bill Clinton’s rotten) while leftists d
```
*   *Audit Focus:* Compare our system metrics, workflows, alert patterns, boundaries, and operability directly to the sociological and biological constraints extracted from the actual PDF pages above.

### Phase 12: Hierarchy, Obedience, and Resistance
*   *Chapter-Specific Concluding Summary:*
```text
499
more willingness to punish unfair players (for both). I find this to be a
particularly challenging pair of findings, especially when framed as the
authors thoughtfully did.
The authors ask where the uniquely extreme sense of fairness comes
from in humans, particularly in the context of large-scale societies with
strangers frequently interacting. And they offer two traditional types of
explanations that are closely related to our dichotomies of intuition
versus reasoning and animal roots versus cultural inventions:
Our moral anchoring in fairness in large-scale societies
is a residue and extension of our hunter-gatherer and
nonhuman primate past. This was life in small bands,
where fairness was mostly driven by kin selection and
easy scenarios of reciprocal altruism. As our community
size has expanded and we now mostly have one-shot
interactions with unrelated strangers, our prosociality
just represents an expansion of our small-band mind-set,
as we use various green-beard marker shibboleths as
proxies for relatedness. I’d gladly lay down my life for
two brothers, eight cousins, or a guy who is a fellow
Packers fan.
The moral underpinnings of a sense of fairness lie in
cultural institutions and mind-sets that we invented as
our groups became larger and more sophisticated (as
reflected in the emergence of markets, cash economies,
and the like).
This many pages in, it’s obvious that I think the former scenario is
pretty powerful—look, we see the roots of a sense of fairness and justice
in the egalitarian nature of nomadic hunter-gatherers, in other primates,
in infants, in the preeminent limbic rather than cortical involvement. But,
inconveniently for that viewpoint, that’s totally counter to what emerges
from these studies—across the twenty-five cultures it’s the hunt
```
*   *Audit Focus:* Compare our system metrics, workflows, alert patterns, boundaries, and operability directly to the sociological and biological constraints extracted from the actual PDF pages above.

### Phase 13: Morality
*   *Chapter-Specific Concluding Summary:*
```text
CONCLUSIONS
ll things considered, it is a pretty remarkable thing that when an
individual is in pain, we (i.e., we humans, primates, mammals)
often are induced to be in a state of pain also. There have been some
mighty interesting twists and turns for that one to have evolved.
But at the end of the day, the crucial issue is whether an empathic
state actually produces a compassionate act, to avoid the trap of empathy
being an end unto itself. The gap between the state and the act can be
enormous, especially when the goal is for the act to be not only effective
but also pristine in its motives.
For someone reading this book, a first challenge in bridging that gap
is that much of the world’s suffering is felt by distant masses
experiencing things that we haven’t an inkling of—diseases that don’t
touch us; poverty that precludes clean water, a place to live, the certainty
of a next meal; oppression at the hands of political systems that we’ve
been spared; strictures due to repressive cultural norms that might as
well be from another planet. And everything about us makes those the
hardest scenarios for us to actually act—everything about our hominin
past has honed us to be responsive to one face at a time, to a face that is
local and familiar, to a source of pain that we ourselves have suffered.
Yes, best that our compassion be driven by the most need rather than by
the most readily shared pain. Nevertheless, there’s no reason why we
should expect ourselves to have particularly good intuitions when aiming
to heal this far-flung, heterogeneous world. We probably need to be a bit
easier on ourselves in this regard.
Likewise, we should perhaps ease up a bit on the scratching-an-
altruist problem. It has always struck me as a bit mean-spirited to
conclude that it is a hypocrite
```
*   *Audit Focus:* Compare our system metrics, workflows, alert patterns, boundaries, and operability directly to the sociological and biological constraints extracted from the actual PDF pages above.

### Phase 14: Feeling / Understanding / Alleviating Pain
*   *Chapter-Specific Concluding Summary:*
```text
599
socioeconomic status of your family of birth; a concussive head injury;
sleep quality and quantity; prenatal environment; stress and
glucocorticoid levels; whether you’re in pain; if you have Parkinson’s
disease and which medication you’ve been prescribed; perinatal hypoxia;
your dopamine D4 receptor gene variant; if you have had a stroke in
your frontal cortex; if you suffered childhood abuse; how much of a
cognitive load you’ve borne in the last few minutes; your MAO-A gene
variant; if you’re infected with a particular parasite; if you have the gene
for Huntington’s disease; lead levels in your tap water when you were a
kid; if you live in an individualist or a collectivist culture; if you’re a
heterosexual male and there’s an attractive woman around; if you’ve
been smelling the sweat of someone who is frightened. On and on. Of all
the stances of mitigated free will, the one that assigns aptitude to biology
and effort to free will, or impulse to biology and resisting it to free will,
is the most permeating and destructive. “You must have worked so hard”
is as much a property of the physical universe and the biology that
emerged from it as is “You must be so smart.” And yes, being a child
molester is as much a product of biology as is being a pedophile. To
think otherwise is little more than folk psychology.
```
*   *Audit Focus:* Compare our system metrics, workflows, alert patterns, boundaries, and operability directly to the sociological and biological constraints extracted from the actual PDF pages above.

### Phase 15: Metaphors We Kill By
*   *Chapter-Specific Concluding Summary:*
```text
649
Breakfast time, as garbage is dumped from a cart
Somehow baboons in “my” troop got word of the feasting over the
hill, and soon half a dozen would head over each morning to join. It
wasn’t random who did this, who would try to compete for food against
fifty or sixty Thems. The ones who tried were male, big, and aggressive.
And morning is when baboons do much of their socializing—sitting in
contact, grooming, playing—so going for garbage meant forgoing the
socializing. The males who went each morning were the most
aggressive, least affiliative members of the troop.
Not long after, there was a tuberculosis outbreak among the Garbage
Dump baboons. In humans tuberculosis is a chronic disease, slowly
consuming you with “consumption.” In nonhuman primates, TB is
wildfire, spreading rapidly, killing within weeks. Kenyan wildlife vet
colleagues and I identified the cause of the outbreak—the meat inspector
at the lodge was being bribed to approve tubercular cows for slaughter;
animals were killed, unsightly lesionish organs discarded and then
consumed by baboons. Most of the Garbage Dump troop died, as did all
my males who raided the dump.47
This was kinda upsetting to me; I habituated a new troop at the other
end of the park and wouldn’t go anywhere near the remnants of my troop
for half a dozen years. Finally, my soon-to-be wife visiting Kenya for the
first time, I worked up the nerve to return to the troop, to show her the
baboons of my youth.
```
*   *Audit Focus:* Compare our system metrics, workflows, alert patterns, boundaries, and operability directly to the sociological and biological constraints extracted from the actual PDF pages above.

### Phase 16: Biology, the Criminal Justice System, and Free Will
*   *Chapter-Specific Concluding Summary:*
```text
699
pain—say, an insect bite. How can you stop the throbbing? Briefly
stimulate the fast fiber. This adds to the pain for an instant, but by
stimulating neuron C, you shut the system down for a while. And that is
precisely what we often do in such circumstances. An insect bite throbs
unbearably, we scratch hard right around it to dull the pain, and the slow,
chronic pain pathway is shut down for up to a few minutes.
The fact that pain works this way has important clinical implications.
For one thing, it has allowed scientists to design treatments for people
with severe chronic pain syndromes (for example, someone with a severe
back injury). Implanting a little electrode into the fast pain pathway and
attaching it to a stimulator on the person’s hip enables the patient to buzz
that pathway now and then to turn off the chronic pain; it works wonders
in many cases.
Thus we have a circuit that encompasses a temporal sharpening
mechanism, introduces the double negative of inhibiting inhibitors, and
is just all-around cool. And one of the biggest reasons why I love it is
that it was first proposed in 1965 by the great neurobiologists Ronald
Melzack and Patrick Wall. It was merely proposed as a theoretical model
—“No one has ever seen this sort of wiring, but we propose that it’s got
to look something like this, given how pain works.” And subsequent
work showed that’s exactly how this part of the nervous system is wired.
Which Guy Is It?
One final, completely hypothetical circuit.
Suppose we have a circuit made of two layers of neurons:
```
*   *Audit Focus:* Compare our system metrics, workflows, alert patterns, boundaries, and operability directly to the sociological and biological constraints extracted from the actual PDF pages above.

### Phase 17: War and Peace
*   *Chapter-Specific Concluding Summary:*
```text
749
Violence), 2000, p. 1; C. MacMullin and M. Loughry, “Investigating Psychosocial Adjustment
of Former Child Soldiers in Sierra Leone and Uganda,” J Refugee Studies 17 (2004): 472.
3. J. Giedd, “The Teen Brain: Insights from Neuroimaging,” J Adolescent Health 42 (2008): 335.
Demonstration of increased intrinsic connectivity of PFC neurons during adolescence in
monkeys: X. Zhou et al., “Age-Dependent Changes in Prefrontal Intrinsic Connectivity,” PNAS
111 (2014): 3853; T. Singer, “The Neuronal Basis and Ontogeny of Empathy and Mind
Reading: Review of Literature and Implications for Future Research,” Nsci Biobehav Rev 30
(2006): 855; P. Shaw et al., “Intellectual Ability and Cortical Development in Children and
Adolescents,” Nat 440 (2006): 676.
4. D. Yurelun-Todd, “Emotional and Cognitive Changes During Adolescence,” Curr Opinion in
Neurobiol 17 (2007): 251; B. Luna et al., “Maturation of Widely Distributed Brain Function
Subserves Cognitive Development,” Neuroimage 13 (2001): 786; B. Schlaggar et al.,
“Functional Neuroanatomical Differences Between Adults and School-Age Children in the
Processing of Single Words,” Sci 296 (2002): 1476.
5. A. Wang et al., “Developmental Changes in the Neural Basis of Interpreting Communicative
Intent,” SCAN 1 (2006): 107.
6. T. Paus et al., “Maturation of White Matter in the Human Brain: A Review of Magnetic
Resonance Studies,” Brain Res Bull 54 (2001): 255; A. Raznahan et al., “Patterns of
Coordinated Anatomical Change in Human Cortical Development: A Longitudinal
Neuroimaging Study of Maturational Coupling,” Neuron 72 (2011): 873; N. Strang et al.,
“Developmental Changes in Adolescents’ Neural Response to Challenge,” Developmental Cog
Nsci 1 (2011): 560.
7. C. Masten et al., “Neural Correlates of Social Exclusion During Adolescence:
```
*   *Audit Focus:* Compare our system metrics, workflows, alert patterns, boundaries, and operability directly to the sociological and biological constraints extracted from the actual PDF pages above.

---

## MANDATORY DELIVERABLE: CRITICAL PLAN & AUTOMATED CI/CD ENFORCEMENT
Perform your codebase discovery through the lens of this work and map how we worked to build on this knowledge. Once complete, you must compile and deliver:

1. **Executive Summary & Lens Review:** An evaluation of the codebase and application specifically through the lens of this work, detailing how we built on this knowledge to benefit the application and codebase.
2. **Phase Audit Matrix:** A table mapping *Phase* | *Citations/Files* | *Identified Gaps, Risks, or Overlaps* | *Priority (H/M/L)*.
3. **Critical Adjustment Plan:** If any critical gaps or boundary overlaps are identified, form a full, actionable plan to adjust and partition roles and responsibilities.
4. **Automated CI/CD Enforcement Script:** Write a fully functional automated validation script (e.g., Python validation check, Bash Git-hook, custom linter rule, or unit test assertions) that can be integrated directly into our current build scripts and CI/CD deployment pipelines to continuously maintain and enforce these rules!
