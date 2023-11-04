import React, { useEffect, useState } from "react";
import styles from "./MultiToolInterfaceConfiguration.module.css";
import {
  getConfiguredTools,
  removeInterfaceConfiguration,
} from "../../bindings";
import { MenuSeparator } from "../menuSeparator/MenuSeparator";
import { ToolMenu } from "../toolMenu/ToolMenu";
import { InterfaceConfiguration } from "./InterfaceConfiguration";
import IMultiToolInterfaceConfiguration from "../../models/IMultiToolInterfaceConfiguration";
import ITool from "../../models/ITool";
import AnimationComponent from "../animation/animatedComponent/AnimationComponent";

/**
 * Backend only stores an array of tools, but we also need to represent an option for "any" tool.
 * This function converts a tool string to an object with a displayName and tool value.
 *
 * @param tool if null, then the configuration is tool-independent, otherwise it is tool-specific for the tool specified
 * @returns
 */
const toolObjectFromValue = (tool: string | null): ITool =>
  !tool
    ? { displayName: "Any", tool: null }
    : { displayName: tool, tool: tool };

/**
 * This component wraps `InterfaceConfigurationComponent` and enables per-tool configuration
 */
export const MultiToolInterfaceConfiguration: React.FC<
  IMultiToolInterfaceConfiguration
> = (props) => {
  const [tools, setTools] = useState<ITool[]>([]);
  const [selectedTool, setSelectedTool] = useState<ITool>();

  const loadTools = async () => {
    let configuredTools = await getConfiguredTools(props.interfaceType);
    let mappedTools = configuredTools.map(toolObjectFromValue);
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
      <AnimationComponent uniqueKey={selectedTool?.displayName || ""}>
        <InterfaceConfiguration
          canBeDisabled={props.canBeDisabled}
          interfaceType={props.interfaceType}
          displayName={props.displayName}
          tool={selectedTool!}
        />
      </AnimationComponent>
    </div>
  );
};
