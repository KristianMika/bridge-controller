import { CryptographicInterface } from "../bindings";

interface IMultiToolInterfaceConfiguration {
  canBeDisabled: boolean;
  interfaceType: CryptographicInterface;
  displayName: string;
}

export default IMultiToolInterfaceConfiguration;
