# mini-kubernetes

A small, Linux-first container orchestrator built from scratch to learn Kubernetes

The path: `process → isolated process → managed container → node workload → scheduled workload
→ reconciled cluster workload`.

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
