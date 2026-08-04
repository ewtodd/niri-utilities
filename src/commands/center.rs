use crate::niri::{
    connect, get_focused_workspace_id, is_focused_workspace_on_output,
    get_windows_on_workspace, count_tiling_windows_per_workspace, NiriClient,
};
use anyhow::{Context, Result};
use niri_ipc::{Action, Request};
use niri_ipc::state::{EventStreamState, EventStreamStatePart};
use std::time::Instant;

fn format_layout_fingerprint(layouts: &std::collections::HashMap<u64, &niri_ipc::WindowLayout>) -> String {
    let mut parts: Vec<String> = Vec::new();
    for (id, layout) in layouts {
        if let Some((x, y)) = layout.tile_pos_in_workspace_view {
            let (w, h) = layout.tile_size;
            parts.push(format!("{id}:{x:.2},{y:.2}:{w:.2},{h:.2}"));
        }
    }
    parts.sort();
    parts.join(";")
}

pub fn centering_daemon(
    mut command_socket: niri_ipc::socket::Socket,
    output_filter: Option<&str>,
) -> Result<()> {
    let mut stream_socket = connect()?;
    let stream_reply = stream_socket.send(Request::EventStream)?;
    if !matches!(stream_reply, Ok(niri_ipc::Response::Handled)) {
        anyhow::bail!("Unexpected response to EventStream request: {stream_reply:?}");
    }
    let mut read_event = stream_socket.read_events();

    if let Some(name) = output_filter {
        eprintln!("Starting centering daemon (output: {name})...");
    } else {
        eprintln!("Starting centering daemon (all outputs)...");
    }

    let mut state = EventStreamState::default();
    let mut previous_counts =
        std::collections::HashMap::<u64, usize>::new();
    let mut last_center_time = Instant::now();

    loop {
        let event = read_event().context("Error reading event")?;

        let old_focused = get_focused_workspace_id(&state);
        let old_fingerprint = if let Some(workspace_id) = old_focused {
            let layouts = get_windows_on_workspace(&state, workspace_id);
            format_layout_fingerprint(&layouts)
        } else {
            String::new()
        };

        state.apply(event);

        let new_focused = get_focused_workspace_id(&state);
        let new_fingerprint = if let Some(workspace_id) = new_focused {
            let layouts = get_windows_on_workspace(&state, workspace_id);
            format_layout_fingerprint(&layouts)
        } else {
            String::new()
        };
        let new_counts = count_tiling_windows_per_workspace(&state);

        let focused_changed = old_focused != new_focused;
        let count_changed = previous_counts != new_counts;
        let layout_changed = old_fingerprint != new_fingerprint;

        if (count_changed || layout_changed || focused_changed)
            && new_focused.is_some()
            && output_filter.is_none_or(|name| {
                is_focused_workspace_on_output(&state, name)
            })
        {
            let now = Instant::now();
            if now.duration_since(last_center_time).as_millis() > 250 {
                eprintln!("Centering visible columns");
                let _ = command_socket.send_action(Action::CenterVisibleColumns {});
                last_center_time = now;
            }
        }

        previous_counts = new_counts;
    }

    Ok(())
}
