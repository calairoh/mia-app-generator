# Application Generator

## Overview

The Application Generator is a tool designed to simplify the creation and management of application configurations for Mia-Platform Service Catalog. It starts from the JSON state of a Mia-Platform project and creates the associated application to publish on Mia-Platform Marketplace. This allows users to easily go from a Mia-Platform Project to an item to be published in the Marketplace.

## Features

- JSON mapping between Mia-Platform JSON project state to Mia-Platform Application format.

## Disclaimer

This tool has been created to automate a very specific case and is based on reverse engineering techniques. If you want to use it on your project to create application configurations, it might happen that you'll need to manually fix some wrong mappings on the output file. In such cases, please report any feature that may improve the tool.

## Getting Started

### Prerequisites

- Rust (version 1.56 or higher)
- Cargo (version 1.56 or higher)

### Installation

1. Clone the repository:
    ```sh
    git clone https://github.com/your-username/application-generator.git
    cd application-generator
    ```

2. Build the project:
    ```sh
    cargo build
    ```

### Usage

1. Define your application configuration in the `console_config.json` file. Here is an example structure:
    ```json
    {
      "services": {
        "example-service": {
          "name": "example-service",
          "description": "An example service",
          "type": "plugin",
          "dockerImage": "example/image:latest",
          "environment": [
            {
              "name": "LOG_LEVEL",
              "value": "info",
              "valueType": "plain"
            }
          ],
          // ...other service configurations...
        }
      },
      "endpoints": {
        "/example-endpoint": {
          "basePath": "/example-endpoint",
          "description": "An example endpoint",
          "type": "custom",
          "tags": ["example"],
          "public": true,
          // ...other endpoint configurations...
        }
      },
      // ...other configurations...
    }
    ```

2. Run the application generator:
    ```sh
    cargo run -- ./tests/files/console_config.json ./tests/files/output_app_definition.json
    ```

3. The generated configuration files will be available in the specified output path.

### Configuration

- **Services**: Define microservices with environment variables, resources, probes, and more.
- **Endpoints**: Configure API endpoints with custom paths, descriptions, and access controls.
- **Collections**: Manage data collections with fields, indexes, and internal endpoints.
- **Config Maps**: Define configuration maps for services.
- **Secrets**: Manage secrets for secure configurations.

### Example

Here is an example of a service definition in `console_config.json`:
```json
{
  "services": {
    "example-service": {
      "name": "example-service",
      "description": "An example service",
      "type": "plugin",
      "dockerImage": "example/image:latest",
      "environment": [
        {
          "name": "LOG_LEVEL",
          "value": "info",
          "valueType": "plain"
        }
      ],
      "resources": {
        "cpuLimits": {
          "max": "100m",
          "min": "10m"
        },
        "memoryLimits": {
          "max": "256Mi",
          "min": "128Mi"
        }
      },
      "probes": {
        "liveness": {
          "path": "/healthz",
          "port": "http",
          "initialDelaySeconds": 15,
          "periodSeconds": 20,
          "timeoutSeconds": 1,
          "successThreshold": 1,
          "failureThreshold": 3
        },
        "readiness": {
          "path": "/ready",
          "port": "http",
          "initialDelaySeconds": 5,
          "periodSeconds": 10,
          "timeoutSeconds": 1,
          "successThreshold": 1,
          "failureThreshold": 3
        }
      },
      "terminationGracePeriodSeconds": 30,
      "logParser": "json",
      "swaggerPath": "/swagger",
      "configMaps": [
        {
          "name": "example-config",
          "mountPath": "/etc/config",
          "viewAsReadOnly": true,
          "files": [
            {
              "name": "config.json",
              "content": "{}"
            }
          ]
        }
      ],
      "secrets": [
        {
          "name": "example-secret",
          "mountPath": "/etc/secrets"
        }
      ],
      "containerPorts": [
        {
          "name": "http",
          "from": 80,
          "to": 8080
        }
      ],
      "execPreStop": [
        "bash",
        "-c",
        "echo 'Stopping service...'"
      ]
    }
  }
}
```

## Contributing

Contributions are welcome! Please read the [CONTRIBUTING.md](CONTRIBUTING.md) file for more information on how to contribute to this project.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Contact

For any questions or suggestions, please open an issue or contact the maintainers.
