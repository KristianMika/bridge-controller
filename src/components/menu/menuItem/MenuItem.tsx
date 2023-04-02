import { MouseEventHandler } from "react";
import { IconType } from "react-icons";
import styles from "./MenuItem.module.css";

export interface IMenuItem {
  onClick: MouseEventHandler;
  icon: IconType;
  title: string;
  isSelected: boolean;
}

/**
 * A single menu item in the menu component.
 */
export const MenuItem: React.FC<IMenuItem> = (props) => {
  const menuIconStyle = { color: "#e6e6e6", fontSize: "2.5em" };

  const addClassIfSelected = (): string => {
    if (props.isSelected) {
      return styles["menu__menu-item--selected"];
    }
    return "";
  };

  return (
    <a
      className={`${styles["menu__menu-item"]} ${addClassIfSelected()}`}
      data-name={props.title}
      href="#"
      onClick={props.onClick}
    >
      {<props.icon style={menuIconStyle} title={props.title} />}
    </a>
  );
};
