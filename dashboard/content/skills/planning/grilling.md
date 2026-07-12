+++
title = "grilling"
description = "Interview the user relentlessly, one question at a time, to stress-test a plan or design before any code is written. Use when the user wants to pressure-test an approach, resolve open design decisions, or asks you to 'grill me', 'poke holes in this', 'stress-test this plan', or 'interview me about this design'. Walk every branch of the decision tree, look up facts in the codebase, and put each real decision to the user with a recommended answer before proceeding."
[extra]
category = "planning"
mermaid = false
+++


# Grilling

Interview the user relentlessly about every aspect of this plan until you reach a shared understanding. Walk down each branch of the design tree, resolving the dependencies between decisions one by one. For each question, provide your recommended answer.

Ask the questions one at a time, waiting for feedback on each before continuing. Asking several questions at once is bewildering.

If a _fact_ can be found by exploring the codebase, look it up rather than asking. The _decisions_, though, belong to the user — put each one to them and wait for an answer.

Do not enact the plan until the user confirms you have reached a shared understanding.

This is a reusable primitive: other planning skills invoke it to run their interview step.

> Adapted from [mattpocock/skills](https://github.com/mattpocock/skills) (MIT).

