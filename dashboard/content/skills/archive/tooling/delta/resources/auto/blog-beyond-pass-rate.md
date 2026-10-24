+++
title = "blog-beyond-pass-rate"
[extra]
skill = false
category = "tooling"
mermaid = false
skill_name = "delta"
archived = "2026-10-18"
source_url = "https://github.com/MAHDTech/agent-skills/tree/main/skills-archive/tooling/delta"
+++

---
title: "Beyond Pass Rate: Harness Evals, and Their Blind Spots — Delta Blog"
url: "https://delta.dev/blog/beyond-pass-rate"
description: "Delta keeps pace with native harnesses on public evals."
---

Blog

# Beyond Pass Rate: Harness Evals, and Their Blind Spots

Anant GoelSeptember 24, 2026

Whatever lab you get tokens from likely gives you an agent harness along with it. The labs building these harnesses have a level of model access that nobody else has, and therefore often the model is trained inside the harness.

Switching harnesses always involves trade-offs. When you bring tokens into Delta from other tools, what are you giving up?

The current thing™ in agent evaluation is to rely on a subset of popular evals to compare the pass rate and cost of models inside their native harnesses. **On the models we tested, Delta matches or exceeds the native harness on accuracy for all of them, while costing 0.80×–1.12× as much per pass** depending on the model. Delta was cheaper per pass on Sol, roughly on par on Fable, and slightly more expensive on Astra and Opus. Given we use one system prompt across all models, I expected some model families to perform better than others, but instead I found both accuracy and costs for Delta to be competitive with their native harnesses with no directional result.

### Models do as well in Delta as in their own harness

What changes when each model runs in Delta instead of its native harness.

Model (effort)

Harness

Passed

Cost per pass[1](#user-content-fn-costperpass)

GPT-5.6 Sol (xhigh)

Delta

21 / 29

$0.88

Codex

19 / 29

$1.10

GPT-6 Astra (xhigh)

Delta

25 / 29

$1.83

Codex

24 / 29

$1.65

Claude Opus 5 (high)

Delta

24 / 29

$1.75

Claude Code

24 / 29

$1.56

Claude Fable 5.1[2](#user-content-fn-fable) (high)

Delta

20 / 25

$1.63

Claude Code

20 / 25

$1.54

Source: Terminal-Bench 2.1, 29 tasks (25 for Fable) with the same reasoning effort in both harnesses.

A similar trend continues with open-weight models such as Kimi K3 \[[6](#ref-6)\], where Delta exceeds popular harnesses on accuracy while being competitive on cost.

### Delta solves the most tasks at lower cost

Each dot is one harness running Kimi K3 on the same 30 tasks.

Harness

Passed

Cost per pass

Delta

21 / 30

$2.95

Codex

20 / 30

$3.47

DSH Creator

19 / 30

$3.28

Claude Code

19 / 30

$18.34

Pi

18 / 30

$2.43

DSH Standard

18 / 30

$3.46

DSH PTC

18 / 30

$4.58

Kimi Code

17 / 30

$3.65

DSH Minimal

17 / 30

$4.72

Oh My Pi

17 / 30

$4.75

Exo Harness

16 / 30

$1.05

Hermes

15 / 30

$2.90

OpenCode

15 / 30

$3.24

Sources: Delta from our own Harbor runs of the 30 FrontierHarness tasks (Sept 23, 2026). All other harnesses from results published at frontierharness.org (Aug 22, 2026).

Time to pop the champagne, right? For us, not yet. These numbers don’t describe any of the reasons we at Zed love using [Delta](https://zed.dev/blog/introducing-delta), or why we even built it. Delta is collaborative; it keeps the conversation and the code in one document and keeps you in the loop while the software is written.

Because public evals can’t showcase how our harness feels different, I want to explore more of what evals tell us today, and what they can’t yet capture.

## [Pass rates can’t reflect how code looks](#pass-rates-cant-reflect-how-code-looks)

Pass rates do matter, but the metric has known limitations. We could tell you Delta has a better pass rate than Codex for GPT-5.6 Sol, but this does not convey much of the human experience of using either tool. Pass rate can’t measure what the code looks like, how many files the agent touched, or whether a teammate could review it.

At Zed, each eval rollout is a thread we can open and inspect in Delta itself. This makes it easier to dig into the behavior of any model in Delta.

Let’s take a look at an example together and compare it against the same model completing the same task in its native harness:

In `code-from-image` from Terminal-Bench \[[7](#ref-7)\], the agent gets a picture of a pseudocode snippet, has to implement it and write the value it prints to `output.txt`. The snippet hashes the image file, then hashes that digest together with its first 10 bytes and a salt.

GPT-5.6 Sol in Codex worked out the answer with a one-off command in its shell:

```
node -e "const fs=require('fs'),c=require('crypto'); const b=fs.readFileSync('/app/code.png'); const h0=c.createHash('sha256').update(b).digest(); const H=c.createHash('sha256').update(Buffer.concat([h0,h0.subarray(0,10),Buffer.from('0000TBENCH-SALT')])).digest('hex'); process.stdout.write(H+'\n')"
```

Then it wrote the 64-character result into `output.txt`, and that file is all it left behind. The implementation the task asked for only ever existed inside that one command.

The same model in Delta wrote the same `output.txt`, and next to it `solution.py`:

```
from hashlib import sha256
from pathlib import Path


salt = b"0000TBENCH-SALT"
image_bytes = Path(__file__).with_name("code.png").read_bytes()
h0 = sha256(image_bytes).digest()
result = sha256(h0 + h0[:10] + salt).hexdigest()

print(result)
```

Before finishing, it ran `python3 solution.py` and compared the output against `output.txt`. A teammate opening this change can read the logic, rerun it, and see where the number came from.

We see a similar difference when using Claude Fable 5.1 in Claude Code and Delta. Neither Claude Code nor Delta left a script this time. In both, Fable worked out the hash in a throwaway Python snippet and checked it against a hint the task gives. The difference is the last step. Claude Code wrote the file by pasting the answer back in:

```
printf 'bee26a133f103b9ecda444c70ec22cafef6e31a3de7af6d047974dc90ce3defe\n' > /app/output.txt
```

In Delta, the code that hashed the image also wrote the file:

```
import hashlib
h0 = hashlib.sha256(open('/app/code.png','rb').read()).digest()
H = hashlib.sha256(h0 + h0[:10] + b'0000TBENCH-SALT').hexdigest()
open('/app/output.txt','w').write(H + '\n')
print(H)
```

If you’re just looking at the eval metrics, you wouldn’t notice this difference, but if you dig into the code, they’re different styles of writing code. In Delta, we don’t fold away the agent’s output from you the way other harnesses do, so we care deeply about how that output appears. Delta’s collaborative nature extends not just between two humans, but also to the human-agent interactions.

## [(Tool calls) failing upwards](#tool-calls-failing-upwards)

Another example of something invisible to the evals yet obvious (and painful) to a developer is tool call failures. If an agent takes 5 attempts to make a tool call instead of just 1, are both equally correct? The eval scores would say yes, but [users can feel the pain of the trade-offs made in the harnesses](https://lucumr.pocoo.org/2026/7/4/better-models-worse-tools/) \[[1](#ref-1)\].

Claude Code, for example, has high [reported failure rates](https://github.com/earendil-works/pi/issues/6278#issuecomment-4890765697). When I use Claude Code, I don’t see any of these failures because the harness silently absorbs them, but I feel the time I spend waiting on repeated requests. Delta and Claude Code are both getting the same tokens at the same speed, and Delta’s failure rate is ~1.5% on Anthropic’s models. The difference is that we don’t have to hide failures to get there, because we have DeltaDB.

Instead of hiding and retrying failed tool calls, we use our increased context to do something different (and better, we believe). Because Delta runs on DeltaDB, our harness knows exactly which version of a file the model read and what a past agent did, and we can use that intent and history to make better inferences that avoid failures in the first place. It’s not that we won’t ever fail, but richer context lets our harness be wiser without folding failures away.

Indulge me in an example: an edit that matches two places in the same file. Between the model reading the file and issuing a patch for it, the file can change: a formatter runs, the model’s own earlier edit shifts the lines, or a teammate types in the same buffer. Here’s how Delta uses its increased context to apply that tool call successfully:

### How Delta handles ambiguous edits

When a patch matches two lines, Delta knows which one the model meant.

After the teammate's import, the patch matches two lines. Most harnesses reject it and the model reads the file again; Delta applies it to the line the model read, now line 9.

No retry, no re-read, no prompt guidance spent explaining any of it. And if the teammate had changed that line itself, Delta wouldn’t guess: it tells the model the content is stale and to read the file again.

## [The art of arranging tool calls](#the-art-of-arranging-tool-calls)

Even on successful tool calls, the ordering of those tool calls make a difference in the perceived speed of a model. Say you ask an agent to build a new feature (a pretty common way we use Delta ourselves). It will usually start by making a handful of tool calls to gather context around your codebase. You could arrange these tool calls in parallel, sequentially or in some combination of the two. Those choices cost you both time and tokens, but evals don’t score any of that. More tool calls per turn means fewer turns to finish the same task, and for you that means it feels a little faster and costs a little less.

Recently, Codex introduced async tool calls as a way to get the best of both worlds. However, the harness still plays a role in steering the model towards efficient behaviors.

Let’s look at the `count-dataset-tokens` task with GPT-6 Astra in Codex and Delta. The agent has to count the tokens in the science domain of a Hugging Face dataset, using a specific tokenizer, and write the number to `answer.txt`. Both harnesses pass this task on the eval.

*   **Codex, 14 requests.** It opens with a file search and a web search at once, and later installs packages alongside another command. In five of its requests it was checking on commands that were still running; Delta’s terminal waits for a command to finish instead.
*   **Delta, 7 requests.** It reads the dataset’s README while checking which Python packages are installed. In the next request it installs the missing packages while fetching the dataset’s file listing and the tokenizer’s config. Then it counts, writes the answer, and checks it.

### Same task, half the requests

GPT-6 Astra on Terminal-Bench's count-dataset-tokens. Both runs passed.

Harness

Request

Step

Started (s)

Finished (s)

Codex

1

search files

5.1

12.1

1

open dataset README

5.1

13.1

2

check installed packages

17.8

21.2

3

check installed packages

25.2

32.0

3

read AGENTS.md

25.2

31.8

4

pip install transformers

36.0

37.5

4

download dataset files

36.0

37.5

5

check on pip install

46.0

51.0

5

check on download

46.0

49.0

6

download dataset files again

56.0

57.0

7

check on pip install

70.1

75.1

7

check on download

70.1

70.1

7

count rows per domain

70.1

71.1

8

write count\_tokens.py

83.9

83.9

9

check on pip install

85.8

85.8

9

check on download

85.8

85.8

10

edit count\_tokens.py

88.7

88.7

10

run count\_tokens.py

88.7

89.7

11

check on count\_tokens.py

95.2

98.7

12

verify answer.txt

105.1

106.1

13

check on verification

108.0

108.4

14

final answer

109.5

109.5

Delta

1

fetch dataset README

5.1

not timed

1

check installed packages

5.1

5.2

2

pip install datasets transformers

17.8

35.1

2

fetch dataset file listing

17.8

not timed

2

fetch tokenizer config

17.8

not timed

3

count tokens

48.0

51.9

4

recount physics, chemistry, biology

63.8

67.8

5

write answer.txt

71.8

not timed

6

verify answer.txt

75.7

75.8

7

final answer

78.3

78.3

We’ve found that this difference meaningfully affects wall time and cost, and that it varies across harnesses over many tasks, not just this one. Delta is built for people collaborating in real time, and people shouldn’t have to wait on machines when they don’t need to. Every request they sit through is time spent waiting, which is why the way a harness steers tool calls matters so much to us.

## [What’s next](#whats-next)

So far, our focus for Delta’s agent harness has been to ensure we’re just as accurate and efficient as the harnesses we’re asking users to give up to work in Delta. That’s not enough; we want to design evals that communicate the reasons we reach for Delta (even for side projects where we pay for our tokens). Evals have always been a good way to imbue both the model and the harness with one’s judgment. In the next few months, we’ll standardize and publish deeper eval metrics (no more vanity metrics!), on a regular release cadence, to reflect the continuous and iterative process of building a harness. Our next update to this set of chosen tasks will shift focus to collaborative tasks that represent how users get work done in Delta.

## [Appendix](#appendix)

### [Why these evals?](#why-these-evals)

Over the course of building Delta, I’ve spent a lot of my time using evals to understand how models behave in it compared to other harnesses. This question has also recently become part of AI research and discourse more broadly \[[2](#ref-2), [3](#ref-3), [4](#ref-4)\], with [HarnessTax](https://harnesstax.github.io/) \[[5](#ref-5)\] and [FrontierHarness](https://frontierharness.org) \[[6](#ref-6)\] leading that conversation. We decided to use the same evals from these projects, drawn from Terminal-Bench \[[7](#ref-7)\] and DeepSWE \[[8](#ref-8)\], to evaluate Delta’s agent harness, to ensure there were public points of comparison and our results were not being artificially inflated by internal evals.

### [References](#references)

1.  Armin Ronacher. [Better Models: Worse Tools](https://lucumr.pocoo.org/2026/7/4/better-models-worse-tools/). July 2026.
2.  Naman Vats and Oleg Golev. [The Scaffold Effect in Coding Agents: Harness Choice as a Hidden Variable in Coding-Agent Evaluation](https://arxiv.org/abs/2607.22585). arXiv:2607.22585, 2026.
3.  Mohsen Arjmandi. [Harness or Model? Isolating the Harness Effect in Agentic Coding with a Contamination-Controlled Private Suite](https://arxiv.org/abs/2609.11987). arXiv:2609.11987, 2026.
4.  Sydney Lewis. [Same Model, Different Harness: Different Coding-Agent Results](https://arxiv.org/abs/2608.26218). arXiv:2608.26218, 2026.
5.  Melissa Z. Pan, Shuo Yang, Negar Arabzadeh, Wei-Lin Chiang, Ion Stoica, and Matei Zaharia. [HarnessTax: How Much Does Harness Matter for Coding Agents?](https://harnesstax.github.io/) 2026.
6.  Shilin Zhu and Shiqi Mei. [Introducing FrontierHarness Eval](https://runta.com/blog/introducing-frontierharness-eval/). Runta, September 2026. Results at [frontierharness.org](https://frontierharness.org).
7.  Mike A. Merrill, Alexander Glenn Shaw, Nicholas Carlini, et al. [Terminal-Bench: Benchmarking Agents on Hard, Realistic Tasks in Command Line Interfaces](https://openreview.net/forum?id=a7Qa4CcHak). The Fourteenth International Conference on Learning Representations (ICLR), 2026.
8.  Wenqi Huang, Charley Lee, Leonard Tng, and Serena Ge. [DeepSWE: Measuring frontier coding agents on original, long-horizon engineering tasks](https://github.com/datacurve-ai/deep-swe). 2026.

## [Footnotes](#footnote-label)

1.  Total cost of all tasks, including failed ones, divided by the number of tasks solved. FrontierHarness reports this figure as “median cost per task”, but it is not a statistical median, so we’ve renamed it to describe what it measures. [↩](#user-content-fnref-costperpass)
    
2.  Fable has fewer tasks because its security classifier is triggered on some of the tasks. [↩](#user-content-fnref-fable)
    

## More from the blog

*   ### [Escaping the tyranny of worktrees](/blog/escaping-the-tyranny-of-worktrees)
    
    ![Conrad Irwin](https://avatars.githubusercontent.com/u/94272?v=4 "Conrad Irwin")Oct 6, 2026
    
*   ### [Please Stop Sending Me Google Docs](/blog/please-stop-sending-me-google-docs)
    
    ![Franciska Dethlefsen](https://avatars.githubusercontent.com/u/10458212?v=4 "Franciska Dethlefsen")Oct 8, 2026
    
*   ### [The Diff: September 2026](/blog/the-diff-2026-09)
    
    ![Joseph Lyons](https://avatars.githubusercontent.com/u/19867440?v=4 "Joseph Lyons")Oct 7, 2026
    

[Download Delta Now](/download)[Open Delta](/threads)
