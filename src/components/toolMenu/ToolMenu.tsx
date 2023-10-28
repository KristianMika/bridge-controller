import styles from "./ToolMenu.module.css";
import { IoMdAddCircleOutline } from "react-icons/io";
import React, { useCallback, useEffect, useState } from "react";
import { ToolMenuItem } from "./toolMenuItem/ToolMenuItem";
import { ToolMenuItemCreation } from "./toolMenuItemCreation/ToolMenuItemCreation";
import { ToolMenuSeparator } from "./toolMenuSeparator/ToolMenuSeparator";
import { ITool } from "../interfaceConfiguration/InterfaceConfiguration";

interface IToolMenu {
  tools: ITool[];
  addTool: (tool: ITool) => void;
  removeTool: (tool: ITool) => void;
  selectedTool: ITool;
  setSelectedTool: (tool: ITool) => void;
}

export const ToolMenu: React.FC<IToolMenu> = (props) => {
  const [isToolCreationEntryVisible, setIsToolCreationEntryVisible] =
    useState<boolean>(false);

  const keyDowns = useCallback(
    (event: any) => {
      if (event.key === "Delete") {
        removeCurrentTool();
      }
    },
    // this dependecy is a hack, I don't know why the function can't reference the variable,
    // but it sticks with the value present on variable initialization
    [props.selectedTool]
  );

  useEffect(() => {
    document.addEventListener("keydown", keyDowns, false);

    return () => {
      document.removeEventListener("keydown", keyDowns, false);
    };
  }, [keyDowns]);

  const removeCurrentTool = () => {
    props.removeTool(props.selectedTool);
  };

  const addTool = (event: React.MouseEvent<HTMLAnchorElement>) => {
    event.preventDefault();
    setIsToolCreationEntryVisible(true);
  };

  const createMenuItem = (tool: ITool): JSX.Element => (
    <>
      <ToolMenuItem
        tool={tool}
        isSelected={tool === props.selectedTool}
        setSelected={props.setSelectedTool}
      />
      <ToolMenuSeparator />
    </>
  );

  const createMenuItems = (tools: ITool[]): JSX.Element[] =>
    tools.map((tool) => createMenuItem(tool));

  return (
    <div className={styles["tool_menu"]}>
      {createMenuItems(props.tools)}
      <ToolMenuItemCreation
        isCreationActive={isToolCreationEntryVisible}
        cancelCreation={() => setIsToolCreationEntryVisible(false)}
        newItemCreationHandler={props.addTool}
      />
      <a className={styles["tool_menu__add_button"]} href="#" onClick={addTool}>
        <IoMdAddCircleOutline size={18} />
      </a>
    </div>
  );
};
