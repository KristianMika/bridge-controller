import {
  BsFillUsbDriveFill,
  BsSimFill,
  BsCurrencyBitcoin,
} from "react-icons/bs";
import { SiFidoalliance } from "react-icons/si";
import { MenuItem } from "./menuItem/MenuItem";
import styles from "./Menu.module.css";
import { IconType } from "react-icons";
import { useState } from "react";
import { useNavigate } from "react-router-dom";

interface IMenuItem {
  title: string;
  icon: IconType;
  link: string;
}

/**
 * Bottom navigation menu that allows changing interface configurations
 */
export const Menu: React.FC = () => {
  const [selectedItem, setSelectedItem] = useState<string>();
  const navigate = useNavigate();
  const onClick = (event: React.MouseEvent<HTMLAnchorElement>) => {
    event.preventDefault();
    setSelectedItem(event.currentTarget.dataset.name);
    navigate(event.currentTarget.dataset.link as string);
  };

  const menuItems: IMenuItem[] = [
    { title: "FIDO", icon: SiFidoalliance, link: "/webauthn" },
    { title: "PKCS#11", icon: BsFillUsbDriveFill, link: "/cryptoki" },
    { title: "PC/SC", icon: BsSimFill, link: "/pcsc" },
    { title: "HWI", icon: BsCurrencyBitcoin, link: "/hwi" },
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
            link={menuItem.link}
            key={menuItem.title}
          />
        ))}
      </div>
    </div>
  );
};
