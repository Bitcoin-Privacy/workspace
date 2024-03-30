import { useApp, useLogout } from "@/hooks";
import {
  Box,
  HStack,
  Menu,
  MenuButton,
  MenuItem,
  MenuList,
  Text,
  VStack,
} from "@chakra-ui/react";
import { useRouter } from "next/router";
import React from "react";
import { FiMenu } from "react-icons/fi";

interface ILayout {
  title?: string;
  type?: string;
  children: React.ReactNode;
}

export const Layout: React.FC<ILayout> = ({ title, children }) => {
  const router = useRouter();
  const {
    method: { logout },
  } = useLogout();

  return (
    <VStack
      h="100vh"
      w="100%"
      bg="transparent"
      overflowY="auto"
      alignItems="start"
      position="relative"
    >
      <Box
        top="-50px"
        bottom="-50px"
        left="-50px"
        right="-50px"
        bg="bg.secondary"
        zIndex="-1"
        position="fixed"
      />
      {/* Header */}
      {title && (
        <HStack
          fontSize="10"
          color="white"
          justify="space-between"
          w="100%"
          p="10px 20px"
          bg="bg.primary"
          position="sticky"
          top="0"
        >
          <Text fontSize="18px" fontWeight="700">
            {title}
          </Text>
          <Menu>
            <MenuButton>
              <FiMenu size="20px" />
            </MenuButton>
            <MenuList>
              <MenuItem>Change Password</MenuItem>
              <MenuItem onClick={logout}>Log out</MenuItem>
            </MenuList>
          </Menu>
        </HStack>
      )}
      <Box w="100%">{children}</Box>
    </VStack>
  );
};
