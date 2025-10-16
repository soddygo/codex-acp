use anyhow::Result;
use clap::Parser;
use codex_arg0::arg0_dispatch_or_else;
use codex_common::CliConfigOverrides;

/// Codex ACP - An ACP-compatible coding agent powered by Codex
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Override a configuration value that would otherwise be loaded from
    /// `~/.codex/config.toml`. Use a dotted path (`foo.bar.baz`) to override
    /// nested values. The `value` portion is parsed as JSON. If it fails to
    /// parse as JSON, the raw string is used as a literal.
    ///
    /// Examples:
    ///   - `-c model="o3"`
    ///   - `-c 'sandbox_permissions=["disk-full-read-access"]'`
    ///   - `-c shell_environment_policy.inherit=all`
    #[arg(
        short = 'c',
        long = "config",
        value_name = "key=value",
        action = clap::ArgAction::Append,
        global = true,
    )]
    raw_overrides: Vec<String>,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let cli_config_overrides = CliConfigOverrides {
        raw_overrides: args.raw_overrides,
    };

    arg0_dispatch_or_else(|codex_linux_sandbox_exe| async move {
        codex_acp::run_main(codex_linux_sandbox_exe, cli_config_overrides).await?;
        Ok(())
    })
}
