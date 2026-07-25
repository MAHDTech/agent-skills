+++
title = "edit-prediction"
[extra]
skill = false
category = "tooling"
mermaid = false
skill_name = "zed"
+++

# Edit Prediction

Zed predicts your next edit based on your activity. Powered by Zeta2,
our open-weight model, or any provider you choose.

[Download nowD](https://zed.dev/download)[Clone
sourceC](https://github.com/zed-industries/zed)

[Read the blog post →](https://zed.dev/blog/zeta2)

![The UI for Zed, the text
editor](https://zed.dev/cdn-cgi/image/width=1920,quality=75,format=auto/_next/static/media/video-poster.0p4d-n3.ayrkk.webp)![The
UI for Zed, the text
editor](https://zed.dev/cdn-cgi/image/width=1920,quality=75,format=auto/_next/static/media/video-poster.0p4d-n3.ayrkk.webp)

What developers are saying

## Predictions that feel right

Here's what developers love about edit predictions.

> So far the Edit Prediction feature has been great. Big fan of the
> privacy-focused default & controls around sharing training data,
> especially for non-open-source projects. Predictions are fast + have
> been contextual.

![Matt Silverlock's
picture](https://avatars.githubusercontent.com/u/18544?v=4&s=2048)

Matt Silverlock

Sr. Director of Product

> The inline completions are pretty magical. I'm converting/updating a
> project to 11ty, and it's doing better at contextual predictions of
> what I want to add to files better than any other AI assistive tool
> I've used so far.

![Bob Rudis's picture](https://i.ibb.co/tTC1TVTG/Image-from-i-OS.jpg)

Bob Rudis

V.P. Research

> Zed's Edit Prediction is impressively accurate and feels super snappy.
> Where other autocomplete solutions often struggle with reliability,
> Zed's implementation stands out.

![Andreas Thomas's picture](https://unavatar.io/x/chronark_)

Andreas Thomas

CTO & Cofounder

## Predicting intent, preserving focus

Edit predictions that fit into your existing workflow. Other tools
complete code. Zed predicts what you're trying to do next.

![](https://zed.dev/cdn-cgi/image/width=2048,quality=75,format=auto/img/edit-prediction/edit-1-poster.webp)

Plow through edits by repeatedly hitting tab

As you work, Zed now predicts your next edit, so you can apply it just
by hitting tab. Once you accept a prediction, you can perform multiple
follow-up edits by pressing tab repeatedly, saving you time and
keystrokes.

![](https://zed.dev/cdn-cgi/image/width=2048,quality=75,format=auto/img/edit-prediction/edit-2-poster.webp)

Thoughtful integration with Language Server completions

When language server completions are visible, Zed won't preview the
predicted edit until you press option or alt. As soon as you press the
modifier, Zed previews the edit and hides the menu to enable an
unobstructed review.

![](https://zed.dev/cdn-cgi/image/width=2048,quality=75,format=auto/img/edit-prediction/subtle-mode-poster.webp)

Subtle Mode: predictions on demand

If you find predictions distracting, Subtle Mode keeps them hidden until
you ask. Hold alt-tab to see the prediction, release to dismiss. The
model keeps working; you just choose when to look.

How Zeta2 works

## Built for edit prediction, trained on real edits

Zeta2 is the only model purpose-built for how Zed works, trained
entirely on open source code with public weights.

[View on Hugging Face](https://huggingface.co/zed-industries/zeta-2)

Predictions speak your language

Zeta2 uses the language server to retrieve the type and symbol
definitions around your cursor, so predictions understand your code's
actual structure rather than guessing.

[See Pull Request→](https://github.com/zed-industries/zed/pull/44036)

Evaluation built into the pipeline

We score predictions only on the code that changed, using diff-aware
metrics and line-level exact match to measure real accuracy rather than
copy fidelity.

[See Pull Request→](https://github.com/zed-industries/zed/pull/43485)

Training data from real edits

Multi-file commits are split into single-change examples, stratified by
repository, and supplemented with distillation from a larger teacher
model.

[See Pull Request→](https://github.com/zed-industries/zed/pull/44369)

Experimenting with different, more efficient prompt formats

Allowing the model to output a subset of the editable region to reduce
the number of tokens it has to generate

[See Pull Request→](https://github.com/zed-industries/zed/pull/51185)

Your choice of model

## Works with the providers you already use

Edit Prediction also supports Copilot Next Edit Suggestions, Mercury
Coder, Codestral, and Ollama. Choose the provider that best fits your
needs.

[](https://githubnext.com/projects/copilot-next-edit-suggestions/)

GitHub Copilot

[](https://mistral.ai/news/codestral)

Codestral

[](https://zed.dev/docs/ai/edit-prediction)

Zed's Zeta

[](https://inceptionlabs.ai)

Mercury Coder

[](https://ollama.com/)

Ollama

## So light, so fast, so helpful

Pleasantly type together with Edit Predictions on Zed, right now.

[Download nowD](https://zed.dev/download)[Clone
sourceC](https://github.com/zed-industries/zed)

Tab to Accept • Escape to Leave

Click to start predicting…

Zed Industries

Edit Prediction

1

2

3

4

5

6

7

8

9

0

\-

=

Q

W

E

R

T

Y

U

I

O

P

\[

\]

A

S

D

F

G

H

J

K

L

;

'

Tab

Z

X

C

V

B

N

M

,

.

/

![Zed's
logo](https://zed.dev/cdn-cgi/image/width=2048,quality=100,format=auto/_next/static/media/logo_wordmark_white_bigger.11rssg-7g1jg..png)

## Daily drive with Zed

Code at the speed of thought.

[Download nowD](https://zed.dev/download)[Clone
sourceC](https://github.com/zed-industries/zed)

