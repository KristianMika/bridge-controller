import { ITool } from "../../interfaceConfiguration/InterfaceConfiguration";
import styles from "./ToolMenuItem.module.css";

interface IToolMenuItem {
  tool: ITool;
  isSelected: boolean;
  setSelected: (tool: ITool) => void;
}

export const ToolMenuItem: React.FC<IToolMenuItem> = (props) => {
  return (
    <a
      title={props.tool.displayName}
      className={`${styles["tool_menu__item"]} ${
        props.isSelected ? styles["tool_menu__item--selected"] : ""
      }`}
      href="#"
      onClick={() => props.setSelected(props.tool)}
    >
      {props.tool.displayName}
    </a>
  );
};
