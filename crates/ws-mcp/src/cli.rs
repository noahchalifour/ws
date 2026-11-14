use std::net::{IpAddr, Ipv4Addr};

use clap::{Args, Parser, Subcommand, ValueEnum};

use ws_policy::TicketId;

use crate::read::Read;
use crate::trade::Trade;

#[derive(Debug, Parser)]
#[command(name = "ws-mcp", version, about = "Unofficial Wealthsimple MCP server and CLI")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Command>,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Log in, pick the mode and the accounts the agent may trade in, then show the status.
    Setup,
    /// Show the login, mode, accounts, kill switch, and today's limits.
    Status,
    /// Show every setting, or change one, like `ws-mcp config market_hours_only false`.
    Config {
        #[arg(requires = "value")]
        key: Option<String>,
        value: Option<String>,
    },
    /// Log in through a Chrome window and keep the session in the system keychain.
    Login,
    /// Remove the stored session.
    Logout,
    /// Remove the session, the audit log, tickets, budgets, and the paper ledger. Keeps your config.
    Clear,
    /// Run the MCP server, on stdio by default or over streamable HTTP with `--transport http`.
    Serve(ServeArgs),
    /// Open a dashboard in your browser.
    Dashboard,
    /// Approve a ticket that needs it, after showing what it would place. Asks you to confirm.
    Approve { ticket_id: TicketId },
    /// Engage the kill switch: no order can be previewed, placed, or cancelled until `resume`.
    Kill,
    /// Lift the kill switch. Asks you to confirm.
    Resume,
    #[command(flatten)]
    Read(Read),
    #[command(flatten)]
    Trade(Trade),
}

impl Default for Command {
    fn default() -> Self {
        Self::Serve(ServeArgs::default())
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, ValueEnum)]
pub enum TransportKind {
    #[default]
    Stdio,
    Http,
}

/// HTTP callers are not authenticated: serve it only on a private network.
#[derive(Debug, Clone, Args)]
pub struct ServeArgs {
    #[arg(long, value_enum, default_value_t, env = "WS_TRANSPORT")]
    pub transport: TransportKind,
    /// Address to listen on with `--transport http`.
    #[arg(long, default_value_t = IpAddr::V4(Ipv4Addr::LOCALHOST), env = "WS_HOST")]
    pub host: IpAddr,
    #[arg(long, default_value_t = 3000, env = "WS_PORT")]
    pub port: u16,
    /// URL path of the MCP endpoint.
    #[arg(long, default_value = "/mcp", env = "WS_PATH")]
    pub path: String,
    /// `Host` header to answer, repeatable or comma-separated. Others get 403, against DNS rebinding.
    #[arg(long, value_delimiter = ',', default_values_t = ["localhost".to_owned(), "127.0.0.1".to_owned(), "::1".to_owned()], env = "WS_ALLOWED_HOSTS")]
    pub allowed_host: Vec<String>,
}

impl Default for ServeArgs {
    fn default() -> Self {
        Self {
            transport: TransportKind::Stdio,
            host: IpAddr::V4(Ipv4Addr::LOCALHOST),
            port: 3000,
            path: "/mcp".to_owned(),
            allowed_host: vec!["localhost".to_owned(), "127.0.0.1".to_owned(), "::1".to_owned()],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(args: &[&str]) -> Option<Command> {
        Cli::try_parse_from(std::iter::once("ws-mcp").chain(args.iter().copied())).ok()?.command
    }

    #[test]
    fn read_commands_take_the_same_fields_as_the_tools() {
        assert!(matches!(
            parse(&["activities", "tfsa-1", "--limit", "5"]),
            Some(Command::Read(Read::Activities(a))) if a.account == "tfsa-1" && a.limit == Some(5)
        ));
        assert!(matches!(parse(&["pending-orders", "tfsa-1"]), Some(Command::Read(Read::PendingOrders(_)))));
        assert!(matches!(parse(&["search", "XEQT"]), Some(Command::Read(Read::Search(s))) if s.query == "XEQT"));
    }

    #[test]
    fn takes_names_but_rejects_malformed_ticket_ids() {
        assert!(matches!(parse(&["quote", "XEQT"]), Some(Command::Read(Read::Quote(q))) if q.security == "XEQT"));
        assert!(parse(&["approve", "../ticket-1"]).is_none());
    }

    #[test]
    fn order_commands_take_the_same_fields_as_the_tools() {
        assert!(matches!(
            parse(&["preview-order", "tfsa", "XEQT", "buy", "--shares", "1", "--limit", "0.20"]),
            Some(Command::Trade(Trade::Preview(p))) if p.shares == Some(rust_decimal::dec!(1)) && !p.good_till_cancelled
        ));
        assert!(matches!(parse(&["place-order", "ticket-1"]), Some(Command::Trade(Trade::Place(_)))));
        assert!(matches!(parse(&["approve", "ticket-1"]), Some(Command::Approve { .. })));
    }

    #[test]
    fn serves_stdio_by_default_and_http_on_request() {
        assert!(matches!(parse(&["serve"]), Some(Command::Serve(t)) if t.transport == TransportKind::Stdio));
        assert!(matches!(
            parse(&["serve", "--transport=http", "--host=0.0.0.0", "--port=8080", "--allowed-host=a.svc,b.svc"]),
            Some(Command::Serve(t)) if t.transport == TransportKind::Http
                && t.host == IpAddr::V4(Ipv4Addr::UNSPECIFIED)
                && t.port == 8080
                && t.path == "/mcp"
                && t.allowed_host == ["a.svc", "b.svc"]
        ));
    }
}
