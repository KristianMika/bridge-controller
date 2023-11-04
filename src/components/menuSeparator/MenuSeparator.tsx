import styles from "./MenuSeparator.module.css";
/**
 * Creates a separator between the main area and the bottom menu
 */
export const MenuSeparator: React.FC = () => {
  return <div className={styles["menu-separator"]}></div>;
};
