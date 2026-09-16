# mini-kubernetes

A small, Linux-first container orchestrator built from scratch to learn *why* Kubernetes has
containers, nodes, scheduling, controllers, desired state, and reconciliation — not an attempt
to reproduce Kubernetes itself.

The path: `process → isolated process → managed container → node workload → scheduled workload
→ reconciled cluster workload`.

## Host requirements

- **Linux only.** Namespaces and cgroups v2 (added from Stage 2 onward) require a real Linux
  kernel — this does not run on macOS/Windows directly.
- Developed and tested against **Ubuntu 24.04 LTS** (kernel 6.8+) with the unified cgroup v2
  hierarchy.
- Run inside a **disposable VM**, not a production or shared host. This project runs
  unconstrained/experimental workloads and manipulates namespaces, cgroups, and (later) network
  devices directly.
- Later stages (namespaces, cgroups, mounts, network devices) require root or equivalent
  capabilities (`CAP_SYS_ADMIN`, `CAP_NET_ADMIN`). Stage 0 needs no elevated privileges.
- Never run untrusted workloads through this project. It makes no security or multi-tenant
  isolation guarantees.

### This repo's dev setup

Built and run inside a Lima VM (`kube-lab`, Ubuntu 24.04) from a macOS host, since macOS can't
run the Linux-specific syscalls this project depends on.

```bash
limactl shell kube-lab -- bash -lc 'cd /path/to/mini-kubernetes && cargo build'
```

## Data model

- **Workload spec**: command, environment, root filesystem, resource limits, restart policy,
  replicas, optional port ([src/model/workload.rs](src/model/workload.rs)).
- **Instance**: one concrete runtime execution of a workload, with an explicit lifecycle state
  machine ([src/model/instance.rs](src/model/instance.rs)):

  ```text
  Pending → Assigned → Starting → Running → Succeeded | Failed
                     ↘ Stopping → Stopped
  ```
- **Node**: a machine/agent with capacity, labels, and health ([src/model/node.rs](src/model/node.rs)).
  Unused until Stage 4.

Workload records are immutable-id, versioned (`version` increments on every `apply`), and
persisted as flat JSON files under the configured data directory (a real store arrives in
Stage 5).

## CLI

```bash
mini apply <manifest.yaml>   # create or update a workload (idempotent)
mini workloads               # list workloads
mini describe <id|name>      # show the full record
mini delete <id|name>        # delete a workload
mini nodes                   # stub until Stage 4
mini instances               # stub until Stage 1
mini logs <instance-id>      # stub until Stage 1
```

Data directory resolution order: `--data-dir` flag > `MINI_DATA_DIR` env var > platform data
dir (`~/.local/share/mini` on Linux) > `./.mini`.

Logging is structured via `tracing`; control verbosity with `RUST_LOG` (defaults to `info`).

## Example manifest

See [examples/echo-service.yaml](examples/echo-service.yaml).

## Roadmap

This project follows a staged roadmap (process supervisor → container runtime → local
networking → node agent → control plane/scheduler → cluster reconciliation → observability).
Each stage has its own demo and is developed on its own branch/PR. Stage 0 (this state):
data model, CLI skeleton, structured logging, config, workload CRUD via a flat-file store —
no containers yet.
