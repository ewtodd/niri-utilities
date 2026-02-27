use crate::niri::NiriClient;
use anyhow::Result;
use niri_ipc::socket::Socket;

pub fn centering_daemon(mut socket: Socket) -> Result<()> {
    let windows = socket.get_windows()?;
    let workspaces = socket.get_workspaces()?;
    let output = socket.get_focused_output()?;
    println!("Found windows: {windows:?}");
    println!("Found workspaces: {workspaces:?}");
    println!("Found output: {output:?}");
    Ok(())
}
