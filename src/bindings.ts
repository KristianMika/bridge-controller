/* eslint-disable */
// This file was generated by [tauri-specta](https://github.com/oscartbeaumont/tauri-specta). Do not edit this file manually.

declare global {
    interface Window {
        __TAURI_INVOKE__<T>(cmd: string, args?: Record<string, unknown>): Promise<T>;
    }
}

// Function avoids 'window not defined' in SSR
const invoke = () => window.__TAURI_INVOKE__;

/**
 * Stores a configuration for a specific cryptographic interface and tool.
 * In case the tool is None, the configuration is handled as tool-independent
 * 
 * # Arguments
 * 
 * * `cryptographic_interface` - The cryptographic interface for which
 * the configuration should be stored
 * * `tool` - The tool for which the configuration should be stored,
 * if None, the configuration is handled as tool-independent
 * * `configuration` - The configuration that should be stored
 * * `state` - The state of the application
 */
export function setInterfaceConfiguration(cryptographicInterface: CryptographicInterface, tool: string | null, configuration: FrontEndInterfaceConfiguration) {
    return invoke()<null>("set_interface_configuration", { cryptographicInterface,tool,configuration })
}

/**
 * Gets a configuration for a specific cryptographic interface and tool.
 * 
 * # Arguments
 * 
 * * `cryptographic_interface` - The cryptographic interface
 * for which the configuration should be fetched
 * * `tool` - The tool for which the configuration should be fetched,
 * if None, the general, tool-independent configuration is returned
 * * `state` - The state of the application
 */
export function getInterfaceConfiguration(cryptographicInterface: CryptographicInterface, tool: string | null) {
    return invoke()<FrontEndInterfaceConfiguration | null>("get_interface_configuration", { cryptographicInterface,tool })
}

/**
 * Removes a configuration for a specific cryptographic interface and tool.
 * 
 * # Arguments
 * 
 * * `state` - The state of the application
 * * `cryptographic_interface` - The cryptographic interface
 * for which the configuration should be removed
 * * `tool` - The tool for which the configuration should be removed,
 * if None, the general, tool-independent configuration is removed
 */
export function removeInterfaceConfiguration(cryptographicInterface: CryptographicInterface, tool: string | null) {
    return invoke()<null>("remove_interface_configuration", { cryptographicInterface,tool })
}

/**
 * Fetches and filters authentication groups present on the specified communicator
 * 
 * # Arguments
 * 
 * * `communicator_hostname` - The hostname of the communicator.
 * * `state` - The state of the application.
 */
export function getGroups(communicatorHostname: string) {
    return invoke()<Group[]>("get_groups", { communicatorHostname })
}

/**
 * Stores the communicator certificate that was uploaded using the front-end page
 * 
 * # Arguments
 * 
 * * `certificate_path` - The path to the certificate file.
 * * `communicator_hostname` - The hostname of the communicator.
 * * `state` - The state of the application.
 */
export function storeCommunicatorCertificate(certificatePath: string, communicatorHostname: string) {
    return invoke()<null>("store_communicator_certificate", { certificatePath,communicatorHostname })
}

/**
 * Launches an emulated interface process. If the process is running,
 * returns an error.
 * 
 * # Arguments
 * 
 * * `creatable_interface` - The emulated interface that
 * should be launched
 * * `state` - The state of the application
 */
export function spawnInterfaceProcess(creatableInterface: CreatableInterface) {
    return invoke()<null>("spawn_interface_process", { creatableInterface })
}

/**
 * Kills an emulated interface process. If the process is not running,
 * returns an error.
 * 
 * # Arguments
 * 
 * * `creatable_interface` - The interface whose process should be killed
 * * `state` - The state of the application
 */
export function killInterfaceProcess(creatableInterface: CreatableInterface) {
    return invoke()<null>("kill_interface_process", { creatableInterface })
}

/**
 * Checks if there is a certificate stored for the specified communicator hostname.
 * 
 * # Arguments
 * * `communicator_hostname` - The hostname of the communicator.
 * * `state` - The state of the application.
 */
export function isCertificatePresent(communicatorHostname: string) {
    return invoke()<boolean>("is_certificate_present", { communicatorHostname })
}

/**
 * Returns a list of tools for which there is a configuration present.
 * Value None present in the list means that the configuration is tool-independent.
 * 
 * # Arguments
 * 
 * * `cryptographic_interface` - The cryptographic interface
 * for which the configured tools should be returned
 * * `state` - The state of the application
 */
export function getConfiguredTools(cryptographicInterface: CryptographicInterface) {
    return invoke()<(string | null)[]>("get_configured_tools", { cryptographicInterface })
}

export type CryptographicInterface = "pcsc" | "cryptoki" | "webauthn"
/**
 * Represents an interface that a process can be spawned for.
 */
export type CreatableInterface = "pcsc" | "webauthn"
/**
 * Interface configuration used within the front-end
 */
export type FrontEndInterfaceConfiguration = { isEnabled: boolean; communicatorHostname: string; selectedGroup: string }
export type Group = { name: string; group_id: string }
