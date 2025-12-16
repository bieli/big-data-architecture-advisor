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

## How It Works?
The advisor is implemented in **Rust** using the [`crepe`](https://crates.io/crates/crepe) Datalog engine. It works as follows:

```rust
crepe! {
    @input
    struct Volume(&'static str);
    @input
    struct Workload(&'static str);
    @input
    struct SLA(&'static str);
    @input
    struct Budget(&'static str);
    @input
    struct Observability(&'static str);

    @output
    struct Recommendation(&'static str, i32);
    @output
    struct Explanation(&'static str);

...
```

### Important points
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


## Multi Mode (`--multi`)

By default, the advisor uses **strict rule-based matching**: only architectures that exactly fit the provided inputs (volume, workload, SLA, budget, observability) are recommended.

With the `--multi` flag, the advisor switches to **fuzzy similarity scoring**:

- Every architecture profile is evaluated against the user’s inputs.
- Exact matches contribute full points; `"any"` or partial matches contribute partial points.
- User-defined weights (SLA, budget, volume, workload, observability) influence the scoring.
- All architectures are ranked and displayed, not just the top match.

This mode is useful when:
- You want to explore **alternative architectures** beyond the strict best fit.
- You need to see **trade-offs** between SLA, cost, observability, and workload types.
- You’re experimenting with different weights to understand how priorities shift recommendations.


## Example Run

### Strict onr recommendation mode

```bash
$ cargo run
=== Big Data Architecture Advisor CLI ===
*** STRICT Rule-Based Mode (using Crepe library) ***

Enter data volume (small/medium/large):
medium
Enter workload type (batch/streaming/mixed):
mixed
Enter SLA requirement (low/high):
low
Enter budget (low/high):
high
Enter observability requirement (low/medium/high):
medium
Enter SLA weight (default 5):
2
Enter Budget weight (default 3):
3
Enter Volume weight (default 2):
1
Enter Workload weight (default 1):
1
Enter Observability weight (default 4):
4

=== Recommendations ===
-> Cloud-native Monitoring (CloudWatch/Stackdriver/Azure Monitor) (score: 82)

=== Explanations ===
- Cloud-native monitoring chosen for integrated observability in cloud ecosystems.
```

### Multi recommendations mode

```bash
$ cargo run -- --multi
=== Big Data Architecture Advisor CLI ===
*** MULTI Recommendations Mode ***

Enter data volume (small/medium/large):
large
Enter workload type (batch/streaming/mixed):
mixed
Enter SLA requirement (low/high):
low
Enter budget (low/high):
high
Enter observability requirement (low/medium/high):
medium
Enter SLA weight (default 5):
3
Enter Budget weight (default 3):
2
Enter Volume weight (default 2):
1
Enter Workload weight (default 1):
1
Enter Observability weight (default 4):
3

=== Recommendations ===
-> Cloud-native Monitoring (CloudWatch/Stackdriver/Azure Monitor) (score: 150)
-> Lakehouse (score: 135)
-> ELK Stack (Elasticsearch/Logstash/Kibana) (score: 125)
-> ML Platform (Kubeflow/MLflow) (score: 123)
-> Data Mesh (score: 120)
-> Monitoring Stack (Prometheus/Grafana) (score: 120)
-> Data Lineage Tools (OpenLineage/Marquez) (score: 120)
-> Distributed Tracing (OpenTelemetry/Jaeger) (score: 118)
-> Hybrid Cloud (score: 110)
-> Orchestration (Airflow/Prefect) (score: 110)
-> Alerting & Incident Response (PagerDuty/OpsGenie) (score: 110)
-> Data Warehouse (score: 110)
-> Kafka/Flink (score: 105)
-> Data Lake (S3/ADLS) (score: 105)
-> FinOps Dashboards (score: 103)
-> Hadoop/Spark (score: 100)
-> Governance Layer (Collibra/Alation) (score: 95)
-> ETL Pipelines (score: 85)
```

## TODO list
- [ ] Add --clear-history flag.
- [ ] Extend rules for serverless, data fabric, AI pipelines.
- [ ] Add YAML/JSON config for batch runs.
- [ ] Integrate with web UI for interactive decision support.
