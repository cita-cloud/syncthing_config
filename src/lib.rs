// struct follow https://github.com/syncthing/syncthing/tree/main/proto/lib/config
// version: 1.9.0
// explanation: https://docs.syncthing.net/users/config.html

use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ObservedDevice {
    #[serde(rename = "deviceID")]
    pub id: String,
    pub name: String,
    pub address: String,
}

// sub struct of OptionsConfiguration
#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Tuning {
    #[serde(rename = "auto")]
    TUNING_AUTO,
    #[serde(rename = "small")]
    TUNING_SMALL,
    #[serde(rename = "large")]
    TUNING_LARGE,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct OptionsConfiguration {
    #[serde(rename = "listenAddresses")]
    pub listen_addresses: Vec<String>,
    #[serde(rename = "globalAnnounceServers")]
    pub global_discovery_servers: Vec<String>,
    #[serde(rename = "globalAnnounceEnabled")]
    pub global_discovery_enabled: bool,
    #[serde(rename = "localAnnounceEnabled")]
    pub local_discovery_enabled: bool,
    #[serde(rename = "localAnnouncePort")]
    pub local_announce_port: i32,
    #[serde(rename = "localAnnounceMCAddr")]
    pub local_announce_multicast_address: String,
    #[serde(rename = "maxSendKbps")]
    pub max_send_kbps: i32,
    #[serde(rename = "maxRecvKbps")]
    pub max_recv_kbps: i32,
    #[serde(rename = "reconnectionIntervalS")]
    pub reconnection_interval_s: i32,
    #[serde(rename = "relaysEnabled")]
    pub relays_enabled: bool,
    #[serde(rename = "relayReconnectIntervalM")]
    pub relays_reconnect_interval_m: i32,
    #[serde(rename = "startBrowser")]
    pub start_browser: bool,
    #[serde(rename = "natEnabled")]
    pub nat_traversal_enabled: bool,
    #[serde(rename = "natLeaseMinutes")]
    pub nat_traversal_lease_m: i32,
    #[serde(rename = "natRenewalMinutes")]
    pub nat_traversal_renewal_m: i32,
    #[serde(rename = "natTimeoutSeconds")]
    pub nat_traversal_timeout_s: i32,
    #[serde(rename = "urAccepted")]
    pub usage_reporting_accepted: i32,
    #[serde(rename = "urSeen")]
    pub usage_reporting_seen: i32,
    #[serde(rename = "urUniqueId")]
    pub usage_reporting_unique_id: String,
    #[serde(rename = "urURL")]
    pub usage_reporting_url: String,
    #[serde(rename = "urPostInsecurely")]
    pub usage_reporting_post_insecurely: bool,
    #[serde(rename = "urInitialDelayS")]
    pub usage_reporting_initial_delay_s: i32,
    #[serde(rename = "restartOnWakeup")]
    pub restart_on_wakeup: bool,
    #[serde(rename = "autoUpgradeIntervalH")]
    pub auto_upgrade_interval_h: i32,
    #[serde(rename = "upgradeToPreReleases")]
    pub upgrade_to_pre_releases: bool,
    #[serde(rename = "keepTemporariesH")]
    pub keep_temporaries_h: i32,
    #[serde(rename = "cacheIgnoredFiles")]
    pub cache_ignored_files: bool,
    #[serde(rename = "progressUpdateIntervalS")]
    pub progress_update_interval_s: i32,
    #[serde(rename = "limitBandwidthInLan")]
    pub limit_bandwidth_in_lan: bool,
    #[serde(rename = "minHomeDiskFree")]
    pub min_home_disk_free: Capacity,
    #[serde(rename = "releasesURL")]
    pub releases_url: String,
    #[serde(rename = "alwaysLocalNets")]
    pub always_local_nets: Vec<String>,
    #[serde(rename = "overwriteRemoteDeviceNamesOnConnect")]
    pub overwrite_remote_device_names_on_connect: bool,
    #[serde(rename = "tempIndexMinBlocks")]
    pub temp_index_min_blocks: i32,
    #[serde(rename = "unackedNotificationIDs")]
    pub unacked_notification_ids: Vec<String>,
    #[serde(rename = "trafficClass")]
    pub traffic_class: i32,
    #[serde(rename = "defaultFolderPath")]
    pub default_folder_path: String,
    #[serde(rename = "setLowPriority")]
    pub set_low_priority: bool,
    #[serde(rename = "maxFolderConcurrency")]
    pub max_folder_concurrency: i32,
    #[serde(rename = "crURL")]
    pub crash_reporting_url: String,
    #[serde(rename = "crashReportingEnabled")]
    pub crash_reporting_enabled: bool,
    #[serde(rename = "stunKeepaliveStartS")]
    pub stun_keepalive_start_s: i32,
    #[serde(rename = "stunKeepaliveMinS")]
    pub stun_keepalive_min_s: i32,
    #[serde(rename = "stunServers")]
    pub stun_servers: Vec<String>,
    #[serde(rename = "databaseTuning")]
    pub database_tuning: Tuning,
    #[serde(rename = "maxConcurrentIncomingRequestKiB")]
    pub max_concurrent_incoming_request_kib: i32,
}

// sub struct of LDAPConfiguration
#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum LDAPTransport {
    #[serde(rename = "plain")]
    LDAP_TRANSPORT_PLAIN,
    #[serde(rename = "tls")]
    LDAP_TRANSPORT_TLS,
    #[serde(rename = "starttls")]
    LDAP_TRANSPORT_START_TLS,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LDAPConfiguration {
    pub address: String,
    #[serde(rename = "bindDN")]
    pub bind_dn: String,
    pub transport: LDAPTransport,
    #[serde(rename = "insecureSkipVerify")]
    pub insecure_skip_verify: bool,
    #[serde(rename = "searchBaseDN")]
    pub search_base_dn: String,
    #[serde(rename = "searchFilter")]
    pub search_filter: String,
}

// sub struct of GUIConfiguration
#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum AuthMode {
    #[serde(rename = "static")]
    AUTH_MODE_STATIC,
    #[serde(rename = "ldap")]
    AUTH_MODE_LDAP,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GUIConfiguration {
    pub enabled: bool,
    pub address: String,
    #[serde(rename = "unixSocketPermissions")]
    pub unix_socket_permissions: String,
    pub user: String,
    pub password: String,
    #[serde(rename = "authMode")]
    pub auth_mode: AuthMode,
    #[serde(rename = "useTLS")]
    pub use_tls: bool,
    #[serde(rename = "apiKey")]
    pub api_key: String,
    #[serde(rename = "insecureAdminAccess")]
    pub insecure_admin_access: bool,
    pub theme: String,
    pub debugging: bool,
    #[serde(rename = "insecureSkipHostcheck")]
    pub insecure_skip_host_check: bool,
    #[serde(rename = "insecureAllowFrameLoading")]
    pub insecure_allow_frame_loading: bool,
}

// sub struct of DeviceConfiguration
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ObservedFolder {
    pub id: String,
    pub label: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum Compression {
    #[serde(rename = "metadata")]
    METADATA,
    #[serde(rename = "never")]
    NEVER,
    #[serde(rename = "always")]
    ALWAYS,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DeviceConfiguration {
    #[serde(rename = "deviceID")]
    pub device_id: String,
    pub name: String,
    pub addresses: Vec<String>,
    pub compression: Compression,
    #[serde(rename = "certName")]
    pub cert_name: String,
    pub introducer: bool,
    #[serde(rename = "skipIntroductionRemovals")]
    pub skip_introduction_removals: bool,
    #[serde(rename = "introducedBy")]
    pub introduced_by: String,
    pub paused: bool,
    #[serde(rename = "allowedNetworks")]
    pub allowed_networks: Vec<String>,
    #[serde(rename = "autoAcceptFolders")]
    pub auto_accept_folders: bool,
    #[serde(rename = "maxSendKbps")]
    pub max_send_kbps: i32,
    #[serde(rename = "maxRecvKbps")]
    pub max_recv_kbps: i32,
    #[serde(rename = "ignoredFolders")]
    pub ignored_folders: Vec<ObservedFolder>,
    #[serde(rename = "pendingFolders")]
    pub pending_folders: Vec<ObservedFolder>,
    #[serde(rename = "maxRequestKiB")]
    pub max_request_kib: i32,
}

// sub struct of FolderConfiguration
#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum CopyRangeMethod {
    #[serde(rename = "standard")]
    COPY_RANGE_METHOD_STANDARD,
    #[serde(rename = "ioctl")]
    COPY_RANGE_METHOD_IOCTL,
    #[serde(rename = "copy_file_range")]
    COPY_RANGE_METHOD_COPY_FILE_RANGE,
    #[serde(rename = "sendfile")]
    COPY_RANGE_METHOD_SEND_FILE,
    #[serde(rename = "duplicate_extents")]
    COPY_RANGE_METHOD_DUPLICATE_EXTENTS,
    #[serde(rename = "all")]
    COPY_RANGE_METHOD_ALL_WITH_FALLBACK,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum BlockPullOrder {
    #[serde(rename = "standard")]
    BLOCK_PULL_ORDER_STANDARD,
    #[serde(rename = "random")]
    BLOCK_PULL_ORDER_RANDOM,
    #[serde(rename = "inOrder")]
    BLOCK_PULL_ORDER_IN_ORDER,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum PullOrder {
    #[serde(rename = "random")]
    PULL_ORDER_RANDOM,
    #[serde(rename = "alphabetic")]
    PULL_ORDER_ALPHABETIC,
    #[serde(rename = "smallestFirst")]
    PULL_ORDER_SMALLEST_FIRST,
    #[serde(rename = "largestFirst")]
    PULL_ORDER_LARGEST_FIRST,
    #[serde(rename = "oldestFirst")]
    PULL_ORDER_OLDEST_FIRST,
    #[serde(rename = "newestFirst")]
    PULL_ORDER_NEWEST_FIRST,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct VersioningConfiguration {
    #[serde(rename = "type")]
    pub version_type: String,
    #[serde(rename = "params")]
    pub parameters: HashMap<String, String>,
    #[serde(rename = "cleanupIntervalS")]
    pub cleanup_interval_s: i32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Capacity {
    pub value: f64,
    pub unit: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FolderDeviceConfiguration {
    #[serde(rename = "deviceID")]
    pub device_id: String,
    #[serde(rename = "introducedBy")]
    pub introduced_by: String,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum FolderType {
    #[serde(rename = "sendreceive")]
    FOLDER_TYPE_SEND_RECEIVE,
    #[serde(rename = "sendonly")]
    FOLDER_TYPE_SEND_ONLY,
    #[serde(rename = "receiveonly")]
    FOLDER_TYPE_RECEIVE_ONLY,
}

#[allow(non_camel_case_types)]
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum FilesystemType {
    #[serde(rename = "basic")]
    FILESYSTEM_TYPE_BASIC,
    #[serde(rename = "fake")]
    FILESYSTEM_TYPE_FAKE,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FolderConfiguration {
    pub id: String,
    pub label: String,
    #[serde(rename = "filesystemType")]
    pub filesystem_type: FilesystemType,
    pub path: String,
    #[serde(rename = "type")]
    pub folder_type: FolderType,
    pub devices: Vec<FolderDeviceConfiguration>,
    #[serde(rename = "rescanIntervalS")]
    pub rescan_interval_s: i32,
    #[serde(rename = "fsWatcherEnabled")]
    pub fs_watcher_enabled: bool,
    #[serde(rename = "fsWatcherDelayS")]
    pub fs_watcher_delay_s: i32,
    #[serde(rename = "ignorePerms")]
    pub ignore_perms: bool,
    #[serde(rename = "autoNormalize")]
    pub auto_normalize: bool,
    #[serde(rename = "minDiskFree")]
    pub min_disk_free: Capacity,
    pub versioning: VersioningConfiguration,
    pub copiers: i32,
    #[serde(rename = "pullerMaxPendingKiB")]
    pub puller_max_pending_kib: i32,
    pub hashers: i32,
    pub order: PullOrder,
    #[serde(rename = "ignoreDelete")]
    pub ignore_delete: bool,
    #[serde(rename = "scanProgressIntervalS")]
    pub scan_progress_interval_s: i32,
    #[serde(rename = "pullerPauseS")]
    pub puller_pause_s: i32,
    #[serde(rename = "maxConflicts")]
    pub max_conflicts: i32,
    #[serde(rename = "disableSparseFiles")]
    pub disable_sparse_files: bool,
    #[serde(rename = "disableTempIndexes")]
    pub disable_temp_indexes: bool,
    pub paused: bool,
    #[serde(rename = "weakHashThresholdPct")]
    pub weak_hash_threshold_pct: i32,
    #[serde(rename = "markerName")]
    pub marker_name: String,
    #[serde(rename = "copyOwnershipFromParent")]
    pub copy_ownership_from_parent: bool,
    #[serde(rename = "modTimeWindowS")]
    pub mod_time_window_s: i32,
    #[serde(rename = "maxConcurrentWrites")]
    pub max_concurrent_writes: i32,
    #[serde(rename = "disableFsync")]
    pub disable_fsync: bool,
    #[serde(rename = "blockPullOrder")]
    pub block_pull_order: BlockPullOrder,
    #[serde(rename = "copyRangeMethod")]
    pub copy_range_method: CopyRangeMethod,
    #[serde(rename = "caseSensitiveFS")]
    pub case_sensitive_fs: bool,
    #[serde(rename = "junctionsAsDirs")]
    pub follow_junctions: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Configuration {
    pub version: i32,
    pub folders: Vec<FolderConfiguration>,
    pub devices: Vec<DeviceConfiguration>,
    pub gui: GUIConfiguration,
    pub ldap: LDAPConfiguration,
    pub options: OptionsConfiguration,
    #[serde(rename = "remoteIgnoredDevices")]
    pub ignored_devices: Vec<ObservedDevice>,
    #[serde(rename = "pendingDevices")]
    pub pending_devices: Vec<ObservedDevice>,
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
