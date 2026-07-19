+++
title = "critical-thinking"
description = "Analyze your own immediately preceding response with rigorous, skeptical critical thinking — surfacing flaws, hidden assumptions, logical gaps, and overlooked risks. Use when you want your last answer stress-tested for weaknesses before the user acts on it."
sort_by = "title"
template = "skill.html"
[extra]
skill = true
category = "reflection"
mermaid = false
+++


# Critical Thinking

Act as a skeptical, detail-oriented, ruthlessly honest analyst. The objective is NOT to defend or justify your previous response, but to actively identify its weaknesses, hidden assumptions, and overlooked risks.

**IMPORTANT - Language Matching & Tool Usage:**

- Detect the primary language used in your immediately preceding response.
- Conduct this entire critical analysis in that same language.
- Maintain consistency with the language of the conversation.
- When investigating codebase facts to support your critical analysis, prefer your agent's built-in file-search and file-reading tools over ad-hoc shell commands like `cat`, `find`, or `grep` where they are available.

Analyze your OWN immediately preceding response in this conversation based on the following comprehensive framework. Structure your output using these exact headings and numbering.

---

## Critical Thinking Analysis

### 1. Core Thesis

- In a single, concise sentence, what was the central solution or argument I proposed in my previous answer?

### 2. Foundational Analysis: Assumptions & Context

- **2-1. High-Impact Assumptions:** What are the top 3 most critical assumptions I made that, if proven wrong, would completely invalidate my proposed solution? Focus on technical, environmental, and resource-based assumptions.
- **2-2. Contextual Integrity:** Did I fully respect all constraints and requirements mentioned earlier in this conversation? Point out any potential contradictions or forgotten details.

### 3. Logical Integrity Analysis

- **3-1. Premise Identification:** What were the fundamental premises or starting points of my argument? (e.g., "The user needs a scalable solution," "Redis is the best tool for rate limiting.")
- **3-2. Chain of Inference:** Is there a clear, step-by-step logical chain connecting the identified premises to the final conclusion? Point out any significant logical leaps, gaps, or steps where the conclusion does not necessarily follow from the evidence provided.
- **3-3. Potential Fallacies:** Does my reasoning contain any common logical fallacies (e.g., asserting a false dichotomy, making a hasty generalization, appealing to a questionable authority)?

### 4. AI-Specific Pitfall Analysis

Evaluate my previous response against these common failure modes for AI agents. Provide a "Pass" or "Fail" for each, with a brief justification for any "Fail".

- **4-1. Problem Evasion:** (Pass/Fail) Did I solve the user's stated problem but avoid the _actual, underlying_ difficult problem?
- **4-2. "Happy Path" Bias:** (Pass/Fail) Did I neglect to address error handling, edge cases, or potential failure scenarios?
- **4-3. Over-Engineering:** (Pass/Fail) Did I propose a solution that is unnecessarily complex?
- **4-4. Factual Accuracy & Hallucination:** (Pass/Fail) Are all technical details verifiably correct?

### 5. Risk & Mitigation Analysis

- **5-1. Overlooked Risks:** What are the top 3 practical risks or negative consequences of implementing my suggestion?
- **5-2. Alternative Scenarios:** What is a fundamentally different approach that I failed to consider?

### 6. Synthesis & Revised Recommendation

- **6-1. Summary of Flaws:** In bullet points, summarize the most critical weaknesses discovered.
- **6-2. Confidence:** Given this analysis, how much should the original proposal be trusted (1-10), and why?
- **6-3. Actionable Next Step:** What is the single most important action the user should take _before_ acting on my original advice?

