import { CryptographicInterface } from "../bindings";
import { ITool } from "../components/interfaceConfiguration/InterfaceConfiguration";

interface IInterfaceConfiguration {
  canBeDisabled: boolean;
  interfaceType: CryptographicInterface;
  displayName: string;
  tool: ITool;
}

export default IInterfaceConfiguration;
