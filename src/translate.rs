#![allow(non_snake_case)]

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

// Input structs (Console Config)

#[derive(Serialize, Deserialize, Debug)]
pub struct ConsoleConfig {
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    collections: HashMap<String, ConsoleCollection>,
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    endpoints: HashMap<String, ConsoleEndpoint>,
    groups: Vec<Value>,
    secrets: Vec<Value>,
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    cmsCategories: HashMap<String, Value>,
    cmsSettings: CmsSettings,
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    cmsAnalytics: HashMap<String, Value>,
    cmsDashboard: Vec<Value>,
    decorators: ConsoleDecorators,
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    services: HashMap<String, ConsoleService>,
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    configMaps: HashMap<String, ConsoleConfigMap>,
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    serviceSecrets: HashMap<String, ConsoleServiceSecret>,
    apiVersions: Vec<Value>,
    unsecretedVariables: Vec<ConsoleUnsecretedVariable>,
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    listeners: HashMap<String, ConsoleListener>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ConsoleResources {
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    services: HashMap<String, ConsoleResourceService>,
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    endpoints: HashMap<String, ConsoleResourceEndpoint>,
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    collections: HashMap<String, ConsoleResourceCollection>,
    #[serde(skip_serializing_if = "HashMap::is_empty")]
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
    description: Option<String>,
    tags: Vec<String>,
    indexes: Option<Vec<ConsoleIndex>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ConsoleField {
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(rename = "type")]
    field_type: String,
    required: bool,
    nullable: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    sensitivityValue: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    encryptionEnabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "Vec::is_empty")]
    tags: Vec<String>,
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    listeners: HashMap<String, bool>,
    pathRewrite: String,
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    service: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    port: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    useDownstreamProtocol: Option<bool>,
    public: bool,
    secreted: bool,
    showInDocumentation: bool,
    acl: String,
    backofficeAcl: ConsoleBackofficeAcl,
    allowUnknownRequestContentType: bool,
    allowUnknownResponseContentType: bool,
    forceMicroserviceGatewayProxy: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    routes: Option<HashMap<String, ConsoleRoute>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
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
     inherited: bool
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
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    preDecorators: HashMap<String, Value>,
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    postDecorators: HashMap<String, Value>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ConsoleService {
    name: String,
    description: Option<String>,
    #[serde(rename = "type")]
    service_type: String,
    advanced: bool,
    replicas: i32,
    dockerImage: String,
    createdAt: String,
    annotations: Vec<ConsoleAnnotation>,
    labels: Vec<ConsoleLabel>,
    tags: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sourceComponentId: Option<String>,
    environment: Vec<ConsoleEnvironmentVariable>,
    resources: ConsoleServiceResources,
    probes: Option<ConsoleProbes>,
    terminationGracePeriodSeconds: i32,
    logParser: String,
    swaggerPath: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    configMaps: Option<Vec<ConsoleServiceConfigMap>>,
    #[serde(default)]
    secrets: Vec<ConsoleServiceSecretRef>,
    containerPorts: Vec<ConsoleContainerPort>,
    #[serde(default)]
    execPreStop: Vec<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "HashMap::is_empty")]
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
    description: Option<String>,
    readOnly: bool,
}

#[derive(Serialize, Deserialize, Debug)]
struct ConsoleLabel {
    name: String,
    value: String,
    description: Option<String>,
    readOnly: bool,
}

#[derive(Serialize, Deserialize, Debug)]
struct ConsoleEnvironmentVariable {
    name: String,
    value: Option<String>,
    secretName: Option<String>,
    secretKey: Option<String>,
    valueType: String,
     #[serde(default)]
    readOnly: bool,
}

#[derive(Serialize, Deserialize, Debug)]
struct ConsoleServiceResources {
    cpuLimits: Option<ConsoleResourceLimits>,
    memoryLimits: Option<ConsoleResourceLimits>,
}

#[derive(Serialize, Deserialize, Debug)]
struct ConsoleResourceLimits {
    max: String,
    min: String,
}


#[derive(Serialize, Deserialize, Debug)]
struct ConsoleProbes {
    liveness: ConsoleProbe,
    readiness: ConsoleProbe,
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
      #[serde(skip_serializing_if = "Option::is_none")]
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
    #[serde(skip_serializing_if = "HashMap::is_empty")]
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
    description: Option<String>,
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
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    services: HashMap<String, ApplicationService>,
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    endpoints: HashMap<String, ApplicationEndpoint>,
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    collections: HashMap<String, ApplicationCollection>,
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    unsecretedVariables: HashMap<String, ApplicationUnsecretedVariable>,
    #[serde(skip_serializing_if = "HashMap::is_empty")]
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
    defaultProbes: Option<ApplicationProbes>,
    defaultDocumentationPath: Option<String>,
    defaultConfigMaps: Vec<ApplicationServiceConfigMap>,
    defaultLogParser: String,
    defaultTerminationGracePeriodSeconds: i32,
    name: String,
    description: Option<String>,
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
    to: i32
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct ApplicationEnvironmentVariable {
    name: String,
    valueType: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    secretKey: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    secretName: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct ApplicationResources {
    cpuLimits: ApplicationResourceLimits,
    memoryLimits: ApplicationResourceLimits,
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct ApplicationResourceLimits {
    max: String,
    min: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct ApplicationProbes {
    liveness: ApplicationProbe,
    readiness: ApplicationProbe,
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
    #[serde(skip_serializing_if = "Option::is_none")]
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
    description: Option<String>,
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
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    routes: HashMap<String, ApplicationRoute>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    collectionId: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "HashMap::is_empty")]
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
    description: Option<String>,
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
    description: Option<String>,
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
                            to: port.to
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
                            secretKey: env.secretKey.clone(),
                            secretName: env.secretName.clone(),
                        })
                        .collect(),
                         defaultSecrets: service.secrets.iter().map(|secret| ApplicationServiceSecretRef{
                            name: secret.name.clone(),
                            mountPath: secret.mountPath.clone(),
                        }).collect(),
                    defaultResources: ApplicationResources {
                        cpuLimits: ApplicationResourceLimits {
                            max: service.resources.cpuLimits.as_ref().map_or_else(|| "".to_string(), |limits| limits.max.clone()),
                            min: service.resources.cpuLimits.as_ref().map_or_else(|| "".to_string(), |limits| limits.min.clone()),
                        },
                        memoryLimits: ApplicationResourceLimits {
                            max: service.resources.memoryLimits.as_ref().map_or_else(|| "".to_string(), |limits| limits.max.clone()),
                            min: service.resources.memoryLimits.as_ref().map_or_else(|| "".to_string(), |limits| limits.min.clone()),
                        },
                    },
                    defaultProbes: service.probes.as_ref().map(|probes| ApplicationProbes {
                        liveness: ApplicationProbe {
                            path: probes.liveness.path.clone(),
                            port: probes.liveness.port.clone(),
                            initialDelaySeconds: probes.liveness.initialDelaySeconds,
                            periodSeconds: probes.liveness.periodSeconds,
                            timeoutSeconds: probes.liveness.timeoutSeconds,
                            successThreshold: probes.liveness.successThreshold,
                            failureThreshold: probes.liveness.failureThreshold,
                        },
                        readiness: ApplicationProbe {
                            path: probes.readiness.path.clone(),
                            port: probes.readiness.port.clone(),
                            initialDelaySeconds: probes.readiness.initialDelaySeconds,
                            periodSeconds: probes.readiness.periodSeconds,
                            timeoutSeconds: probes.readiness.timeoutSeconds,
                            successThreshold: probes.readiness.successThreshold,
                            failureThreshold: probes.readiness.failureThreshold,
                        }
                    }),
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
                    routes: endpoint.routes.as_ref().unwrap_or(&HashMap::new()).iter().map(|(route_key, route)| (route_key.clone(), ApplicationRoute {
                            id: route.id.clone(),
                            path: route.path.clone(),
                            public: RoutePublic { inherited: route.public.inherited },
                             showInDocumentation: RouteShowInDocumentation { inherited: route.showInDocumentation.inherited },
                             secreted: RouteSecreted { inherited: route.secreted.inherited },
                            acl: RouteAcl { inherited: route.acl.inherited },
                            backofficeAcl: RouteBackofficeAcl { inherited: route.backofficeAcl.inherited },
                            allowUnknownRequestContentType: RouteAllowUnknownRequestContentType { inherited: route.allowUnknownRequestContentType.inherited },
                            allowUnknownResponseContentType: RouteAllowUnknownResponseContentType { inherited: route.allowUnknownResponseContentType.inherited },
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
                    indexes: collection.indexes.as_ref().unwrap_or(&Vec::new()).clone()

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
