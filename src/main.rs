//! Takes 2 audio inputs and outputs them to 2 audio outputs.
//! All JACK notifications are also printed out.
use std::sync::mpsc::channel;
use crate::jack_notification_handler::Notifications;
use crate::log::{Logger};
use jack;
use crate::jack_midi::MidiCopy;
use std::sync::mpsc::{sync_channel, Receiver};
use ctrlc;

mod jack_midi;
mod log;
mod jack_notification_handler;


fn main() {
    // Create client
    let log = Logger::new(None);
    
    let (client, _status) =
        jack::Client::new("midimonitor", jack::ClientOptions::NO_START_SERVER).unwrap();

    
    let in_midi = client
        .register_port("midi_in", jack::MidiIn)
        .unwrap();
    
    let mut out_midi = client
        .register_port("midi_out", jack::MidiOut)
        .unwrap();


    let (sender, receiver) = sync_channel(64);

    let process_callback = move |_: &jack::Client, ps: &jack::ProcessScope| -> jack::Control {
        let midi_iter = in_midi.iter(ps);
        let mut out_midi_writer = out_midi.writer(ps);

        for midi_message in midi_iter {
            let _ = out_midi_writer.write(&midi_message);
            
            let midi_copy: MidiCopy = midi_message.into();
            let _ = sender.try_send(midi_copy);
        }
        
        jack::Control::Continue
    };
    
    let process = jack::ClosureProcessHandler::new(process_callback);

    let jack_log = log.sub("jack".to_string()); 

    start_midi_monitor_thread(log, receiver);


    // Activate the client, which starts the processing.
    let active_client = Box::new(client.activate_async(Notifications { log: jack_log }, process).unwrap());

    wait_for_ctrlc();

    active_client.deactivate().unwrap();
    
}

pub fn wait_for_ctrlc() {
    let (interrupt_sender, interrupt_receiver) = channel();
    
    ctrlc::set_handler(move || {
        interrupt_sender.send(()).expect("Could not send signal on channel.")
    }).expect("Error setting Ctrl-C handler");
    
    interrupt_receiver.recv().expect("Could not receive from channel.");
}

pub fn start_midi_monitor_thread(log: Logger, receiver: Receiver<jack_midi::MidiCopy> ) {
    let midi_log = log.sub("midi".to_string());
    std::thread::spawn(move || {
        while let Ok(m) = receiver.recv() {
            midi_log.info(format!("{:#04X?} {:#04X?} {:#04X?}", m.data[0], m.data[1], m.data[2]));
        }
    });
}