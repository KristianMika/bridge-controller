import styles from "./MenuSeparator.module.css";
/**
 * Creates a separator between the main area and the bottom menu
 */
const MenuSeparator: React.FC = () => {
  return <div className={styles["menu-separator"]}></div>;
};
export default MenuSeparator;
