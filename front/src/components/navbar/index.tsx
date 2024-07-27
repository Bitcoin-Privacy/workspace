"use client";

import {
  // Box,
  Flex,
  Text,
  Button,
  HStack,
  Box,
  // Menu,
  // MenuButton,
  // MenuList,
  // MenuItem,
  // useColorModeValue,
  // HStack,
  // Spacer,
  // MenuGroup,
} from "@chakra-ui/react";
import { useRouter } from "next/router";
// import { AiOutlineMenu, AiOutlinePlus } from "react-icons/ai";
// import { BsChevronDown } from "react-icons/bs";
// import { IconButton } from "@chakra-ui/react";
import { FiArrowLeft } from "react-icons/fi";
import { memo } from "react";

// const NavLink = (props: Props) => {
//   const { children } = props;
//
//   return (
//     <Box
//       as="a"
//       px={2}
//       py={1}
//       rounded={"md"}
//       _hover={{
//         textDecoration: "none",
//         bg: useColorModeValue("gray.200", "gray.700"),
//       }}
//       href={"#"}
//     >
//       {children}
//     </Box>
//   );
// };

type NavBarProps = {
  title: string;
};

export const NavBar = memo((props: NavBarProps) => {
  const router = useRouter();

  return (
    <HStack
      p="18px 24px"
      w="full"
      h={12}
      alignItems="center"
      justifyContent="center"
      pos="relative"
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
      <Button
        variant="link"
        leftIcon={<FiArrowLeft />}
        color="#a6a6a6"
        onClick={() => router.back()}
        pos="absolute"
        left="0"
      >
        Home
      </Button>
      <Text color="white" fontWeight="700" fontSize="18px">
        {props.title}
      </Text>
      {/* <Flex w={'20%'}> */}
      {/*   <Menu placement="bottom"> */}
      {/*     <MenuButton */}
      {/*       as={Button} */}
      {/*       rounded={"full"} */}
      {/*       rightIcon={<BsChevronDown />} */}
      {/*       transition="all 0.2s" */}
      {/*       borderRadius="md" */}
      {/*       colorScheme="black" */}
      {/*       minW={0} */}
      {/*       _hover={{ bg: "gray.400" }} */}
      {/*     > */}
      {/*       Bitcoin Testnet */}
      {/*     </MenuButton> */}
      {/*     <MenuList minWidth="240px"> */}
      {/*       <MenuGroup title="Select a network"> */}
      {/*         <MenuItem>Bitcoin mainnet</MenuItem> */}
      {/*         <MenuItem>Bitcoin testnet</MenuItem> */}
      {/*         <MenuItem justifyContent={"center"}> */}
      {/*           <Button */}
      {/*             leftIcon={<AiOutlinePlus />} */}
      {/*             colorScheme="blue" */}
      {/*             w={"85%"} */}
      {/*           > */}
      {/*             Add network */}
      {/*           </Button> */}
      {/*         </MenuItem> */}
      {/*       </MenuGroup> */}
      {/*     </MenuList> */}
      {/*   </Menu> */}
      {/* </Flex> */}

      {/* <Flex w={'30%'}> */}
      {/*   <Menu placement="bottom"> */}
      {/*     <MenuButton */}
      {/*       as={Button} */}
      {/*       rounded={"full"} */}
      {/*       transition="all 0.2s" */}
      {/*       borderRadius="md" */}
      {/*       colorScheme="black" */}
      {/*       minW={0} */}
      {/*       w='full' */}
      {/*       _hover={{ bg: "gray.400" }} */}
      {/*     > */}
      {/*       <HStack justifyContent={'center'}> */}
      {/*         <Text> Account 1</Text> */}
      {/*         <BsChevronDown /> */}
      {/*       </HStack> */}
      {/*     </MenuButton> */}
      {/*     <MenuList minWidth="420px"> */}
      {/*       <MenuGroup title="Select an account"> */}
      {/*         <MenuItem justifyContent={"center"}> */}
      {/*           <Flex direction={"column"} w={"full"}> */}
      {/*             <Flex p="4"> */}
      {/*               <Text>Account</Text> */}
      {/*               <Spacer /> */}
      {/*               <Text> 0 BTC</Text> */}
      {/*             </Flex> */}
      {/*             <Flex p="4"> */}
      {/*               <Text>0x12312423123</Text> */}
      {/*               <Spacer /> */}
      {/*               <Text> 0.00 USD</Text> */}
      {/*             </Flex> */}
      {/*           </Flex> */}
      {/*         </MenuItem> */}
      {/*         <MenuItem justifyContent={"center"}> */}
      {/*           <Button */}
      {/*             leftIcon={<AiOutlinePlus />} */}
      {/*             colorScheme="blue" */}
      {/*             w={"85%"} */}
      {/*           > */}
      {/*             Add account or hardware wallet */}
      {/*           </Button> */}
      {/*         </MenuItem> */}
      {/*       </MenuGroup> */}
      {/*     </MenuList> */}
      {/*   </Menu> */}
      {/* </Flex> */}
      {/* <Flex w={'20%'} justifyContent={'flex-end'}> */}
      {/*   <IconButton */}
      {/*     aria-label="Menu" */}
      {/*     variant={"ghost"} */}
      {/*     bg="white" */}
      {/*     icon={<AiOutlineMenu */}
      {/*       onClick={() => router.back()} /> */}
      {/*     } */}
      {/*   /> */}
      {/* </Flex> */}
    </HStack>
  );
});

NavBar.displayName = "NavBar";
