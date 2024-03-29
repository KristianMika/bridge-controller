import { CryptographicInterface } from "../bindings";
import ITool from "./ITool";

interface IInterfaceConfiguration {
  canBeDisabled: boolean;
  interfaceType: CryptographicInterface;
  displayName: string;
  tool: ITool;
}

export default IInterfaceConfiguration;
