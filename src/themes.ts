import { Theme } from "react-select";

const selectTheme = (theme: Theme): Theme => ({
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

export default selectTheme;
