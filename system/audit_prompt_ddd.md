# Architectural Audit Prompt: Domain-Driven Design (DDD)

Copy and paste this codebase-agnostic prompt into your developer agent's session to conduct a systematic, phase-by-phase logical architecture and domain model audit.

---

## CONTEXT & OBJECTIVE
You are an expert Domain Model Architect and Software Design Analyst. Your task is to audit our codebase against the logical modeling and strategic design principles of Eric Evans' *Domain-Driven Design*.

Your goal is to inspect the codebase and answer these three questions for each phase:
1. **Current Coverage:** What are we doing in our codebase to cover this Phase? (Cite specific directories, files, or packages).
2. **Gaps & Overlaps:** Are there gaps (e.g., missing business validation, direct database leaks into business models, anemic domain structures) or overlaps (e.g., duplicate domain rules, split responsibilities between entities, unaligned vocabularies)?
3. **Actionable Recommendations:** How should we complete this Phase to clean up domain boundaries, enforce business invariants, and guarantee zero overlap?

---

---

## MANDATORY PRE-REQUISITE: READ THE ENTIRE BOOK
Before starting your codebase audit, you MUST read and analyze the full text of the sibling book PDF file to obtain complete context, grasp the subtle mathematical/neuromodulatory/logical models, and preserve absolute architectural fidelity.

*   **Target PDF File Name:** `Domain Driven Design.pdf`
*   **Target PDF Path:** `/Users/devonshigaki/Desktop/Books/Domain Driven Design.pdf`

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

### Phase 1: Crunching Knowledge
*   *Chapter-Specific Concluding Summary:*
```text
(Final M
anuscript
, April 1
5, 2003) 
© Eric Ev
ans, 2003
 23 
 
 
1. In ord
er to bri
ng it to 
this stag
e, the pr
ogrammers
 and ever
yone invo
lved will
 have 
come to u
nderstand
 the natu
re of ove
rbooking 
as a dist
inct and 
important
 business
 
rule, not
 just an 
obscure c
alculatio
n. 
2. Progra
mmers can
 show bus
iness exp
erts tech
nical art
ifacts, e
ven code,
 that sho
uld be 
intelligi
ble to th
em (with 
guidance)
, closing
 the feed
back loop
. 
 
Deep Mode
ls 
Useful mo
dels seld
om lie on
 the surf
ace. As w
e underst
and the d
omain and
 the need
s of the 
applicati
on, we 
usually d
iscard su
perficial
 model el
ements th
at seemed
 importan
t in the 
beginning
, or shif
t their 
perspecti
ve. 
One of th
e project
s that I’
ll be dra
wing on f
or exampl
es throug
hout the 
book was 
a contain
er shippi
ng 
system. S
ince the 
beginning
 of a shi
pment is 
booking c
argo, we 
developed
 a model 
that allo
wed us to
 
describe 
the cargo
, its iti
nerary, a
nd so on.
 This was
 all nece
ssary and
 useful, 
yet the d
omain exp
erts felt
 
dissatisf
ied. Ther
e was a w
ay they l
ooked at 
their bus
iness tha
t we were
 missing.
 
Eventuall
y, we rea
lized tha
t our foc
us on the
 handling
 of cargo
, the phy
sical loa
ding and 
unloading
, the 
movements
 from pla
ce to pla
ce, was l
argely ha
ndled by 
subcontra
ctors or 
by operat
ional peo
ple in th
e 
company. 
The view 
of our cu
stomers w
as of a s
eries of 
transfers
 of respo
nsibility
 between 
parties. 
A 
process g
overned t
ransfer o
f legal a
nd practi
cal respo
nsibility
 from the
 shipper 
to some l
ocal carr
ier, from
 
one carri
er to ano
ther, and
 to the c
onsignee.
 Often, t
he cargo 
would sit
 in a war
ehouse wh
ile impor
tant 
steps wer
e being t
aken. At 
other tim
es, the
```
*   *Audit Focus:* Review how domain information, business constraints, and logic are extracted, codified, and evolved compared directly to the original chapter-ending extract above.

### Phase 2: Communication and the Use of Language
*   *Chapter-Specific Concluding Summary:*
```text
(Final M
anuscript
, April 1
5, 2003) 
© Eric Ev
ans, 2003
 35 
 
 
Class Dia
gram for 
Shipping 
Route 
Route
Leg
{ordered}
*
Port Oper
ation
load
unload
departure
Time
arrivalTi
me
Transport
 Leg
Voyage
{ordered}
*
Location
from
to
 
Figure 2.
 4 
In a case
 like thi
s, an exp
lanatory 
model can
 go a lon
g way to 
helping t
eam membe
rs 
understan
d what th
is actual
ly means.
 Here is 
another w
ay of loo
king at t
he same c
oncepts. 
 
Explanato
ry Model 
for Shipp
ing Route
 
unloading
 by
vendor XY
Z
loading b
y vendor
XYZ
loading b
y vendor
ABC
on ground
 in port
location 
LGB03
on-board 
vessel
voyage A
on-board 
train
voyage B
 
Figure 2.
 5 
 
Each line
 represen
ts either
 a port o
peration 
(loading 
or unload
ing the c
argo), or
 cargo 
sitting i
n storage
 on the g
round, or
 cargo si
tting on 
a ship in
 route. 
This sort
 of diagr
am can he
lp a deve
loper, or
 a domain
 expert, 
understan
d the mor
e 
rigorous 
software 
model dia
grams. To
gether th
ey are ea
sier to u
nderstand
 than eit
her of 
them alon
e.
```
*   *Audit Focus:* Review how domain information, business constraints, and logic are extracted, codified, and evolved compared directly to the original chapter-ending extract above.

### Phase 3: Binding Model and Implementation
*   *Chapter-Specific Concluding Summary:*
```text
(Final M
anuscript
, April 1
5, 2003) 
© Eric Ev
ans, 2003
 50 
 
 
Navigatio
n Map of 
the Langu
age of M
ODEL
-D
RIVEN 
D
ESIGN
 
encapsula
te with
M
ODEL
-D
RIVEN
D
ESIGN
express m
odel with
isolate d
omain wit
h
encapsula
te with
E
NTITIES
V
ALUE
 O
BJECTS
L
AYERED
A
RCHITECTU
RE
A
GGREGATES
R
EPOSITORI
ES
act as ro
ot of
S
MART
 UI
X
F
ACTORIES
encapsula
te with
express m
odel with
encapsula
te with
mutually 
exclusive
choices
access wi
th
encapsula
te with
access wi
th
S
ERVICES
express m
odel with
 
 
 
Part II p
resents a
 pattern 
language 
of curren
t best pr
actices f
or the ba
sic const
ruction o
f domain 
models. T
his diagr
am 
shows the
 patterns
 that wil
l be pres
ented in 
Part II, 
and gives
 a quick 
indicatio
n of how 
they rela
te to eac
h other.
```
*   *Audit Focus:* Review how domain information, business constraints, and logic are extracted, codified, and evolved compared directly to the original chapter-ending extract above.

### Phase 4: Isolating the Domain
*   *Chapter-Specific Concluding Summary:*
```text
(Final M
anuscript
, April 1
5, 2003) 
© Eric Ev
ans, 2003
 59 
 
 
But that 
comes lat
er. Next,
 we’ll lo
ok at the
 nuts and
 bolts of
 coevolvi
ng the de
tails of 
an effect
ive domai
n 
model and
 an expre
ssive imp
lementati
on. After
 all, the
 best par
t of isol
ating the
 domain i
s getting
 all that
 
other stu
ff out of
 the way 
so that w
e can rea
lly focus
 on the d
omain des
ign…
```
*   *Audit Focus:* Review how domain information, business constraints, and logic are extracted, codified, and evolved compared directly to the original chapter-ending extract above.

### Phase 5: A Model Expressed in Software
*   *Chapter-Specific Concluding Summary:*
```text
(Final M
anuscript
, April 1
5, 2003) 
© Eric Ev
ans, 2003
 87 
 
 
It is imp
ortant to
 continue
 to think
 in terms
 of model
s while w
orking wi
th rules.
 Otherwis
e, the da
ta and th
e 
rules bec
ome uncon
nected. T
he rules 
in the en
gine end 
up more l
ike littl
e program
s than co
nceptual 
rules in 
the domai
n model. 
 
 
Other par
adigm mix
tures ari
se becaus
e some ma
jor part 
of the pr
oblem dom
ain is no
t natural
 to model
 in 
the domin
ant parad
igm. When
 the inte
rdependen
ce is sma
ll, a sub
system in
 the othe
r paradig
m can be 
encapsula
ted, such
 as a com
plex math
 calculat
ion that 
simply ne
eds to be
 called b
y an obje
ct. Other
 times 
the diffe
rent aspe
cts are m
ore inter
twined, a
s when th
e interac
tion of t
he object
s is depe
ndent on 
some 
mathemati
cal relat
ionships.
 Then a m
odel must
 be devis
ed with e
lements o
f both pa
radigms. 
While a 
MODEL
-
DRIVEN DE
SIGN
 does not
 have to 
be object
-oriented
, it does
 depend o
n having 
an expres
sive 
implement
ation of 
the model
 construc
ts, be th
ey object
s, rules,
 or workf
lows. If 
the avail
able tool
 does not
 
facilitat
e that ex
pressiven
ess, reco
nsider th
e choice 
of tools.
 An unexp
ressive i
mplementa
tion obvi
ates the 
advantage
 of the e
xtra para
digm 
 Rules of
 thumb fo
r mixing 
non-objec
t element
s into a 
predomina
ntly obje
ct-orient
ed system
: 
• Don’t f
ight the 
implement
ation par
adigm. Wh
en modeli
ng, find 
concepts 
that fit 
the parad
igm that 
they will
 be imple
mented in
. 
• Lean on
 the 
UBIQUITOU
S LANGUAG
E
. Even wh
en there 
is no rig
orous con
nection b
etween to
ols, 
very cons
istent us
e of lang
uage can 
keep part
s of the 
design fr
om diverg
ing. 
• Be skep
tical. Is
 the tool
 really p
ulling it
s weight?
 Just bec
aus
```
*   *Audit Focus:* Review how domain information, business constraints, and logic are extracted, codified, and evolved compared directly to the original chapter-ending extract above.

### Phase 6: The Lifecycle of a Domain Object
*   *Chapter-Specific Concluding Summary:*
```text
(Final M
anuscript
, April 1
5, 2003) 
© Eric Ev
ans, 2003
 121 
 
 
Now we mu
st consid
er the ap
plication
 requirem
ents.  In
 order to
 take a b
ooking th
rough 
the Booki
ng Applic
ation, th
e user to
 select t
he Custom
er(s) pla
ying the 
various r
oles 
(shipper,
 receiver
, etc), s
o we have
 a Custom
er Reposi
tory.  We
 also nee
d to find
 a 
Location 
to specif
y as the 
destinati
on for th
e Cargo, 
so we hav
e a Locat
ion Repos
itory.  
The Activ
ity Loggi
ng Applic
ation nee
ds to all
ow the us
er to loo
k up the 
Carrier M
ovement 
that a Ca
rgo is be
ing loade
d onto, s
o we need
 a Carrie
r Movemen
t Reposit
ory.  Thi
s user 
must also
 tell the
 system w
hich Carg
o has bee
n loaded,
 so we ne
ed a Carg
o Reposit
ory.  
find by C
argo Trac
king ID(S
tring)
find by C
ustomer I
D(String)
Cargo Rep
ository
Customer
Cargo
Carrier
Movement
Delivery 
History
0..1
Handling 
Event
*
Delivery
Specifica
tion
*
goal
Location
destinati
on
from
to
handled
role
*
*
find by C
ustomer I
D(String)
find by n
ame(Strin
g)
find by C
argo Trac
king ID(S
tring)
Customer 
Repositor
y
R
EPOSITORI
ES
 are proh
ibited
from inte
rior of 
AGGREGATE
.
find by p
ort code(
String)
find by c
ity name(
String)
Location 
Repositor
y
*
*
*
find by S
chedule I
D(String)
find by F
romTo(Loc
ation, Lo
cation)
Location 
Repositor
y
*
 
Figure 7.
 4 
For now t
here is n
o Handlin
g Event R
epository
 because 
we decide
d to impl
ement the
 
associati
on with D
elivery H
istory as
 a collec
tion in t
he first 
iteration
, and we 
have no 
applicati
on requir
ement to 
find out 
what has 
been load
ed onto a
 Carrier 
Movement.
  Either 
of these 
reasons c
ould chan
ge, and t
hen we wo
uld add a
 
REPOSITOR
Y
.
```
*   *Audit Focus:* Review how domain information, business constraints, and logic are extracted, codified, and evolved compared directly to the original chapter-ending extract above.

### Phase 7: Using the Language: An Extended Example
*   *Chapter-Specific Concluding Summary:*
```text
(Final M
anuscript
, April 1
5, 2003) 
© Eric Ev
ans, 2003
 135 
 
 
In additi
on to fac
ilitating
 change, 
a supple 
design co
ntributes
 to the r
efinement
 of the m
odel itse
lf. A 
MODEL
-
DRIVEN DE
SIGN
 stands o
n two leg
s. A deep
 model ma
kes possi
ble an ex
pressive 
design. A
t the sam
e 
time, a d
esign wit
h the fle
xibility 
to let a 
developer
 experime
nt and th
e clarity
 to show 
a develop
er what i
s 
happening
 actually
 feeds in
sight int
o the mod
el discov
ery proce
ss. This 
half of t
he feedba
ck loop i
s 
essential
 because 
the model
 we are l
ooking fo
r is not 
just a ni
ce set of
 ideas, i
t is the 
foundatio
n of the 
system. 
 
The Disco
very Proc
ess 
To create
 a design
 really f
itted to 
the probl
em at han
d, you mu
st first 
have a mo
del that 
captures 
the centr
al 
relevant 
concepts 
of the do
main. Act
ively sea
rching fo
r these a
nd bringi
ng them i
nto the d
esign is 
the 
subject o
f Chapter
 9, “Maki
ng Implic
it Concep
ts Explic
it”. 
Because o
f the clo
se relati
onship be
tween mod
el and de
sign, the
 modeling
 process 
comes to 
a halt wh
en 
the code 
is hard t
o refacto
r. Chapte
r 10, “Su
pple Desi
gn” discu
sses how 
to write 
software 
for softw
are 
developer
s, not le
ast yours
elf, so t
hat it is
 producti
ve to ext
end and c
hange. Th
is goes h
and in ha
nd with 
further r
efinement
s to the 
model. It
 often en
tails mor
e advance
d design 
technique
s and mor
e rigor i
n model 
definitio
ns. 
You will 
usually h
ave to ca
ll on cre
ativity a
nd trial 
and error
 to find 
good ways
 to model
 the conc
epts you 
discover,
 but some
times som
eone has 
laid down
 a patter
n you can
 follow. 
Chapters 
11 and 12
 discuss 
the 
applicati
on of “an
alysis pa
tterns” a
nd “desig
n pattern
s”. These
 few patt
ern
```
*   *Audit Focus:* Review how domain information, business constraints, and logic are extracted, codified, and evolved compared directly to the original chapter-ending extract above.

### Phase 8: Breakthrough
*   *Chapter-Specific Concluding Summary:*
```text
(Final M
anuscript
, April 1
5, 2003) 
© Eric Ev
ans, 2003
 144 
 
 
(
The const
raints in
 the diag
ram illus
trate the
 easy pre
cision ga
ined with
 the new 
model.)
 
 
As is com
monly the
 case aft
er a real
 breakthr
ough to a
 deep mod
el, the c
larity (a
nd 
typically
 simplici
ty) of th
e new des
ign, comb
ined with
 the enha
nced comm
unication
 based 
on the ne
w 
UBIQUITOU
S LANGUAG
E
 had led 
to yet an
other mod
eling bre
akthrough
. 
Our pace 
of develo
pment was
 accelera
ting at a
 stage wh
ere most 
projects 
are begin
ning to 
be bogged
 down by 
the mass 
and compl
exity of 
what they
 have alr
eady buil
t.
```
*   *Audit Focus:* Review how domain information, business constraints, and logic are extracted, codified, and evolved compared directly to the original chapter-ending extract above.

### Phase 9: Making Implicit Concepts Explicit
*   *Chapter-Specific Concluding Summary:*
```text
(Final M
anuscript
, April 1
5, 2003) 
© Eric Ev
ans, 2003
 169 
 
 
         
   return
 containe
r; 
      } 
      thr
ow new No
AnswerFou
ndExcepti
on(); 
   } 
 
} 
Granted t
hat this 
leaves a 
lot to be
 desired.
 It may p
ack sand 
into spec
ialty con
tainers a
nd then r
un out of
 
room befo
re it pac
ks the ha
zardous c
hemicals.
 It certa
inly does
n’t optim
ize reven
ues. But 
a lot of 
optimizat
ion probl
ems are n
ever solv
ed perfec
tly anywa
y. This d
oes follo
w the rul
es that h
ave been 
stated 
so far.  
The point
 is that 
having an
y working
 implemen
tation at
 all allo
ws projec
t work to
 go on mu
ch more 
flexibly.
 When the
 time is 
right, it
 can (and
 probably
 must) be
 replaced
 by a mor
e effecti
ve 
implement
ation, qu
ite possi
bly devel
oped by s
pecialist
s in opti
mization 
algorithm
s. In the
 mean tim
e, all 
other par
ts of the
 system h
ave somet
hing to i
nteract w
ith durin
g develop
ment. 
Here is a
n example
 of a “si
mplest th
ing that 
could pos
sibly wor
k” that a
ctually b
ecomes po
ssible be
cause 
of a more
 sophisti
cated mod
el. We ca
n have a 
functioni
ng protot
ype of a 
very comp
lex compo
nent in a
 
couple do
zen lines
 of easil
y underst
ood code.
 A less m
odel-driv
en approa
ch would 
be harder
 to under
stand, 
be harder
 to upgra
de (becau
se the Pa
cker woul
d be more
 coupled 
to the re
st of the
 design),
 and, in 
this 
case, wou
ld likely
 take lon
ger to pr
ototype. 
<<End Sid
ebar>>
```
*   *Audit Focus:* Review how domain information, business constraints, and logic are extracted, codified, and evolved compared directly to the original chapter-ending extract above.

### Phase 10: Supple Design
*   *Chapter-Specific Concluding Summary:*
```text
(Final M
anuscript
, April 1
5, 2003) 
© Eric Ev
ans, 2003
 192 
 
 
Declarati
ve Design
 
We’ve dis
cussed as
sertions,
 and our 
relativel
y informa
l way of 
testing t
hem. They
 can lead
 to much 
better 
designs, 
but there
 can’t be
 any real
 guarante
es in han
dwritten 
software.
 To name 
just one 
way of ev
ading 
the 
ASSERTION
S
, there c
ould be a
dditional
 side eff
ects that
 were not
 specific
ally excl
uded. No 
matter ho
w 
MODEL
-
DRIVEN
 our desi
gn is, we
 still en
d up writ
ing proce
dures to 
produce t
he effect
 of the c
onceptual
 
interacti
ons. And 
we always
 spend so
 much of 
our time 
writing b
oilerplat
e code th
at doesn’
t really 
add any 
meaning o
r behavio
r. This i
s tedious
 and frau
ght with 
error, an
d the bul
k of it o
bscures t
he meanin
g of our 
model. (S
ome langu
ages are 
better th
an others
, but all
 require 
us to do 
a lot of 
grunt-wor
k.) I
NTENTION 
REVEALING
 INTERFAC
ES
 and the 
other pat
terns in 
this chap
ter help,
 but can 
never giv
e convent
ional obj
ect-
oriented 
programs 
formal ri
gor. 
These are
 some of 
the motiv
ations be
hind decl
arative d
esign. Th
is term m
eans many
 things t
o many 
people, b
ut usuall
y it indi
cates a w
ay to wri
te a prog
ram, or s
ome part 
of it, as
 a kind o
f executa
ble 
specifica
tion. A v
ery preci
se descri
ption of 
propertie
s actuall
y control
s the sof
tware. In
 its vari
ous forms
 
this coul
d be done
 through 
a reflect
ion mecha
nism or a
t compile
-time thr
ough code
 generati
on (produ
cing 
conventio
nal code 
automatic
ally, bas
ed on the
 declarat
ion). Thi
s allows 
another d
eveloper 
to take t
he 
declarati
on at fac
e-value. 
It is an 
absolute 
guarantee
.  
This is a
 kind of 
Holy Grai
l of mode
l-driven 
design. I
t does ha
ve its pi
tfalls i
```
*   *Audit Focus:* Review how domain information, business constraints, and logic are extracted, codified, and evolved compared directly to the original chapter-ending extract above.

### Phase 11: Applying Analysis Patterns
*   *Chapter-Specific Concluding Summary:*
```text
(Final M
anuscript
, April 1
5, 2003) 
© Eric Ev
ans, 2003
 206 
 
 
To see th
e deviati
on of eac
h lender 
from its 
agreed co
ntributio
n, take t
he theore
tical 
distribut
ion of th
e outstan
ding Loan
 amount a
nd subtra
ct it fro
m the act
ual Loan 
shares: 
SharePie 
originalA
greement 
= aFacili
ty.getSha
res().pro
rated(aLo
an.getAmo
unt()); 
SharePie 
actual = 
aLoan.get
Shares();
 
SharePie 
deviation
 = actual
.minus(or
iginalAgr
eement); 
 
Certain c
haracteri
stics of 
the Share
 Pie desi
gn make f
or this e
asy recom
bination 
and 
communica
tion. 
• Complex
 logic en
capsulate
d in spec
ialized 
VALUE OBJ
ECTS
 
Since Sha
re Pies a
re 
VALUE OBJ
ECTS
, these m
ath opera
tions can
 create n
ew 
instances
 which we
 can free
ly use to
 replace 
outdated 
instances
. 
• S
IDE
-
EFFECT
-
FREE FUNC
TIONS
 
None of t
he Share 
Pie metho
ds causes
 any chan
ge to any
 existing
 object. 
This 
allows us
 to use p
lus(), mi
nus(), an
d prorate
d() freel
y in inte
rmediate 
calculati
ons, comb
ining the
m, expect
ing them 
to do wha
t their n
ames sugg
est, and 
nothing m
ore. It a
lso allow
s us to b
uild anal
ytical fe
atures ba
sed on th
e same 
methods. 
(Before, 
they coul
d only be
 called w
hen an ac
tual dist
ribution 
was made,
 
since the
 data wou
ld change
 after ea
ch call.)
 
• C
LOSURE OF
 OPERATIO
NS
 
Each Shar
e Pie ope
ration pr
oduces ei
ther anot
her Share
 Pie or a
 simple a
mount. 
As a resu
lt, the S
hare Pie 
is self-c
ontained,
 easily u
nderstood
, easily 
tested, a
nd 
easily co
mbined (e
.g. the d
eviation 
is the ac
tual pie 
minus the
 loan amo
unt 
prorated 
based on 
the facil
ity share
s). 
Pulling o
ut the pa
rt of the
 problem 
that corr
esponded 
to the fo
rmalism o
f math, w
e arrived
 
at a supp
le design
 for shar
es that f
```
*   *Audit Focus:* Review how domain information, business constraints, and logic are extracted, codified, and evolved compared directly to the original chapter-ending extract above.

### Phase 12: Relating Design Patterns to the Model
*   *Chapter-Specific Concluding Summary:*
```text
(Final M
anuscript
, April 1
5, 2003) 
© Eric Ev
ans, 2003
 218 
 
 
12. Relat
ing Desig
n Pattern
s to the 
Model 
The patte
rns so fa
r are spe
cifically
 for solv
ing probl
ems in a 
domain mo
del in th
e context
 of a 
MODEL
-
DRIVEN DE
SIGN
. Actuall
y, though
, most of
 the patt
erns publ
ished to 
date are 
more tech
nical in 
focus. Wh
at 
is the di
fference 
between a
 design p
attern an
d a domai
n pattern
? For sta
rters, th
e authors
 of the s
eminal 
book, Des
ign Patte
rns, had 
this to s
ay. 
Point of 
view affe
cts one’s
 interpre
tation of
 what is 
and isn’t
 a patter
n. One pe
rson’s 
pattern c
an be ano
ther pers
on’s prim
itive bui
lding blo
ck. For t
his book 
we have 
concentra
ted on pa
tterns at
 a certai
n level o
f abstrac
tion. Des
ign patte
rns are n
ot 
about des
igns such
 as linke
d lists a
nd hash t
ables tha
t can be 
encoded i
n classes
 
and reuse
d as is. 
Nor are t
hey compl
ex, domai
n-specifi
c designs
 for an e
ntire 
applicati
on or sub
system. T
he design
 patterns
 in this 
book are 
descripti
ons of 
communica
ting obje
cts and c
lasses th
at are cu
stomized 
to solve 
a general
 design 
problem i
n a parti
cular con
text. [GH
JV1995, p
.3] 
 
 Some, no
t all, of
 the desi
gn patter
ns in [GH
JV1995] c
an be use
d as doma
in patter
ns. It re
quires a 
shift in 
emphasis.
 Design P
atterns p
resented 
a catalog
 of desig
n element
s that so
lved prob
lems comm
only 
encounter
ed in a v
ariety of
 contexts
. The mot
ivations 
of these 
patterns 
and the p
atterns t
hemselves
 were 
presented
 in purel
y technic
al terms.
 But a su
bset of t
hese elem
ents can 
be applie
d in the 
broader c
ontext of
 
domain mo
deling an
d design,
 because 
they corr
espond to
 general 
concepts 
that emer
ge in man
y domains
.  
In additi
on
```
*   *Audit Focus:* Review how domain information, business constraints, and logic are extracted, codified, and evolved compared directly to the original chapter-ending extract above.

### Phase 13: Refactoring Toward Deeper Insight
*   *Chapter-Specific Concluding Summary:*
```text
(Final M
anuscript
, April 1
5, 2003) 
© Eric Ev
ans, 2003
 224 
 
 
Example: 
Shipment 
Routes Ma
de of Rou
tes 
A complet
e cargo s
hipment r
oute is c
omplicate
d. First,
 the cont
ainer mus
t be truc
ked to a 
rail-head
, then ca
rried to 
a port, t
hen on a 
ship to a
nother po
rt, possi
bly trans
ferred to
 other 
ships, fi
nishing w
ith groun
d transpo
rtation o
n the oth
er end. 
Schematic
 of a “ro
ute” made
 up of “l
egs” 
unloading
 by
vendor XY
Z
loading b
y vendor
XYZ
loading b
y vendor
ABC
on ground
 in port
location 
LGB03
on-board 
vessel
voyage A
on-board 
train
voyage B
leg 1
leg 2
 
Figure 12
. 3 
An applic
ation dev
elopment 
team has 
developed
 an 
OBJECT MO
DEL
 to expre
ss these 
arbitrari
ly long s
trings of
 legs tha
t assembl
e into a 
route. 
Class Dia
gram of R
oute made
 up of Le
gs 
Route
Leg
{ordered}
*
Port Oper
ation
load
unload
departure
Time
arrivalTi
me
Transport
 Leg
Voyage
{ordered}
*
Location
from
to
 
Figure 12
. 4
```
*   *Audit Focus:* Review how domain information, business constraints, and logic are extracted, codified, and evolved compared directly to the original chapter-ending extract above.

### Phase 14: Maintaining Model Integrity
*   *Chapter-Specific Concluding Summary:*
```text
(Final M
anuscript
, April 1
5, 2003) 
© Eric Ev
ans, 2003
 272 
 
 
of coordi
nation no
t just in
 developm
ent but a
lso in de
ployment.
 S
EPARATE W
AYS
 can make
 life muc
h 
simpler. 
The Trade
off 
To sum up
 these gu
idelines,
 there is
 a range 
of strate
gies for 
unifying 
or integr
ating mod
els. In g
eneral 
terms, yo
u will tr
adeoff th
e benefit
s of seam
less inte
gration o
f functio
nality ag
ainst the
 addition
al effort
 of 
coordinat
ion and c
ommunicat
ion. You 
trade mor
e indepen
dent acti
on agains
t smoothe
r communi
cation. 
More ambi
tious uni
fication 
requires 
control o
ver the d
esign of 
the subsy
stems inv
olved. 
Relative 
Demands o
f C
ONTEXT 
Relations
hip
 
Patterns 
communica
tions com
mitment/c
apability
 of team(
s)
c
o
n
t
r
o
l
 
o
f
 
a
l
l
 
a
s
s
o
c
i
a
t
e
d
 
s
y
s
t
e
m
s
S
EPARATE
 W
AYS
A
NTI
-
CORRUPTIO
N
L
AYER
 Single B
OUNDED
C
ONTEXT
C
USTOMER
/
S
UPPLIER
 T
EAMS
S
HARED
 K
ERNEL
C
ONFORMIST
O
PEN
 H
OST
S
ERVICE
 
Figure 14
. 12 
 
 
 
When Your
 Project 
Is Alread
y Underwa
y 
Most like
ly, you a
re not st
arting a 
project b
ut are lo
oking to 
improve a
 project 
that is a
lready un
derway. I
n 
this case
, the fir
st step i
s to defi
ne 
BOUNDED C
ONTEXT
 accordin
g to the 
way thing
s are now
. This is
 crucial.
 
To be eff
ective, t
he unific
ation con
texts mus
t reflect
 the true
 practice
 of the t
eams, not
 the idea
l 
organizat
ion you m
ight deci
de on fol
lowing th
e guideli
nes above
. 
Once you 
have deli
neated yo
ur true c
urrent 
BOUNDED C
ONTEXTS
 and desc
ribed the
 relation
ships the
y 
currently
 have, th
e next st
ep is to 
tighten u
p the tea
m’s pract
ices arou
nd that c
urrent or
ganizatio
n. Improv
e 
your 
CONTINUOU
S INTEGRA
TION
 within t
he 
CONTEXTS
. Refacto
r any str
ay transl
at
```
*   *Audit Focus:* Review how domain information, business constraints, and logic are extracted, codified, and evolved compared directly to the original chapter-ending extract above.

### Phase 15: Distillation
*   *Chapter-Specific Concluding Summary:*
```text
(Final M
anuscript
, April 1
5, 2003) 
© Eric Ev
ans, 2003
 310 
 
 
E
VOLVING 
O
RDER
 
Many deve
lopers ha
ve experi
enced the
 cost of 
an unstru
ctured de
sign. To 
avoid thi
s anarchy
, project
s 
impose ar
chitectur
es that c
onstrain 
developme
nt in var
ious ways
. Some te
chnical a
rchitectu
res do so
lve 
technical
 problems
, such as
 networki
ng or dat
a persist
ence, but
 when arc
hitecture
s start v
enturing 
into the 
arena of 
the appli
cation an
d domain 
model the
y can cre
ate probl
ems of th
eir own. 
They ofte
n prevent
 the 
developer
s from cr
eating de
signs and
 models t
hat work 
well for 
the speci
fics of t
he proble
m. The mo
st 
ambitious
 ones can
 even tak
e away fr
om applic
ation dev
elopers f
amiliarit
y and tec
hnical po
wer of th
e 
programmi
ng langua
ge itself
. And whe
ther tech
nical or 
domain or
iented, a
rchitectu
res freez
e a lot o
f up-
front des
ign decis
ions that
 can beco
me a stra
ight jack
et as req
uirements
 change a
nd as und
erstandin
g 
deepens. 
While som
e technic
al archit
ectures h
ave becom
e promine
nt over t
he years 
(e.g., J2
EE), larg
e-scale 
structure
 in the d
omain lay
er has no
t been ex
plored mu
ch. Needs
 vary wid
ely from 
one appli
cation to
 the 
next. An 
upfront i
mposition
 of a lar
ge-scale 
structure
 is likel
y to be c
ostly. As
 developm
ent proce
eds, you 
will almo
st certai
nly find 
a more su
itable st
ructure, 
and you m
ay even f
ind that 
the presc
ribed str
ucture is
 
prohibiti
ng you fr
om taking
 a design
 route th
at would 
greatly c
larify or
 simplify
 the appl
ication. 
You may 
be able t
o use som
e of the 
structure
, but you
’re forgo
ing oppor
tunities.
 Your wor
k slows d
own as yo
u try 
workaroun
ds or try
 to negot
iate with
 the arch
itects. B
ut your m
anagers
```
*   *Audit Focus:* Review how domain information, business constraints, and logic are extracted, codified, and evolved compared directly to the original chapter-ending extract above.

### Phase 16: Large-Scale Structure
*   *Chapter-Specific Concluding Summary:*
```text
(Final M
anuscript
, April 1
5, 2003) 
© Eric Ev
ans, 2003
 335 
 
 
User plac
es lot in
 next mac
hine and 
logs move
 into com
puter 
MES Appli
cation
:Process 
Flow
X : Lot
Z : Proce
ss
Operation
get next 
operation
operation
 Z
get next 
operation
Y : Proce
ss
Machine
is operat
ion Z sup
ported
choose re
cipe on m
achine Y
confirm
login lot
 X to mac
h Y
Handled I
n Legacy
System
load reci
pe
 
Figure 16
. 18 
The frame
work has 
very spec
ific infr
astructur
e require
ments. It
 is tight
ly couple
d to CORB
A 
to provid
e persist
ence, tra
nsactions
, events 
and other
 technica
l service
s. But th
e 
interesti
ng thing 
about it 
is the de
finition 
of a 
PLUGGABLE
 COMPONEN
T FRAMEWO
RK
 that 
allows pe
ople to d
evelop pa
rts of th
ese immen
se system
s indepen
dently an
d smoothl
y 
integrate
 them int
o immense
 systems 
that no o
ne knows 
all the d
etails of
, but eve
ryone 
understan
ds an ove
rview of.
 
 
♦ ♦ ♦  
The plug-
in interf
ace of hu
b might u
se a 
PUBLISHED
 LANGUAGE
. 
 
<<Side Ba
r>> 
How can t
housands 
of people
 work ind
ependentl
y to crea
te a quil
t of more
 than 
40,000 pa
nels? A f
ew simple
 rules pr
ovide a l
arge-scal
e structu
re for th
e AIDS Me
morial 
Quilt, le
aving the
 details 
to indivi
dual cont
ributors.
 
Here's ho
w to crea
te a pane
l for the
 Quilt: 
Design th
e panel 
Include t
he name o
f the per
son you a
re rememb
ering. Fe
el free t
o include
 addition
al inform
ation suc
h as the 
dates of 
birth and
 death, a
nd a home
town. Ple
ase limit
 each pan
el to one
 individu
al.  
Choose yo
ur materi
als
```
*   *Audit Focus:* Review how domain information, business constraints, and logic are extracted, codified, and evolved compared directly to the original chapter-ending extract above.

### Phase 17: Bringing the Strategy Together
*   *Chapter-Specific Concluding Summary:*
```text
summary. 
Solution 
consequen
ces. Impl
ementatio
n conside
rations. 
♦ ♦ ♦
 
 [Optiona
l: Discus
sion of i
mplementa
tion chal
lenges. (
In Alexan
der’s ori
ginal for
mat, this
 would ha
ve been 
folded in
to the re
solution 
of proble
m section
, and it 
is often 
handled t
hat way i
n this bo
ok. But s
ome 
patterns 
demand le
ngthier d
iscussion
s of impl
ementatio
n. To kee
p the cor
e pattern
 discussi
on tight,
 I have 
moved the
se into t
his exten
sion. 
♦ ♦ ♦
 ] 
Resulting
 context:
 Brief ex
planation
 of how t
he patter
n leads t
o later p
atterns. 
 
The patte
rn names 
are meant
 to becom
e terms i
n the lan
guage of 
the team.
 When a p
attern na
me appear
s in 
a discuss
ion, it w
ill be 
FORMATTED
 to call 
it out.
```
*   *Audit Focus:* Review how domain information, business constraints, and logic are extracted, codified, and evolved compared directly to the original chapter-ending extract above.

---

## MANDATORY DELIVERABLE: CRITICAL PLAN & AUTOMATED CI/CD ENFORCEMENT
Perform your codebase discovery through the lens of this work and map how we worked to build on this knowledge. Once complete, you must compile and deliver:

1. **Executive Summary & Lens Review:** An evaluation of the codebase and application specifically through the lens of this work, detailing how we built on this knowledge to benefit the application and codebase.
2. **Phase Audit Matrix:** A table mapping *Phase* | *Citations/Files* | *Identified Gaps, Risks, or Overlaps* | *Priority (H/M/L)*.
3. **Critical Adjustment Plan:** If any critical gaps or boundary overlaps are identified, form a full, actionable plan to adjust and partition roles and responsibilities.
4. **Automated CI/CD Enforcement Script:** Write a fully functional automated validation script (e.g., Python validation check, Bash Git-hook, custom linter rule, or unit test assertions) that can be integrated directly into our current build scripts and CI/CD deployment pipelines to continuously maintain and enforce these rules!
