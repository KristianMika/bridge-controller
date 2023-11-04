import { components, OptionProps } from "react-select";
import styles from "./MultilineSelectOption.module.css";

export interface IMultilineSelectOption extends OptionProps<any, false, any> {
  data: {
    label: string;
    subLabel: string;
  };
}
export const MultilineSelectOption: React.FC<IMultilineSelectOption> = (
  props
) => {
  return (
    <components.Option {...props}>
      <div
        className={`${styles["option__label"]} ${styles["option__main-label"]}`}
      >
        {props.data.label}
      </div>
      <div
        className={`${styles["option__label"]} ${styles["option__sublabel"]}`}
      >
        {props.data.subLabel}
      </div>
    </components.Option>
  );
};
