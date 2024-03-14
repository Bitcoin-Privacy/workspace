import { menuAnatomy } from "@chakra-ui/anatomy";
import { createMultiStyleConfigHelpers } from "@chakra-ui/react";

const { definePartsStyle, defineMultiStyleConfig } =
  createMultiStyleConfigHelpers(menuAnatomy.keys);

const baseStyle = definePartsStyle({
  button: {
    fontWeight: "medium",
    bg: "text.primary",
    p: "8px",
    borderRadius: "6px",
    color: "bg.primary",
    _hover: {
      opacity: "0.7",
    },
  },
  list: {
    py: "4",
    borderRadius: "xl",
    border: "none",
    bg: "bg.modal",
  },
  item: {
    color: "text.primary",
    bg: "transparent",
    fontSize: "14px",
    fontWeight: "500",
    _hover: {
      bg: "bg.primary",
    },
    _focus: {
      bg: "bg.primary",
    },
  },
});
// export the base styles in the component theme
export const menuTheme = defineMultiStyleConfig({ baseStyle });
