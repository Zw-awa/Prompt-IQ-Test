# Prompt IQ Test

[简体中文](README.zh-CN.md)

Prompt IQ Test is a local-first desktop app for assessing how well a person can use LLMs and AI agents.

It is not only about whether someone can write a prompt. The goal is to evaluate whether a user can clearly define goals, structure vague requests, provide enough context and constraints, iterate effectively, and drive an AI system toward a high-quality result.

## Status

This repository is currently in the planning stage.

- Product scope is defined
- Core interaction modes are defined
- Public repository README is ready

## Vision

Prompt IQ Test is designed to answer two practical questions:

1. Can a normal user actually use AI well?
2. Can an advanced user drive an LLM or Agent at a high level?

The first release focuses on individual users. Hiring, training, and enterprise assessment workflows are future directions, not part of `v1`.

## Planned Features

### Assessment Modes

- `Fun Quick Test`: short, lightweight, lower confidence
- `Full Assessment`: broader coverage, more reliable diagnosis
- `Static Assessment`: user submits a prompt for a given task
- `Dynamic Assessment`: user collaborates with an execution model over multiple turns
- `Built-in Question Bank`: sample tasks from local question sets
- `AI-Generated Questions`: generate tasks from user-provided constraints or random generation

### Static Assessment

- `Prompt Only`: score the prompt itself without executing it
- `Execute Then Evaluate`: run the prompt with an execution model, then score both prompt quality and execution result

### Dynamic Assessment

- Main chat with an execution AI
- Live side-panel scoring by an evaluator AI
- Trend and current estimated score during the session
- Final report after completion

### Scoring Focus

The first release centers on a fixed core rubric, including:

- clarity of request expression
- ability to define output format
- ability to turn vague needs into structured tasks
- sufficiency of context and constraints
- ability to identify and fill missing conditions
- iteration and acceptance awareness

## Privacy Boundary

Prompt IQ Test is designed as a local-first application.

- The program does not upload data to the author's server
- The program is fully free and open source
- Actual cost comes from the user's own AI API usage
- User content sent to configured AI providers may still be visible to those providers
- The author does not assume liability for outcomes caused by use of the software

## Planned Stack

The current recommended stack for `v1` is:

- `React`
- `TypeScript`
- `Vite`
- `Tauri`
- `SQLite`

Target form factor:

- cross-platform desktop
- Windows-first adaptation for the first release

## Planned App Structure

The first release is expected to include:

- Home page
- Assessment setup page
- Static assessment page
- Dynamic assessment page
- Result report page
- History pages
- Settings page
- About page
- Plugin marketplace placeholder page
- Vent-game placeholder page
- Startup privacy and disclaimer dialog

## First Release Non-Goals

The first release does not aim to include:

- account system
- cloud sync
- real plugin runtime
- real plugin marketplace
- real vent-game chat window
- JSON or image report export
- difficulty system for questions
- enterprise or recruitment scoring mode
- user-editable hidden prompts or scoring rubrics

## Open Source

- License: `Apache-2.0`

## Repository Notes

- Runtime files, local databases, exports, caches, and secrets should not be committed
- Example configuration files can be added later if needed

## Roadmap

High-level next steps:

1. finish the public repository baseline
2. implement the desktop application shell
3. implement static and dynamic assessment flows
4. implement local persistence and Markdown export
5. refine question seeds, prompts, and evaluation contracts
