import { extendTheme } from "@chakra-ui/react";
import { menuTheme } from "./menu";

const theme = extendTheme({
  components: {
    Menu: menuTheme,
  },
  colors: {
    text: {
      primary: "#eee",
      secondary: "#a6a6a6",
    },
    bg: {
      primary: "#1e1e1e",
      secondary: "#141414",
      modal: "#3a3a3a",
    },
  },
});

export default theme;
