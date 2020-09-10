use std::env;
use syncthing_config::*;

#[derive(Debug, Clone)]
struct Peer {
    ip: String,
    port: u16,
}

fn insert_folder_device(config: &mut Configuration, ids: &Vec<String>) {
    let mut devices: Vec<FolderDeviceConfiguration> = Vec::new();
    for id in ids {
        let device = FolderDeviceConfiguration {
            device_id: id.to_string(),
            introduced_by: String::new(),
        };
        devices.push(device);
    }
    for folder in config.folders.iter_mut() {
        folder.devices = devices.clone();
    }
}

fn insert_devices(
    config: &mut Configuration,
    chain_name: &str,
    ids: &Vec<String>,
    peers: &Vec<Peer>,
) {
    let mut devices: Vec<DeviceConfiguration> = Vec::new();
    for (i, peer) in peers.iter().enumerate() {
        let device = DeviceConfiguration {
            device_id: ids[i].to_string(),
            name: format!("{}-{}", chain_name, i),
            addresses: vec![format!("quic://{}:{}", peer.ip, peer.port)],
            compression: Compression::ALWAYS,
            cert_name: String::new(),
            introducer: false,
            skip_introduction_removals: false,
            introduced_by: String::new(),
            paused: false,
            allowed_networks: Vec::new(),
            auto_accept_folders: false,
            max_send_kbps: 0,
            max_recv_kbps: 0,
            ignored_folders: Vec::new(),
            pending_folders: Vec::new(),
            max_request_kib: 0,
        };
        devices.push(device);
    }
    config.devices = devices;
}

fn insert_gui(config: &mut Configuration, chain_name: &str) {
    config.gui.user = chain_name.to_string();
    config.gui.password = chain_name.to_string();
    config.gui.api_key = chain_name.to_string();
}

fn insert_options(config: &mut Configuration, peer: &Peer) {
    config.options.listen_addresses = vec![format!("quic://0.0.0.0:{}", peer.port)];
}

use std::fs::File;
use std::io::Write;
use std::path::Path;

fn main() {
    if env::args().len() != 4 {
        println!("Usage: need three argument!");
        println!("First argument: chain name");
        println!(
            "Second argument: syncthing node list, looks like: \"localhost:22000;localhost:22001\""
        );
        println!("Third argument: syncthing node ID list, looks like: \"DS5DKOE-AB4NXBU-A7G4KY5-A6ARRTF-SRQPJHX-TNETIII-W6VTYYJ-SHEZUQZ;I557BDW-FEL36FD-RLZG2XZ-64ZFR4C-CRE3HBN-WWC6S4E-GBEB5HT-KZ7SRAT\"");
        return;
    }

    // proc args
    let chain_name = env::args().nth(1).unwrap();
    println!("chain_name: {}", chain_name);

    let node_list = env::args().nth(2).unwrap();
    let peers: Vec<Peer> = node_list
        .split_terminator(';')
        .map(|node_str| {
            let ip_and_port: Vec<&str> = node_str.split_terminator(':').collect();
            let ip = ip_and_port[0].to_owned();
            let port = ip_and_port[1].parse::<u16>().unwrap();
            Peer { ip, port }
        })
        .collect();
    println!("peers: {:?}", peers);

    let id_list = env::args().nth(3).unwrap();
    let ids: Vec<String> = id_list
        .split_terminator(';')
        .map(|id| id.to_string())
        .collect();
    println!("ids: {:?}", ids);

    if peers.len() != ids.len() {
        println!("node list must be the same length with ids!");
        return;
    }

    // parse template config.json
    let config_str = include_str!("../../test_data/config.json");
    let config = Configuration::load(config_str);

    println!("version: {}", config.version);

    for (index, peer) in peers.iter().enumerate() {
        let mut node_config = config.clone();
        insert_folder_device(&mut node_config, &ids);
        insert_devices(&mut node_config, &chain_name, &ids, &peers);
        insert_gui(&mut node_config, &chain_name);
        insert_options(&mut node_config, peer);

        let path = format!("node-{}-config.json", index);
        let node_config_path = Path::new(&path);
        let ret = File::create(node_config_path);
        if let Err(e) = ret {
            println!("create node{} net config file failed: {:?}", index, e);
            return;
        }
        let mut f = ret.unwrap();
        f.write_all(serde_json::to_string_pretty(&node_config).unwrap().as_bytes())
            .unwrap();
    }
}
