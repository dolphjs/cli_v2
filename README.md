# Dolph CLI

The official command-line tool for [DolphJS](https://github.com/dolphjs/dolph) — scaffolds new projects, generates components, and runs your app in development and production. Written in Rust for a fast, dependency-free binary.

- [Installation](#installation)
- [Quick Start](#quick-start)
- [Commands](#commands)
  - [`dolph new`](#dolph-new-name)
  - [`dolph generate`](#dolph-generate)
  - [`dolph watch`](#dolph-watch---bun)
  - [`dolph build`](#dolph-build)
  - [`dolph start`](#dolph-start---bun)
- [Project Structure](#project-structure)
- [Configuration Files](#configuration-files)
- [Database Support](#database-support)
- [Testing](#testing)
- [WebSockets](#websockets)
- [GraphQL Mode](#graphql-mode)
- [Building From Source](#building-from-source)
- [License](#license)

## Installation

```bash
cargo install --path .
```

(Published-crate/npm-wrapper installation instructions go here once this ships to a registry — for now, build from source per [Building From Source](#building-from-source).)

## Quick Start

```bash
dolph new my-app
cd my-app
npm install
npm run dev:start
```

`dolph new` walks you through a handful of prompts (API style, routing style, language, database) and scaffolds a working project with those choices already wired in — a real `server.ts`, a real `dolph_config.yaml`, and a `package.json` with the right dependencies for what you picked.

## Commands

### `dolph new <name>`

Scaffolds a new project in a directory named `<name>` (or the current directory, if `<name>` is `.`).

```bash
dolph new my-app
dolph new .        # scaffold into the current directory
```

You'll be walked through:

| Prompt | Options | Notes |
|---|---|---|
| API style | `rest`, `graphql` | GraphQL mode always uses `spring` routing and TypeScript, regardless of the next two prompts. |
| Routing style | `express`, `spring` (REST only) | **`spring`** is the fully-featured, `@Component`/DI-based architecture and the one every other command in this CLI is built around. `express` gives you a bare `DolphRouteHandler`-based setup with no generator support beyond the initial scaffold — see [Architecture](https://docs.dolphjs.dev/architecture) for the tradeoffs. |
| Language | `ts`, `js` (REST only) | GraphQL mode is always TypeScript. |
| Database | `mongo`, `mysql`, `postgresql`, `other` | Drives what `dolph_config.yaml`, the generated model, and the generated service look like — see [Database Support](#database-support). |

This writes, depending on your answers: `dolph_cli.yaml` (records your choices — auto-generated, don't hand-edit it), `dolph_config.yaml`, `server.ts`, `tsconfig.json`, `.swcrc`, `package.json`, `jest.config.js` (TypeScript projects only), and `.gitignore`.

### `dolph generate`

Generates individual files, or a full feature at once, into `src/components/<name>/`. Must be run from a project root (somewhere with a `src/` directory and a `dolph_cli.yaml`).

```bash
dolph generate [OPTIONS]
```

| Flag | Long form | Generates |
|---|---|---|
| `-a` | `--all <name>` | Everything below appropriate to your project's API style — see [Generating a Full Feature](#generating-a-full-feature). |
| `-c` | `--controller <name>` | A controller (`@Route`, auto-return `greet` handler). |
| `-s` | `--service <name>` | A service (`@DService()`, database-aware — see [Database Support](#database-support)). |
| `-y` | `--component <name>` | A `@Component({ controllers: [], services: [] })` shell. |
| `-d` | `--dto <name>` | A `class-validator`-ready DTO stub. |
| `-m` | `--model <name>` | A Mongoose schema or Sequelize model, depending on your database. |
| `-t` | `--test <name>` | A controller spec and a service spec — see [Testing](#testing). |
| `-k` | `--socket <name>` | A Socket Service **and** its Socket Component — see [WebSockets](#websockets). |
| `-e` | `--entity <name>` | (GraphQL) A `type-graphql`/`typeorm` entity. |
| `-v` | `--resolver <name>` | (GraphQL) A `@Resolver()` stub. |
| `-i` | `--input <name>` | (GraphQL) An `@InputType()` stub. |
| `-r` | `--route <name>` | (Express routing) Currently a no-op — express-routing projects don't get generator support beyond the initial scaffold yet. |

Flags can be combined in one invocation:

```bash
dolph generate --controller users --service users
```

#### Generating a Full Feature

```bash
dolph generate --all users
```

For a **REST + spring** project, this generates, in order: service, DTO, controller, model, test specs, and a component — six files across `src/components/users/`.

```text
src/components/users/
├── users.component.ts        # @Component({ controllers: [], services: [] }) — see note below
├── users.controller.ts
├── users.controller.spec.ts
├── users.dto.ts
├── users.model.ts
├── users.service.ts
└── users.service.spec.ts
```

> [!NOTE]
> **The generated component doesn't wire itself up.** `users.component.ts` is written with empty `controllers`/`services` arrays — you still need to import `UsersController`/`UsersService` and add them yourself, then add `UsersComponent` to the array you pass into `new DolphFactory([...])` in `server.ts`. This is intentional: the generator won't guess which components belong together across files it didn't just create.

For a **GraphQL** project, `--all` instead generates an entity, a resolver, and an input, each in their own subfolder (`entities/`, `resolvers/`, `inputs/`) alongside a `services/` folder for the service.

#### Generating Tests

```bash
dolph generate --test users
```

Writes `users.controller.spec.ts` and `users.service.spec.ts` into the same component folder, importing and testing the *exact* classes the controller/service generators produce — they pass immediately against a freshly generated component, and are meant as a starting point for real assertions as the component grows. See [Testing](#testing).

### `dolph watch [--bun]`

Runs your app in development mode with automatic restarts on file changes.

```bash
dolph watch          # via ts-node (or plain node for JS projects)
dolph watch --bun     # via bun, falling back to node if bun isn't installed
```

Watches `./src` recursively for `.ts`/`.js`/`.json` changes (ignoring `node_modules`, `.git`, editor swap files), debounced 500ms, and restarts the server process on each change. `Ctrl+C` stops it cleanly.

### `dolph build`

Compiles a TypeScript project to JavaScript for production, via `swc` (`src` → `app`, with source maps).

```bash
dolph build
```

Only applies to TypeScript projects — running it against a JS project is a no-op error. Test spec files (`*.spec.ts`, `*.e2e-spec.ts`) are excluded from the build regardless of where they live under `src/`, so they never ship into `app/`.

### `dolph start [--bun]`

Runs the compiled production build (`app/src/server.js`) — run `dolph build` first.

```bash
dolph build
dolph start
dolph start --bun     # via bun, falling back to node if bun isn't installed
```

## Project Structure

A freshly scaffolded REST + spring project:

```text
my-app/
├── src/
│   ├── server.ts
│   ├── shared/
│   │   ├── configs/           # db.config.ts lives here for MySQL projects
│   │   └── socket/            # socket services/components generated here
│   └── components/
│       └── <name>/
│           ├── <name>.component.ts
│           ├── <name>.controller.ts
│           ├── <name>.controller.spec.ts
│           ├── <name>.service.ts
│           ├── <name>.service.spec.ts
│           ├── <name>.dto.ts
│           └── <name>.model.ts
├── dolph_cli.yaml              # this CLI's own record of your choices — don't hand-edit
├── dolph_config.yaml           # the framework's runtime config
├── tsconfig.json
├── .swcrc
├── jest.config.js
└── package.json
```

## Configuration Files

| File | Purpose |
|---|---|
| `dolph_cli.yaml` | Records the answers you gave `dolph new` (`language`, `api`, `routing`, `database`) — every `dolph generate` call reads this to decide what to write and how. Auto-generated; a header comment says as much. |
| `dolph_config.yaml` | Read by `DolphFactory` at runtime. Ships with `port`, `env`, `jsonLimit`, `routing.base: '/v1'`, and `middlewares.cors`; a `database.mongo` block is added automatically if you chose Mongo (see [Database Support](#database-support)). |
| `tsconfig.json` / `.swcrc` | Both carry the same `@/*`-style path aliases scoped to a spring project's layout (`@/shared/*`, `@/components/*`, etc.), and both exclude `*.spec.ts`/`*.e2e-spec.ts` from what actually gets emitted. |
| `jest.config.js` | `ts-jest`, with `testMatch` covering both `*.spec.ts` and `*.e2e-spec.ts` — Jest's own default only matches the former. |

## Database Support

Your `dolph new` database choice changes what `generate --service`/`generate --model` produce:

| Database | Service | Model |
|---|---|---|
| `mongo` | `@InjectMongo(...)` + `@DService()`, typed against the generated Mongoose model. | A Mongoose schema (`I<Name>` interface + `<Name>Model`). |
| `mysql` | `@InjectMySQL(...)` + `@DService()`, typed against a Sequelize model. First `generate --model` also writes `src/shared/configs/db.config.ts` (a real `initMySql(...)` call, called from `server.ts`). | A `sequelizeInstance.define(...)` model. |
| `postgresql` / `other` | Plain `@DService()`-decorated service — no ORM wiring generated yet. | Not generated for these — bring your own. |

`@DService()` shows up on every generated service regardless of database, even ones with no dependencies today: TypeScript's `emitDecoratorMetadata` only emits constructor-parameter reflection metadata for a class that has *some* decorator on it, and `@Component` depends on exactly that metadata to resolve a service's own constructor-injected dependencies. Without it, a service that later grows a dependency on another service fails to receive it, silently.

For Mongo specifically: `dolph_config.yaml` gets a `database.mongo` block pointed at `mongodb://localhost:27017/dolph-app` by default, because `server.ts` never calls an explicit connect function for a Mongo project — `DolphFactory` auto-connects from that config section alone. Point it at a real connection string before you rely on it outside local dev.

## Testing

Every TypeScript project scaffolded by `dolph new` already has `jest`, `ts-jest`, `supertest`, and [`@dolphjs/testing`](https://github.com/dolphjs/testing) as devDependencies, plus a `test` script and `jest.config.js` — nothing extra to install.

- `dolph generate --test <name>` (or `--all`) writes a **controller spec** and a **service spec**, co-located with the component. Both are plain unit tests — no `@Component`, no Express, no real database — that construct the generated classes directly and assert against what the generator actually produced.
- For the routing/middleware path itself (real `@Route` metadata, a real Express app, a mocked service underneath), reach for `@dolphjs/testing`'s `createTestingApp` in a hand-written `*.e2e-spec.ts` — the generator doesn't scaffold this tier for you, since it needs a wired-up component to test against.

## WebSockets

```bash
dolph generate --socket chat
```

Writes **two** files to `src/shared/socket/`: `chat.socket.service.ts` (extends `DolphSocketServiceHandler`, a starter `connection` handler) and `chat.socket.component.ts` (`@Socket({ services: [], socketServices: [ChatSocketService] })`). Neither does anything until you wire the component into `server.ts` yourself — the CLI prints the snippet you need after generating:

```typescript
new DolphFactory([...], { socketService: SocketService, component: new ChatSocketComponent() })
```

## GraphQL Mode

Choosing `graphql` at `dolph new` always uses spring routing and TypeScript, and additionally scaffolds:

- `src/setup.ts` — `buildSchema()` (empty `resolvers: []` to start) and a `context` function.
- `src/shared/configs/data_source.ts` — a TypeORM `DataSource`, **unless** you chose `mongo` as your database (TypeORM isn't in play there — Mongoose is, same as REST mode).
- `server.ts` wired to `AppDataSource.initialize().then(() => dolph.start())`, passing `schema()` (a `Promise<GraphQLSchema>`) straight into `DolphFactory`'s GraphQL adapter, which awaits it internally.

`dolph generate --all <name>` in GraphQL mode produces an entity, a resolver, and an input instead of a REST controller/DTO/model.

## Building From Source

```bash
cargo build
cargo run --bin dolph new <project_name>
cargo run --bin dolph generate <flags> <name>
cargo run --bin dolph watch
cargo run --bin dolph watch --bun
cargo run --bin dolph build
cargo run --bin dolph start
cargo run --bin dolph start --bun
```

## License

MIT — see [LICENSE](./LICENSE).
