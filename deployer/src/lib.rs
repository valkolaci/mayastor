pub mod infra;

use infra::*;

use composer::{Builder, TEST_LABEL_PREFIX};
use std::{convert::TryInto, str::FromStr, time::Duration};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct CliArgs {
    #[structopt(subcommand)]
    action: Action,
}

#[derive(Debug, StructOpt)]
#[structopt(about = "Deployment actions")]
pub(crate) enum Action {
    Start(StartOptions),
    Stop(StopOptions),
    List(ListOptions),
}

/// docker network label:
/// $prefix.name = $name
const DEFAULT_CLUSTER_LABEL: &str = ".cluster";

#[derive(Debug, StructOpt)]
#[structopt(about = "Stop and delete all components")]
pub struct StopOptions {
    /// Label for the cluster
    /// In the form of "prefix.name"
    #[structopt(short, long, default_value = DEFAULT_CLUSTER_LABEL)]
    pub cluster_label: ClusterLabel,
}

#[derive(Debug, Default, StructOpt)]
#[structopt(about = "List all running components")]
pub struct ListOptions {
    /// Simple list without using the docker executable
    #[structopt(short, long)]
    pub no_docker: bool,

    /// Format the docker output
    #[structopt(short, long, conflicts_with = "no_docker")]
    pub format: Option<String>,

    /// Label for the cluster
    #[structopt(short, long, default_value = DEFAULT_CLUSTER_LABEL)]
    pub cluster_label: ClusterLabel,
}

/// Label for a cluster: $filter.name = $name
#[derive(Default, Clone)]
pub struct ClusterLabel {
    prefix: String,
    name: String,
}

/// default enabled agents
pub fn default_agents() -> &'static str {
    "Core"
}

#[derive(Debug, Default, Clone, StructOpt)]
#[structopt(about = "Create and start all components")]
pub struct StartOptions {
    /// Use the following Control Plane Agents
    /// Specify one agent at a time or as a list.
    /// ( "" for no agents )
    /// todo: specify start arguments, eg: Node="-v"
    #[structopt(
        short,
        long,
        default_value = default_agents(),
        value_delimiter = ","
    )]
    pub agents: Vec<ControlPlaneAgent>,

    /// Kubernetes Config file if using operators
    /// [default: "~/.kube/config"]
    #[structopt(long)]
    pub kube_config: Option<String>,

    /// Use a base image for the binary components (eg: alpine:latest)
    #[structopt(long)]
    pub base_image: Option<String>,

    /// Use the jaegertracing service
    #[structopt(short, long)]
    pub jaeger: bool,

    /// Use the elasticsearch service
    #[structopt(short, long)]
    pub elastic: bool,

    /// Use the kibana service.
    /// Note: the index pattern is only created when used in conjunction with `wait_timeout`
    #[structopt(short, long)]
    pub kibana: bool,

    /// Disable the REST Server
    #[structopt(long)]
    pub no_rest: bool,

    /// Use `N` mayastor instances
    #[structopt(short, long, default_value = "1")]
    pub mayastors: u32,

    /// Cargo Build each component before deploying
    #[structopt(short, long)]
    pub build: bool,

    /// Cargo Build the workspace before deploying
    #[structopt(long)]
    pub build_all: bool,

    /// Use a dns resolver for the cluster: defreitas/dns-proxy-server
    /// Note this messes with your /etc/resolv.conf so use at your own risk
    #[structopt(short, long)]
    pub dns: bool,

    /// Show information from the cluster after creation
    #[structopt(short, long)]
    pub show_info: bool,

    /// Name of the cluster - currently only one allowed at a time.
    /// Note: Does not quite work as intended, as we haven't figured out of to bridge between
    /// different networks
    #[structopt(short, long, default_value = DEFAULT_CLUSTER_LABEL)]
    pub cluster_label: ClusterLabel,

    /// Disable the etcd service
    #[structopt(long)]
    pub no_etcd: bool,

    /// Disable the nats service
    #[structopt(long)]
    pub no_nats: bool,

    /// The period at which the registry updates its cache of all
    /// resources from all nodes
    #[structopt(long)]
    pub cache_period: Option<humantime::Duration>,

    /// Override the node's deadline for the Core Agent
    #[structopt(long)]
    pub node_deadline: Option<humantime::Duration>,

    /// Override the node's request timeout
    #[structopt(long)]
    pub node_req_timeout: Option<humantime::Duration>,

    /// Override the node's connection timeout
    #[structopt(long)]
    pub node_conn_timeout: Option<humantime::Duration>,

    /// Override the core agent's store operation timeout
    #[structopt(long)]
    pub store_timeout: Option<humantime::Duration>,

    /// Override the core agent's reconcile period
    #[structopt(long)]
    pub reconcile_period: Option<humantime::Duration>,

    /// Override the core agent's reconcile idle period
    #[structopt(long)]
    pub reconcile_idle_period: Option<humantime::Duration>,

    /// Amount of time to wait for all containers to start.
    #[structopt(short, long)]
    pub wait_timeout: Option<humantime::Duration>,

    /// Don't stop/remove existing containers on the same cluster
    /// Allows us to start "different" stacks independently, eg:
    /// > deployer start -ejk -s -a "" --no-nats --no-rest --no-etcd -m 0
    /// > deployer start -s -m 2
    #[structopt(short, long)]
    pub reuse_cluster: bool,
}

impl StartOptions {
    pub fn with_agents(mut self, agents: Vec<&str>) -> Self {
        let agents: ControlPlaneAgents = agents.try_into().unwrap();
        self.agents = agents.into_inner();
        self
    }
    pub fn with_cache_period(mut self, period: &str) -> Self {
        self.cache_period = Some(humantime::Duration::from_str(period).unwrap());
        self
    }
    pub fn with_node_deadline(mut self, deadline: &str) -> Self {
        self.node_deadline = Some(humantime::Duration::from_str(deadline).unwrap());
        self
    }
    pub fn with_store_timeout(mut self, timeout: Duration) -> Self {
        self.store_timeout = Some(timeout.into());
        self
    }
    pub fn with_reconcile_period(mut self, busy: Duration, idle: Duration) -> Self {
        self.reconcile_period = Some(busy.into());
        self.reconcile_idle_period = Some(idle.into());
        self
    }
    pub fn with_node_timeouts(mut self, connect: Duration, request: Duration) -> Self {
        self.node_conn_timeout = Some(connect.into());
        self.node_req_timeout = Some(request.into());
        self
    }
    pub fn with_rest(mut self, enabled: bool) -> Self {
        self.no_rest = !enabled;
        self
    }
    pub fn with_jaeger(mut self, jaeger: bool) -> Self {
        self.jaeger = jaeger;
        self
    }
    pub fn with_build(mut self, build: bool) -> Self {
        self.build = build;
        self
    }
    pub fn with_build_all(mut self, build: bool) -> Self {
        self.build_all = build;
        self
    }
    pub fn with_mayastors(mut self, mayastors: u32) -> Self {
        self.mayastors = mayastors;
        self
    }
    pub fn with_show_info(mut self, show_info: bool) -> Self {
        self.show_info = show_info;
        self
    }
    pub fn with_cluster_name(mut self, cluster_name: &str) -> Self {
        self.cluster_label = format!(".{}", cluster_name).parse().unwrap();
        self
    }
    pub fn with_base_image(mut self, base_image: impl Into<Option<String>>) -> Self {
        self.base_image = base_image.into();
        self
    }
}

impl CliArgs {
    /// Act upon the requested action
    pub async fn execute(&self) -> Result<(), Error> {
        self.action.execute().await
    }
}

impl Action {
    async fn execute(&self) -> Result<(), Error> {
        match self {
            Action::Start(options) => options.start(self).await,
            Action::Stop(options) => options.stop(self).await,
            Action::List(options) => options.list(self).await,
        }
    }
}

impl StartOptions {
    async fn start(&self, _action: &Action) -> Result<(), Error> {
        let components = Components::new(self.clone());
        let composer = Builder::new()
            .name(&self.cluster_label.name())
            .label_prefix(&self.cluster_label.prefix())
            .with_clean(false)
            .with_base_image(self.base_image.clone())
            .with_prune_reuse(!self.reuse_cluster, self.reuse_cluster, self.reuse_cluster)
            .autorun(false)
            .load_existing_containers()
            .await
            .configure(components.clone())?
            .build()
            .await?;

        match self.wait_timeout {
            Some(timeout) => {
                components.start_wait(&composer, timeout.into()).await?;
            }
            None => {
                components.start(&composer).await?;
            }
        }

        if self.show_info {
            let lister = ListOptions {
                cluster_label: self.cluster_label.clone(),
                ..Default::default()
            };
            lister.list_simple().await?;
        }
        Ok(())
    }
}
impl StopOptions {
    async fn stop(&self, _action: &Action) -> Result<(), Error> {
        let composer = Builder::new()
            .name(&self.cluster_label.name())
            .label_prefix(&self.cluster_label.prefix())
            .with_prune(false)
            .with_clean(true)
            .build()
            .await?;
        let _ = composer.stop_network_containers().await;
        let _ = composer
            .remove_network_containers(&self.cluster_label.name())
            .await?;
        Ok(())
    }
}
impl ListOptions {
    fn list_docker(&self) -> Result<(), Error> {
        let label_filter = format!("label={}", self.cluster_label.filter());
        let mut args = vec!["ps", "-a", "--filter", &label_filter];
        if let Some(format) = &self.format {
            args.push("--format");
            args.push(format)
        }
        let status = std::process::Command::new("docker").args(args).status()?;
        build_error("docker", status.code())
    }
    /// Simple listing of all started components
    pub async fn list_simple(&self) -> Result<(), Error> {
        let cfg = Builder::new()
            .name(&self.cluster_label.name())
            .label_prefix(&self.cluster_label.prefix())
            .with_prune_reuse(false, false, false)
            .with_clean(false)
            .build()
            .await?;

        for component in cfg.list_cluster_containers().await? {
            let ip = match component.network_settings.clone() {
                None => None,
                Some(networks) => match networks.networks {
                    None => None,
                    Some(network) => match network.get(&self.cluster_label.name()) {
                        None => None,
                        Some(endpoint) => endpoint.ip_address.clone(),
                    },
                },
            };
            println!(
                "[{}] [{}] {}",
                component
                    .names
                    .unwrap_or_default()
                    .first()
                    .unwrap_or(&"?".to_string()),
                ip.unwrap_or_default(),
                option_str(component.command),
            );
        }
        Ok(())
    }
    async fn list(&self, _action: &Action) -> Result<(), Error> {
        match self.no_docker {
            true => self.list_simple().await,
            false => self.list_docker(),
        }
    }
}

fn option_str<F: ToString>(input: Option<F>) -> String {
    match input {
        Some(input) => input.to_string(),
        None => "?".into(),
    }
}

impl ClusterLabel {
    /// Get the network prefix
    pub fn prefix(&self) -> String {
        if self.prefix.is_empty() {
            TEST_LABEL_PREFIX.to_string()
        } else {
            self.prefix.clone()
        }
    }
    /// Get the network name
    pub fn name(&self) -> String {
        self.name.clone()
    }
    /// Get the network as the filter
    pub fn filter(&self) -> String {
        format!("{}.name={}", self.prefix(), self.name())
    }
}

impl FromStr for ClusterLabel {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split('.');
        if split.clone().count() == 2 {
            return Ok(ClusterLabel {
                prefix: split.next().unwrap().to_string(),
                name: split.next().unwrap().to_string(),
            });
        }
        Err("Should be in the format 'prefix.name'".to_string())
    }
}
impl std::fmt::Display for ClusterLabel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "'{}'.'{}'", self.prefix(), self.name())
    }
}
impl std::fmt::Debug for ClusterLabel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}