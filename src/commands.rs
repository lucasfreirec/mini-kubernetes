use crate::model::{Workload, WorkloadManifest};
use crate::store::WorkloadStore;
use anyhow::{Context, Result};
use tracing::info;

pub fn apply(store: &WorkloadStore, file: &std::path::Path) -> Result<()> {
    let raw = std::fs::read_to_string(file)
        .with_context(|| format!("reading manifest {}", file.display()))?;
    let manifest: WorkloadManifest = serde_yaml::from_str(&raw)
        .with_context(|| format!("parsing manifest {}", file.display()))?;
    manifest.validate().context("invalid workload manifest")?;

    let workload = match store.find(&manifest.metadata.name) {
        Ok(mut existing) => {
            existing.apply_update(manifest);
            info!(workload_id = %existing.id, name = %existing.name, version = existing.version, "updated workload");
            existing
        }
        Err(_) => {
            let workload = Workload::new(manifest);
            info!(workload_id = %workload.id, name = %workload.name, "created workload");
            workload
        }
    };
    store.save(&workload)?;
    println!("workload/{} applied (id={})", workload.name, workload.id);
    Ok(())
}

pub fn list_workloads(store: &WorkloadStore) -> Result<()> {
    let workloads = store.list()?;
    if workloads.is_empty() {
        println!("No workloads found.");
        return Ok(());
    }
    println!(
        "{:<36} {:<20} {:>8} {:>7}",
        "ID", "NAME", "REPLICAS", "VERSION"
    );
    for w in workloads {
        println!(
            "{:<36} {:<20} {:>8} {:>7}",
            w.id, w.name, w.spec.replicas, w.version
        );
    }
    Ok(())
}

pub fn describe(store: &WorkloadStore, id: &str) -> Result<()> {
    let workload = store
        .find(id)
        .with_context(|| format!("workload '{id}' not found"))?;
    println!("{}", serde_json::to_string_pretty(&workload)?);
    Ok(())
}

pub fn delete(store: &WorkloadStore, id: &str) -> Result<()> {
    let workload = store
        .delete(id)
        .with_context(|| format!("workload '{id}' not found"))?;
    info!(workload_id = %workload.id, name = %workload.name, "deleted workload");
    println!("workload/{} deleted", workload.name);
    Ok(())
}

pub fn stub(feature: &str, stage: &str) {
    println!("'{feature}' is not implemented yet -- arrives in {stage}.");
}
