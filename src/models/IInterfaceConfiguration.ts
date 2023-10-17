import { CryptographicInterface } from "../bindings";

interface IInterfaceConfiguration {
  canBeDisabled: boolean;
  interfaceType: CryptographicInterface;
  displayName: string;
}

export default IInterfaceConfiguration;
