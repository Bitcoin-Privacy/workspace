import {
  Drawer,
  DrawerBody,
  DrawerHeader,
  DrawerOverlay,
  DrawerContent,
  useDisclosure,
  IconButton,
  Text,
  HStack,
} from "@chakra-ui/react";
import React from "react";

import {
  MdKeyboardDoubleArrowRight,
  MdKeyboardDoubleArrowLeft,
} from "react-icons/md";

export default function CustomDrawer() {
  const { isOpen, onOpen, onClose } = useDisclosure();

  return (
    <>
      <IconButton
        colorScheme="blue"
        size="md"
        aria-label="Send Button"
        icon={<MdKeyboardDoubleArrowRight />}
        onClick={onOpen}
      />
      <Drawer placement="left" size="xs" onClose={onClose} isOpen={isOpen}>
        <DrawerOverlay />
        <DrawerContent>
          <DrawerHeader borderBottomWidth="1px">
            <HStack alignItems={"center"} justifyContent={"space-between"}>
              <Text> OPTION</Text>
              <IconButton
                colorScheme="gray"
                size="md"
                aria-label="Send Button"
                icon={<MdKeyboardDoubleArrowLeft />}
                onClick={onClose}
              />
            </HStack>
          </DrawerHeader>
          <DrawerBody>
            <p>Account details</p>
            <p>View on explorer</p>
            <p>Connected sites</p>
            <p>Support</p>
            <p>Settings</p>
            <p>Lock MetaMask</p>
          </DrawerBody>
        </DrawerContent>
      </Drawer>
    </>
  );
}
