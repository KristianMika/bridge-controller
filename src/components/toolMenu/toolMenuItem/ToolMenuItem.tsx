import IToolMenuItem from "../../../models/IToolMenuItem";
import styles from "./ToolMenuItem.module.css";

/**
 * A component representing a single tool item in the tool menu
 */
const ToolMenuItem: React.FC<IToolMenuItem> = (props) => {
  return (
    <a
      title={props.tool.displayName}
      className={`${styles["tool-menu__item"]} ${
        props.isSelected ? styles["tool-menu__item--selected"] : ""
      }`}
      href="#"
      onClick={() => props.setSelected(props.tool)}
    >
      {props.tool.displayName}
    </a>
  );
};

export default ToolMenuItem;
