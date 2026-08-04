use anyhow::{bail, Context, Result};
use niri_ipc::{socket::Socket, Action, Request, Response, WindowLayout};
pub use niri_ipc::{Event, Output, Window, Workspace};
use niri_ipc::state::EventStreamState;

pub trait NiriClient {
    fn get_windows(&mut self) -> Result<Vec<Window>>;
    fn get_workspaces(&mut self) -> Result<Vec<Workspace>>;
    fn get_focused_output(&mut self) -> Result<Output>;
    fn send_action(&mut self, action: Action) -> Result<()>;
}

pub fn connect() -> Result<Socket> {
    Socket::connect().context("Failed to connect to Niri socket")
}

impl NiriClient for Socket {
    fn get_windows(&mut self) -> Result<Vec<Window>> {
        let reply = self.send(Request::Windows)?;
        match reply {
            Ok(response) => match response {
                Response::Windows(windows) => Ok(windows),
                other => bail!("Unexpected response: {other:?}"),
            },
            Err(msg) => bail!("Niri rejected Windows request: {msg}"),
        }
    }
    fn get_workspaces(&mut self) -> Result<Vec<Workspace>> {
        let reply = self.send(Request::Workspaces)?;
        match reply {
            Ok(response) => match response {
                Response::Workspaces(workspaces) => Ok(workspaces),
                other => bail!("Unexpected response: {other:?}"),
            },
            Err(msg) => bail!("Niri rejected Workspaces request: {msg}"),
        }
    }
    fn get_focused_output(&mut self) -> Result<Output> {
        let reply = self.send(Request::FocusedOutput)?;
        match reply {
            Ok(response) => match response {
                Response::FocusedOutput(output) => match output {
                    Some(o) => Ok(o),
                    None => bail!("No focused output"),
                },
                other => bail!("Niri rejected Output request: {other:?}"),
            },
            Err(msg) => bail!("Niri rejected FocusedOutput request: {msg}"),
        }
    }
    fn send_action(&mut self, action: Action) -> Result<()> {
        let reply = self.send(Request::Action(action))?;
        match reply {
            Ok(Response::Handled) => Ok(()),
            Ok(other) => bail!("Unexpected response: {other:?}"),
            Err(msg) => bail!("Niri rejected action: {msg}"),
        }
    }
}

pub fn count_tiling_windows_per_workspace(
    state: &EventStreamState,
) -> std::collections::HashMap<u64, usize> {
    let mut counts: std::collections::HashMap<u64, usize> = std::collections::HashMap::new();
    for window in state.windows.windows.values() {
        if !window.is_floating {
            if let Some(workspace_id) = window.workspace_id {
                *counts.entry(workspace_id).or_default() += 1;
            }
        }
    }
    counts
}

pub fn has_focused_workspace(state: &EventStreamState) -> bool {
    state
        .workspaces
        .workspaces
        .values()
        .any(|workspace| workspace.is_focused)
}

pub fn get_focused_workspace_id(state: &EventStreamState) -> Option<u64> {
    state
        .workspaces
        .workspaces
        .values()
        .find(|workspace| workspace.is_focused)
        .map(|workspace| workspace.id)
}

pub fn is_focused_workspace_on_output(
    state: &EventStreamState,
    output_name: &str,
) -> bool {
    state
        .workspaces
        .workspaces
        .values()
        .find(|workspace| workspace.is_focused)
        .map(|workspace| workspace.output.as_deref() == Some(output_name))
        .unwrap_or(false)
}

pub fn get_windows_on_workspace<'a>(
    state: &'a EventStreamState,
    workspace_id: u64,
) -> std::collections::HashMap<u64, &'a WindowLayout> {
    state
        .windows
        .windows
        .values()
        .filter(|window| {
            !window.is_floating && window.workspace_id == Some(workspace_id)
        })
        .map(|window| (window.id, &window.layout))
        .collect()
}
