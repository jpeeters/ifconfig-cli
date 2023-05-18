<h1 align="center">ifconfig CLI</h1>

# Introduction

This little CLI is a client to https://ifconfig.co. It retrieves the information from the API
and displays it in the terminal.

This project is a toy CLI that demonstrates an implementation of clean architecture.

It also serves as a skeleton for other projects.

# Usage

The call to the CLI does not need an argument. As a result of the call, it prints the JSON
output directly from the API.

# Code organization

The repository uses the workspace organization offered by Cargo and Rust. Each package
belongs to at most one layer as described in clean architecture.

```
app/      (use cases) the applicative code that integrates all use cases and provides
          a clear view of functionalities.
cmd/      (adapter) the command line code that provides decoding of options and calls
          the applicative code.
domain/   (domain) the core of business types and interfaces (i.e. use cases).
ifconfig/ (adapter) implementation to fetch ifconfig data from public API,
          implementing the domain::Fetcher trait.
```
