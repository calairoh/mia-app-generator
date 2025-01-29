#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

// Input structs (Console Config)

#[derive(Serialize, Deserialize, Debug)]
pub struct ConsoleConfig {
    applications: HashMap<String, ConsoleApplication>,
    collections: HashMap<String, ConsoleCollection>,
    endpoints: HashMap<String, ConsoleEndpoint>,
    groups: Vec<Value>,
    secrets: Vec<Value>,
    cmsCategories: HashMap<String, Value>,
    cmsSettings: CmsSettings,
    cmsAnalytics: HashMap<String, Value>,
    cmsDashboard: Vec<Value>,
    decorators: ConsoleDecorators,
    services: HashMap<String, ConsoleService>,
    configMaps: HashMap<String, ConsoleConfigMap>,
    serviceSecrets: HashMap<String, ConsoleServiceSecret>,
    apiVersions: Vec<Value>,
    unsecretedVariables: Vec<ConsoleUnsecretedVariable>,
    listeners: HashMap<String, ConsoleListener>,
    version: String,
    servicesToDeploy: HashMap<String, ConsoleServiceToDeploy>,
    platformVersion: String,
    updatedAt: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct ConsoleApplication {
    name: String,
    description: String,
    generatedFrom: String,
    resources: ConsoleResources,
    sourceMarketplaceItem: ConsoleSourceMarketplaceItem,
}

#[derive(Serialize, Deserialize, Debug)]
struct ConsoleResources {
    services: HashMap<String, ConsoleResourceService>,
    endpoints: HashMap<String, ConsoleResourceEndpoint>,
    collections: HashMap<String, ConsoleResourceCollection>,
    unsecretedVariables: HashMap<String, ConsoleResourceUnsecretedVariable>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ConsoleResourceService {
    name: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct ConsoleResourceEndpoint {
    basePath: String,
    service: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct ConsoleResourceCollection {
    name: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct ConsoleResourceUnsecretedVariable {
    name: String,
}


#[derive(Serialize, Deserialize, Debug)]
struct ConsoleSourceMarketplaceItem {
    itemId: String,
    tenantId: String,
    version: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct ConsoleCollection {
    id: String,
    name: String,
    fields: Vec<ConsoleField>,
    #[serde(rename = "type")]
    collection_type: String,
    internalEndpoints: Vec<ConsoleInternalEndpoint>,
    description: String,
    tags: Vec<String>,
    indexes: Vec<ConsoleIndex>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ConsoleField {
    name: String,
    description: Option<String>,
    #[serde(rename = "type")]
    field_type: String,
    required: bool,
    nullable: bool,
    sensitivityValue: Option<i32>,
    encryptionEnabled: Option<bool>,
    encryptionSearchable: Option<bool>,
}


#[derive(Serialize, Deserialize, Debug, Clone)]
struct ConsoleInternalEndpoint {
    basePath: String,
    defaultState: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ConsoleIndex {
    name: String,
    #[serde(rename = "type")]
    index_type: String,
    unique: bool,
    fields: Vec<ConsoleIndexField>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ConsoleIndexField {
    name: String,
    order: i32,
}

#[derive(Serialize, Deserialize, Debug)]
struct ConsoleEndpoint {
    basePath: String,
    #[serde(rename = "type")]
    endpoint_type: String,
    tags: Vec<String>,
    listeners: HashMap<String, bool>,
    pathRewrite: String,
    description: String,
    service: Option<String>,
    port: Option<String>,
    useDownstreamProtocol: Option<bool>,
    public: bool,
    secreted: bool,
    showInDocumentation: bool,
    acl: String,
    backofficeAcl: ConsoleBackofficeAcl,
    allowUnknownRequestContentType: bool,
    allowUnknownResponseContentType: bool,
    forceMicroserviceGatewayProxy: bool,
    #[serde(default)]
     routes: HashMap<String, ConsoleRoute>,
     #[serde(default)]
    collectionId: Option<String>
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct ConsoleRoute {
    id: String,
    verb: String,
    path: String,
    public: RoutePublic,
    secreted: RouteSecreted,
    showInDocumentation: RouteShowInDocumentation,
    acl: RouteAcl,
    backofficeAcl: RouteBackofficeAcl,
    rateLimit: RouteRateLimit,
    allowUnknownRequestContentType: RouteAllowUnknownRequestContentType,
    allowUnknownResponseContentType: RouteAllowUnknownResponseContentType,
     preDecorators: Vec<Value>,
    postDecorators: Vec<Value>
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct RoutePublic {
    inherited: bool
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct RouteSecreted {
    inherited: bool
}


#[derive(Serialize, Deserialize, Debug, Default)]
struct RouteShowInDocumentation {
    inherited: bool
}
#[derive(Serialize, Deserialize, Debug, Default)]
struct RouteAcl {
    inherited: bool
}
#[derive(Serialize, Deserialize, Debug, Default)]
struct RouteBackofficeAcl {
    inherited: bool
}
#[derive(Serialize, Deserialize, Debug, Default)]
struct RouteRateLimit {
    inherited: bool
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct RouteAllowUnknownRequestContentType {
    inherited: bool
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct RouteAllowUnknownResponseContentType {
     inherited: bool,
     #[serde(default)]
    value: bool
}

#[derive(Serialize, Deserialize, Debug)]
struct ConsoleBackofficeAcl {
    inherited: bool,
}

#[derive(Serialize, Deserialize, Debug)]
struct CmsSettings {
    accessGroupsExpression: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct ConsoleDecorators {
    preDecorators: HashMap<String, Value>,
    postDecorators: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ConsoleService {
    name: String,
    description: String,
    #[serde(rename = "type")]
    service_type: String,
    advanced: bool,
    replicas: i32,
    dockerImage: String,
    createdAt: String,
    annotations: Vec<ConsoleAnnotation>,
    labels: Vec<ConsoleLabel>,
    tags: Vec<String>,
    sourceComponentId: Option<String>,
    environment: Vec<ConsoleEnvironmentVariable>,
    resources: ConsoleServiceResources,
    probes: ConsoleProbes,
    terminationGracePeriodSeconds: i32,
    logParser: String,
    swaggerPath: String,
    configMaps: Option<Vec<ConsoleServiceConfigMap>>,
    #[serde(default)]
    secrets: Vec<ConsoleServiceSecretRef>,
    containerPorts: Vec<ConsoleContainerPort>,
    #[serde(default)]
    execPreStop: Vec<String>,
     #[serde(default)]
     mapEnvVarToMountPath: HashMap<String, ConsoleMapEnvVarToMountPath>
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct ConsoleMapEnvVarToMountPath {
    #[serde(rename = "type")]
    map_type: String,
    envName: String
}

#[derive(Serialize, Deserialize, Debug)]
struct ConsoleAnnotation {
    name: String,
    value: String,
    description: String,
    readOnly: bool,
}

#[derive(Serialize, Deserialize, Debug)]
struct ConsoleLabel {
    name: String,
    value: String,
    description: String,
    readOnly: bool,
}

#[derive(Serialize, Deserialize, Debug)]
struct ConsoleEnvironmentVariable {
    name: String,
    value: String,
    valueType: String,
     #[serde(default)]
    readOnly: bool,
}

#[derive(Serialize, Deserialize, Debug)]
struct ConsoleServiceResources {
    cpuLimits: ConsoleResourceLimits,
    memoryLimits: ConsoleResourceLimits,
}

#[derive(Serialize, Deserialize, Debug)]
struct ConsoleResourceLimits {
    max: Option<String>,
    min: Option<String>,
}


#[derive(Serialize, Deserialize, Debug)]
struct ConsoleProbes {
    liveness: ConsoleProbe,
    readiness: ConsoleProbe,
    startup: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ConsoleProbe {
     port: String,
    #[serde(default)]
    path: String,
    #[serde(default)]
    initialDelaySeconds: i32,
    #[serde(default)]
     periodSeconds: i32,
    #[serde(default)]
     timeoutSeconds: i32,
    #[serde(default)]
     successThreshold: i32,
    #[serde(default)]
     failureThreshold: i32,
}

#[derive(Serialize, Deserialize, Debug)]
struct ConsoleServiceConfigMap {
    name: String,
    mountPath: String,
     #[serde(default)]
     viewAsReadOnly: bool,
     #[serde(default)]
     subPaths: Vec<String>,
      #[serde(default)]
    link: Option<ConsoleConfigMapLink>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ConsoleConfigMapLink {
    targetSection: String
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct ConsoleServiceSecretRef {
    name: String,
    mountPath: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct ConsoleContainerPort {
    name: String,
    from: i32,
    to: i32,
     #[serde(default)]
    protocol: String
}

#[derive(Serialize, Deserialize, Debug)]
struct ConsoleConfigMap {
    name: String,
    #[serde(default)]
    gitFilesFolder: String,
    files: Vec<ConsoleConfigMapFile>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ConsoleConfigMapFile {
    name: String,
     #[serde(default)]
    content: String
}

#[derive(Serialize, Deserialize, Debug)]
struct ConsoleServiceSecret {
    name: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct ConsoleUnsecretedVariable {
    name: String,
    environments: HashMap<String, ConsoleEnvironmentVariableValue>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ConsoleEnvironmentVariableValue {
    value: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct ConsoleListener {
    name: String,
    port: i32,
    description: String,
    selectedByDefault: bool,
}

#[derive(Serialize, Deserialize, Debug)]
struct ConsoleServiceToDeploy {
    #[serde(rename = "type")]
    service_type: String,
    name: String,
    tags: Vec<String>,
    version: String,
    checksum: String,
}

// Output structs (Application Config)
#[derive(Serialize, Deserialize, Debug, Default)]
struct ApplicationConfig {
    services: HashMap<String, ApplicationService>,
    endpoints: HashMap<String, ApplicationEndpoint>,
    collections: HashMap<String, ApplicationCollection>,
    unsecretedVariables: HashMap<String, ApplicationUnsecretedVariable>,
    listeners: HashMap<String, ApplicationListener>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct ApplicationService {
    componentId: String,
    containerPorts: Vec<ApplicationContainerPort>,
    #[serde(rename = "type")]
    service_type: String,
    defaultEnvironmentVariables: Vec<ApplicationEnvironmentVariable>,
    defaultResources: ApplicationResources,
    defaultProbes: ApplicationProbes,
    defaultDocumentationPath: String,
    defaultConfigMaps: Vec<ApplicationServiceConfigMap>,
    defaultLogParser: String,
    defaultTerminationGracePeriodSeconds: i32,
    name: String,
    description: String,
    dockerImage: String,
     #[serde(default)]
     execPreStop: Vec<String>,
    #[serde(default)]
     repositoryUrl: String,
     #[serde(default)]
    defaultSecrets: Vec<ApplicationServiceSecretRef>,
    #[serde(default)]
    tags: Vec<String>,

}

#[derive(Serialize, Deserialize, Debug, Default)]
struct ApplicationContainerPort {
    name: String,
    from: i32,
    to: i32,
    #[serde(default)]
    protocol: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct ApplicationEnvironmentVariable {
    name: String,
    value: String,
    valueType: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct ApplicationResources {
    cpuLimits: ApplicationResourceLimits,
    memoryLimits: ApplicationResourceLimits,
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct ApplicationResourceLimits {
    max: Option<String>,
    min: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct ApplicationProbes {
    liveness: ApplicationProbe,
    readiness: ApplicationProbe,
     #[serde(default)]
    startup: HashMap<String, Value>,
}


#[derive(Serialize, Deserialize, Debug, Default)]
struct ApplicationProbe {
    path: String,
    port: String,
    #[serde(default)]
    initialDelaySeconds: i32,
    #[serde(default)]
    periodSeconds: i32,
    #[serde(default)]
    timeoutSeconds: i32,
    #[serde(default)]
    successThreshold: i32,
    #[serde(default)]
    failureThreshold: i32
}


#[derive(Serialize, Deserialize, Debug, Default, Clone)]
struct ApplicationServiceConfigMap {
    name: String,
    mountPath: String,
    viewAsReadOnly: bool,
    files: Vec<ApplicationConfigMapFile>,
    #[serde(default)]
    link: Option<ApplicationConfigMapLink>
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
struct ApplicationConfigMapLink {
    targetSection: String
}


#[derive(Serialize, Deserialize, Debug, Default, Clone)]
struct ApplicationConfigMapFile {
    name: String,
    content: String
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct ApplicationServiceSecretRef {
   name: String,
    mountPath: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct ApplicationEndpoint {
    defaultBasePath: String,
    defaultPathRewrite: String,
    description: String,
    #[serde(rename = "type")]
    endpoint_type: String,
    tags: Vec<String>,
    public: bool,
    secreted: bool,
    showInDocumentation: bool,
    allowUnknownRequestContentType: bool,
    allowUnknownResponseContentType: bool,
    forceMicroserviceGatewayProxy: bool,
    service: String,
    #[serde(default)]
    routes: HashMap<String, ApplicationRoute>,
    #[serde(default)]
    collectionId: Option<String>,
    #[serde(default)]
    listeners: HashMap<String, bool>,
    #[serde(default)]
    acl: String,
    #[serde(default)]
    backofficeAcl: ApplicationBackofficeAcl,
}
#[derive(Serialize, Deserialize, Debug, Default)]
struct ApplicationRoute {
    id: String,
    path: String,
    #[serde(default)]
    public: RoutePublic,
    #[serde(default)]
    showInDocumentation: RouteShowInDocumentation,
     #[serde(default)]
    secreted: RouteSecreted,
    #[serde(default)]
    acl: RouteAcl,
    #[serde(default)]
    backofficeAcl: RouteBackofficeAcl,
    #[serde(default)]
    allowUnknownRequestContentType: RouteAllowUnknownRequestContentType,
     #[serde(default)]
     allowUnknownResponseContentType: RouteAllowUnknownResponseContentType,
     verb: String,
      preDecorators: Vec<Value>,
    postDecorators: Vec<Value>
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct ApplicationBackofficeAcl {
    inherited: bool
}


#[derive(Serialize, Deserialize, Debug, Default)]
struct ApplicationCollection {
    defaultName: String,
    id: String,
    description: String,
    tags: Vec<String>,
    fields: Vec<ConsoleField>,
    internalEndpoints: Vec<ConsoleInternalEndpoint>,
     #[serde(rename = "type")]
    collection_type: String,
    indexes: Vec<ConsoleIndex>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct ApplicationUnsecretedVariable {
    productionEnv: String,
    noProductionEnv: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct ApplicationListener {
    name: String,
    port: String,
    description: String,
    selectedByDefault: bool,
    ownedBy: ApplicationListenerOwnedBy,
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct ApplicationListenerOwnedBy {
     componentIds: Vec<String>,
}

pub fn translate_config(console_json: String) -> Result<String, serde_json::Error> {
    // 1. Parse JSON
    let console_config: ConsoleConfig = serde_json::from_str(console_json.as_str())?;

    // 2. Initialize the output struct
    let mut app_config = ApplicationConfig::default();
    app_config.services = console_config
        .services
        .iter()
        .map(|(key, service)| {
            (
                key.clone(),
                ApplicationService {
                    componentId: service.sourceComponentId.clone().unwrap_or("".to_string()),
                    containerPorts: service
                        .containerPorts
                         .iter()
                        .map(|port| ApplicationContainerPort {
                            name: port.name.clone(),
                            from: port.from,
                            to: port.to,
                            protocol: port.protocol.clone(),
                        })
                        .collect(),
                    service_type: "plugin".to_string(),
                    defaultEnvironmentVariables: service
                        .environment
                        .iter()
                        .map(|env| ApplicationEnvironmentVariable {
                            name: env.name.clone(),
                            value: env.value.clone(),
                            valueType: env.valueType.clone(),
                        })
                        .collect(),
                         defaultSecrets: service.secrets.iter().map(|secret| ApplicationServiceSecretRef{
                            name: secret.name.clone(),
                            mountPath: secret.mountPath.clone(),
                        }).collect(),
                    defaultResources: ApplicationResources {
                        cpuLimits: ApplicationResourceLimits {
                            max: service.resources.cpuLimits.max.clone(),
                            min: service.resources.cpuLimits.min.clone(),
                        },
                        memoryLimits: ApplicationResourceLimits {
                            max: service.resources.memoryLimits.max.clone(),
                            min: service.resources.memoryLimits.min.clone(),
                        },
                    },
                    defaultProbes: ApplicationProbes {
                        liveness: ApplicationProbe {
                            path: service.probes.liveness.path.clone(),
                            port: service.probes.liveness.port.clone(),
                            initialDelaySeconds: service.probes.liveness.initialDelaySeconds,
                             periodSeconds: service.probes.liveness.periodSeconds,
                             timeoutSeconds: service.probes.liveness.timeoutSeconds,
                            successThreshold: service.probes.liveness.successThreshold,
                             failureThreshold: service.probes.liveness.failureThreshold
                        },
                        readiness: ApplicationProbe {
                            path: service.probes.readiness.path.clone(),
                            port: service.probes.readiness.port.clone(),
                            initialDelaySeconds: service.probes.readiness.initialDelaySeconds,
                             periodSeconds: service.probes.readiness.periodSeconds,
                            timeoutSeconds: service.probes.readiness.timeoutSeconds,
                            successThreshold: service.probes.readiness.successThreshold,
                             failureThreshold: service.probes.readiness.failureThreshold
                        },
                         startup: service.probes.startup.clone()
                    },
                    defaultDocumentationPath: service.swaggerPath.clone(),
                    defaultConfigMaps: service
                        .configMaps
                        .as_ref()
                        .map(|config_maps| {
                            config_maps
                                .iter()
                                .map(|cm| ApplicationServiceConfigMap {
                                    name: cm.name.clone(),
                                    mountPath: cm.mountPath.clone(),
                                    viewAsReadOnly: cm.viewAsReadOnly,
                                    files: console_config.configMaps.get(&cm.name).map_or_else(|| vec![], |config_map| config_map.files.iter().map(|file| ApplicationConfigMapFile{name: file.name.clone(), content: file.content.clone()}).collect()),
                                    link: cm.link.clone().map(|link| ApplicationConfigMapLink{ targetSection: link.targetSection})
                                })
                                .collect()
                        })
                        .unwrap_or_else(|| vec![]),
                        execPreStop: service.execPreStop.clone(),
                    defaultLogParser: service.logParser.clone(),
                    defaultTerminationGracePeriodSeconds: service.terminationGracePeriodSeconds,
                    name: service.name.clone(),
                    description: service.description.clone(),
                    dockerImage: service.dockerImage.clone(),
                    repositoryUrl: "".to_string(), // Placeholder
                    tags: service.tags.clone()
                },
            )
        })
        .collect();

    app_config.endpoints = console_config
        .endpoints
        .iter()
        .map(|(key, endpoint)| {
            (
                key.clone(),
                ApplicationEndpoint {
                    defaultBasePath: endpoint.basePath.clone(),
                    defaultPathRewrite: endpoint.pathRewrite.clone(),
                    description: endpoint.description.clone(),
                    endpoint_type: endpoint.endpoint_type.clone(),
                     tags: endpoint.tags.clone(),
                    public: endpoint.public,
                    secreted: endpoint.secreted,
                    showInDocumentation: endpoint.showInDocumentation,
                    allowUnknownRequestContentType: endpoint.allowUnknownRequestContentType,
                    allowUnknownResponseContentType: endpoint.allowUnknownResponseContentType,
                    forceMicroserviceGatewayProxy: endpoint.forceMicroserviceGatewayProxy,
                    service: endpoint.service.clone().unwrap_or("".to_string()),
                    routes: endpoint.routes.iter().map(|(route_key, route)| (route_key.clone(), ApplicationRoute {
                            id: route.id.clone(),
                            path: route.path.clone(),
                            public: RoutePublic { inherited: route.public.inherited },
                             showInDocumentation: RouteShowInDocumentation { inherited: route.showInDocumentation.inherited },
                             secreted: RouteSecreted { inherited: route.secreted.inherited },
                            acl: RouteAcl { inherited: route.acl.inherited },
                            backofficeAcl: RouteBackofficeAcl { inherited: route.backofficeAcl.inherited },
                            allowUnknownRequestContentType: RouteAllowUnknownRequestContentType { inherited: route.allowUnknownRequestContentType.inherited },
                            allowUnknownResponseContentType: RouteAllowUnknownResponseContentType { inherited: route.allowUnknownResponseContentType.inherited, value: route.allowUnknownResponseContentType.value },
                            verb: route.verb.clone(),
                            preDecorators: route.preDecorators.clone(),
                           postDecorators: route.postDecorators.clone()
                        })).collect(),
                    collectionId: endpoint.collectionId.clone(),
                    listeners: endpoint.listeners.clone(),
                      acl: endpoint.acl.clone(),
                    backofficeAcl: ApplicationBackofficeAcl {inherited: endpoint.backofficeAcl.inherited}

                },
            )
        })
        .collect();
     app_config.collections = console_config
        .collections
        .iter()
        .map(|(key, collection)| {
            (
                key.clone(),
                ApplicationCollection {
                     defaultName: collection.name.clone(),
                    id: collection.id.clone(),
                    description: collection.description.clone(),
                    tags: collection.tags.clone(),
                     fields: collection.fields.clone(),
                    internalEndpoints: collection.internalEndpoints.clone(),
                    collection_type: collection.collection_type.clone(),
                     indexes: collection.indexes.clone()

                },
            )
        })
        .collect();
        app_config.unsecretedVariables = console_config
        .unsecretedVariables
        .iter()
        .map(|variable| {
            (
                variable.name.clone(),
                ApplicationUnsecretedVariable {
                    productionEnv: variable.environments.get("PROD").map_or("".to_string(), |env| env.value.clone()),
                     noProductionEnv:  variable.environments.get("DEV").map_or("".to_string(), |env| env.value.clone())
                },
            )
        })
        .collect();


         app_config.listeners = console_config.listeners
        .iter()
        .map(|(key, listener)| {
           (key.clone(), ApplicationListener {
                name: listener.name.clone(),
                port: listener.port.to_string(),
                description: listener.description.clone(),
                selectedByDefault: listener.selectedByDefault,
                ownedBy: ApplicationListenerOwnedBy{
                     componentIds: console_config.services.iter().filter(|(_, service)| service.containerPorts.iter().any(|port| port.name == listener.name)).map(|(key, _)| key.clone()).collect()
                    },
            })
        })
        .collect();


    // 3. Serialize JSON
    let output_json = serde_json::to_string_pretty(&app_config)?;

    Ok(output_json)
}
