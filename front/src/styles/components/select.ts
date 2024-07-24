import { ChakraStylesConfig } from "chakra-react-select";

export const selectStyles = (width: string): ChakraStylesConfig => ({
  menuList: (provided) => ({
    ...provided,
    // ...bgThemeListSearch,
  }),
  menu: (provided) => ({
    ...provided,
    // ...bgThemeListSearch,
  }),
  inputContainer: (provided) => ({
    ...provided,
    fontSize: "14px",
    color: "white",
    textAlign: "start",
  }),
  dropdownIndicator: (provided) => ({
    ...provided,
    w: "80px",
  }),
  control: (provided) => ({
    ...provided,
    background: "transparent",
    fontSize: "12px",
    color: "textSloganHomepage",
  }),
  container: (provided) => ({
    ...provided,
    width: width,
  }),
  singleValue: (provided) => ({
    ...provided,
    fontSize: "14px",
    color: "white",
    textAlign: "start",
  }),
  placeholder: (provided) => ({
    ...provided,
    color: "#a6a6a6",
    fontSize: "14px",
    textAlign: "start",
  }),
});
