#![doc = "generated by AutoRust 0.1.0"]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClusterDefinition {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blueprint: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[serde(rename = "componentVersion", skip_serializing_if = "Option::is_none")]
    pub component_version: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configurations: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClientGroupInfo {
    #[serde(rename = "groupName", skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
    #[serde(rename = "groupId", skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct KafkaRestProperties {
    #[serde(rename = "clientGroupInfo", skip_serializing_if = "Option::is_none")]
    pub client_group_info: Option<ClientGroupInfo>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SecurityProfile {
    #[serde(rename = "directoryType", skip_serializing_if = "Option::is_none")]
    pub directory_type: Option<security_profile::DirectoryType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    #[serde(rename = "organizationalUnitDN", skip_serializing_if = "Option::is_none")]
    pub organizational_unit_dn: Option<String>,
    #[serde(rename = "ldapsUrls", skip_serializing_if = "Vec::is_empty")]
    pub ldaps_urls: Vec<String>,
    #[serde(rename = "domainUsername", skip_serializing_if = "Option::is_none")]
    pub domain_username: Option<String>,
    #[serde(rename = "domainUserPassword", skip_serializing_if = "Option::is_none")]
    pub domain_user_password: Option<String>,
    #[serde(rename = "clusterUsersGroupDNs", skip_serializing_if = "Vec::is_empty")]
    pub cluster_users_group_d_ns: Vec<String>,
    #[serde(rename = "aaddsResourceId", skip_serializing_if = "Option::is_none")]
    pub aadds_resource_id: Option<String>,
    #[serde(rename = "msiResourceId", skip_serializing_if = "Option::is_none")]
    pub msi_resource_id: Option<String>,
}
pub mod security_profile {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum DirectoryType {
        ActiveDirectory,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AutoscaleTimeAndCapacity {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<String>,
    #[serde(rename = "minInstanceCount", skip_serializing_if = "Option::is_none")]
    pub min_instance_count: Option<i32>,
    #[serde(rename = "maxInstanceCount", skip_serializing_if = "Option::is_none")]
    pub max_instance_count: Option<i32>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AutoscaleSchedule {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub days: Vec<String>,
    #[serde(rename = "timeAndCapacity", skip_serializing_if = "Option::is_none")]
    pub time_and_capacity: Option<AutoscaleTimeAndCapacity>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AutoscaleCapacity {
    #[serde(rename = "minInstanceCount", skip_serializing_if = "Option::is_none")]
    pub min_instance_count: Option<i32>,
    #[serde(rename = "maxInstanceCount", skip_serializing_if = "Option::is_none")]
    pub max_instance_count: Option<i32>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AutoscaleRecurrence {
    #[serde(rename = "timeZone", skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub schedule: Vec<AutoscaleSchedule>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Autoscale {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capacity: Option<AutoscaleCapacity>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurrence: Option<AutoscaleRecurrence>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AutoscaleConfigurationUpdateParameter {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub autoscale: Option<Autoscale>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct HardwareProfile {
    #[serde(rename = "vmSize", skip_serializing_if = "Option::is_none")]
    pub vm_size: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VirtualNetworkProfile {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subnet: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DataDisksGroups {
    #[serde(rename = "disksPerNode", skip_serializing_if = "Option::is_none")]
    pub disks_per_node: Option<i32>,
    #[serde(rename = "storageAccountType", skip_serializing)]
    pub storage_account_type: Option<String>,
    #[serde(rename = "diskSizeGB", skip_serializing)]
    pub disk_size_gb: Option<i64>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SshPublicKey {
    #[serde(rename = "certificateData", skip_serializing_if = "Option::is_none")]
    pub certificate_data: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SshProfile {
    #[serde(rename = "publicKeys", skip_serializing_if = "Vec::is_empty")]
    pub public_keys: Vec<SshPublicKey>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LinuxOperatingSystemProfile {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(rename = "sshProfile", skip_serializing_if = "Option::is_none")]
    pub ssh_profile: Option<SshProfile>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OsProfile {
    #[serde(rename = "linuxOperatingSystemProfile", skip_serializing_if = "Option::is_none")]
    pub linux_operating_system_profile: Option<LinuxOperatingSystemProfile>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Role {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "minInstanceCount", skip_serializing_if = "Option::is_none")]
    pub min_instance_count: Option<i32>,
    #[serde(rename = "targetInstanceCount", skip_serializing_if = "Option::is_none")]
    pub target_instance_count: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub autoscale: Option<Autoscale>,
    #[serde(rename = "hardwareProfile", skip_serializing_if = "Option::is_none")]
    pub hardware_profile: Option<HardwareProfile>,
    #[serde(rename = "osProfile", skip_serializing_if = "Option::is_none")]
    pub os_profile: Option<OsProfile>,
    #[serde(rename = "virtualNetworkProfile", skip_serializing_if = "Option::is_none")]
    pub virtual_network_profile: Option<VirtualNetworkProfile>,
    #[serde(rename = "dataDisksGroups", skip_serializing_if = "Vec::is_empty")]
    pub data_disks_groups: Vec<DataDisksGroups>,
    #[serde(rename = "scriptActions", skip_serializing_if = "Vec::is_empty")]
    pub script_actions: Vec<ScriptAction>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ComputeProfile {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub roles: Vec<Role>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StorageAccount {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "isDefault", skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container: Option<String>,
    #[serde(rename = "fileSystem", skip_serializing_if = "Option::is_none")]
    pub file_system: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(rename = "resourceId", skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[serde(rename = "msiResourceId", skip_serializing_if = "Option::is_none")]
    pub msi_resource_id: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StorageProfile {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub storageaccounts: Vec<StorageAccount>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NetworkProperties {
    #[serde(rename = "resourceProviderConnection", skip_serializing_if = "Option::is_none")]
    pub resource_provider_connection: Option<network_properties::ResourceProviderConnection>,
    #[serde(rename = "privateLink", skip_serializing_if = "Option::is_none")]
    pub private_link: Option<network_properties::PrivateLink>,
}
pub mod network_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ResourceProviderConnection {
        Inbound,
        Outbound,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum PrivateLink {
        Disabled,
        Enabled,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClusterCreateProperties {
    #[serde(rename = "clusterVersion", skip_serializing_if = "Option::is_none")]
    pub cluster_version: Option<String>,
    #[serde(rename = "osType", skip_serializing_if = "Option::is_none")]
    pub os_type: Option<cluster_create_properties::OsType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tier: Option<cluster_create_properties::Tier>,
    #[serde(rename = "clusterDefinition", skip_serializing_if = "Option::is_none")]
    pub cluster_definition: Option<ClusterDefinition>,
    #[serde(rename = "kafkaRestProperties", skip_serializing_if = "Option::is_none")]
    pub kafka_rest_properties: Option<KafkaRestProperties>,
    #[serde(rename = "securityProfile", skip_serializing_if = "Option::is_none")]
    pub security_profile: Option<SecurityProfile>,
    #[serde(rename = "computeProfile", skip_serializing_if = "Option::is_none")]
    pub compute_profile: Option<ComputeProfile>,
    #[serde(rename = "storageProfile", skip_serializing_if = "Option::is_none")]
    pub storage_profile: Option<StorageProfile>,
    #[serde(rename = "diskEncryptionProperties", skip_serializing_if = "Option::is_none")]
    pub disk_encryption_properties: Option<DiskEncryptionProperties>,
    #[serde(rename = "encryptionInTransitProperties", skip_serializing_if = "Option::is_none")]
    pub encryption_in_transit_properties: Option<EncryptionInTransitProperties>,
    #[serde(rename = "minSupportedTlsVersion", skip_serializing_if = "Option::is_none")]
    pub min_supported_tls_version: Option<String>,
    #[serde(rename = "networkProperties", skip_serializing_if = "Option::is_none")]
    pub network_properties: Option<NetworkProperties>,
}
pub mod cluster_create_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum OsType {
        Windows,
        Linux,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Tier {
        Standard,
        Premium,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClusterCreateParametersExtended {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<ClusterCreateProperties>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity: Option<ClusterIdentity>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClusterPatchParameters {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct QuotaInfo {
    #[serde(rename = "coresUsed", skip_serializing_if = "Option::is_none")]
    pub cores_used: Option<i32>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Errors {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConnectivityEndpoint {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i32>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClusterGetProperties {
    #[serde(rename = "clusterVersion", skip_serializing_if = "Option::is_none")]
    pub cluster_version: Option<String>,
    #[serde(rename = "osType", skip_serializing_if = "Option::is_none")]
    pub os_type: Option<cluster_get_properties::OsType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tier: Option<cluster_get_properties::Tier>,
    #[serde(rename = "clusterId", skip_serializing_if = "Option::is_none")]
    pub cluster_id: Option<String>,
    #[serde(rename = "clusterDefinition")]
    pub cluster_definition: ClusterDefinition,
    #[serde(rename = "kafkaRestProperties", skip_serializing_if = "Option::is_none")]
    pub kafka_rest_properties: Option<KafkaRestProperties>,
    #[serde(rename = "securityProfile", skip_serializing_if = "Option::is_none")]
    pub security_profile: Option<SecurityProfile>,
    #[serde(rename = "computeProfile", skip_serializing_if = "Option::is_none")]
    pub compute_profile: Option<ComputeProfile>,
    #[serde(rename = "provisioningState", skip_serializing_if = "Option::is_none")]
    pub provisioning_state: Option<cluster_get_properties::ProvisioningState>,
    #[serde(rename = "createdDate", skip_serializing_if = "Option::is_none")]
    pub created_date: Option<String>,
    #[serde(rename = "clusterState", skip_serializing_if = "Option::is_none")]
    pub cluster_state: Option<String>,
    #[serde(rename = "quotaInfo", skip_serializing_if = "Option::is_none")]
    pub quota_info: Option<QuotaInfo>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub errors: Vec<Errors>,
    #[serde(rename = "connectivityEndpoints", skip_serializing_if = "Vec::is_empty")]
    pub connectivity_endpoints: Vec<ConnectivityEndpoint>,
    #[serde(rename = "diskEncryptionProperties", skip_serializing_if = "Option::is_none")]
    pub disk_encryption_properties: Option<DiskEncryptionProperties>,
    #[serde(rename = "encryptionInTransitProperties", skip_serializing_if = "Option::is_none")]
    pub encryption_in_transit_properties: Option<EncryptionInTransitProperties>,
    #[serde(rename = "minSupportedTlsVersion", skip_serializing_if = "Option::is_none")]
    pub min_supported_tls_version: Option<String>,
    #[serde(rename = "networkProperties", skip_serializing_if = "Option::is_none")]
    pub network_properties: Option<NetworkProperties>,
}
pub mod cluster_get_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum OsType {
        Windows,
        Linux,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Tier {
        Standard,
        Premium,
    }
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum ProvisioningState {
        InProgress,
        Failed,
        Succeeded,
        Canceled,
        Deleting,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Cluster {
    #[serde(flatten)]
    pub tracked_resource: TrackedResource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<ClusterGetProperties>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity: Option<ClusterIdentity>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RuntimeScriptAction {
    pub name: String,
    pub uri: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<String>,
    pub roles: Vec<String>,
    #[serde(rename = "applicationName", skip_serializing)]
    pub application_name: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExecuteScriptActionParameters {
    #[serde(rename = "scriptActions", skip_serializing_if = "Vec::is_empty")]
    pub script_actions: Vec<RuntimeScriptAction>,
    #[serde(rename = "persistOnSuccess")]
    pub persist_on_success: bool,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClusterListPersistedScriptActionsResult {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<RuntimeScriptAction>,
    #[serde(rename = "nextLink", skip_serializing)]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScriptActionExecutionSummary {
    #[serde(skip_serializing)]
    pub status: Option<String>,
    #[serde(rename = "instanceCount", skip_serializing)]
    pub instance_count: Option<i32>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RuntimeScriptActionDetail {
    #[serde(flatten)]
    pub runtime_script_action: RuntimeScriptAction,
    #[serde(flatten)]
    pub serde_json_value: serde_json::Value,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClusterListRuntimeScriptActionDetailResult {
    #[serde(skip_serializing)]
    pub value: Vec<RuntimeScriptActionDetail>,
    #[serde(rename = "nextLink", skip_serializing)]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClusterIdentity {
    #[serde(rename = "principalId", skip_serializing)]
    pub principal_id: Option<String>,
    #[serde(rename = "tenantId", skip_serializing)]
    pub tenant_id: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<cluster_identity::Type>,
    #[serde(rename = "userAssignedIdentities", skip_serializing_if = "Option::is_none")]
    pub user_assigned_identities: Option<serde_json::Value>,
}
pub mod cluster_identity {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Type {
        SystemAssigned,
        UserAssigned,
        #[serde(rename = "SystemAssigned, UserAssigned")]
        SystemAssignedUserAssigned,
        None,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClusterListResult {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Cluster>,
    #[serde(rename = "nextLink", skip_serializing)]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClusterResizeParameters {
    #[serde(rename = "targetInstanceCount", skip_serializing_if = "Option::is_none")]
    pub target_instance_count: Option<i32>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClusterDiskEncryptionParameters {
    #[serde(rename = "vaultUri", skip_serializing_if = "Option::is_none")]
    pub vault_uri: Option<String>,
    #[serde(rename = "keyName", skip_serializing_if = "Option::is_none")]
    pub key_name: Option<String>,
    #[serde(rename = "keyVersion", skip_serializing_if = "Option::is_none")]
    pub key_version: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DiskEncryptionProperties {
    #[serde(rename = "vaultUri", skip_serializing_if = "Option::is_none")]
    pub vault_uri: Option<String>,
    #[serde(rename = "keyName", skip_serializing_if = "Option::is_none")]
    pub key_name: Option<String>,
    #[serde(rename = "keyVersion", skip_serializing_if = "Option::is_none")]
    pub key_version: Option<String>,
    #[serde(rename = "encryptionAlgorithm", skip_serializing_if = "Option::is_none")]
    pub encryption_algorithm: Option<disk_encryption_properties::EncryptionAlgorithm>,
    #[serde(rename = "msiResourceId", skip_serializing_if = "Option::is_none")]
    pub msi_resource_id: Option<String>,
    #[serde(rename = "encryptionAtHost", skip_serializing_if = "Option::is_none")]
    pub encryption_at_host: Option<bool>,
}
pub mod disk_encryption_properties {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum EncryptionAlgorithm {
        #[serde(rename = "RSA-OAEP")]
        RsaOaep,
        #[serde(rename = "RSA-OAEP-256")]
        RsaOaep256,
        #[serde(rename = "RSA1_5")]
        Rsa15,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EncryptionInTransitProperties {
    #[serde(rename = "isEncryptionInTransitEnabled", skip_serializing_if = "Option::is_none")]
    pub is_encryption_in_transit_enabled: Option<bool>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateGatewaySettingsParameters {
    #[serde(rename = "restAuthCredential.isEnabled", skip_serializing_if = "Option::is_none")]
    pub rest_auth_credential_is_enabled: Option<bool>,
    #[serde(rename = "restAuthCredential.username", skip_serializing_if = "Option::is_none")]
    pub rest_auth_credential_username: Option<String>,
    #[serde(rename = "restAuthCredential.password", skip_serializing_if = "Option::is_none")]
    pub rest_auth_credential_password: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GatewaySettings {
    #[serde(rename = "restAuthCredential.isEnabled", skip_serializing)]
    pub rest_auth_credential_is_enabled: Option<String>,
    #[serde(rename = "restAuthCredential.username", skip_serializing)]
    pub rest_auth_credential_username: Option<String>,
    #[serde(rename = "restAuthCredential.password", skip_serializing)]
    pub rest_auth_credential_password: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationResource {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<operation_resource::Status>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<Errors>,
}
pub mod operation_resource {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Status {
        InProgress,
        Succeeded,
        Failed,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Resource {
    #[serde(skip_serializing)]
    pub id: Option<String>,
    #[serde(skip_serializing)]
    pub name: Option<String>,
    #[serde(rename = "type", skip_serializing)]
    pub type_: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TrackedResource {
    #[serde(flatten)]
    pub resource: Resource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProxyResource {
    #[serde(flatten)]
    pub resource: Resource,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Operation {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<operation::Display>,
}
pub mod operation {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub struct Display {
        #[serde(skip_serializing_if = "Option::is_none")]
        pub provider: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub resource: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub operation: Option<String>,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OperationListResult {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Operation>,
    #[serde(rename = "nextLink", skip_serializing_if = "Option::is_none")]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScriptActionExecutionHistoryList {
    #[serde(skip_serializing)]
    pub value: Vec<RuntimeScriptActionDetail>,
    #[serde(rename = "nextLink", skip_serializing)]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScriptAction {
    pub name: String,
    pub uri: String,
    pub parameters: String,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScriptActionPersistedGetResponseSpec {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub roles: Vec<String>,
    #[serde(rename = "applicationName", skip_serializing_if = "Option::is_none")]
    pub application_name: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScriptActionsList {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<RuntimeScriptActionDetail>,
    #[serde(rename = "nextLink", skip_serializing)]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationGetHttpsEndpoint {
    #[serde(rename = "accessModes", skip_serializing_if = "Vec::is_empty")]
    pub access_modes: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(rename = "destinationPort", skip_serializing_if = "Option::is_none")]
    pub destination_port: Option<i32>,
    #[serde(rename = "publicPort", skip_serializing_if = "Option::is_none")]
    pub public_port: Option<i32>,
    #[serde(rename = "subDomainSuffix", skip_serializing_if = "Option::is_none")]
    pub sub_domain_suffix: Option<String>,
    #[serde(rename = "disableGatewayAuth", skip_serializing_if = "Option::is_none")]
    pub disable_gateway_auth: Option<bool>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationGetEndpoint {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    #[serde(rename = "destinationPort", skip_serializing_if = "Option::is_none")]
    pub destination_port: Option<i32>,
    #[serde(rename = "publicPort", skip_serializing_if = "Option::is_none")]
    pub public_port: Option<i32>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationProperties {
    #[serde(rename = "computeProfile", skip_serializing_if = "Option::is_none")]
    pub compute_profile: Option<ComputeProfile>,
    #[serde(rename = "installScriptActions", skip_serializing_if = "Vec::is_empty")]
    pub install_script_actions: Vec<RuntimeScriptAction>,
    #[serde(rename = "uninstallScriptActions", skip_serializing_if = "Vec::is_empty")]
    pub uninstall_script_actions: Vec<RuntimeScriptAction>,
    #[serde(rename = "httpsEndpoints", skip_serializing_if = "Vec::is_empty")]
    pub https_endpoints: Vec<ApplicationGetHttpsEndpoint>,
    #[serde(rename = "sshEndpoints", skip_serializing_if = "Vec::is_empty")]
    pub ssh_endpoints: Vec<ApplicationGetEndpoint>,
    #[serde(rename = "provisioningState", skip_serializing)]
    pub provisioning_state: Option<String>,
    #[serde(rename = "applicationType", skip_serializing_if = "Option::is_none")]
    pub application_type: Option<String>,
    #[serde(rename = "applicationState", skip_serializing)]
    pub application_state: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub errors: Vec<Errors>,
    #[serde(rename = "createdDate", skip_serializing)]
    pub created_date: Option<String>,
    #[serde(rename = "marketplaceIdentifier", skip_serializing)]
    pub marketplace_identifier: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Application {
    #[serde(flatten)]
    pub proxy_resource: ProxyResource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<ApplicationProperties>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApplicationListResult {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Application>,
    #[serde(rename = "nextLink", skip_serializing)]
    pub next_link: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VersionSpec {
    #[serde(rename = "friendlyName", skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(rename = "isDefault", skip_serializing_if = "Option::is_none")]
    pub is_default: Option<String>,
    #[serde(rename = "componentVersions", skip_serializing_if = "Option::is_none")]
    pub component_versions: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VersionsCapability {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub available: Vec<VersionSpec>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RegionsCapability {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub available: Vec<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VmSizesCapability {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub available: Vec<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VmSizeCompatibilityFilter {
    #[serde(rename = "FilterMode", skip_serializing_if = "Option::is_none")]
    pub filter_mode: Option<String>,
    #[serde(rename = "Regions", skip_serializing_if = "Vec::is_empty")]
    pub regions: Vec<String>,
    #[serde(rename = "ClusterFlavors", skip_serializing_if = "Vec::is_empty")]
    pub cluster_flavors: Vec<String>,
    #[serde(rename = "NodeTypes", skip_serializing_if = "Vec::is_empty")]
    pub node_types: Vec<String>,
    #[serde(rename = "ClusterVersions", skip_serializing_if = "Vec::is_empty")]
    pub cluster_versions: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub vmsizes: Vec<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RegionalQuotaCapability {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cores_used: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cores_available: Option<i64>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct QuotaCapability {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cores_used: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_cores_allowed: Option<i64>,
    #[serde(rename = "regionalQuotas", skip_serializing_if = "Vec::is_empty")]
    pub regional_quotas: Vec<RegionalQuotaCapability>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CapabilitiesResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub versions: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regions: Option<serde_json::Value>,
    #[serde(rename = "vmSizes", skip_serializing_if = "Option::is_none")]
    pub vm_sizes: Option<serde_json::Value>,
    #[serde(rename = "vmSize_filters", skip_serializing_if = "Vec::is_empty")]
    pub vm_size_filters: Vec<VmSizeCompatibilityFilter>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub features: Vec<String>,
    #[serde(skip_serializing)]
    pub quota: Option<QuotaCapability>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LocalizedName {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(rename = "localizedValue", skip_serializing_if = "Option::is_none")]
    pub localized_value: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Usage {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[serde(rename = "currentValue", skip_serializing_if = "Option::is_none")]
    pub current_value: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<LocalizedName>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UsagesListResult {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub value: Vec<Usage>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BillingResponseListResult {
    #[serde(rename = "vmSizes", skip_serializing_if = "Vec::is_empty")]
    pub vm_sizes: Vec<String>,
    #[serde(rename = "vmSizeFilters", skip_serializing_if = "Vec::is_empty")]
    pub vm_size_filters: Vec<VmSizeCompatibilityFilterV2>,
    #[serde(rename = "billingResources", skip_serializing_if = "Vec::is_empty")]
    pub billing_resources: Vec<BillingResources>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct VmSizeCompatibilityFilterV2 {
    #[serde(rename = "filterMode", skip_serializing_if = "Option::is_none")]
    pub filter_mode: Option<vm_size_compatibility_filter_v2::FilterMode>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub regions: Vec<String>,
    #[serde(rename = "clusterFlavors", skip_serializing_if = "Vec::is_empty")]
    pub cluster_flavors: Vec<String>,
    #[serde(rename = "nodeTypes", skip_serializing_if = "Vec::is_empty")]
    pub node_types: Vec<String>,
    #[serde(rename = "clusterVersions", skip_serializing_if = "Vec::is_empty")]
    pub cluster_versions: Vec<String>,
    #[serde(rename = "osType", skip_serializing_if = "Vec::is_empty")]
    pub os_type: Vec<String>,
    #[serde(rename = "vmSizes", skip_serializing_if = "Vec::is_empty")]
    pub vm_sizes: Vec<String>,
}
pub mod vm_size_compatibility_filter_v2 {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum FilterMode {
        Exclude,
        Include,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BillingResources {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(rename = "billingMeters", skip_serializing_if = "Vec::is_empty")]
    pub billing_meters: Vec<BillingMeters>,
    #[serde(rename = "diskBillingMeters", skip_serializing_if = "Vec::is_empty")]
    pub disk_billing_meters: Vec<DiskBillingMeters>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BillingMeters {
    #[serde(rename = "meterParameter", skip_serializing_if = "Option::is_none")]
    pub meter_parameter: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meter: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DiskBillingMeters {
    #[serde(rename = "diskRpMeter", skip_serializing_if = "Option::is_none")]
    pub disk_rp_meter: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sku: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tier: Option<disk_billing_meters::Tier>,
}
pub mod disk_billing_meters {
    use super::*;
    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
    pub enum Tier {
        Standard,
        Premium,
    }
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClusterConfiguration {}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClusterConfigurations {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configurations: Option<serde_json::Value>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Extension {
    #[serde(rename = "workspaceId", skip_serializing_if = "Option::is_none")]
    pub workspace_id: Option<String>,
    #[serde(rename = "primaryKey", skip_serializing_if = "Option::is_none")]
    pub primary_key: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClusterMonitoringResponse {
    #[serde(rename = "clusterMonitoringEnabled", skip_serializing_if = "Option::is_none")]
    pub cluster_monitoring_enabled: Option<bool>,
    #[serde(rename = "workspaceId", skip_serializing_if = "Option::is_none")]
    pub workspace_id: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ClusterMonitoringRequest {
    #[serde(rename = "workspaceId", skip_serializing_if = "Option::is_none")]
    pub workspace_id: Option<String>,
    #[serde(rename = "primaryKey", skip_serializing_if = "Option::is_none")]
    pub primary_key: Option<String>,
}
