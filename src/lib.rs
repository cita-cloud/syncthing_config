// struct follow https://github.com/syncthing/syncthing/tree/main/proto/lib/config
// version: 1.9.0
// explanation: https://docs.syncthing.net/users/config.html

use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ObservedDevice {
    #[serde(rename="deviceID")]
    id: String,
    name: String,
    address: String,
}

// sub struct of OptionsConfiguration
#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Tuning {
    #[serde(rename="auto")]
    TUNING_AUTO,
    #[serde(rename="small")]
    TUNING_SMALL,
    #[serde(rename="large")]
    TUNING_LARGE,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct OptionsConfiguration {
    #[serde(rename="listenAddresses")]
    listen_addresses: Vec<String>,
    #[serde(rename="globalAnnounceServers")]
    global_discovery_servers: Vec<String>,
    #[serde(rename="globalAnnounceEnabled")]
    global_discovery_enabled: bool,
    #[serde(rename="localAnnounceEnabled")]
    local_discovery_enabled: bool,
    #[serde(rename="localAnnouncePort")]
    local_announce_port: i32,
    #[serde(rename="localAnnounceMCAddr")]
    local_announce_multicast_address: String,
    #[serde(rename="maxSendKbps")]
    max_send_kbps: i32,
    #[serde(rename="maxRecvKbps")]
    max_recv_kbps: i32,
    #[serde(rename="reconnectionIntervalS")]
    reconnection_interval_s: i32,
    #[serde(rename="relaysEnabled")]
    relays_enabled: bool,
    #[serde(rename="relayReconnectIntervalM")]
    relays_reconnect_interval_m: i32,
    #[serde(rename="startBrowser")]
    start_browser: bool,
    #[serde(rename="natEnabled")]
    nat_traversal_enabled: bool,
    #[serde(rename="natLeaseMinutes")]
    nat_traversal_lease_m: i32,
    #[serde(rename="natRenewalMinutes")]
    nat_traversal_renewal_m: i32,
    #[serde(rename="natTimeoutSeconds")]
    nat_traversal_timeout_s: i32,
    #[serde(rename="urAccepted")]
    usage_reporting_accepted: i32,
    #[serde(rename="urSeen")]
    usage_reporting_seen: i32,
    #[serde(rename="urUniqueId")]
    usage_reporting_unique_id: String,
    #[serde(rename="urURL")]
    usage_reporting_url: String,
    #[serde(rename="urPostInsecurely")]
    usage_reporting_post_insecurely: bool,
    #[serde(rename="urInitialDelayS")]
    usage_reporting_initial_delay_s: i32,
    #[serde(rename="restartOnWakeup")]
    restart_on_wakeup: bool,
    #[serde(rename="autoUpgradeIntervalH")]
    auto_upgrade_interval_h: i32,
    #[serde(rename="upgradeToPreReleases")]
    upgrade_to_pre_releases: bool,
    #[serde(rename="keepTemporariesH")]
    keep_temporaries_h: i32,
    #[serde(rename="cacheIgnoredFiles")]
    cache_ignored_files: bool,
    #[serde(rename="progressUpdateIntervalS")]
    progress_update_interval_s: i32,
    #[serde(rename="limitBandwidthInLan")]
    limit_bandwidth_in_lan: bool,
    #[serde(rename="minHomeDiskFree")]
    min_home_disk_free: Capacity,
    #[serde(rename="releasesURL")]
    releases_url: String,
    #[serde(rename="alwaysLocalNets")]
    always_local_nets: Vec<String>,
    #[serde(rename="overwriteRemoteDeviceNamesOnConnect")]
    overwrite_remote_device_names_on_connect: bool,
    #[serde(rename="tempIndexMinBlocks")]
    temp_index_min_blocks: i32,
    #[serde(rename="unackedNotificationIDs")]
    unacked_notification_ids: Vec<String>,
    #[serde(rename="trafficClass")]
    traffic_class: i32,
    #[serde(rename="defaultFolderPath")]
    default_folder_path: String,
    #[serde(rename="setLowPriority")]
    set_low_priority: bool,
    #[serde(rename="maxFolderConcurrency")]
    max_folder_concurrency: i32,
    #[serde(rename="crURL")]
    crash_reporting_url: String,
    #[serde(rename="crashReportingEnabled")]
    crash_reporting_enabled: bool,
    #[serde(rename="stunKeepaliveStartS")]
    stun_keepalive_start_s: i32,
    #[serde(rename="stunKeepaliveMinS")]
    stun_keepalive_min_s: i32,
    #[serde(rename="stunServers")]
    stun_servers: Vec<String>,
    #[serde(rename="databaseTuning")]
    database_tuning: Tuning,
    #[serde(rename="maxConcurrentIncomingRequestKiB")]
    max_concurrent_incoming_request_kib: i32,
}


// sub struct of LDAPConfiguration
#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum LDAPTransport {
    #[serde(rename="plain")]
    LDAP_TRANSPORT_PLAIN,
    #[serde(rename="tls")]
    LDAP_TRANSPORT_TLS,
    #[serde(rename="starttls")]
    LDAP_TRANSPORT_START_TLS,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LDAPConfiguration {
    address: String,
    #[serde(rename="bindDN")]
    bind_dn: String,
    transport: LDAPTransport,
    #[serde(rename="insecureSkipVerify")]
    insecure_skip_verify: bool,
    #[serde(rename="searchBaseDN")]
    search_base_dn: String,
    #[serde(rename="searchFilter")]
    search_filter: String,
}

// sub struct of GUIConfiguration
#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum AuthMode {
    #[serde(rename="static")]
    AUTH_MODE_STATIC,
    #[serde(rename="ldap")]
    AUTH_MODE_LDAP,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GUIConfiguration {
    enabled: bool,
    address: String,
    #[serde(rename="unixSocketPermissions")]
    unix_socket_permissions: String,
    user: String,
    password: String,
    #[serde(rename="authMode")]
    auth_mode: AuthMode,
    #[serde(rename="useTLS")]
    use_tls: bool,
    #[serde(rename="apiKey")]
    api_key: String,
    #[serde(rename="insecureAdminAccess")]
    insecure_admin_access: bool,
    theme: String,
    debugging: bool,
    #[serde(rename="insecureSkipHostcheck")]
    insecure_skip_host_check: bool,
    #[serde(rename="insecureAllowFrameLoading")]
    insecure_allow_frame_loading: bool,
}

// sub struct of DeviceConfiguration
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ObservedFolder {
    id: String,
    label: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Compression {
    #[serde(rename="metadata")]
    METADATA,
    #[serde(rename="never")]
    NEVER,
    #[serde(rename="always")]
    ALWAYS,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DeviceConfiguration {
    #[serde(rename="deviceID")]
    device_id: String,
    name: String,
    addresses: Vec<String>,
    compression: Compression,
    #[serde(rename="certName")]
    cert_name: String,
    introducer: String,
    #[serde(rename="skipIntroductionRemovals")]
    skip_introduction_removals: bool,
    #[serde(rename="introducedBy")]
    introduced_by: String,
    paused: bool,
    #[serde(rename="allowedNetworks")]
    allowed_networks: Vec<String>,
    #[serde(rename="autoAcceptFolders")]
    auto_accept_folders: bool,
    #[serde(rename="maxSendKbps")]
    max_send_kbps: i32,
    #[serde(rename="maxRecvKbps")]
    max_recv_kbps: i32,
    #[serde(rename="ignoredFolders")]
    ignored_folders: Vec<ObservedFolder>,
    #[serde(rename="pendingFolders")]
    pending_folders: Vec<ObservedFolder>,
    #[serde(rename="maxRequestKiB")]
    max_request_kib: i32,
}

// sub struct of FolderConfiguration
#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum CopyRangeMethod {
    #[serde(rename="standard")]
    COPY_RANGE_METHOD_STANDARD,
    #[serde(rename="ioctl")]
    COPY_RANGE_METHOD_IOCTL,
    #[serde(rename="copy_file_range")]
    COPY_RANGE_METHOD_COPY_FILE_RANGE,
    #[serde(rename="sendfile")]
    COPY_RANGE_METHOD_SEND_FILE,
    #[serde(rename="duplicate_extents")]
    COPY_RANGE_METHOD_DUPLICATE_EXTENTS,
    #[serde(rename="all")]
    COPY_RANGE_METHOD_ALL_WITH_FALLBACK,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Clone, Debug)]
enum BlockPullOrder {
    #[serde(rename="standard")]
    BLOCK_PULL_ORDER_STANDARD,
    #[serde(rename="random")]
    BLOCK_PULL_ORDER_RANDOM,
    #[serde(rename="inOrder")]
    BLOCK_PULL_ORDER_IN_ORDER,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum PullOrder {
    #[serde(rename="random")]
    PULL_ORDER_RANDOM,
    #[serde(rename="alphabetic")]
    PULL_ORDER_ALPHABETIC,
    #[serde(rename="smallestFirst")]
    PULL_ORDER_SMALLEST_FIRST,
    #[serde(rename="largestFirst")]
    PULL_ORDER_LARGEST_FIRST,
    #[serde(rename="oldestFirst")]
    PULL_ORDER_OLDEST_FIRST,
    #[serde(rename="newestFirst")]
    PULL_ORDER_NEWEST_FIRST,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct VersioningConfiguration {
    #[serde(rename="type")]
    version_type: String,
    #[serde(rename="params")]
    parameters: HashMap<String, String>,
    #[serde(rename="cleanupIntervalS")]
    cleanup_interval_s: i32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Capacity {
    value: f64,
    unit: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FolderDeviceConfiguration {
    #[serde(rename="deviceID")]
    device_id: String,
    #[serde(rename="introducedBy")]
    introduced_by: String,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum FolderType {
    #[serde(rename="sendreceive")]
    FOLDER_TYPE_SEND_RECEIVE,
    #[serde(rename="sendonly")]
    FOLDER_TYPE_SEND_ONLY,
    #[serde(rename="receiveonly")]
    FOLDER_TYPE_RECEIVE_ONLY,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum FilesystemType {
    #[serde(rename="basic")]
    FILESYSTEM_TYPE_BASIC,
    #[serde(rename="fake")]
    FILESYSTEM_TYPE_FAKE,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FolderConfiguration {
    id: String,
    label: String,
    #[serde(rename="filesystemType")]
    filesystem_type: FilesystemType,
    path: String,
    #[serde(rename="type")]
    folder_type: FolderType,
    devices: Vec<FolderDeviceConfiguration>,
    #[serde(rename="rescanIntervalS")]
    rescan_interval_s: i32,
    #[serde(rename="fsWatcherEnabled")]
    fs_watcher_enabled: bool,
    #[serde(rename="fsWatcherDelayS")]
    fs_watcher_delay_s: i32,
    #[serde(rename="ignorePerms")]
    ignore_perms: bool,
    #[serde(rename="autoNormalize")]
    auto_normalize: bool,
    #[serde(rename="minDiskFree")]
    min_disk_free: Capacity,
    versioning: VersioningConfiguration,
    copiers: i32,
    #[serde(rename="pullerMaxPendingKiB")]
    puller_max_pending_kib: i32,
    hashers: i32,
    order: PullOrder,
    #[serde(rename="ignoreDelete")]
    ignore_delete: bool,
    #[serde(rename="scanProgressIntervalS")]
    scan_progress_interval_s: i32,
    #[serde(rename="pullerPauseS")]
    puller_pause_s: i32,
    #[serde(rename="maxConflicts")]
    max_conflicts: i32,
    #[serde(rename="disableSparseFiles")]
    disable_sparse_files: bool,
    #[serde(rename="disableTempIndexes")]
    disable_temp_indexes: bool,
    paused: bool,
    #[serde(rename="weakHashThresholdPct")]
    weak_hash_threshold_pct: i32,
    #[serde(rename="markerName")]
    marker_name: String,
    #[serde(rename="copyOwnershipFromParent")]
    copy_ownership_from_parent: bool,
    #[serde(rename="modTimeWindowS")]
    mod_time_window_s: i32,
    #[serde(rename="maxConcurrentWrites")]
    max_concurrent_writes: i32,
    #[serde(rename="disableFsync")]
    disable_fsync: bool,
    #[serde(rename="blockPullOrder")]
    block_pull_order: BlockPullOrder,
    #[serde(rename="copyRangeMethod")]
    copy_range_method: CopyRangeMethod,
    #[serde(rename="caseSensitiveFS")]
    case_sensitive_fs: bool,
    #[serde(rename="junctionsAsDirs")]
    follow_junctions: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Configuration {
    version: i32,
    folders: Vec<FolderConfiguration>,
    devices: Vec<DeviceConfiguration>,
    gui: GUIConfiguration,
    ldap: LDAPConfiguration,
    options: OptionsConfiguration,
    #[serde(rename="remoteIgnoredDevices")]
    ignored_devices: Vec<ObservedDevice>,
    #[serde(rename="pendingDevices")]
    pending_devices: Vec<ObservedDevice>,
}

impl Configuration {
    pub fn load(config_str: &str) -> Self {
        serde_json::from_str::<Configuration>(config_str).expect("Error while parsing config")
    }
}

#[cfg(test)]
mod tests {
    use super::Configuration;

    #[test]
    fn basic_test() {
        let config_str = include_str!("../test_data/config.json");
        let _config = Configuration::load(config_str);
    }
}
