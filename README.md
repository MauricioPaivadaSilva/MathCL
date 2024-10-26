<div align="center">

# MathCL

![GitHub top language](https://img.shields.io/github/languages/top/MauricioPaivadaSilva/MathCL) ![GitHub language count](https://img.shields.io/github/languages/count/MauricioPaivadaSilva/MathCL) ![GitHub License](https://img.shields.io/github/license/MauricioPaivadaSilva/MathCL) ![GitHub last commit (branch)](https://img.shields.io/github/last-commit/MauricioPaivadaSilva/MathCL/main) ![GitHub repo size](https://img.shields.io/github/repo-size/MauricioPaivadaSilva/MathCL)

**GitHub actions tests will be added soon**

<!-- | Teste | Resultado |
|---|---|
| Testes | [![Resultados](https://github.com/MauricioPaivadaSilva/MCEpy/actions/workflows/pytest.yaml/badge.svg)](https://github.com/MauricioPaivadaSilva/MathCL/actions/workflows/pytest.yaml) | -->


</div>

## About the project

This project aims to be a way of studying the programming language [Rust](https://www.rust-lang.org), as well as applying knowledge developed in my graduation (mathematics degree).

[**Documentação em pt-BR**](https://github.com/MauricioPaivadaSilva/MathCL/docs/README_pt-BR.md)

**Note:** This study is **NOT** primarily intended to be used on real projects.

---

## Sumário

- [1 Technologies employed](#1-technologies-employed)
- [2 Library functions](#2-library-functions)
- [3 Dev mode](#3-dev-mode)
    - [How to contribute](#how-to-contribute)
        - [Rules](#rules)
    - [Important commands](#important-commands)

## 1 Technologies employed

**Note**: My principle is to use Rust in a "pure" form in this project, therefore, I do not intend to use any third-party libraries. This decision is based on the intention of creating this project.

Tecnologias:

<div align="center">

| Name | Version | Official website |
| --- | --- | --- |
| Rust | 1.82.0 | https://www.rust-lang.org/pt-BR/ |

</div>

---

## 2 Library functions

---

## 3 Dev mode

The steps below are intended for anyone who wants to contribute to the development of the project.

### How to contribute?

Any contribution to the development of the project is welcome! However, as a student, I must ask you to follow some rules to help me.

#### Rules

1. Enter a comment explaining what was changed, as well as the reason for the change. As I am studying the language, knowing programming methods that improve performance can help me;
2. The modifications must be made in a _branch_ other than the _main_, so that I can see the modifications more easily and help me study the language;
3. Each modification or addition made must contain at least 3 tests to validate the reliability of the method created/applied.

### Important commands

- **Test**: Before submitting any modifications, do not forget to run tests to ensure the integrity of the code. You can use the command:

```sh
> cargo test
```
- **Build**: If you want to make use of the library, you can use one of the commands below to create the library aimed at Rust (a .rlib):
    - Low performance, but with debugging capabilities:
    ```sh
    > cargo build
    ```
    - High performance, for lib use only:
    ```sh
    > cargo build --release
    ```