# Method

Explore an Idea

Goals:

0. Cooperatively explore topics which matter to people around me.


Assumptions:

0. Positive feeling precedes cooperative communication.
0. Not all disagreements can be resolved.
0. Disagreement does not necessitate conflict.
0. Not all conflicts can be resolved.
0. Cooperation can maximize value for all parties even when conflict is irreconcilable.


Rules:

0. Follow a "Tomato Clock" /w breaks.
   - Play / Break / LongBreak
   - 9 / 3 / 9
   - 20 / 5 / 20
0. Anyone can stop the conversation for any reason with a TimeOut or a GameOver
  - Person who initiates GameOver must leave the space within a reasonable timeframe.
  - TimeOut can be started with hand or verbal signal and everyone must stop talking immediately.
  - Timeout lasts for the duration of a LongBreak or until TimeIn is called by same person.
  - Each player get's 2 TimeOuts per game.
0. One person speaks at a time.
0. Non-normatively (excluding "positivism").
   - "No should-ing on others" - but evidence of a claim is expected if asked for.
   - Use logic and data to make arguments.



## Overview

0. Clarify Values
0. Formulate Problem Statement
0. Formulate Goal
0. Identify Causal Factors
0. Consider Theory
0. Formulate Action Plan
0. Consider Resolution
0. Ratify Resolution



### Theory

0. Develop Ontology
   - Forces
   - Objects
   - Events
0. Validate Epistemology
   - Intuitive
   - Bayesian
0. Consider Ethical Implications
   - Personal
   - Institutional



---

## Example

### Values

- Individual Flourishing
- Collaboration (Gricean Pragmatics)
    - Quantity
        - Try to be informative.
        - Do not be more informative than is required.
    - Quality
        - Try to convey truth.
        - Do not say what you believe to be false.
        - Do not say that for which you lack adequate evidence.
    - Relation
        - Try to be relevant.
    - Manner
        - Try to be easy to understand.
        - Avoid obscure, ambiguous, lengthy, disordered statements.
        - Frame your ideas to facilitate an appropriate reply.


### Problem

> Coming to a clear understanding of any problem using Metaphysics is not reliable.

### Goal

> Create a Problem Solving Game based on "Positive Methodology".

### Propositions

 - Metaphysics form an unreliable foundation for solving problems.

 - Metaphysical Ontology does not allow for clear linking of causal relationships.

 - Relationships which used to be explained by Metaphysics are now explained by Physics.
    - This is because problems with Physics can be demonstrated and explored in relatively unambiguous terms, and by using models which rely on mathematics to ensure logical coherence.

 - Metaphysics are not "bad" - just indeterminate.

### Metaphysical Ontology

```mermaid
classDiagram
    class Metaphysics{
        Anything
        ...
        creation_from_nothing() -> Universe
        ....() -> Anything

    }
    Metaphysics --> Universe
    Metaphysics --|> Universe
    Metaphysics --> Universe
    Metaphysics --|> Universe
    Metaphysics --> Universe
    Metaphysics --|> Universe
    Metaphysics --> Universe
    Metaphysics --|> Universe
    Metaphysics --> Universe
    Metaphysics --|> Universe
    Metaphysics --> Universe
    Metaphysics --|> Universe
    Metaphysics --> Universe
    Metaphysics --|> Universe
    Metaphysics --> Universe
    Metaphysics --|> Universe
    Metaphysics --> Universe
    Metaphysics --|> Universe
    Metaphysics --> Universe
    Metaphysics --|> Universe
    Metaphysics --> Universe
    Metaphysics --|> Universe
    Metaphysics --> Universe
    Metaphysics --|> Universe
    Metaphysics --> Universe
    Metaphysics --|> Universe
    Metaphysics --> Universe
    Metaphysics --|> Universe
    Metaphysics --> Universe
    Metaphysics --|> Universe
    Metaphysics --> Universe
    Metaphysics --|> Universe
    Metaphysics --> Universe
    Metaphysics --|> Universe
    Metaphysics --> Universe
    Metaphysics --> Galaxy
    Metaphysics --> Planet
    Metaphysics --> Life
    Metaphysics --> Eukarya
    Metaphysics --> Human
    Metaphysics --> Society

    class Universe{
        Galaxy
        static_friction(n) -> n
        kinetic_friction(n) -> n
        centripetal_force(n) -> n
        gravity(n) -> n
        electric_force(n) -> n
        magnetic_force(n) -> n
        ....(Numbers) ->  Numbers
    }
    Universe --|> Galaxy
    Universe --> Life
    Universe --> Human

    class Galaxy{
        Black Hole
        Planet
    }
    Galaxy --|> Planet

    class Planet{
        Life
        NonLife
        NaturalSelection(NonLife) -> Life
    }
    Planet --|> Life

    class Life{
         Archaea
         Bacteria
         Eukarya
    }
    Life --|> Eukarya

    class Eukarya{
        Human
        Plant
        Fungi
        Slime Mold
        Flagellates
    }
    Eukarya --|>  Human

    class Human{
        idea
        communication(idea) -> Society
    }
    Human --|> Society

    class Society{
        problem
        solution(problem) -> Flourishing | Suffering
    }
```

> Applying "Occam's Razor" makes the ontology easier to deal with:


```mermaid
classDiagram
    class Universe{
        Galaxy
        static_friction(n) -> n
        kinetic_friction(n) -> n
        centripetal_force(n) -> n
        gravity(n) -> n
        electric_force(n) -> n
        magnetic_force(n) -> n
        ....(Numbers) ->  Numbers
    }
    Universe --|> Galaxy
    Universe --> Life
    Universe --> Human

    class Galaxy{
        Black Hole
        Planet
    }
    Galaxy --|> Planet

    class Planet{
        Life
        NonLife
        NaturalSelection(NonLife) -> Life
    }
    Planet --|> Life

    class Life{
         Archaea
         Bacteria
         Eukarya
    }
    Life --|> Eukarya

    class Eukarya{
        Human
        Plant
        Fungi
        Slime Mold
        Flagellates
    }
    Eukarya --|>  Human

    class Human{
        idea
        communication(idea) -> Society
    }
    Human --|> Society

    class Society{
        problem
        solution(problem) -> Flourishing | Suffering
    }
```


### Positive Epistemology

Why should you accept that life proceeds by a process of Natural Selection and not by a Metaphysical Miracle?  

You "shouldn't".

You may believe whatever you find compelling and and useful, and to consider your beliefs in ways that can be tested.

It is important to recognize our `Fundamental Ignorance` which leaves open the possibility for new discovery.

A valid model is not expected to explain everything.

Valid models should have some predictive power.

> Note: that we can only update our Knowledge by executing a Test or (Axiomatically) by exploring a Paradigm.

```mermaid
stateDiagram-v2
    Knowledge --> Theory
    Knowledge --> Paradigm
    Paradigm --> Knowledge
    Paradigm --> Analysis
    Theory --> Proposition
    Proposition --> Test
    Test --> Analysis 
    Analysis --> Knowledge
```

 - `Knowledge` == Sensible information stored in human memory.
 - `Paradigm` == Predictive Models /w explanation; taken to be true.
 - `Theory` == Provisional models which fit with a given paradigm.
 - `Proposition` == Statement which can be tested.
 - `Test` == Method of testing propositions.
 - `Analysis` == Consider what the test means in relationship to the working paradigm.


#### Bayesian Credence

<iframe width="741" height="417" src="https://www.youtube.com/embed/HZGCoVF3YvM" title="Bayes theorem, the geometry of changing beliefs" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" allowfullscreen></iframe>

### Positive Ethics

 - Ethics is a highly speculative subject.
 - Much of the subject is based on metaphysical concepts.
 - Legal Positivism is a highly relevant.
 - Sociology is dubious, and becoming more positive over time.
 

#### Pragmatics

Physics and Biology are rigorous subjects with clear definitions, repeated experiment, and peer review - however many relevant problems are social and not physical in nature.

For this we have Pragmatics:

>  The facts with which pragmatics deals are of various sorts, including:
> - Facts about the objective facts of the utterance, including: who the speaker is, when the utterance occurred, and where;
> - Facts about the speaker’s intentions. ...what language the speaker intends to be using, what meaning she intends to be using, whom she intends to refer... and the like. ...what she intends to achieve by saying what she does.
> - Facts about beliefs of the speaker and those to whom she speaks, and the conversation they are engaged in; what beliefs do they share; what is the focus of the conversation, what are they talking about, etc.
> - Facts about relevant social institutions, such as promising, marriage ceremonies, courtroom procedures, and the like, which affect what a person accomplishes in or by saying what she does.
> [--Stanford.edu](https://plato.stanford.edu/entries/pragmatics/)

Pragmatic facts of social life 