# Andraste Distribution Builder

This repository contains tooling and scripts for building distributions of the Andraste Framework.

> **Note:**  
> This project is primarily intended for internal use by the Andraste Framework team. While you are welcome to explore the code, please be aware that documentation, stability, and support may be limited.  
> External contributions and usage are not currently a focus for this repository.

## Purpose

The distribution builder automates the process of packaging and assembling Andraste Framework components for deployment and distribution.

## CLI Usage

The main interface to this tool is a command-line application. The CLI provides the following primary operations:

### Commands

- **create-bundle**  
  Creates a distribution bundle from specified framework and launcher releases.
  - `--version <string>` (**required**)  
    The version (tag/release) to use when fetching content.
  - `--framework-repo <organisation/repository>` (optional)  
    The repository to use for the framework (defaults to `AndrasteFramework/Payload.Generic`).
  - `--readme-template <path>` (optional)  
    Path to a README.txt template file (defaults to internal template).

- **clear**  
  Removes the output directory created by previous bundle operations.

### Examples

#### Run with increased verbosity

```bash
andraste-distribution-builder -vv create-bundle --version 1.2.3
```

#### Clear the output directory

```bash
andraste-distribution-builder clear
```

#### Run via Docker

If you have built the Docker image (see Dockerfile), you can run the builder with:

```bash
docker run --rm -it \
  -v "$PWD/dist:/app/dist" \
  andraste-distribution-builder \
  -vv create-bundle --version 1.2.3
```

This mounts the output (`dist`) directory so that outputs are accessible on your host.

### Outputs

- Bundles are created in the `dist` directory as zipped archives.
- Temporary/intermediate files are placed in the `out` directory.
- A README.txt is generated in the bundle (can be customized using a template).

## Getting Started

As this repository is geared toward internal workflows, setup instructions and documentation may be incomplete or subject to change.
The best reference may be the Github Actions Workflows interacting with the repository / docker image.

---

For more information about the Andraste Framework, see the [main organization page](https://github.com/AndrasteFramework).
