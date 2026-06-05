Read the document at the path provided in $ARGUMENTS.

Parse $ARGUMENTS for optional flags:
- `--lang <code>` — language to write the report in: `pt` (PT-BR) or `en` (default: `en`)
- `--mode <name>` — one of: `academic` (full citation + technical + gap check), `claims` (only unsupported assertions), `quick` (only hard errors — wrong citations, technical mistakes)

If `--mode` is missing, default to `academic`. If `--lang` is missing, default to `en`. Do NOT ask — proceed with defaults.

---

## Step 1 — Read and understand the document

Read the full document. Identify:
- **Domain**: what field/topic is this about? (e.g., distributed systems, CQRS, machine learning)
- **Type**: is this an academic paper, a presentation source doc, a technical report?
- **Audience**: who is this for? (infer from the document if not stated)
- **Language**: what language is the document written in?

---

## Step 2 — Collect all factual claims

Go through the document paragraph by paragraph. Collect every statement that makes a claim about the world — statistics, historical facts, technical behaviors, attributions to named individuals or papers, causal relationships. Ignore editorial framing, rhetorical questions, and narrative fiction explicitly marked as illustrative.

For each claim, note:
- The exact text of the claim
- Whether it has a citation
- What the citation is (if present)

---

## Step 3 — Run the checks

### Check A — Unsupported assertions (always run)
List every factual claim that has NO citation. For each:
- Quote the claim
- Classify: `[missing citation]` if it needs one, `[acceptable]` if it is clearly editorial/illustrative framing that does not need one
- Suggest a citation if one is known

### Check B — Reference accuracy (run in `academic` and `quick` modes)
For each cited reference, verify:
1. **Does this work exist?** (author, title, venue/publisher, year — all plausible?)
2. **Does it say what the document claims?** (does the cited finding match what is attributed to it?)
3. **Is the attribution correct?** (is this the right person/paper for this finding, or is it commonly misattributed?)
4. **Is the year correct?**

Flag each issue as:
- `[wrong source]` — the cited paper is not the origin of this claim
- `[misquoted]` — the paper exists but the claim distorts what it says
- `[unverifiable]` — grey-area source (blog post, informal talk, industry report) — note the limitation
- `[correct]` — citation checks out

### Check C — Technical accuracy (run in `academic` and `quick` modes)
For each technical claim (e.g., how a system works, what a pattern does, what a protocol guarantees):
- Is it correct?
- Is it oversimplified to the point of being misleading?
- Are there important caveats that are omitted?

Flag each issue as:
- `[error]` — factually wrong
- `[oversimplified]` — technically defensible but omits a critical nuance
- `[correct]`

### Check D — Content gaps (run in `academic` mode only)
Given the domain and the document's argument, what important concepts, counterarguments, or failure modes are absent? Focus on:
- Things a professor or expert reviewer would likely ask about in Q&A
- Standard critiques of the approach being described
- Operational realities not covered by the theory

For each gap, note:
- What is missing
- Why it matters
- A reference if one is known

### Check E — Reference upgrades (run in `academic` mode only)
For any cited work, suggest a stronger or more canonical alternative if one exists — e.g., a peer-reviewed paper instead of a blog post, a primary source instead of a secondary one, a more recent work that supersedes the one cited.

---

## Step 4 — Output the report

Write the report in the language specified by `--lang`.

Structure:

```
# Check Report — [Document title or filename]
Mode: [academic | claims | quick] · Lang: [pt | en]

## A — Unsupported Assertions
[list or "None found."]

## B — Reference Issues
[list or "All references check out."]

## C — Technical Accuracy
[list or "No issues found."]

## D — Content Gaps  ← academic mode only
[list or "No critical gaps identified."]

## E — Reference Upgrades  ← academic mode only
[list or "No upgrades suggested."]

## Summary
[2–4 sentences: overall quality, most critical issues to fix, confidence level in the document's academic standing]
```

For each finding, use this format:
> **[SEVERITY]** · *"exact quote from document"*
> Issue: [explanation]
> Fix: [what to do]

Severity levels: `BLOCKER` (wrong citation, factual error), `WARNING` (missing citation, oversimplification), `SUGGESTION` (gap, upgrade opportunity).

Do not invent issues. If something is correct, say so. Be direct.
