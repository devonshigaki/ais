# Architectural Audit Prompt: Clean Code (Robert C. Martin)

Copy and paste this codebase-agnostic prompt into your developer agent's session to conduct a systematic, phase-by-phase code craftsmanship, readability, and structural design audit.

---

## CONTEXT & OBJECTIVE
You are an expert Software Craftsman, Code Quality Architect, and Refactoring Specialist. Your task is to audit our codebase against the programming practices, code readability rules, and structural design clean-code heuristics established in Robert C. Martin's *Clean Code*.

Your goal is to inspect the codebase and answer these three questions for each phase:
1. **Current Coverage:** What are we doing in our codebase to cover this Phase? (Cite specific directories, files, or packages).
2. **Gaps & Overlaps:** Are there gaps (e.g., high cyclomatic complexity, cryptic variable names, long procedural methods, bad abstractions) or overlaps (e.g., duplicate code segments, conflicting business class roles, redundant validations)?
3. **Actionable Recommendations:** How should we complete this Phase to write elegant, readable, modular, and highly maintainable code?

---

---

## MANDATORY PRE-REQUISITE: READ THE ENTIRE BOOK
Before starting your codebase audit, you MUST read and analyze the full text of the sibling book PDF file to obtain complete context, grasp the subtle mathematical/neuromodulatory/logical models, and preserve absolute architectural fidelity.

*   **Target PDF File Name:** `Clean Code.pdf`
*   **Target PDF Path:** `/Users/devonshigaki/Desktop/Books/Clean Code.pdf`

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

### Phase 1: Clean Code
*   *Chapter-Specific Concluding Summary:*
```text
Conclusion............................................................................................49
SetupTeardownIncluder.....................................................................50
Bibliography.........................................................................................52
Chapter 4: Comments.......................................................................53
Comments Do Not Make Up for Bad Code.......................................55
Explain Y ourself in Code....................................................................55
Good Comments..................................................................................55
Legal Comments...............................................................................55
Informative Comments.....................................................................56
Explanation of Intent........................................................................56
Clariﬁcation......................................................................................57
Warning of Consequences................................................................58
TODO Comments.............................................................................58
Ampliﬁcation....................................................................................59
Javadocs in Public APIs....................................................................59
Bad Comments....................................................................................59
Mumbling .........................................................................................59
Redundant Comments ......................................................................60
Misleading Comments.............................................................
```
*   *Audit Focus:* Compare our codebase directly with the clean code guidelines, rules, and code smells extracted above. Review variable naming conventions, function sizes, comment cleanliness, error boundaries, TDD principles, class cohesiveness, and design simple heuristics.

### Phase 2: Meaningful Names
*   *Chapter-Specific Concluding Summary:*
```text
xxix
On the Cover
The image on the cover is M104: The Sombrero Galaxy. M104 is located in Virgo and is
just under 30 million light-years from us. At it’ s core is a supermassive black hole weigh-
ing in at about a billion solar masses.
Does the image remind you of the explosion of the Klingon power moon Praxis? I
vividly remember the scene in Star Trek VIthat showed an equatorial ring of debris ﬂying
away from that explosion. Since that scene, the equatorial ring has been a common artifact
in sci-ﬁ movie explosions. It was even added to the explosion of Alderaan in later editions
of the ﬁrst Star W ars movie.
What caused this ring to form around M104? Why does it have such a huge central
bulge and such a bright and tiny nucleus? It looks to me as though the central black hole
lost its cool and blew a 30,000 light-year hole in the middle of the galaxy. Woe befell any
civilizations that might have been in the path of that cosmic disruption.
Supermassive black holes swallow whole stars for lunch, converting a sizeable frac-
tion of their mass to energy. E = MC2is leverage enough, but when Mis a stellar mass:
Look out! How many stars fell headlong into that maw before the monster was satiated?
Could the size of the central void be a hint? 
The image of M104 on the cover is a
combination of the famous visible light pho-
tograph from Hubble (right), and the recent
infrared image from the Spitzer orbiting
observatory (below, right). It’ s the infrared
image that clearly shows us the ring nature
of the galaxy. In visible light we only see the
front edge of the ring in silhouette. The cen-
tral bulge obscures the rest of the ring.
But in the infrared, the hot particles in
the ring shine through the central bulge. The
two images combined give us a view we’ve
not seen before and i
```
*   *Audit Focus:* Compare our codebase directly with the clean code guidelines, rules, and code smells extracted above. Review variable naming conventions, function sizes, comment cleanliness, error boundaries, TDD principles, class cohesiveness, and design simple heuristics.

### Phase 3: Functions
*   *Chapter-Specific Concluding Summary:*
```text
conclusions.1SoaccountGroupor
bunchOfAccounts or just plain accounts would be better.
1. As we’ll see later on, even if the container is a List, it’ s probably better not to encode the container type into the name. 
20 Chapter 2: Meaningful Names
Beware of using names which vary in small ways. How long does it take to spot the
subtle difference between a XYZControllerForEfficientHandlingOfStringsin one module
and, somewhere a little more distant, XYZControllerForEfficientStorageOfStrings? The
words have frightfully similar shapes.
Spelling similar concepts similarly is information. Using inconsistent spellings is dis-
information. With modern Java environments we enjoy automatic code completion. We
write a few characters of a name and press some hotkey combination (if that) and are
rewarded with a list of possible completions for that name. It is very helpful if names for
very similar things sort together alphabetically and if the differences are very obvious,
because the developer is likely to pick an object by name without seeing your copious
comments or even the list of methods supplied by that class.
A truly awful example of disinformative names would be the use of lower-case Lor
uppercaseOas variable names, especially in combination. The problem, of course, is that
they look almost entirely like the constants one and zero, respectively.
int a = l;
if ( O == l )
  a = O1;
else
  l = 01;
The reader may think this a contrivance, but we have examined code where such
things were abundant. In one case the author of the code suggested using a different font
so that the differences were more obvious, a solution that would have to be passed down to
all future developers as oral tradition or in a written document. The problem is conquered
with ﬁnality and without creating ne
```
*   *Audit Focus:* Compare our codebase directly with the clean code guidelines, rules, and code smells extracted above. Review variable naming conventions, function sizes, comment cleanliness, error boundaries, TDD principles, class cohesiveness, and design simple heuristics.

### Phase 4: Comments
*   *Chapter-Specific Concluding Summary:*
```text
43Function Arguments
Argument Objects
When a function seems to need more than two or three arguments, it is likely that some of
those arguments ought to be wrapped into a class of their own. Consider, for example, the
difference between the two following declarations:
Circle makeCircle(double x, double y, double radius);
Circle makeCircle(Point center, double radius);
Reducing the number of arguments by creating objects out of them may seem like
cheating, but it’ s not. When groups of variables are passed together, the way xand
y are in the example above, they are likely part of a concept that deserves a name of its
own.
Argument Lists
Sometimes we want to pass a variable number of arguments into a function. Consider, for
example, the String.format method:
String.format("%s worked %.2f hours.", name, hours);
If the variable arguments are all treated identically, as they are in the example above, then
they are equivalent to a single argument of type List. By that reasoning, String.format is
actually dyadic. Indeed, the declaration of String.formatas shown below is clearly
dyadic.
public String format(String format, Object... args)
So all the same rules apply. Functions that take variable arguments can be monads,
dyads, or even triads. But it would be a mistake to give them more arguments than
that.
void monad(Integer... args);
void dyad(String name, Integer... args);
void triad(String name, int count, Integer... args);
V erbs and Keywords
Choosing good names for a function can go a long way toward explaining the intent of
the function and the order and intent of the arguments. In the case of a monad, the
function and argument should form a very nice verb/noun pair. For example,
write(name)is very evocative. Whatever this “name” thing is, it is being “written.” An
even be
```
*   *Audit Focus:* Compare our codebase directly with the clean code guidelines, rules, and code smells extracted above. Review variable naming conventions, function sizes, comment cleanliness, error boundaries, TDD principles, class cohesiveness, and design simple heuristics.

### Phase 5: Formatting
*   *Chapter-Specific Concluding Summary:*
```text
63Bad Comments
Misleading Comments
Sometimes, with all the best intentions, a programmer makes a statement in his comments
that isn’t precise enough to be accurate. Consider for another moment the badly redundant
but also subtly misleading comment we saw in Listing 4-1.
Did you discover how the comment was misleading? The method does not return
whenthis.closedbecomestrue. It returns ifthis.closedistrue; otherwise, it waits for a
blind time-out and then throws an exception ifthis.closed is still not true.
This subtle bit of misinformation, couched in a comment that is harder to read than
the body of the code, could cause another programmer to blithely call this function in the
expectation that it will return as soon as this.closedbecomestrue. That poor programmer
would then ﬁnd himself in a debugging session trying to ﬁgure out why his code executed
so slowly.
Mandated Comments
It is just plain silly to have a rule that says that every function must have a javadoc, or
every variable must have a comment. Comments like this just clutter up the code, propa-
gate lies, and lend to general confusion and disorganization. 
For example, required javadocs for every function lead to abominations such as List-
ing 4-3. This clutter adds nothing and serves only to obfuscate the code and create the
potential for lies and misdirection.
Journal Comments
Sometimes people add a comment to the start of a module every time they edit it. These
comments accumulate as a kind of journal, or log, of every change that has ever been
made. I have seen some modules with dozens of pages of these run-on journal entries. 
Listing 4-3 
  /**
   * 
   * @param title The title of the CD
   * @param author The author of the CD
   * @param tracks The number of tracks on the CD
   * @param durationInMinutes
```
*   *Audit Focus:* Compare our codebase directly with the clean code guidelines, rules, and code smells extracted above. Review variable naming conventions, function sizes, comment cleanliness, error boundaries, TDD principles, class cohesiveness, and design simple heuristics.

### Phase 6: Objects and Data Structures
*   *Chapter-Specific Concluding Summary:*
```text
71Bad Comments
Javadocs in Nonpublic Code
As useful as javadocs are for public APIs, they are anathema to code that is not intended
for public consumption. Generating javadoc pages for the classes and functions inside a
system is not generally useful, and the extra formality of the javadoc comments amounts
to little more than cruft and distraction. 
Example
I wrote the module in Listing 4-7 for the ﬁrst XP Immersion. It was intended to be an
example of bad coding and commenting style. Kent Beck then refactored this code into a
much more pleasant form in front of several dozen enthusiastic students. Later I adapted
the example for my book Agile Software Development, Principles, Patterns, and Practices
and the ﬁrst of my Craftsman articles published in Software Development magazine. 
What I ﬁnd fascinating about this module is that there was a time when many of us
would have considered it “well documented.” Now we see it as a small mess. See how
many different comment problems you can ﬁnd.
Listing 4-7 
GeneratePrimes.java
/**
 * This class Generates prime numbers up to a user specified
 * maximum.  The algorithm used is the Sieve of Eratosthenes.
 * <p>
 * Eratosthenes of Cyrene, b. c. 276 BC, Cyrene, Libya --
 * d. c. 194, Alexandria.  The first man to calculate the
 * circumference of the Earth.  Also known for working on
 * calendars with leap years and ran the library at Alexandria.
 * <p>
 * The algorithm is quite simple.  Given an array of integers
 * starting at 2.  Cross out all multiples of 2.  Find the next
 * uncrossed integer, and cross out all of its multiples.
 * Repeat untilyou have passed the square root of the maximum
 * value.
 *
 * @author Alphonse
 * @version 13 Feb 2002 atp
 */
import java.util.*;
public class GeneratePrimes
{
  /**
   * @param maxVal
```
*   *Audit Focus:* Compare our codebase directly with the clean code guidelines, rules, and code smells extracted above. Review variable naming conventions, function sizes, comment cleanliness, error boundaries, TDD principles, class cohesiveness, and design simple heuristics.

### Phase 7: Error Handling
*   *Chapter-Specific Concluding Summary:*
```text
81V ertical Formatting
public int countTestCases() {
int count= 0;
for (Test each : tests)
count += each.countTestCases();
return count;
}
In rare cases a variable might be declared at the top of a block or just before a loop in a
long-ish function. Y ou can see such a variable in this snippet from the midst of a very long
function in TestNG.
...
for (XmlTest test : m_suite.getTests()) {
TestRunner tr = m_runnerFactory.newTestRunner(this, test);
      tr.addListener(m_textReporter);
      m_testRunners.add(tr);
      invoker = tr.getInvoker();
      for (ITestNGMethod m : tr.getBeforeSuiteMethods()) {
        beforeSuiteMethods.put(m.getMethod(), m);
      }
      for (ITestNGMethod m : tr.getAfterSuiteMethods()) {
        afterSuiteMethods.put(m.getMethod(), m);
      }
    }
...
Instance variables,  on the other hand, should be declared at the top of the class. This
should not increase the vertical distance of these variables, because in a well-designed
class, they are used by many, if not all, of the methods of the class.
There have been many debates over where instance variables should go. In C++ we
commonly practiced the so-called scissors rule, which put all the instance variables at the
bottom. The common convention in Java, however, is to put them all at the top of the class.
I see no reason to follow any other convention. The important thing is for the instance vari-
ables to be declared in one well-known place. Everybody should know where to go to see
the declarations. 
Consider, for example, the strange case of the TestSuiteclass in JUnit 4.3.1. I have
greatly attenuated this class to make the point. If you look about halfway down the listing,
you will see two instance variables declared there. It would be hard to hide them in a better
place. Someone reading
```
*   *Audit Focus:* Compare our codebase directly with the clean code guidelines, rules, and code smells extracted above. Review variable naming conventions, function sizes, comment cleanliness, error boundaries, TDD principles, class cohesiveness, and design simple heuristics.

### Phase 8: Boundaries
*   *Chapter-Specific Concluding Summary:*
```text
89Horizontal Formatting
  public FitNesseServer(FitNesseContext context) {
    this.context = context;
  }
  public void serve(Socket s) {
    serve(s, 10000);
  }
  public void serve(Socket s, long requestTimeout) {
    try {
      FitNesseExpediter sender = new FitNesseExpediter(s, context);
      sender.setRequestParsingTimeLimit(requestTimeout);
      sender.start();
    }
    catch (Exception e) {
      e.printStackTrace();
    }
  }
}
Y our eye can rapidly discern the structure of the indented ﬁle. Y ou can almost instantly
spot the variables, constructors, accessors, and methods. It takes just a few seconds to real-
ize that this is some kind of simple front end to a socket, with a time-out. The unindented
version, however, is virtually impenetrable without intense study. 
Breaking Indentation. It is sometimes tempting to break the indentation rule for short
ifstatements, short whileloops, or short functions. Whenever I have succumbed to this
temptation, I have almost always gone back and put the indentation back in. So I avoid col-
lapsing scopes down to one line like this:
public class CommentWidget extends TextWidget
{
public static final String REGEXP = "^#[^\r\n]*(?:(?:\r\n)|\n|\r)?";
public CommentWidget(ParentWidget parent, String text){super(parent, text);}
public String render() throws Exception {return ""; }
}
I prefer to expand and indent the scopes instead, like this:
public class CommentWidget extends TextWidget {
  public static final String REGEXP = "^#[^\r\n]*(?:(?:\r\n)|\n|\r)?";
  public CommentWidget(ParentWidget parent, String text) {
    super(parent, text);
  }
  public String render() throws Exception {
    return "";
  }
}
```
*   *Audit Focus:* Compare our codebase directly with the clean code guidelines, rules, and code smells extracted above. Review variable naming conventions, function sizes, comment cleanliness, error boundaries, TDD principles, class cohesiveness, and design simple heuristics.

### Phase 9: Unit Tests
*   *Chapter-Specific Concluding Summary:*
```text
97The Law of Demeter
Again, we see the complimentary nature of these two deﬁnitions; they are virtual
opposites! This exposes the fundamental dichotomy between objects and data structures: 
Procedural code (code using data structures) makes it easy to add new functions without
changing the existing data structures. OO code, on the other hand, makes it easy to add
new classes without changing existing functions.
The complement is also true: 
Procedural code makes it hard to add new data structures because all the functions must
change. OO code makes it hard to add new functions because all the classes must change.
So, the things that are hard for OO are easy for procedures, and the things that are
hard for procedures are easy for OO!
In any complex system there are going to be times when we want to add new data
types rather than new functions. For these cases objects and OO are most appropriate. On
the other hand, there will also be times when we’ll want to add new functions as opposed
to data types. In that case procedural code and data structures will be more appropriate.
Mature programmers know that the idea that everything is an object is a myth. Some-
times you really do want simple data structures with procedures operating on them.
The Law of Demeter
There is a well-known heuristic called the Law of Demeter2that says a module should not
know about the innards of the objectsit manipulates. As we saw in the last section, objects
hide their data and expose operations. This means that an object should not expose its
internal structure through accessors because to do so is to expose, rather than to hide, its
internal structure. 
More precisely, the Law of Demeter says that a method fof a class Cshould only call
the methods of these:
•C
• An object created by f
public cl
```
*   *Audit Focus:* Compare our codebase directly with the clean code guidelines, rules, and code smells extracted above. Review variable naming conventions, function sizes, comment cleanliness, error boundaries, TDD principles, class cohesiveness, and design simple heuristics.

### Phase 10: Classes
*   *Chapter-Specific Concluding Summary:*
```text
111Don’t Pass Null
In many cases, special case objects are an easy remedy. Imagine that you have code
like this:
List<Employee> employees = getEmployees();
if (employees != null) {
  for(Employee e : employees) {
    totalPay += e.getPay();
  }
}
Right now, getEmployeescan return null, but does it have to? If we change getEmployeeso
that it returns an empty list, we can clean up the code:
List<Employee> employees = getEmployees();
for(Employee e : employees) {
  totalPay += e.getPay();
}
Fortunately, Java has Collections.emptyList(), and it returns a predeﬁned immutable list
that we can use for this purpose:
public List<Employee> getEmployees() {
  if( .. there are no employees .. ) 
    return Collections.emptyList();
}
If you code this way, you will minimize the chance of NullPointerExceptionsand your
code will be cleaner.
Don’t Pass Null
Returning nullfrom methods is bad, but passing nullinto methods is worse. Unless you
are working with an API which expects you to pass null, you should avoid passing null in
your code whenever possible.
Let’ s look at an example to see why. Here is a simple method which calculates a met-
ric for two points:
public class MetricsCalculator 
{
  public double xProjection(Point p1, Point p2) {
    return (p2.x – p1.x) * 1.5;
  }
  …
}
What happens when someone passes null as an argument?
calculator.xProjection(null, new Point(12, 13));
We’ll get a NullPointerException, of course.
How can we ﬁx it? We could create a new exception type and throw it:
public class MetricsCalculator 
{
```
*   *Audit Focus:* Compare our codebase directly with the clean code guidelines, rules, and code smells extracted above. Review variable naming conventions, function sizes, comment cleanliness, error boundaries, TDD principles, class cohesiveness, and design simple heuristics.

### Phase 11: Systems
*   *Chapter-Specific Concluding Summary:*
```text
Conclusion
We have barely scratched the surface of this topic. Indeed, I think an entire book could be
written about clean tests. Tests are as important to the health of a project as the production
code is. Perhaps they are even more important, because tests preserve and enhance the
ﬂexibility, maintainability, and reusability of the production code. So keep your tests con-
stantly clean. Work to make them expressive and succinct. Invent testing APIs that act as
domain-speciﬁc language that helps you write the tests. 
If you let the tests rot, then your code will rot too. Keep your tests clean.
Bibliography
[RSpec]:  RSpec: Behavior Driven Development for Ruby Programmers,
Aslak Hellesøy, David Chelimsky, Pragmatic Bookshelf, 2008.
[GOF]:  Design Patterns: Elements of Reusable Object Oriented Software, Gamma et al.,
Addison-Wesley, 1996.
```
*   *Audit Focus:* Compare our codebase directly with the clean code guidelines, rules, and code smells extracted above. Review variable naming conventions, function sizes, comment cleanliness, error boundaries, TDD principles, class cohesiveness, and design simple heuristics.

### Phase 12: Emergence
*   *Chapter-Specific Concluding Summary:*
```text
145Classes Should Be Small!
Listing 10-8 
PrimeGenerator.java
package literatePrimes;
import java.util.ArrayList;
public class PrimeGenerator {
  private static int[] primes;
  private static ArrayList<Integer> multiplesOfPrimeFactors;
  protected static int[] generate(int n) {
    primes = new int[n];
    multiplesOfPrimeFactors = new ArrayList<Integer>();
    set2AsFirstPrime();
    checkOddNumbersForSubsequentPrimes();
    return primes;
  }
  private static void set2AsFirstPrime() {
    primes[0] = 2;
    multiplesOfPrimeFactors.add(2);
  }
  private static void checkOddNumbersForSubsequentPrimes() {
    int primeIndex = 1;
    for (int candidate = 3;
         primeIndex < primes.length;
         candidate += 2) {
      if (isPrime(candidate))
        primes[primeIndex++] = candidate;
    }
  }
  private static boolean isPrime(int candidate) {
    if (isLeastRelevantMultipleOfNextLargerPrimeFactor(candidate)) {
      multiplesOfPrimeFactors.add(candidate);
      return false;
    }
    return isNotMultipleOfAnyPreviousPrimeFactor(candidate);
  }
  private static boolean
  isLeastRelevantMultipleOfNextLargerPrimeFactor(int candidate) {
    int nextLargerPrimeFactor = primes[multiplesOfPrimeFactors.size()];
    int leastRelevantMultiple = nextLargerPrimeFactor * nextLargerPrimeFactor;
    return candidate == leastRelevantMultiple;
  }
  private static boolean 
  isNotMultipleOfAnyPreviousPrimeFactor(int candidate) {
    for (int n = 1; n < multiplesOfPrimeFactors.size(); n++) {
      if (isMultipleOfNthPrimeFactor(candidate, n))
        return false;
    }
```
*   *Audit Focus:* Compare our codebase directly with the clean code guidelines, rules, and code smells extracted above. Review variable naming conventions, function sizes, comment cleanliness, error boundaries, TDD principles, class cohesiveness, and design simple heuristics.

### Phase 13: Concurrency
*   *Chapter-Specific Concluding Summary:*
```text
155Separate Constructing a System from Using It
However, we now have a hard-coded dependency on MyServiceImpland everything its
constructor requires (which I have elided). We can’t compile without resolving these
dependencies, even if we never actually use an object of this type at runtime! 
Testing can be a problem. If MyServiceImplis a heavyweight object, we will need to
make sure that an appropriate TEST DOUBLE1or MOCK OBJECTgets assigned to the ser-
vice ﬁeld before this method is called during unit testing. Because we have construction
logic mixed in with normal runtime processing, we should test all execution paths (for
example, thenulltest and its block). Having both of these responsibilities means that the
method is doing more than one thing, so we are breaking the Single Responsibility Principle
in a small way.
Perhaps worst of all, we do not know whether MyServiceImplis the right object in all
cases. I implied as much in the comment. Why does the class with this method have to
know the global context? Can we everreally know the right object to use here? Is it even
possible for one type to be right for all possible contexts?
One occurrence of LAZY-INITIALIZATIONisn’t a serious problem, of course. However,
there are normally many instances of little setup idioms like this in applications. Hence,
the global setup strategy(if there is one) is scatteredacross the application, with little
modularity and often signiﬁcant duplication. 
If we are diligent about building well-formed and robust systems, we should never let
little,convenient idioms lead to modularity breakdown. The startup process of object con-
struction and wiring is no exception. We should modularize this process separately from
the normal runtime logic and we should make sure that we have a global, co
```
*   *Audit Focus:* Compare our codebase directly with the clean code guidelines, rules, and code smells extracted above. Review variable naming conventions, function sizes, comment cleanliness, error boundaries, TDD principles, class cohesiveness, and design simple heuristics.

### Phase 14: Successive Refinement
*   *Chapter-Specific Concluding Summary:*
```text
229String Arguments
  public String usage() {
    if (schema.length() > 0)
      return "-[" + schema + "]";
    else
      return "";
  }
  public String errorMessage() throws Exception {
    switch (errorCode) {
      case OK:
        throw new Exception("TILT: Should not get here.");
      case UNEXPECTED_ARGUMENT:
        return unexpectedArgumentMessage();
      case MISSING_STRING:
        return String.format("Could not find string parameter for -%c.", 
                             errorArgumentId);
      case INVALID_INTEGER:
        return String.format("Argument -%c expects an integer but was '%s'.", 
                             errorArgumentId, errorParameter);
      case MISSING_INTEGER:
        return String.format("Could not find integer parameter for -%c.", 
                             errorArgumentId);
    }
    return "";
  }
  private String unexpectedArgumentMessage() {
    StringBuffer message = new StringBuffer("Argument(s) -");
    for (char c : unexpectedArguments) {
      message.append(c);
    }
    message.append(" unexpected.");
    return message.toString();
  }
  public boolean getBoolean(char arg) {
    Args.ArgumentMarshaler am = marshalers.get(arg);
    boolean b = false;
    try {
      b = am != null && (Boolean) am.get();
    } catch (ClassCastException e) {
      b = false;
    }
    return b;
  }
  public String getString(char arg) {
    Args.ArgumentMarshaler am = marshalers.get(arg);
    try {
      return am == null ? "" : (String) am.get();
    } catch (ClassCastException e) {
      return "";
    }
  }
Listing 14-12 (continued)
Args.java (After first refactoring)
```
*   *Audit Focus:* Compare our codebase directly with the clean code guidelines, rules, and code smells extracted above. Review variable naming conventions, function sizes, comment cleanliness, error boundaries, TDD principles, class cohesiveness, and design simple heuristics.

### Phase 15: JUnit Internals
*   *Chapter-Specific Concluding Summary:*
```text
245String Arguments
      new ArgsException(ArgsException.ErrorCode.UNEXPECTED_ARGUMENT, 
                        'x', null);
    assertEquals("Argument -x unexpected.", e.errorMessage());
  }
  public void testMissingStringMessage() throws Exception {
ArgsException e = new ArgsException(ArgsException.ErrorCode.MISSING_STRING, 
                                        'x', null);
    assertEquals("Could not find string parameter for -x.", e.errorMessage());
  }
  public void testInvalidIntegerMessage() throws Exception {
    ArgsException e = 
      new ArgsException(ArgsException.ErrorCode.INVALID_INTEGER, 
                        'x', "Forty two");
    assertEquals("Argument -x expects an integer but was 'Forty two'.", 
                 e.errorMessage());
  }
  public void testMissingIntegerMessage() throws Exception {
    ArgsException e = 
      new ArgsException(ArgsException.ErrorCode.MISSING_INTEGER, 'x', null);
    assertEquals("Could not find integer parameter for -x.", e.errorMessage());
  }
  public void testInvalidDoubleMessage() throws Exception {
ArgsException e = new ArgsException(ArgsException.ErrorCode.INVALID_DOUBLE, 
                                        'x', "Forty two");
    assertEquals("Argument -x expects a double but was 'Forty two'.", 
                 e.errorMessage());
  }
  public void testMissingDoubleMessage() throws Exception {
ArgsException e = new ArgsException(ArgsException.ErrorCode.MISSING_DOUBLE, 
                                        'x', null);
    assertEquals("Could not find double parameter for -x.", e.errorMessage());
  }
}
Listing 14-15 
ArgsException.java
public class ArgsException extends Exception {
  private char errorArgumentId = '\0';
  private String errorParameter = "TILT";
  private ErrorCode errorCode = ErrorCode
```
*   *Audit Focus:* Compare our codebase directly with the clean code guidelines, rules, and code smells extracted above. Review variable naming conventions, function sizes, comment cleanliness, error boundaries, TDD principles, class cohesiveness, and design simple heuristics.

### Phase 16: Refactoring SerialDate
*   *Chapter-Specific Concluding Summary:*
```text
281Then Make It Right
The next function is getNearestDayOfWeek(lines 695–726), which we corrected back
on page 270. But the changes I made back then aren’t consistent with the current pattern in
the last two functions [G11]. So I made it consistent and used some EXPLAINING TEMPO-
RARY VARIABLES [G19] to clarify the algorithm.
ThegetEndOfCurrentMonthmethod (lines 728–740) is a little strange because it is an
instance method that envies [G14] its own class by taking a DayDateargument. I made it a
true instance method and clariﬁed a few names.
Refactoring weekInMonthToString(lines 742–761) turned out to be very interesting
indeed. Using the refactoring tools of my IDE, I ﬁrst moved the method to the WeekInMonth
enum that I created back on page 275. Then I renamed the method to toString. Next, I
changed it from a static method to an instance method. All the tests still passed. (Can you
guess where I am going?) 
Next, I deleted the method entirely! Five asserts failed (lines 411–415, Listing B-4,
page 374). I changed these lines to use the names of the enumerators ( FIRST,
SECOND,. . .). All the tests passed. Can you see why? Can you also see why each of these
steps was necessary? The refactoring tool made sure that all previous callers of
weekInMonthToStringnow called toStringon the weekInMonthenumerator because all enu-
merators implement toString to simply return their names. . . .
Unfortunately, I was a bit too clever. As elegant as that wonderful chain of refactor-
ings was, I ﬁnally realized that the only users of this function were the tests I had just mod-
iﬁed, so I deleted the tests.
Fool me once, shame on you. Fool me twice, shame on me! So after determining that
nobody other than the tests called relativeToString(lines 765–781), I simply deleted the
function and
```
*   *Audit Focus:* Compare our codebase directly with the clean code guidelines, rules, and code smells extracted above. Review variable naming conventions, function sizes, comment cleanliness, error boundaries, TDD principles, class cohesiveness, and design simple heuristics.

### Phase 17: Smells and Heuristics
*   *Chapter-Specific Concluding Summary:*
```text
319Client/Server Example
Adding Threading
Assume for the moment that the performance test fails. How can we improve the through-
put so that the performance test passes? If the process method of the server is I/O bound,
then here is one way to make the server use threads (just change the processMessage):
void process(final Socket socket) {
    if (socket == null)
        return;
    Runnable clientHandler = new Runnable() {
        public void run() {
            try {
                String message = MessageUtils.getMessage(socket);
                MessageUtils.sendMessage(socket, "Processed: " + message);
                closeIgnoringException(socket);
            } catch (Exception e) {
                e.printStackTrace();
            }
        }
    };
    Thread clientConnection = new Thread(clientHandler);
    clientConnection.start();
}
Assume that this change causes the test to pass;1 the code is complete, correct?
Server Observations
The updated server completes the test successfully in just over one second. Unfortunately,
this solution is a bit naive and introduces some new problems.
How many threads might our server create? The code sets no limit, so the we could
feasibly hit the limit imposed by the Java Virtual Machine (JVM). For many simple sys-
tems this may sufﬁce. But what if the system is meant to support many users on the public
net? If too many users connect at the same time, the system might grind to a halt.
But set the behavioral problem aside for the moment. The solution shown has prob-
lems of cleanliness and structure. How many responsibilities does the server code have? 
•Socket connection management
•Client processing
•Threading policy
•Server shutdown policy
Unfortunately, all these responsibilities live in the process function. In addition,
```
*   *Audit Focus:* Compare our codebase directly with the clean code guidelines, rules, and code smells extracted above. Review variable naming conventions, function sizes, comment cleanliness, error boundaries, TDD principles, class cohesiveness, and design simple heuristics.

---

## MANDATORY DELIVERABLE: CRITICAL PLAN & AUTOMATED CI/CD ENFORCEMENT
Perform your codebase discovery through the lens of this work and map how we worked to build on this knowledge. Once complete, you must compile and deliver:

1. **Executive Summary & Lens Review:** An evaluation of the codebase and application specifically through the lens of this work, detailing how we built on this knowledge to benefit the application and codebase.
2. **Phase Audit Matrix:** A table mapping *Phase* | *Citations/Files* | *Identified Gaps, Risks, or Overlaps* | *Priority (H/M/L)*.
3. **Critical Adjustment Plan:** If any critical gaps or boundary overlaps are identified, form a full, actionable plan to adjust and partition roles and responsibilities.
4. **Automated CI/CD Enforcement Script:** Write a fully functional automated validation script (e.g., Python validation check, Bash Git-hook, custom linter rule, or unit test assertions) that can be integrated directly into our current build scripts and CI/CD deployment pipelines to continuously maintain and enforce these rules!
