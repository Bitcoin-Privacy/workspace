import React, { useMemo, useState } from "react";
import Head from "next/head";
import {
  Box,
  Text,
  VStack,
  Button,
  Input,
  HStack,
  Spacer,
  InputGroup,
  InputRightAddon,
  FormControl,
  FormErrorMessage,
  Flex,
  Square,
  Grid,
  Center,
  useClipboard,
} from "@chakra-ui/react";
import { useRouter } from "next/router";
import { useSendPage } from "@/hooks";
import { ChakraStylesConfig, Select } from "chakra-react-select";
import { FaLongArrowAltRight } from "react-icons/fa";
import { Layout, NavBar } from "@/components";
import { TxStrategyEnum } from "@/dtos";
import { FiCheck, FiCopy } from "react-icons/fi";

const INPUT_WIDTH = "75%";

type TransactionOption = {
  label: string;
  value: TxStrategyEnum;
};

export default function Deposit() {
  const router = useRouter();
  const {
    states: {
      deriv, form, isLoading, balanceQuery
    },
    methods: { handleFormSubmit },
  } = useSendPage();

  const chakraStyles: ChakraStylesConfig = useMemo(
    () => ({
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
        width: INPUT_WIDTH,
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
    }),
    [],
  );

  const {
    value: addr,
    setValue: setAddr,
    onCopy,
    hasCopied,
  } = useClipboard("tb1qajhanrjvkzr5ktpjwxtg82fax2emn5apmhjtff");


  const [hide, setHide] = useState(false);

  return (
    <React.Fragment>
      <Head>
        <title>Deposit Bitcoin</title>
      </Head>
      <Layout>
        <form onSubmit={handleFormSubmit}>
          <VStack p="0px 16px" spacing="20px">
            <HStack justify="start" w="100%">
              <NavBar title={"Account " + deriv.slice(0, deriv.indexOf("/"))} />
            </HStack>
            <Text color="white" fontWeight="700" fontSize="18px">
              Deposit bitcoin
            </Text>
            <VStack width="100%" maxW="500px" spacing="16px" >
             
              <FormControl isInvalid={!!form.formState.errors.amount} >
                <VStack spacing = '24px'>
                <HStack w="full" justify="space-between">
                  <Text color={"white"}>Amount:</Text>
                  <InputGroup w={INPUT_WIDTH}>
                    <Input
                      placeholder="0.12"
                      color={"white"}
                      {...form.register("amount", {
                        required: "Amount is required",
                        pattern: { value: /^[0-9]+(?:\.[0-9]{0,8})?$/, message: "Amount should be a floating-point number with at most 8 decimal places." },
                        max: { value: balanceQuery.data ? balanceQuery.data / 10000000 : Number.MAX_VALUE, message: "Balance is not enough" },
                        min: { value: 0.00000001, message: "Amount must larget than or equal to 1 sat" }
                      })}
                    />
                    <InputRightAddon w="82px" justifyContent="center">
                      BTC
                    </InputRightAddon>
                  </InputGroup>
                </HStack>
                <Button onClick={() => {setHide(!hide)}}>Confirm</Button>
                {form.formState.errors.amount && (
                  <FormErrorMessage justifyContent="end">
                    {form.formState.errors.amount.message}
                  </FormErrorMessage>
                )}
                </VStack>
              </FormControl>
              
            </VStack>
            {hide && (
                <VStack  bg={"gray.900"} borderRadius={"8px"} p = "20px 40px"  w = 'full' spacing= '24px' color={"white"}>
                    <HStack w ='full' alignItems={'end'} >
                        <Square bg ='red'  size = "100px">
                            QR
                        </Square>
                        <VStack  w = 'full'  alignItems={'center'} p = '0px 16px' spacing= '16px'>
                            <Text> The address below is the Multisig Address between you and SE</Text>
                            <HStack spacing ='8px'>
                                <Center borderRadius={"16"} bg ="gray.700" p = "10px 15px"> 0.001 BTC</Center>
                                <Center>
                                <FaLongArrowAltRight size = '40px'/>
                                </Center>
                                
                                <Button
                                    onClick={onCopy}
                                    bgColor={"gray.700"}
                                    rightIcon={hasCopied ? <FiCheck /> : <FiCopy />}
                                    borderRadius={"16"}
                                >
                                <Text color={"white"} isTruncated  p="5px">
                                    {addr}
                                </Text>
                                </Button>
                            </HStack>
                        </VStack>
                    </HStack>
                    <Button>Send</Button>
                </VStack>
            ) }

           

          </VStack>
        </form>
      </Layout>
    </React.Fragment>
  );
}
