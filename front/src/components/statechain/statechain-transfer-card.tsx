import { StatechainApi } from "@/apis";
import { CachePrefixKeys } from "@/consts";
import { StateCoinTransferDto } from "@/dtos";
import { useNoti, useProfilePage } from "@/hooks";
import {
  Avatar,
  Button,
  Flex,
  HStack,
  Text,
  VStack,
  useDisclosure,
} from "@chakra-ui/react";
import { FC, useCallback, useState } from "react";
import { useQueryClient } from "react-query";

interface IStateChainTransferCard {
  key: number;
  deriv: string;
  data: StateCoinTransferDto;
}

export const StateChainTransferCard: FC<IStateChainTransferCard> = (props) => {
  const { key, deriv, data } = props;
  const [isLoading, setIsVerifying] = useState<boolean>(false);
  const noti = useNoti();
  const queryClient = useQueryClient();

  const onVerifyBtnClick = useCallback(async () => {
    setIsVerifying(true);
    try {
      let res = await StatechainApi.verifyTransferStatecoin(
        deriv,
        data.transfer_message,
        data.auth_key,
      );
      console.log("Verify statecoin response:", res);
      noti.success("Verified!");
      queryClient.invalidateQueries([CachePrefixKeys.ListTrasferStatecoins]);
      queryClient.invalidateQueries([CachePrefixKeys.ListStatecoins]);
    } catch (e: any) {
      console.error("Verify statecoin error:", e);
      noti.error("Got an error!", e);
    } finally {
      setIsVerifying(false);
    }
  }, [deriv, noti, data, queryClient]);

  return (
    <HStack
      key={key}
      color="white"
      textAlign="start"
      w="90%"
      maxW="90%"
      bg="#3a3a3a"
      p="8px 16px"
      borderRadius="8px"
      dir="row"
      alignItems={"center"}
      spacing="8px"
    >
      <Avatar h="54px" w="54px" src="/statecoin-icon.png" />
      <Flex w="full" alignItems="center" justify="space-between">
        <VStack alignItems={"flex-start"} spacing="8px">
          <Text isTruncated maxW={"160px"} fontSize={"16"} fontWeight={"400"}>
            Authkey : {data.auth_key}
          </Text>
        </VStack>

        <VStack alignItems={"end"} spacing={"8px"} w="100%">
          <Button
            isLoading={isLoading}
            isDisabled={isLoading}
            onClick={onVerifyBtnClick}
          >
            Verify
          </Button>
        </VStack>
      </Flex>
    </HStack>
  );
};
