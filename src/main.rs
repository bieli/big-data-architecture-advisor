use crepe::crepe;
use std::env;

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

    // === Strict rules for default mode ===
    Recommendation("Hadoop/Spark", 60) <- Volume("large"), Workload("batch"), SLA("low"), Budget("low");
    Explanation("Hadoop/Spark chosen for large batch data, low SLA, limited budget.")
        <- Volume("large"), Workload("batch"), SLA("low"), Budget("low");

    Recommendation("Data Warehouse", 70) <- Volume("medium"), Workload("batch"), SLA("high");
    Explanation("Data Warehouse chosen for medium batch workloads with high SLA.")
        <- Volume("medium"), Workload("batch"), SLA("high");

    Recommendation("Kafka/Flink", 90) <- Workload("streaming"), SLA("high");
    Explanation("Kafka/Flink chosen for real-time streaming with strict SLA.")
        <- Workload("streaming"), SLA("high");

    Recommendation("Lakehouse", 95) <- Volume("large"), Workload("mixed"), SLA("high"), Budget("high");
    Explanation("Lakehouse chosen for large mixed workloads, high SLA, sufficient budget.")
        <- Volume("large"), Workload("mixed"), SLA("high"), Budget("high");

    Recommendation("Cloud DW (BigQuery/Snowflake)", 85) <- Volume("medium"), Workload("mixed"), SLA("high"), Budget("high");
    Explanation("Cloud DW chosen for medium mixed workloads, high SLA, high budget.")
        <- Volume("medium"), Workload("mixed"), SLA("high"), Budget("high");

    Recommendation("Data Mesh", 90) <- Volume("large"), Workload("mixed"), SLA("high");
    Explanation("Data Mesh chosen for large distributed domains, mixed workloads, decentralized ownership.")
        <- Volume("large"), Workload("mixed"), SLA("high");

    Recommendation("ETL Pipelines", 50) <- Workload("batch"), SLA("low"), Budget("low");
    Explanation("ETL Pipelines chosen for batch workloads, relaxed SLA, limited budget.")
        <- Workload("batch"), SLA("low"), Budget("low");

    Recommendation("ML Platform (Kubeflow/MLflow)", 88) <- Workload("mixed"), SLA("high"), Budget("high");
    Explanation("ML Platform chosen for advanced analytics, ML, sufficient budget.")
        <- Workload("mixed"), SLA("high"), Budget("high");

    Recommendation("Hybrid Cloud", 80) <- Volume("large"), Workload("mixed"), SLA("high");
    Explanation("Hybrid Cloud chosen for large mixed workloads, strict SLA, flexibility across on-prem/cloud.")
        <- Volume("large"), Workload("mixed"), SLA("high");

    Recommendation("Data Lake (S3/ADLS)", 75) <- Volume("large"), Workload("batch"), SLA("high"), Budget("high");
    Explanation("Data Lake chosen for large-scale storage, batch workloads, high SLA, high budget.")
        <- Volume("large"), Workload("batch"), SLA("high"), Budget("high");

    Recommendation("Orchestration (Airflow/Prefect)", 65) <- Workload("batch"), SLA("high");
    Explanation("Orchestration chosen for managing complex batch pipelines with SLA guarantees.")
        <- Workload("batch"), SLA("high");

    Recommendation("Governance Layer (Collibra/Alation)", 70) <- Volume("large"), SLA("high");
    Explanation("Governance chosen for large datasets with strict SLA and compliance needs.")
        <- Volume("large"), SLA("high");

    // === Observability rules ===
    Recommendation("Monitoring Stack (Prometheus/Grafana)", 85) <- Observability("high");
    Explanation("Monitoring stack chosen for deep metrics, dashboards, and SLA compliance.")
        <- Observability("high");

    Recommendation("Distributed Tracing (OpenTelemetry/Jaeger)", 88) <- Observability("high"), Workload("streaming");
    Explanation("Tracing chosen for end-to-end visibility across streaming pipelines.")
        <- Observability("high"), Workload("streaming");

    Recommendation("Data Lineage Tools (OpenLineage/Marquez)", 80) <- Observability("high"), Volume("large");
    Explanation("Lineage tools chosen for compliance and auditability in large-scale pipelines.")
        <- Observability("high"), Volume("large");

    Recommendation("Cloud-native Monitoring (CloudWatch/Stackdriver/Azure Monitor)", 75) <- Observability("medium"), Budget("high");
    Explanation("Cloud-native monitoring chosen for integrated observability in cloud ecosystems.")
        <- Observability("medium"), Budget("high");

    Recommendation("ELK Stack (Elasticsearch/Logstash/Kibana)", 70) <- Observability("medium"), Budget("low");
    Explanation("ELK stack chosen for centralized logging with limited budget.")
        <- Observability("medium"), Budget("low");

    Recommendation("Alerting & Incident Response (PagerDuty/OpsGenie)", 90) <- Observability("high"), SLA("high");
    Explanation("Alerting chosen for automated incident response and SLA adherence.")
        <- Observability("high"), SLA("high");

    Recommendation("FinOps Dashboards", 78) <- Observability("high"), Budget("low");
    Explanation("FinOps dashboards chosen for cost control and resource optimization.")
        <- Observability("high"), Budget("low");
}

// === Profiles for fuzzy scoring in --multi mode ===
struct ArchProfile {
    name: &'static str,
    volume: &'static str,
    workload: &'static str,
    sla: &'static str,
    budget: &'static str,
    observability: &'static str,
    base: i32,
}

fn similarity_score(
    user: (&str, &str, &str, &str, &str),
    weights: (i32, i32, i32, i32, i32),
    arch: &ArchProfile,
) -> i32 {
    let (vol, wl, sla, bud, obs) = user;
    let (sla_w, bud_w, vol_w, wl_w, obs_w) = weights;
    let mut score = arch.base;

    // Volume
    if arch.volume == "any" {
        score += vol_w * 5;
    } else if arch.volume == vol {
        score += vol_w * 10;
    }

    // Workload
    if arch.workload == "any" {
        score += wl_w * 5;
    } else if arch.workload == wl {
        score += wl_w * 10;
    }

    // SLA
    if arch.sla == "any" {
        score += sla_w * 5;
    } else if arch.sla == sla {
        score += sla_w * 10;
    }

    // Budget
    if arch.budget == "any" {
        score += bud_w * 5;
    } else if arch.budget == bud {
        score += bud_w * 10;
    }

    // Observability
    if arch.observability == "any" {
        score += obs_w * 5;
    } else if arch.observability == obs {
        score += obs_w * 10;
    }

    score
}

fn read_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().to_lowercase()
}

fn read_weight(prompt: &str, default: i32) -> i32 {
    println!("{} (default {}):", prompt, default);
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    match input.trim().parse::<i32>() {
        Ok(val) => val,
        Err(_) => default,
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    println!("=== Big Data Architecture Advisor CLI ===");
    if args.len() > 1 && args[1] == "--multi" {
        println!("*** MULTI Recommendations Mode ***\n");
    } else {
        println!("*** STRICT Rule-Based Mode (using Crepe library) ***\n");
    }

    let volume = read_input("Enter data volume (small/medium/large):");
    let workload = read_input("Enter workload type (batch/streaming/mixed):");
    let sla = read_input("Enter SLA requirement (low/high):");
    let budget = read_input("Enter budget (low/high):");
    let observability = read_input("Enter observability requirement (low/medium/high):");

    let sla_w = read_weight("Enter SLA weight", 5);
    let budget_w = read_weight("Enter Budget weight", 3);
    let volume_w = read_weight("Enter Volume weight", 2);
    let workload_w = read_weight("Enter Workload weight", 1);
    let observability_w = read_weight("Enter Observability weight", 4);

    if args.len() > 1 && args[1] == "--multi" {
        let profiles = vec![
            ArchProfile {
                name: "Lakehouse",
                volume: "large",
                workload: "mixed",
                sla: "high",
                budget: "high",
                observability: "high",
                base: 95,
            },
            ArchProfile {
                name: "Kafka/Flink",
                volume: "any",
                workload: "streaming",
                sla: "high",
                budget: "any",
                observability: "high",
                base: 90,
            },
            ArchProfile {
                name: "Cloud-native Monitoring (CloudWatch/Stackdriver/Azure Monitor)",
                volume: "any",
                workload: "any",
                sla: "any",
                budget: "high",
                observability: "medium",
                base: 75,
            },
            ArchProfile {
                name: "Data Mesh",
                volume: "large",
                workload: "mixed",
                sla: "high",
                budget: "any",
                observability: "high",
                base: 90,
            },
            ArchProfile {
                name: "ML Platform (Kubeflow/MLflow)",
                volume: "any",
                workload: "mixed",
                sla: "high",
                budget: "high",
                observability: "high",
                base: 88,
            },
            ArchProfile {
                name: "Hybrid Cloud",
                volume: "large",
                workload: "mixed",
                sla: "high",
                budget: "any",
                observability: "high",
                base: 80,
            },
            ArchProfile {
                name: "Data Lake (S3/ADLS)",
                volume: "large",
                workload: "batch",
                sla: "high",
                budget: "high",
                observability: "high",
                base: 75,
            },
            ArchProfile {
                name: "Orchestration (Airflow/Prefect)",
                volume: "any",
                workload: "batch",
                sla: "high",
                budget: "any",
                observability: "medium",
                base: 65,
            },
            ArchProfile {
                name: "Governance Layer (Collibra/Alation)",
                volume: "large",
                workload: "any",
                sla: "high",
                budget: "any",
                observability: "high",
                base: 70,
            },
            ArchProfile {
                name: "Monitoring Stack (Prometheus/Grafana)",
                volume: "any",
                workload: "any",
                sla: "any",
                budget: "any",
                observability: "high",
                base: 85,
            },
            ArchProfile {
                name: "Distributed Tracing (OpenTelemetry/Jaeger)",
                volume: "any",
                workload: "streaming",
                sla: "any",
                budget: "any",
                observability: "high",
                base: 88,
            },
            ArchProfile {
                name: "Data Lineage Tools (OpenLineage/Marquez)",
                volume: "large",
                workload: "any",
                sla: "any",
                budget: "any",
                observability: "high",
                base: 80,
            },
            ArchProfile {
                name: "ELK Stack (Elasticsearch/Logstash/Kibana)",
                volume: "any",
                workload: "any",
                sla: "any",
                budget: "low",
                observability: "medium",
                base: 70,
            },
            ArchProfile {
                name: "Alerting & Incident Response (PagerDuty/OpsGenie)",
                volume: "any",
                workload: "any",
                sla: "high",
                budget: "any",
                observability: "high",
                base: 90,
            },
            ArchProfile {
                name: "FinOps Dashboards",
                volume: "any",
                workload: "any",
                sla: "any",
                budget: "low",
                observability: "high",
                base: 78,
            },
            ArchProfile {
                name: "Hadoop/Spark",
                volume: "large",
                workload: "batch",
                sla: "low",
                budget: "low",
                observability: "low",
                base: 60,
            },
            ArchProfile {
                name: "Data Warehouse",
                volume: "medium",
                workload: "batch",
                sla: "high",
                budget: "any",
                observability: "medium",
                base: 70,
            },
            ArchProfile {
                name: "ETL Pipelines",
                volume: "any",
                workload: "batch",
                sla: "low",
                budget: "low",
                observability: "low",
                base: 50,
            },
        ];

        let user = (
            volume.as_str(),
            workload.as_str(),
            sla.as_str(),
            budget.as_str(),
            observability.as_str(),
        );
        let weights = (sla_w, budget_w, volume_w, workload_w, observability_w);

        let mut scored: Vec<_> = profiles
            .iter()
            .map(|p| (p.name, similarity_score(user, weights, p)))
            .collect();

        scored.sort_by(|a, b| b.1.cmp(&a.1));

        println!("\n=== Recommendations ===");
        for (name, score) in scored {
            println!("-> {} (score: {})", name, score);
        }
    } else {
        let mut runtime = Crepe::new();

        runtime.extend(&[Volume(Box::leak(volume.clone().into_boxed_str()))]);
        runtime.extend(&[Workload(Box::leak(workload.clone().into_boxed_str()))]);
        runtime.extend(&[SLA(Box::leak(sla.clone().into_boxed_str()))]);
        runtime.extend(&[Budget(Box::leak(budget.clone().into_boxed_str()))]);
        runtime.extend(&[Observability(Box::leak(
            observability.clone().into_boxed_str(),
        ))]);

        let (recs, expls) = runtime.run();

        let mut recs_adjusted: Vec<Recommendation> = recs
            .into_iter()
            .map(|Recommendation(r, base)| {
                let weight_sum = sla_w + budget_w + volume_w + workload_w + observability_w;
                let score = base * weight_sum / 10;
                Recommendation(r, score)
            })
            .collect();

        recs_adjusted.sort_by(|a, b| b.1.cmp(&a.1));

        println!("\n=== Recommendations ===");
        if let Some(Recommendation(r, score)) = recs_adjusted.first() {
            println!("-> {} (score: {})", r, score);
        }

        println!("\n=== Explanations ===");
        for Explanation(e) in expls {
            println!("- {}", e);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run_strict(
        volume: &str,
        workload: &str,
        sla: &str,
        budget: &str,
        observability: &str,
        sla_w: i32,
        budget_w: i32,
        volume_w: i32,
        workload_w: i32,
        observability_w: i32,
    ) -> Vec<Recommendation> {
        let mut runtime = Crepe::new();
        runtime.extend(&[Volume(Box::leak(volume.to_string().into_boxed_str()))]);
        runtime.extend(&[Workload(Box::leak(workload.to_string().into_boxed_str()))]);
        runtime.extend(&[SLA(Box::leak(sla.to_string().into_boxed_str()))]);
        runtime.extend(&[Budget(Box::leak(budget.to_string().into_boxed_str()))]);
        runtime.extend(&[Observability(Box::leak(
            observability.to_string().into_boxed_str(),
        ))]);

        let (recs, _expls) = runtime.run();
        let mut recs_adjusted: Vec<Recommendation> = recs
            .into_iter()
            .map(|Recommendation(r, base)| {
                let weight_sum = sla_w + budget_w + volume_w + workload_w + observability_w;
                let score = base * weight_sum / 10;
                Recommendation(r, score)
            })
            .collect();
        recs_adjusted.sort_by(|a, b| b.1.cmp(&a.1));
        recs_adjusted
    }

    fn run_multi(
        volume: &str,
        workload: &str,
        sla: &str,
        budget: &str,
        observability: &str,
        sla_w: i32,
        budget_w: i32,
        volume_w: i32,
        workload_w: i32,
        observability_w: i32,
    ) -> Vec<(String, i32)> {
        let profiles = vec![
            ArchProfile {
                name: "Lakehouse",
                volume: "large",
                workload: "mixed",
                sla: "high",
                budget: "high",
                observability: "high",
                base: 95,
            },
            ArchProfile {
                name: "Kafka/Flink",
                volume: "any",
                workload: "streaming",
                sla: "high",
                budget: "any",
                observability: "high",
                base: 90,
            },
            ArchProfile {
                name: "Cloud-native Monitoring",
                volume: "any",
                workload: "any",
                sla: "any",
                budget: "high",
                observability: "medium",
                base: 75,
            },
            ArchProfile {
                name: "ETL Pipelines",
                volume: "any",
                workload: "batch",
                sla: "low",
                budget: "low",
                observability: "low",
                base: 50,
            },
        ];
        let user = (volume, workload, sla, budget, observability);
        let weights = (sla_w, budget_w, volume_w, workload_w, observability_w);

        let mut scored: Vec<_> = profiles
            .iter()
            .map(|p| (p.name.to_string(), similarity_score(user, weights, p)))
            .collect();
        scored.sort_by(|a, b| b.1.cmp(&a.1));
        scored
    }

    #[test]
    fn test_strict_mode_lakehouse() {
        let recs = run_strict("large", "mixed", "high", "high", "high", 5, 3, 2, 1, 4);
        assert_eq!(recs[0].0, "Lakehouse");
        assert!(recs[0].1 > 90);
    }

    #[test]
    fn test_strict_mode_kafka() {
        let recs = run_strict("small", "streaming", "high", "low", "high", 5, 3, 2, 1, 4);
        assert_eq!(recs[0].0, "Kafka/Flink");
    }

    #[test]
    fn test_multi_mode_ranking() {
        let recs = run_multi("large", "mixed", "low", "high", "medium", 5, 2, 1, 1, 3);
        // Cloud-native Monitoring should appear, but Lakehouse also gets partial score
        assert!(recs
            .iter()
            .any(|(name, _)| name.contains("Cloud-native Monitoring")));
        assert!(recs.iter().any(|(name, _)| name.contains("Lakehouse")));
        // Ensure multiple recommendations are returned
        assert!(recs.len() > 1);
    }

    #[test]
    fn test_multi_mode_ordering() {
        let recs = run_multi("batch", "batch", "low", "low", "low", 5, 2, 1, 1, 3);
        // ETL Pipelines should rank higher than Lakehouse in this scenario
        let etl_score = recs.iter().find(|(n, _)| n == "ETL Pipelines").unwrap().1;
        let lakehouse_score = recs.iter().find(|(n, _)| n == "Lakehouse").unwrap().1;
        assert!(etl_score >= lakehouse_score);
    }
}
