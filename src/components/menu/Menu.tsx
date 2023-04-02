import { BiBitcoin } from "react-icons/bi";
import { AiOutlineQuestion } from "react-icons/ai";
import { MenuItem } from "./menuItem/MenuItem";
import styles from "./Menu.module.css";
import { IconType } from "react-icons";
import { useState } from "react";

interface IMenuItem {
  title: string;
  icon: IconType;
}

/**
 * Bottom navigation menu that allows changing interface configurations
 */
export const Menu: React.FC = () => {
  const [selectedItem, setSelectedItem] = useState<string>();
  const onClick = (event: React.MouseEvent<HTMLAnchorElement>) => {
    setSelectedItem(event.currentTarget.dataset.name);
  };

  const menuItems: IMenuItem[] = [
    { title: "HWI", icon: BiBitcoin },
    { title: "FIDO", icon: AiOutlineQuestion },
    { title: "PKCS#11", icon: AiOutlineQuestion },
  ];

  return (
    <div className={styles.menu}>
      {menuItems.map((menuItem) => (
        <MenuItem
          title={menuItem.title}
          icon={menuItem.icon}
          isSelected={menuItem.title === selectedItem}
          onClick={onClick}
        />
      ))}
    </div>
  );
};
