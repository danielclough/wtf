
<script>
  import FaceRecognition from '$lib/face-recognition.svelte';
  import SvelteMarkdown from 'svelte-markdown'

  const source = `
  # WTF - What The Fuck

## Problem Statement

Nobody has presented comprehensive and satisfactory philosophical system for ethics.

## Project Aim

0. Create a satisfactory system for "Wisdom Lovers" to engage constructively towards personal and group efforts.

1. Engage with sense making practices in public in order to crowd source develop better methods of truth finding and collective action.

Method is a broad and complex domain, for example, 
I'm not sold on "political correctness" - or even politeness - but my readings and discussions with regards "oppression" have convinced me of the importance of "safe space".

As essential to our method we will take heed this wisdom.

2. Create "sufficiently" "safe" "space" for "free speech".

I think all of the above quoted terms need clarification.

"Speech" may be dubious, but "Speech Act Theory", can be used if no objections are raised.

Obviously "Fuck" in the title of the game set's a norm...

This video presents an interesting take on the use of profanity.

<iframe style="margin:0 auto;width:58vw;height:43vw"  src="https://www.youtube.com/embed/1BcdY_wSklo" title="Steven Pinker - The Language of Swearing (1/2)" frameborder="0" allow="clipboard-write; encrypted-media; gyroscope; picture-in-picture" allowfullscreen></iframe>

If you can't accept the most popular expletive in the English language in conversation you should find another game to play, no offense intended, I use profanity in my speech sometimes.

I think it's a good idea to keep things Safe For Work (SFW), but what is safe for my work may not be safe for your work...

Consider that "WTF" is safer than "What The Fuck".

Obviously this is a game for adults.

Use "common sense".

3. Develop a game for clarifying what the goal of the game is; Including a process for changing the rules.

## WTF is going on here?

I think it would be useful for society if more people took an active interest in systematic sense making - and many people have told me that they think I could get financial support for a project supporting that social movement - maybe a game would be fun?

---

> For non-philosophers,
> 
> Resolving apparent conflict often nothing more than involves seeing a broader truth.
> 
> Ideas about truth have a lot to do with history - what does that mean for us.
>
> Even when we disagree, or are disrespectful. Respect intelligent sense making. Respect does't mean emulation.
> 
> Just because people disagree doesn't mean their ideas are not valid and useful, even if one can not see validity or use in the other.
> 
> I don't find any value in pessimism, but it's a relatable position.
> 
> "Venting" is not evil, but I can't advocate for destructive behavior, and expressing negative thoughts should be done with care towards how they impact ones goals - and other presumably sentient beings.
> 
> I think that it is very important to have social cohesiveness, and that is a complicated issue. It seems important to work within social systems and common thought if you want to be happy, productive, and appreciated for your contribution.
> 
> I like to question everything, if for no other reason than habit.
> 
> I generally don't like "shoulding on people".
> 
> While I want all ideas to find clear honest expression, it is important that there is an easy way to stop at any time.

---

> For philosophers,
> 
> I have respect for Hegel and Nietzsche - and love Popper's roasting... but WTF was Schopenhauer trying to do?
> 
> I agree with Dennett that, "we should get in the habit of answering rhetorical questions", but I have no interest in getting people to believe anything in particular, and generally don't like normative ethics and linguistics outside of circumscribed exercises.
> 
> I feel it important that players can express Voice clearly and Exit easily.
---


### CODE OF CONDUCT

[Link]() *broken*

### Inspiration

First and foremost - Bud Clough - Poet, Philosopher, Father.

Heraclitus, Socrates, and others had some pretty good ideas - or so I hear - I can't very well say they are inspiration, although it would be hard not to...

I consider myself affected by "The Postmodern Condition", but I don't like "Post Modernism" all that much.

I don't particularly like Plato, and I generally don't read work older than 100 years or so.

I think the oldest work I have enjoyed unabridged in the original language is William James (The Varieties of Religious Experience; 1902).

I learned Chinese in order to engage with the Daoist community at Wudang in China, and learned some basic Sanskrit while spending time with a Yoga Cult.

I have participated in a number of Cults.
I have formally joined two, with enrollment in a third as a child being officiated by my father.

## Copyright

Any work which is young enough to be under copyright will include attribution and/or citation as appropriate.

I am using the Street Epistemology Survey on the Epistemology page. I don't know if the survey is original enough to be copyrightable, but the logo certainly is.
I have reached out to the community about using their material, and assuming fair use until someone say's otherwise.

## Citations and Credit

It's a good idea when it works, but AI can sort credit later if it's not easy or required by law.
  `
</script>

<svelte:head>
	<title>About</title>
	<meta name="description" content="WTF WTF?" />
</svelte:head>

<FaceRecognition />
  <SvelteMarkdown {source} />
