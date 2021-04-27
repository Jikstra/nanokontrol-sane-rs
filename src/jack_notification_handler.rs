
use crate::Logger;

pub struct Notifications {
    pub log: Logger
}

impl jack::NotificationHandler for Notifications {
    fn thread_init(&self, _: &jack::Client) {
        self.log.verbose("thread init".to_string());
    }

    fn shutdown(&mut self, status: jack::ClientStatus, reason: &str) {
        self.log.verbose(format!(
            "shutdown with status {:?} because \"{}\"",
            status, reason
        ));
    }

    fn freewheel(&mut self, _: &jack::Client, is_enabled: bool) {
        self.log.verbose(
            format!(
            "freewheel mode is {}",
            if is_enabled { "on" } else { "off" }
        ));
    }

    fn buffer_size(&mut self, _: &jack::Client, sz: jack::Frames) -> jack::Control {
        self.log.verbose(format!("buffer size changed to {}", sz));
        jack::Control::Continue
    }

    fn sample_rate(&mut self, _: &jack::Client, srate: jack::Frames) -> jack::Control {
        self.log.verbose(format!("sample rate changed to {}", srate));
        jack::Control::Continue
    }

    fn client_registration(&mut self, _: &jack::Client, name: &str, is_reg: bool) {
        self.log.verbose(format!(
            "{} client with name \"{}\"",
            if is_reg { "registered" } else { "unregistered" },
            name
        ));
    }

    fn port_registration(&mut self, _: &jack::Client, port_id: jack::PortId, is_reg: bool) {
        self.log.verbose(format!(
            "{} port with id {}",
            if is_reg { "registered" } else { "unregistered" },
            port_id
        ));
    }

    fn port_rename(
        &mut self,
        _: &jack::Client,
        port_id: jack::PortId,
        old_name: &str,
        new_name: &str,
    ) -> jack::Control {
        self.log.verbose(format!(
            "port with id {} renamed from {} to {}",
            port_id, old_name, new_name
        ));
        jack::Control::Continue
    }

    fn ports_connected(
        &mut self,
        _: &jack::Client,
        port_id_a: jack::PortId,
        port_id_b: jack::PortId,
        are_connected: bool,
    ) {
        self.log.verbose(format!(
            "ports with id {} and {} are {}",
            port_id_a,
            port_id_b,
            if are_connected {
                "connected"
            } else {
                "disconnected"
            }
        ));
    }

    fn graph_reorder(&mut self, _: &jack::Client) -> jack::Control {
        self.log.verbose(format!("graph reordered"));
        jack::Control::Continue
    }

    fn xrun(&mut self, _: &jack::Client) -> jack::Control {
        self.log.verbose(format!("xrun occurred"));
        jack::Control::Continue
    }

    fn latency(&mut self, _: &jack::Client, mode: jack::LatencyType) {
        self.log.verbose(format!(
            "{} latency has changed",
            match mode {
                jack::LatencyType::Capture => "capture",
                jack::LatencyType::Playback => "playback",
            }
        ));
    }
}