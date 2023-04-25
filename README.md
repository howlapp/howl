# macrostack

An example of a full-stack, microservice-based application written in Rust and TypeScript.

## Dependencies

- [Rust](https://www.rust-lang.org/tools/install)
- [Node.js](https://nodejs.org/en/download/)
- [pnpm](https://pnpm.io/installation)

### Global PNPM Dependencies

Macrostack also relies on some CLI tooling provided by the NPM repository. Since NPM scripts are run from the project directory rather than CWD, these must be installed globally to function correctly.

- [buf](https://docs.buf.build/installation)

## Structure

Below is a brief overview of the structure of this repository.

- `/apps` - This directory contains the source code for applications deployed publicly.
- `/common` - This directory contains the source code for the common libraries in both TypeScript and Rust.
- `/crates` - This directory contains the source code for the crates written in Rust.
- `/infrastructure` - This directory contains the Terraform infrastructure-as-code for the project.
- `/packages` - This directory contains the source code for the packages written in TypeScript.
- `/services` - A dedicated directory containing the source code for the project's microservices.

## Development

Local development is aided using a Docker Compose environment that provides a PostgreSQL database, a Consul agent, and a Redis instance. To start the environment, run the following command:

```sh
docker compose up -d
```

Macrostack makes use of Turborepo to aid in caching of task outputs. You can find the documentation [here](https://turbo.build/repo/docs/reference/command-line-reference).

## License

All code in this repository is licensed under a dual MIT/Apache-2.0 license, at your option. There are some exceptions to this, which are noted in the relevant projects' READMEs. See [LICENSE-MIT](LICENSE-MIT) and [LICENSE-APACHE](LICENSE-APACHE) for more details.

[![FOSSA Status](https://app.fossa.com/api/projects/custom%2B34788%2Fgithub.com%2Fkaylendog%2Fmacrostack.svg?type=large)](https://app.fossa.com/projects/custom%2B34788%2Fgithub.com%2Fkaylendog%2Fmacrostack?ref=badge_large)
