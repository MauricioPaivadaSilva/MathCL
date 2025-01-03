<div align="center">

<img src="./img/logo.png" alt="Logo: MathCL" height="200">

![GitHub top language](https://img.shields.io/github/languages/top/MauricioPaivadaSilva/MathCL) ![GitHub language count](https://img.shields.io/github/languages/count/MauricioPaivadaSilva/MathCL) ![GitHub License](https://img.shields.io/github/license/MauricioPaivadaSilva/MathCL) ![GitHub last commit (branch)](https://img.shields.io/github/last-commit/MauricioPaivadaSilva/MathCL/main) ![GitHub repo size](https://img.shields.io/github/repo-size/MauricioPaivadaSilva/MathCL)

| Teste |
| --- |
| ![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/MauricioPaivadaSilva/MathCL/ci.yml) |


</div>

## Sobre o projeto

Este projeto tem como intuíto ser uma forma de estudar a linguagem de programação [Rust](https://www.rust-lang.org/pt-BR/), assim como aplicar conhecimentos desenvolvidos em minha graduação (licenciatura em matemática).

**Nota:** Este estudo **NÃO TEM** como intuito principal a utilização em projetos reais.

---

## Sumário

- [1 Tecnologias empregadas](#1-tecnologias-empregadas)
- [2 Funções de biblioteca](#2-funções-de-biblioteca)
    - [Módulo básico](#módulo-básico)
- [3 Modo de desenvolvimento](#3-modo-de-desenvolvimento)
    - [Como contribuir?](#como-contribuir)
        - [Regras](#regras)
    - [Comandos importantes](#comandos-importantes)

## 1 Tecnologias empregadas

**Observação**: Tenho como princípio utilizar neste projeto o Rust de form "pura", sendo assim, não pretendo utilizar nenhuma biblioteca de terceiros. Essa decisão tem como base o intuíto de criar este projeto.

Tecnologias:

<div align="center">

| Nome | Verão | Site oficial |
| --- | --- | --- |
| Rust | 1.82.0 | https://www.rust-lang.org/pt-BR/ |

</div>

---

## 2 Funções de biblioteca

A biblioteca é dividida em módulos, cada módulo corresponde a uma dada função.

### Módulo básico

No módulo básico há funções básicas que são utilizadas em matemática, sendo elas:

<div align="center">

| Função | Entrada | Saída | Descrição |
| --- | --- | --- | --- |
| `sum()` | `(f32, u32)` | `f32` | Trata-se de uma função que resolve uma somatória. |
| `pow()` | `(f32, i32)` | `f32` | Trata-se de uma função que resolve uma potência de um número qualquer. |
| `module()` | `f32` | `f32` | Trata-se de uma função que encontra o módulo do valor de entrada. |
| `sqrt()` | `(f32, u32)` | `f32` | Trata-se de uma função que resolve a raíz de um dado valor. |

</div>

Também há um sub módulo que contém valores de constantes importantes como $e$ e o $\pi$.

### Módulo de análise combinatória

No módulo análise combinatória há funções para a resolução de permutação, permutação circular, combinação, arranjo e arranjo com repetição:

<div align="center">

| Função | Entrada | Saída | Descrição |
| --- | --- | --- | --- |
| `fat()` | `u32` | `u32` | Trata-se de uma função que resolve um fatorial. |
| `awr()` | `(u32, u32)` | `u32` | Trata-se de uma função que resolve um arranjo com repetição. |
| `arrangement()` | `(u32, u32)` | `u32` | Trata-se de uma função que resolve um arranjo. |
| `permutation()` | `u32` | `u32` | Trata-se de uma função que resolve uma permutação. |
| `pc()` | `u32` | `u32` | Trata-se de uma função que resolve uma permutação circular. |
| `combinate()` | `(u32, u32)` | `u32` | Trata-se de uma função que resolve uma combinatória. |

</div>

<!-- #### Como importar as funções em seu código -->

---

## 3 Modo de desenvolvimento

Os passos a seguir são destinados a quem quiser contribuir com o desenvolvimento do projeto.

### Como contribuir?

Toda contribuição para o desenvolvimento do projeto é bem vinda! Todavia, como estudante, devo pedir que siga algumas regras para me auxiliar.

#### Regras

1. Inserir um comentário explicando o que foi modificado, assim como o motivo da modificação. Como estou estudando a linguagem, conhecer métodos de programar que melhoram a performace podem me ajudar;
2. As modificações devem ser feitas uma _branch_ que não seja a _main_, para que eu possa ver as modificações de forma mais fácil e me ajudar a estudar a linguagem;
3. Toda modificação ou adição feita deve conter no mínimo 3 testes para validar a confiabilidade do método criado/aplicado.

### Comandos importantes

- **Teste**: Antes de subir qualquer modificação, não se esqueça de rodar os testes para garantir a integridade do código. Pode-se utilizar o comando:

```sh
> cargo test
```
- **Build**: Caso queira fazer uso da biblioteca, você pode usar um dos comandos abaixo para criar a biblioteca voltada para o rust (uma .rlib):
    - Baixa performace, mas com recursos de debug:
    ```sh
    > cargo build
    ```
    - Alta performace, apenas para uso da lib:
    ```sh
    > cargo build --release
    ```