import { BsFillUsbDriveFill, BsSimFill } from "react-icons/bs";
import { SiFidoalliance } from "react-icons/si";
import styles from "./Menu.module.css";
import { useState } from "react";
import { useNavigate } from "react-router-dom";
import IMenuItem from "../../models/IMenuItem";
import MenuItem from "./menuItem/MenuItem";

/**
 * Bottom navigation menu that allows changing interface configurations
 */
const Menu: React.FC = () => {
  const [selectedItem, setSelectedItem] = useState<string>("FIDO");
  const navigate = useNavigate();
  const onClick = (event: React.MouseEvent<HTMLAnchorElement>) => {
    event.preventDefault();
    setSelectedItem(event.currentTarget.dataset.name as string);
    navigate(event.currentTarget.dataset.link as string);
  };

  const menuItems: IMenuItem[] = [
    { title: "FIDO", icon: SiFidoalliance, link: "/webauthn" },
    { title: "PKCS#11", icon: BsFillUsbDriveFill, link: "/cryptoki" },
    { title: "PC/SC", icon: BsSimFill, link: "/pcsc" },
  ];

  return (
    <div className={styles["menu-wrapper"]}>
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

export default Menu;
