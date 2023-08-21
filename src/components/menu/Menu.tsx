import { AiOutlineQuestion } from "react-icons/ai";
import {
  BsFillUsbDriveFill,
  BsSimFill,
  BsCurrencyBitcoin,
} from "react-icons/bs";
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
    { title: "HWI", icon: BsCurrencyBitcoin },
    { title: "PKCS#11", icon: BsFillUsbDriveFill },
    { title: "PC/SC", icon: BsSimFill },
    { title: "FIDO", icon: AiOutlineQuestion },
  ];

  return (
    <div className={styles.menu_wrapper}>
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
    </div>
  );
};
