import React, { useEffect, useState } from "react";
import styles from "./MultiToolInterfaceConfiguration.module.css";
import {
  CryptographicInterface,
  getConfiguredTools,
  removeInterfaceConfiguration,
} from "../../bindings";
import { MenuSeparator } from "../menuSeparator/MenuSeparator";
import { ToolMenu } from "../toolMenu/ToolMenu";
import { InterfaceConfiguration, ITool } from "./InterfaceConfiguration";

interface IMultiToolInterfaceConfiguration {
  canBeDisabled: boolean;
  interfaceType: CryptographicInterface;
  displayName: string;
}
const changeToolnameForFrontend = (tool: string | null): ITool =>
  !tool
    ? { displayName: "Any", tool: null }
    : { displayName: tool, tool: tool };
export const MultiToolInterfaceConfiguration: React.FC<
  IMultiToolInterfaceConfiguration
> = (props) => {
  const [tools, setTools] = useState<ITool[]>([]);
  const [selectedTool, setSelectedTool] = useState<ITool>();

  const loadTools = async () => {
    let configured_tools = await getConfiguredTools(props.interfaceType);
    let mappedTools = configured_tools.map(changeToolnameForFrontend);
    if (mappedTools.length === 0) {
      let anyTool: ITool = { displayName: "Any", tool: null };
      mappedTools = [anyTool];
    }
    let tool = mappedTools[0] as ITool;
    setSelectedTool(tool);
    setTools(mappedTools);
  };
  useEffect(() => {
    loadTools();
  }, []);
  const addTool = (tool: ITool) => {
    setTools([...tools, tool]);
  };
  const removeTool = (tool: ITool) => {
    // TODO: consider fetching tools and refreshing state from the backend
    setTools((currentTools) => currentTools.filter((t) => t != tool));
    removeInterfaceConfiguration(props.interfaceType, tool.tool);
    if (tools.length > 0) {
      setSelectedTool(tools[0]);
    }
  };
  return (
    <div className={styles["multi-tool-interface-configuration"]}>
      <ToolMenu
        tools={tools}
        selectedTool={selectedTool!}
        setSelectedTool={setSelectedTool}
        addTool={addTool}
        removeTool={removeTool}
      />
      <MenuSeparator />
      <InterfaceConfiguration
        canBeDisabled={props.canBeDisabled}
        interfaceType={props.interfaceType}
        displayName={props.displayName}
        tool={selectedTool!}
      />
    </div>
  );
};
