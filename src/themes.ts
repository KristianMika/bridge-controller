import { CSSProperties } from "react";
import { StylesConfig, Theme } from "react-select";
import IOption from "./models/IOption";

export const primaryColor = "#00e4d4";

export const selectTheme = (theme: Theme): Theme => ({
  ...theme,
  borderRadius: 0,
  colors: {
    ...theme.colors,
    primary: "#00e4d4",
    primary25: "#defcfa",
    primary50: "#8ffff8",
    primary75: "#3bf5e9",
  },
});

export const disabledSelectStyles: CSSProperties = {
  background: "rgb(192, 192, 192)",
  borderColor: "rgb(192, 192, 192)",
  color: "rgba(0, 0, 0, 0.4)",
};

export const selectStyle: StylesConfig<IOption, false> = {
  control: (provided, state) => {
    provided.borderRadius = 0;
    if (state.isDisabled) {
      return {
        ...provided,
        ...disabledSelectStyles,
      };
    }
    return provided;
  },
};
