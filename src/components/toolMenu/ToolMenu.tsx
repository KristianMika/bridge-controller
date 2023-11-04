import styles from "./ToolMenu.module.css";
import { IoMdAddCircleOutline } from "react-icons/io";
import React, { useCallback, useEffect, useState } from "react";
import { ToolMenuItem } from "./toolMenuItem/ToolMenuItem";
import { ToolMenuItemCreation } from "./toolMenuItemCreation/ToolMenuItemCreation";
import { ToolMenuSeparator } from "./toolMenuSeparator/ToolMenuSeparator";
import ITool from "../../models/ITool";
import IToolMenu from "../../models/IToolMenu";

/**
 * Enables the selection of a tool for which the configuration will be displayed
 */
export const ToolMenu: React.FC<IToolMenu> = (props) => {
  const [isToolCreationEntryVisible, setIsToolCreationEntryVisible] =
    useState<boolean>(false);

  // Create a callback for the delete key to remove the currently selected tool
  const keyDowns = useCallback(
    (event: any) => {
      if (event.key === "Delete") {
        removeCurrentTool();
      }
    },
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
    <div className={styles["tool-menu"]}>
      {createMenuItems(props.tools)}
      <ToolMenuItemCreation
        isCreationActive={isToolCreationEntryVisible}
        cancelCreation={() => setIsToolCreationEntryVisible(false)}
        newItemCreationHandler={props.addTool}
      />
      <a className={styles["tool-menu__add-button"]} href="#" onClick={addTool}>
        <IoMdAddCircleOutline size={18} />
      </a>
    </div>
  );
};
