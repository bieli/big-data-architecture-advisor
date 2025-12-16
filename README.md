# big-data-architecture-advisor
Kind of "expert system" for BigData architects roles ;-) For fun &amp; learning!

## Motivation
Modern data systems are complex, distributed, and often difficult to design. Architects must balance multiple factors such as **data volume, workload type, SLA requirements, budget constraints, and observability needs**. Traditional decision-making is often ad hoc and opaque, making it hard to justify choices or reproduce them later.

This project was motivated by the need for a **transparent, rule-based expert assistant** that:
- Encodes architectural knowledge as declarative rules.
- Produces clear recommendations with explanations.
- Supports multiple criteria, including observability, cost, and compliance.
- Allows dynamic weighting of priorities (e.g., SLA vs. budget vs. observability).
- Provides reproducible, explainable decisions for engineers and architects.

## How It Works
The advisor is implemented in **Rust** using the [`crepe`](https://crates.io/crates/crepe) Datalog engine. It works as follows:

1. **Inputs**: The user provides facts about their scenario:
   - Data Volume: `small`, `medium`, `large`
   - Workload Type: `batch`, `streaming`, `mixed`
   - SLA Requirement: `low`, `high`
   - Budget: `low`, `high`
   - Observability: `low`, `medium`, `high`

2. **Rules**: The system encodes architectural knowledge as declarative rules. Examples:
   - Large batch + low SLA + low budget → Hadoop/Spark
   - Streaming + high SLA → Kafka/Flink
   - Large mixed + high SLA + high budget → Lakehouse
   - High observability → Prometheus/Grafana, OpenTelemetry/Jaeger, lineage tools

3. **Scoring**: Each recommendation has a base score. The user can assign **weights** to criteria (SLA, budget, volume, workload, observability). Final scores are adjusted dynamically based on these weights.

4. **CLI Interface**: The program runs interactively:
   - Prompts the user for inputs.
   - Prompts for weights (or uses defaults).
   - Produces ranked recommendations.
   - By default shows the top suggestion; with `--multi` flag shows all ranked suggestions.
   - With `--history` flag, prints past saved decisions.

## Example Run
```bash
$ cargo run
=== Big Data Architecture Advisor CLI ===
Enter data volume (small/medium/large): large
Enter workload type (batch/streaming/mixed): mixed
Enter SLA requirement (low/high): high
Enter budget (low/high): high
Enter observability requirement (low/medium/high): high
Enter SLA weight (default 5): 10
Enter Budget weight (default 3): 1
Enter Volume weight (default 2): 2
Enter Workload weight (default 1): 1
Enter Observability weight (default 4): 5

=== Recommendations ===
-> Lakehouse (score: 190)
-> Data Mesh (score: 180)
-> ML Platform (score: 176)
-> Hybrid Cloud (score: 160)
-> Monitoring Stack (Prometheus/Grafana) (score: 155)

=== Explanations ===
- Lakehouse chosen for large mixed workloads, high SLA, sufficient budget.
- Data Mesh chosen for large distributed domains, mixed workloads, decentralized ownership.
- ML Platform chosen for advanced analytics, ML, sufficient budget.
- Hybrid Cloud chosen for large mixed workloads, strict SLA, flexibility across on-prem/cloud.
- Monitoring stack chosen for deep metrics, dashboards, and SLA compliance.
```


## TODO list
- [ ] Add --clear-history flag.
- [ ] Extend rules for serverless, data fabric, AI pipelines.
- [ ] Add YAML/JSON config for batch runs.
- [ ] Integrate with web UI for interactive decision support.
