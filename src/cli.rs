use std::path::PathBuf;

use clap::{Parser, Subcommand};

/// rrun: an OCI container runtime written in Rust.
#[derive(Debug, Parser)]
#[command(name = "rrun", version, about)]
pub struct Cli {
    /// Root directory for storage of container state.
    #[arg(long, global = true, default_value = "/run/rrun")]
    pub root: PathBuf,

    /// Write log messages to this file instead of stderr.
    #[arg(long, global = true)]
    pub log: Option<PathBuf>,

    /// Log format (text or json).
    #[arg(long = "log-format", global = true, default_value = "text")]
    pub log_format: String,

    /// Enable debug output.
    #[arg(long, global = true)]
    pub debug: bool,

    /// Use systemd for cgroup management.
    #[arg(long = "systemd-cgroup", global = true)]
    pub systemd_cgroup: bool,

    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Create a container.
    Create {
        /// Path to the OCI bundle.
        #[arg(short, long, default_value = ".")]
        bundle: PathBuf,
        /// Path to an AF_UNIX socket that receives the pty master fd.
        #[arg(long = "console-socket")]
        console_socket: Option<PathBuf>,
        /// File to write the container process pid to.
        #[arg(long = "pid-file")]
        pid_file: Option<PathBuf>,
        container_id: String,
    },
    /// Start a previously created container.
    Start { container_id: String },
    /// Create and start a container.
    Run {
        #[arg(short, long, default_value = ".")]
        bundle: PathBuf,
        #[arg(long = "console-socket")]
        console_socket: Option<PathBuf>,
        #[arg(long = "pid-file")]
        pid_file: Option<PathBuf>,
        /// Detach from the container process.
        #[arg(short, long)]
        detach: bool,
        container_id: String,
    },
    /// Output the state of a container.
    State { container_id: String },
    /// Send a signal to the container init process.
    Kill {
        container_id: String,
        #[arg(default_value = "SIGTERM")]
        signal: String,
        /// Send the signal to all processes in the container.
        #[arg(short, long)]
        all: bool,
    },
    /// Remove a container.
    Delete {
        container_id: String,
        /// Forcibly delete a running container.
        #[arg(short, long)]
        force: bool,
    },
    /// List known containers.
    List {
        /// Only display container IDs.
        #[arg(short, long)]
        quiet: bool,
    },
    /// Generate a default config.json.
    Spec {
        /// Generate a config for a rootless container.
        #[arg(long)]
        rootless: bool,
    },
}
