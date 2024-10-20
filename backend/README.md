# mindfulnotes backend

## Overview

The backend is written in rust using axum and sqlx to store notes into sqlite local database. In order to analyze notes a potentially local running LLM hosted via ollama is used.

## Code state

Alpha state of the backend:
- stores notes into sqlite db
- retrieves notes from sqlite db
- wrap ollama api in order to be able to specify prompts to analyze notes
- able to analyze notes

## TODO

- adding unit tests
- documentation
- play with anaylsis prompt
  - maybe we should do a sort of weighted best of n approach when analyzing notes because the local models are of pretty low quality...
- let the llm output a json list of categories, prompt done but no api

## rust

Update rust toolchain and rustup command
```sh
rustup update
rustup self update
```