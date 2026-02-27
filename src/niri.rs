use anyhow::{bail, Context, Result};
use niri_ipc::{socket::Socket, Request, Response};
pub use niri_ipc::{Output, Window, Workspace};

pub trait NiriClient {
    fn get_windows(&mut self) -> Result<Vec<Window>>;
    fn get_workspaces(&mut self) -> Result<Vec<Workspace>>;
    fn get_focused_output(&mut self) -> Result<Output>;
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
}
