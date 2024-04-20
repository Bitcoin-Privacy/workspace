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
import React, { useMemo } from "react";
import { FiMenu } from "react-icons/fi";
import { useLogout } from "@/hooks";

interface ILayout {
  title?: string | React.JSX.Element;
  showHeader?: boolean;
  children: React.ReactNode;
}

export const Layout: React.FC<ILayout> = ({
  showHeader = true,
  title,
  children,
}) => {
  const {
    method: { logout },
  } = useLogout();

  const titleComponent = useMemo(() => {
    if (!title) return <Box />;
    else if (typeof title === "string")
      return (
        <Text fontSize="18px" fontWeight="700">
          {title}
        </Text>
      );
    else return title;
  }, [title]);

  return (
    <VStack
      h="100vh"
      w="100%"
      spacing="0"
      overflowY="auto"
      alignItems="center"
      position="relative"
    >
      <Box
        bg="bg.secondary"
        top="-50px"
        bottom="-50px"
        left="-50px"
        right="-50px"
        zIndex="-1"
        position="fixed"
      />
      {showHeader && (
        <>
          {/* Header */}
          <HStack
            color="white"
            justify="space-between"
            w="100%"
            p="10px 20px"
            bg="bg.primary"
            position="sticky"
            top="0"
            zIndex={100}
          >
            {titleComponent}
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
        </>
      )}
      <Box w="100%" maxW="1200px" p="24px">
        {children}
      </Box>
    </VStack>
  );
};
